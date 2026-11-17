use roxmltree::{Document, Node, ParsingOptions};
use std::collections::HashMap;
use std::fmt::Write as _;

const LEAF: u16 = 0x8000;
const FLAT_MAX_BITS: u32 = 16; // up to this: one flat array
const CHUNK_BITS: u32 = 8; // wider groups: trie with 8-bit levels
const MAX_BITS: u32 = 28;

struct Field {
    name: String,
    lo: u32,
    width: u32,
}

struct Cell {
    mask: u32,
    val: u32,
    neg: bool,
}

struct Classifier {
    fields: Vec<Field>,
    shifts: Vec<u32>,
    total: u32,
    variants: Vec<String>,
    kids: Vec<(usize, Vec<(usize, Cell)>)>,
}

struct Trie {
    chunks: Vec<u32>,
    levels: Vec<Vec<Vec<u16>>>,
    dedup: Vec<HashMap<Vec<u16>, u16>>,
}

// ---------- XML helpers ----------

fn attr_u32(n: Node, a: &str) -> u32 {
    n.attribute(a)
        .unwrap_or_else(|| panic!("missing attr {a}"))
        .parse()
        .unwrap()
}

fn elem<'a, 'i>(n: Node<'a, 'i>, tag: &str) -> Option<Node<'a, 'i>> {
    n.children().find(|c| c.has_tag_name(tag))
}

fn camel(s: &str) -> String {
    let mut out: String = s
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    if out.chars().next().map_or(true, |c| c.is_ascii_digit()) {
        out.insert(0, 'V');
    }
    out
}

// Index fields = named boxes with usename="1" in the node's own regdiagram, MSB first.
fn index_fields(parent: Node) -> Vec<Field> {
    let rd = elem(parent, "regdiagram").expect("node has no regdiagram");
    rd.children()
        .filter(|b| b.has_tag_name("box") && b.attribute("usename") == Some("1"))
        .filter_map(|b| {
            let name = b.attribute("name")?;
            let hi = attr_u32(b, "hibit");
            let w = attr_u32(b, "width");
            Some(Field {
                name: name.to_string(),
                lo: hi + 1 - w,
                width: w,
            })
        })
        .collect()
}

// One <box> in a <decode>: concat <c> cells, empty cell = 'x' * colspan, "!=" negates.
fn parse_box(b: Node, width: u32) -> Cell {
    let mut pat = String::new();
    let mut neg = false;
    for c in b.children().filter(|c| c.has_tag_name("c")) {
        let mut t = c.text().unwrap_or("").trim();
        if let Some(rest) = t.strip_prefix("!=") {
            neg = true;
            t = rest.trim();
        }
        if t.is_empty() {
            let span: usize = c.attribute("colspan").map_or(1, |s| s.parse().unwrap());
            pat.extend(std::iter::repeat('x').take(span));
        } else {
            pat.push_str(t);
        }
    }
    assert_eq!(
        pat.len() as u32,
        width,
        "pattern {pat:?} doesn't match width {width}"
    );
    let (mut mask, mut val) = (0u32, 0u32);
    for ch in pat.chars() {
        mask <<= 1;
        val <<= 1;
        match ch {
            '0' => mask |= 1,
            '1' => {
                mask |= 1;
                val |= 1;
            }
            'x' => {}
            _ => panic!("bad pattern char {ch:?} in {pat:?}"),
        }
    }
    Cell { mask, val, neg }
}

// ---------- classification ----------

fn classify_node(parent: Node) -> Classifier {
    let fields = index_fields(parent);
    let total: u32 = fields.iter().map(|f| f.width).sum();
    assert!(
        total <= MAX_BITS,
        "{total} index bits is too many even for the trie"
    );
    let shifts: Vec<u32> = (0..fields.len())
        .map(|i| fields[i + 1..].iter().map(|f| f.width).sum())
        .collect();

    let mut variants: Vec<String> = Vec::new();
    let mut kids = Vec::new();
    for kid in parent.children().filter(|n| n.has_tag_name("node")) {
        let name = if kid.attribute("unallocated") == Some("1") {
            "Unallocated".to_string()
        } else {
            camel(
                kid.attribute("groupname")
                    .or(kid.attribute("iclass"))
                    .expect("unnamed node"),
            )
        };
        let v = match variants.iter().position(|x| *x == name) {
            Some(i) => i,
            None => {
                variants.push(name.clone());
                variants.len() - 1
            }
        };
        let dec = elem(kid, "decode").unwrap_or_else(|| panic!("{name} has no <decode>"));
        let mut cells = Vec::new();
        for b in dec.children().filter(|b| b.has_tag_name("box")) {
            let bname = b.attribute("name").unwrap_or("");
            let fi = fields
                .iter()
                .position(|f| f.name == bname)
                .unwrap_or_else(|| panic!("{name}: decode box {bname:?} isn't an index field"));
            let w = attr_u32(b, "width");
            assert_eq!(w, fields[fi].width, "{name}: width mismatch on {bname}");
            cells.push((fi, parse_box(b, w)));
        }
        kids.push((v, cells));
    }
    Classifier {
        fields,
        shifts,
        total,
        variants,
        kids,
    }
}

impl Classifier {
    fn class_of(&self, idx: u32) -> usize {
        let mut found = None;
        for (v, cells) in &self.kids {
            let ok = cells.iter().all(|(fi, c)| {
                let w = self.fields[*fi].width;
                let x = (idx >> self.shifts[*fi]) & ((1 << w) - 1);
                ((x & c.mask) == c.val) != c.neg
            });
            if ok {
                assert!(
                    found.is_none(),
                    "index {idx:#b} matched more than one child"
                );
                found = Some(*v);
            }
        }
        found.unwrap_or_else(|| panic!("index {idx:#b} matched no child"))
    }
}

// ---------- trie ----------

fn trie_build(cl: &Classifier, t: &mut Trie, depth: usize, base: u32, rem: u32) -> u16 {
    let w = t.chunks[depth];
    let rem = rem - w;
    let mut table = Vec::with_capacity(1 << w);
    for v in 0..(1u32 << w) {
        let b = base | (v << rem);
        table.push(if rem == 0 {
            LEAF | cl.class_of(b) as u16
        } else {
            trie_build(cl, t, depth + 1, b, rem)
        });
    }
    if table[0] & LEAF != 0 && table.iter().all(|&e| e == table[0]) {
        return table[0]; // whole subtree is one class
    }
    if let Some(&id) = t.dedup[depth].get(&table) {
        return id;
    }
    let id = t.levels[depth].len() as u16;
    t.levels[depth].push(table.clone());
    t.dedup[depth].insert(table, id);
    id
}

// ---------- emit ----------

fn emit_head(out: &mut String, prefix: &str, cl: &Classifier) -> String {
    let ty = format!("{}Class", camel(prefix));
    writeln!(out, "#[derive(Clone, Copy, Debug, PartialEq, Eq)]").unwrap();
    writeln!(out, "pub enum {ty} {{").unwrap();
    for v in &cl.variants {
        writeln!(out, "    {v},").unwrap();
    }
    writeln!(out, "}}\n").unwrap();

    let terms: Vec<String> = cl
        .fields
        .iter()
        .zip(&cl.shifts)
        .map(|(f, s)| {
            let mask = (1u32 << f.width) - 1;
            let src = if f.lo == 0 {
                "insn".to_string()
            } else {
                format!("(insn >> {})", f.lo)
            };
            if *s == 0 {
                format!("{src} & {mask:#x}")
            } else {
                format!("({src} & {mask:#x}) << {s}")
            }
        })
        .collect();
    writeln!(out, "#[inline(always)]").unwrap();
    writeln!(out, "pub const fn {prefix}_index(insn: u32) -> usize {{").unwrap();
    writeln!(out, "    ({}) as usize", terms.join(" | ")).unwrap();
    writeln!(out, "}}\n").unwrap();
    ty
}

fn emit_flat(out: &mut String, prefix: &str, cl: &Classifier) {
    let ty = emit_head(out, prefix, cl);
    let n = 1u32 << cl.total;
    writeln!(
        out,
        "pub static DECODE_{}: [{ty}; {n}] = [",
        prefix.to_uppercase()
    )
    .unwrap();
    for idx in 0..n {
        let v = &cl.variants[cl.class_of(idx)];
        writeln!(
            out,
            "    {ty}::{v}, /* {:0w$b} */",
            idx,
            w = cl.total as usize
        )
        .unwrap();
    }
    writeln!(out, "];\n").unwrap();
}

fn emit_wide(out: &mut String, prefix: &str, cl: &Classifier) {
    let mut chunks = Vec::new();
    let mut r = cl.total;
    while r > 0 {
        let w = r.min(CHUNK_BITS);
        chunks.push(w);
        r -= w;
    }
    let mut t = Trie {
        levels: vec![Vec::new(); chunks.len()],
        dedup: vec![HashMap::new(); chunks.len()],
        chunks,
    };
    let top = trie_build(cl, &mut t, 0, 0, cl.total);
    assert_eq!(
        top, 0,
        "trie root collapsed to a single class, which shouldn't happen here"
    );

    let ty = emit_head(out, prefix, cl);
    let up = prefix.to_uppercase();

    writeln!(
        out,
        "static {up}_CLASSES: [{ty}; {}] = [",
        cl.variants.len()
    )
    .unwrap();
    for v in &cl.variants {
        writeln!(out, "    {ty}::{v},").unwrap();
    }
    writeln!(out, "];\n").unwrap();

    for (d, lvl) in t.levels.iter().enumerate() {
        writeln!(
            out,
            "static {up}_L{d}: [[u16; {}]; {}] = [",
            1usize << t.chunks[d],
            lvl.len()
        )
        .unwrap();
        for tab in lvl {
            writeln!(out, "    {tab:?},").unwrap();
        }
        writeln!(out, "];\n").unwrap();
    }

    writeln!(out, "#[inline]").unwrap();
    writeln!(out, "pub fn decode_{prefix}(insn: u32) -> {ty} {{").unwrap();
    writeln!(out, "    let i = {prefix}_index(insn);").unwrap();
    writeln!(out, "    let mut e: u16 = 0;").unwrap();
    let mut shift = cl.total;
    for (d, w) in t.chunks.iter().enumerate() {
        shift -= w;
        let src = if shift == 0 {
            "i".to_string()
        } else {
            format!("(i >> {shift})")
        };
        writeln!(
            out,
            "    e = {up}_L{d}[e as usize][{src} & {:#x}];",
            (1u32 << w) - 1
        )
        .unwrap();
        if d + 1 < t.chunks.len() {
            writeln!(
                out,
                "    if e & 0x8000 != 0 {{ return {up}_CLASSES[(e & 0x7fff) as usize]; }}"
            )
            .unwrap();
        }
    }
    writeln!(out, "    {up}_CLASSES[(e & 0x7fff) as usize]").unwrap();
    writeln!(out, "}}\n").unwrap();

    let tables: usize = t.levels.iter().map(|l| l.len()).sum();
    eprintln!(
        "  {prefix}: trie with {} levels, {tables} unique tables",
        t.chunks.len()
    );
}

// ---------- main ----------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: gen_decode <encodingindex.xml> <out.rs>");
        std::process::exit(1);
    }
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let doc = Document::parse_with_options(
        &text,
        ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )
    .unwrap();
    let root = doc
        .descendants()
        .find(|n| n.has_tag_name("hierarchy"))
        .unwrap();

    let mut out = String::from("// @generated from Arm's encodingindex.xml, do not edit\n\n");

    let rc = classify_node(root);
    eprintln!("root: {} index bits", rc.total);
    emit_flat(&mut out, "root", &rc);

    for g in ["dpimm", "control", "dpreg", "simd_dp", "ldst"] {
        let node = root
            .children()
            .find(|n| n.has_tag_name("node") && n.attribute("groupname") == Some(g))
            .unwrap_or_else(|| panic!("no top-level group {g}"));
        let cl = classify_node(node);
        eprintln!("{g}: {} index bits", cl.total);
        if cl.total <= FLAT_MAX_BITS {
            emit_flat(&mut out, g, &cl);
        } else {
            emit_wide(&mut out, g, &cl);
        }
    }
    std::fs::write(&args[2], out).unwrap();
}
