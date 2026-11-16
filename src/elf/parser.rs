/*
byte offset
0x0000  ┌──────────────────────────────────┐
        │ ELF Header (64 bytes)            │── has e_shoff, e_shnum, e_shstrndx
        ├──────────────────────────────────┤
        │ Program Header Table             │── execution view (segments), loader's concern
        ├──────────────────────────────────┤
        │ .text     (machine code)         │◄──────────┐
        │ .rodata   (constants)            │◄──────────┤
        │ .data     (initialized globals)  │◄──────────┤  each entry's sh_offset
        │ .bss      (NOBITS: no bytes!)    │◄╌╌╌╌╌╌╌╌╌╌┤  points back into the body
        │ .symtab / .strtab (symbols)      │◄──────────┤
        │ .debug_*  (if not stripped)      │◄──────────┘
        ├──────────────────────────────────┤
        │ SECTION HEADER TABLE             │◄── e_shoff points here
        │   [0] NULL (always empty)        │
        │   [1] .text   → sh_offset, size  │
        │   [2] .rodata → ...              │
        │   [n] .shstrtab ◄── e_shstrndx   │
        ├──────────────────────────────────┤
        │ .shstrtab: ".text\0.rodata\0..." │── usually the very last bytes
        └──────────────────────────────────┘
        to access .text we read section header table find .text either by name or flags and etc
        then we do bytes[sh_offset..sh_offset + sh_size] to get contents
*/

use std::{ffi::CStr, io::Read, vec};

use crate::{
    elf::types::{
        EI_CLASS, EI_DATA, EI_NIDENT, EI_PAD, EI_VERSION, ELF_MAGIC, ELF64_DYN_HDR_LEN,
        ELF64_HDRLEN, ELF64_PHDR_LEN, ELF64_SECTION_HEADER_LEN, ELF64_SYM_LEN, ELFCLASS64,
        ELFDATALSB, ELFDATAMSB, EV_CURRENT, Elf64Dyn, Elf64Ehdr, Elf64Phdr, Elf64Shdr, Elf64Sym,
        SHN_UNDEF, dt_tag, program_header_type, sh_type,
    },
    error::ElfError,
};

macro_rules! elf_read_int {
    ($bytes:expr, $offset:expr, $e_data:expr, u16) => {
        match $e_data {
            #[allow(non_snake_case)]
            ELFDATALSB => {
                u16::from_le_bytes(<[u8; 2]>::try_from(&$bytes[$offset..$offset + 2]).unwrap())
            }
            #[allow(non_snake_case)]
            ELFDATAMSB => {
                u16::from_be_bytes(<[u8; 2]>::try_from(&$bytes[$offset..$offset + 2]).unwrap())
            }
            _ => unreachable!(),
        }
    };
    ($bytes:expr, $offset:expr, $e_data:expr, u32) => {
        match $e_data {
            #[allow(non_snake_case)]
            ELFDATALSB => {
                u32::from_le_bytes(<[u8; 4]>::try_from(&$bytes[$offset..$offset + 4]).unwrap())
            }
            #[allow(non_snake_case)]
            ELFDATAMSB => {
                u32::from_be_bytes(<[u8; 4]>::try_from(&$bytes[$offset..$offset + 4]).unwrap())
            }
            _ => unreachable!(),
        }
    };
    ($bytes:expr, $offset:expr, $e_data:expr, u64) => {
        match $e_data {
            #[allow(non_snake_case)]
            ELFDATALSB => {
                u64::from_le_bytes(<[u8; 8]>::try_from(&$bytes[$offset..$offset + 8]).unwrap())
            }
            #[allow(non_snake_case)]
            ELFDATAMSB => {
                u64::from_be_bytes(<[u8; 8]>::try_from(&$bytes[$offset..$offset + 8]).unwrap())
            }
            _ => unreachable!(),
        }
    };

    ($bytes:expr, $offset:expr, $e_data:expr, i64) => {
        match $e_data {
            #[allow(non_snake_case)]
            ELFDATALSB => {
                i64::from_le_bytes(<[u8; 8]>::try_from(&$bytes[$offset..$offset + 8]).unwrap())
            }
            #[allow(non_snake_case)]
            ELFDATAMSB => {
                i64::from_be_bytes(<[u8; 8]>::try_from(&$bytes[$offset..$offset + 8]).unwrap())
            }
            _ => unreachable!(),
        }
    };
}

#[derive(Debug)]
pub struct Section<'a> {
    pub name: &'a str,
    pub header: Elf64Shdr,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Elf<'a> {
    pub header: Elf64Ehdr,
    pub sections: Vec<Section<'a>>,
    pub programs: Vec<Elf64Phdr>,
    raw: &'a [u8],
}

// (todo): parse .dynsym and .got, .plt, reloc
impl<'a> Elf<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, ElfError> {
        if bytes.len() < ELF64_HDRLEN {
            return Err(ElfError::InvalidHeader("Header size must be 64-bytes"));
        }

        /*
         * definition is:
         * offset     size      field
         *  0x00       4         EI_MAG0..EI_MAG3
         *  0x04       1         EICLASS
         *  0x05       1         EIDATA (endianness)
         *  0x06       1         EI_VERSION must be 1
         *  0x07       1         EI_OS_ABI
         *  0x08       1         EI_OS_ABIVERSION
         *  0x09       1         EI_PAD must be 0
         *  ...
         *   0x0F      5         padding must be 0
         */
        let e_ident = &bytes[0..EI_NIDENT];

        let e_data = e_ident[EI_DATA];

        if e_data != ELFDATALSB && e_data != ELFDATAMSB {
            return Err(ElfError::UnsupportedEndian(e_data));
        }

        // after EI_NIDENT
        let rest = &bytes[EI_NIDENT..];

        let elf_hdr = Elf64Ehdr {
            e_ident: e_ident.try_into().unwrap(),
            e_type: elf_read_int!(rest, 0, e_data, u16),
            e_machine: elf_read_int!(rest, 2, e_data, u16),
            e_version: elf_read_int!(rest, 4, e_data, u32),
            e_entry: elf_read_int!(rest, 8, e_data, u64),
            e_phoff: elf_read_int!(rest, 16, e_data, u64),
            e_shoff: elf_read_int!(rest, 24, e_data, u64),
            e_flags: elf_read_int!(rest, 32, e_data, u32),
            e_ehsize: elf_read_int!(rest, 36, e_data, u16),
            e_phentsize: elf_read_int!(rest, 38, e_data, u16),
            e_phnum: elf_read_int!(rest, 40, e_data, u16),
            e_shentsize: elf_read_int!(rest, 42, e_data, u16),
            e_shnum: elf_read_int!(rest, 44, e_data, u16),
            e_shstrndx: elf_read_int!(rest, 46, e_data, u16),
        };

        if elf_hdr.e_ident[0..4] != ELF_MAGIC {
            return Err(ElfError::InvalidMagic);
        }

        let e_class = elf_hdr.e_ident[EI_CLASS];

        // (note): do not support elf32 will see about this later
        if e_class != ELFCLASS64 {
            return Err(ElfError::UnsupportedClass(e_class));
        }

        if elf_hdr.e_ident[EI_VERSION] != EV_CURRENT {
            return Err(ElfError::UnsupportedVersion(bytes[EI_VERSION] as u32));
        }

        if elf_hdr.e_ident[EI_PAD..EI_NIDENT].iter().any(|&b| b != 0) {
            return Err(ElfError::InvalidHeader("EI_PAD must be zero"));
        }

        if elf_hdr.e_version != EV_CURRENT as u32 {
            return Err(ElfError::UnsupportedVersion(elf_hdr.e_version));
        }

        let mut elf = Self {
            header: elf_hdr,
            sections: vec![],
            programs: vec![],
            raw: bytes,
        };

        // ======================== program header ===================================
        let ph_offset = elf.header.e_phoff as usize;
        // number of program header table(s)
        let num_entries = elf.header.e_phnum as usize;
        // number of program header table entries * size of each entry in bytes
        let ph_size = num_entries * elf.header.e_phentsize as usize;

        if elf.header.e_phentsize as usize != ELF64_PHDR_LEN {
            return Err(ElfError::InvalidSectionHeader(
                "each ELF64_PHDR must be exactly 56-bytes",
            ));
        }

        let end = ph_offset
            .checked_add(ph_size)
            .ok_or(ElfError::IntegerOverflow)?;

        if end > bytes.len() {
            return Err(ElfError::InvalidProgramHeader(
                "mailformed header out of bounds",
            ));
        }

        let ph_table = &bytes[ph_offset..ph_offset + ph_size];

        elf.programs = ph_table
            .chunks_exact(elf.header.e_phentsize as usize)
            .map(|entry| Elf64Phdr {
                p_type: elf_read_int!(entry, 0, e_data, u32),
                p_flags: elf_read_int!(entry, 4, e_data, u32),
                p_offset: elf_read_int!(entry, 8, e_data, u64),
                p_vaddr: elf_read_int!(entry, 16, e_data, u64),
                p_paddr: elf_read_int!(entry, 24, e_data, u64),
                p_filesz: elf_read_int!(entry, 32, e_data, u64),
                p_memsz: elf_read_int!(entry, 40, e_data, u64),
                p_align: elf_read_int!(entry, 48, e_data, u64),
            })
            .collect::<Vec<Elf64Phdr>>();

        // ========================== section header =================================
        let sh_offset = elf.header.e_shoff as usize;
        let sh_size = elf.header.e_shnum as usize * elf.header.e_shentsize as usize;
        let entry_size = elf.header.e_shentsize as usize;

        if entry_size != ELF64_SECTION_HEADER_LEN {
            return Err(ElfError::InvalidSectionHeader(
                "entry size must be 64-bytes",
            ));
        }

        let end = sh_offset
            .checked_add(sh_size)
            .ok_or(ElfError::IntegerOverflow)?;

        if end > bytes.len() {
            return Err(ElfError::InvalidSectionHeader(
                "Section header table extends beyond file",
            ));
        }

        let sh_table = &bytes[sh_offset..sh_offset + sh_size];

        elf.sections = sh_table
            .chunks_exact(entry_size)
            .map(|entry| -> Result<Section, ElfError> {
                let header = Elf64Shdr {
                    // this is not actual name but offset into .shstrtab
                    sh_name: elf_read_int!(entry, 0, e_data, u32),
                    sh_type: elf_read_int!(entry, 4, e_data, u32),
                    sh_flags: elf_read_int!(entry, 8, e_data, u64),
                    sh_addr: elf_read_int!(entry, 16, e_data, u64),
                    sh_offset: elf_read_int!(entry, 24, e_data, u64),
                    sh_size: elf_read_int!(entry, 32, e_data, u64),
                    sh_link: elf_read_int!(entry, 40, e_data, u32),
                    sh_info: elf_read_int!(entry, 44, e_data, u32),
                    sh_addralign: elf_read_int!(entry, 48, e_data, u64),
                    sh_entsize: elf_read_int!(entry, 56, e_data, u64),
                };

                let off = header.sh_offset as usize;
                let size = header.sh_size as usize;

                let end = off.checked_add(size).ok_or(ElfError::IntegerOverflow)?;

                if end > bytes.len() {
                    return Err(ElfError::InvalidSectionHeader("mailformed section data"));
                }

                let data = &bytes[off..off + size];

                Ok(Section {
                    name: "",
                    header,
                    data,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // ====================== parse string table .shstrtab ==========================
        let string_header_idx = elf.header.e_shstrndx as usize;

        if string_header_idx > elf.sections.len() {
            return Err(ElfError::InvalidSectionHeader(
                "string header index overflows",
            ));
        }

        if !elf.sections.is_empty() && elf.header.e_shstrndx != SHN_UNDEF {
            // this section which contains the section header table index of the section
            let string_section = &elf.sections[string_header_idx];
            // where it is in file
            let offset = string_section.header.sh_offset as usize;
            // size of string section
            let size = string_section.header.sh_size as usize;

            // section names e.g., \0.text\0.data\0.bss\0
            let content = &bytes[offset..offset + size];

            // now loop over each section and read the name
            for section in &mut elf.sections {
                let name_offset = section.header.sh_name as usize;

                // each name string is null terminated
                let name_bytes = content[name_offset..]
                    .iter()
                    .position(|&b| b == 0)
                    .map(|end| &content[name_offset..name_offset + end])
                    .ok_or(ElfError::InvalidSectionHeader(
                        "string table is invalid no null termination",
                    ))?;

                section.name = core::str::from_utf8(name_bytes).map_err(|e| ElfError::Utf8(e))?;
            } /* for section */
        } /* if ... */

        return Ok(elf);
    }

    #[inline(always)]
    #[must_use]
    fn checked_region_end(&self, bytes_len: usize, offset: usize, size: usize) -> Option<usize> {
        offset.checked_add(size).filter(|&val| val <= bytes_len)
    }

    #[inline]
    pub fn symbol_table(&self) -> Option<Vec<Elf64Sym>> {
        let sym_section = self.sections.iter().find(|&section| {
            section.name == ".symtab" || section.header.sh_type == sh_type::SHT_SYMTAB
        })?;

        let offset = sym_section.header.sh_offset as usize;
        let size = sym_section.header.sh_size as usize;

        offset
            .checked_add(size)
            .filter(|&val| val <= self.raw.len())?;

        self.checked_region_end(self.raw.len(), offset, size)?;

        let contents = &self.raw[offset..offset + size];
        let e_data = self.header.e_ident[EI_DATA];

        Some(
            contents
                .chunks_exact(ELF64_SYM_LEN)
                .map(|entry| {
                    Elf64Sym {
                        st_name: elf_read_int!(entry, 0, e_data, u32),
                        st_info: entry[4], /* just one byte so we don't have to handle endianness */
                        st_other: entry[5],
                        st_shndx: elf_read_int!(entry, 6, e_data, u16),
                        st_value: elf_read_int!(entry, 8, e_data, u64),
                        st_size: elf_read_int!(entry, 16, e_data, u64),
                    }
                }) /* Iterator */
                .collect::<Vec<Elf64Sym>>(),
        ) /* Some */
    }

    #[inline]
    pub fn resolve_symbol_name(&self, sym: &Elf64Sym) -> Option<&str> {
        // symbol names are stored in .strtab
        // (note): investigate about sh_link and strtab_index
        let sec_strtab = self
            .sections
            .iter()
            .find(|&section| section.name == ".strtab")?;

        let offset = sec_strtab.header.sh_offset as usize;
        let size = sec_strtab.header.sh_size as usize;

        self.checked_region_end(self.raw.len(), offset, size)?;

        let contents = &self.raw[offset..offset + size];
        // where name is in contents
        let name_offset = sym.st_name as usize;

        if name_offset > contents.len() {
            return None;
        }

        CStr::from_bytes_until_nul(contents.get(name_offset..)?)
            .ok()?
            .to_str()
            .ok()
    }

    #[inline]
    pub fn dynamic_entries(&self) -> Option<Vec<Elf64Dyn>> {
        let dyn_hdr = self
            .programs
            .iter()
            .find(|&phdr| phdr.p_type == program_header_type::DYNAMIC)?;

        let e_data = self.header.e_ident[EI_DATA];

        let start = usize::try_from(dyn_hdr.p_offset).ok()?;
        let len = usize::try_from(dyn_hdr.p_filesz).ok()?;
        let bytes = self.raw.get(start..start.checked_add(len)?)?;

        Some(
            bytes
                .chunks_exact(ELF64_DYN_HDR_LEN)
                .filter_map(|entry| {
                    let tag = elf_read_int!(entry, 0, e_data, i64);
                    // ignore DT_NULL
                    if tag == dt_tag::DT_NULL {
                        None
                    } else {
                        Some(Elf64Dyn {
                            d_tag: elf_read_int!(entry, 0, e_data, i64),
                            d_val: elf_read_int!(entry, 8, e_data, u64),
                        })
                    }
                })
                .collect(),
        )
    }

    pub fn dyn_symbol_table(&self) -> Option<Vec<Elf64Sym>> {
        // .dynsym
        // [.dynsym]  <- array of Elf64_Sym entries (fixed size, indexable)
        // either we can find using section header or program header (PT_DYNAMIC)

        let dyn_sym = self
            .dynamic_entries()?
            .iter()
            .find(|&e| e.d_tag == dt_tag::DT_SYMTAB)
            .map(|e| e.d_val);

        let e_data = self.header.e_ident[EI_DATA];

        let parse_sym = |bytes: &[u8]| Elf64Sym {
            st_name: elf_read_int!(bytes, 0, e_data, u32),
            st_info: bytes[4],
            st_other: bytes[5],
            st_shndx: elf_read_int!(bytes, 6, e_data, u16),
            st_value: elf_read_int!(bytes, 8, e_data, u64),
            st_size: elf_read_int!(bytes, 16, e_data, u64),
        };

        let mut syms: Vec<Elf64Sym> = vec![];

        match dyn_sym {
            Some(d_val) => {
                // get file offset read symbol etc
                // d_val is a virtual address we need to convert it to file offset
                // find a program header whose interval [p_vaddr, p_vaddr + p_filesz) contains d_val
                // p_vaddr <= d_val < p_vaddr + p_filesz

                let load_seg = self.programs.iter().find(|&program| {
                    (program.p_type == program_header_type::LOAD)
                        && d_val >= program.p_vaddr
                        && d_val < program.p_vaddr + program.p_filesz
                })?;

                // file_offset = d_val - p_vaddr + p_offset
                let file_offset = d_val - load_seg.p_vaddr + load_seg.p_offset;

                todo!()
            }
            None => {
                // check section now
                let dyn_sec = self
                    .sections
                    .iter()
                    .find(|&section| section.header.sh_type == sh_type::SHT_DYNSYM)?;

                let offset = usize::try_from(dyn_sec.header.sh_offset).ok()?;
                let size = usize::try_from(dyn_sec.header.sh_size).ok()?;

                let bytes = self.raw.get(offset..offset.checked_add(size)?)?;

                syms = bytes
                    .chunks_exact(ELF64_SYM_LEN)
                    .map(|entry| parse_sym(entry))
                    .collect();

                return Some(syms);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elf::types::ELF64_SECTION_HEADER_LEN;

    const ELF64_HEADER_SIZE: usize = 64;
    const ELF64_PROGRAM_HEADER_SIZE: usize = ELF64_PHDR_LEN;
    const ELF64_SECTION_HEADER_SIZE: usize = ELF64_SECTION_HEADER_LEN;

    const PT_LOAD: u32 = 1;
    const PF_R: u32 = 4;
    const PF_X: u32 = 1;
    const SHT_PROGBITS: u32 = 1;

    // ---------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------

    fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }

    /// Creates a valid ELF64 little-endian executable.
    ///
    /// Contains:
    /// - `count` program headers (PT_LOAD)
    /// - One section header (SHT_PROGBITS)
    /// - 16 bytes of section data
    fn valid_elf64(count: usize) -> Vec<u8> {
        let phoff = if count > 0 { ELF64_HEADER_SIZE } else { 0 };

        let shoff = ELF64_HEADER_SIZE + count * ELF64_PROGRAM_HEADER_SIZE;

        // Align section data to 4096 bytes.
        let data_offset = (shoff + ELF64_SECTION_HEADER_SIZE + 0xFFF) & !0xFFF;

        let mut bytes = vec![0u8; data_offset + 16];

        // -----------------------------------------------------
        // ELF identification
        // -----------------------------------------------------

        bytes[0..4].copy_from_slice(b"\x7FELF");
        bytes[4] = 2; // ELFCLASS64
        bytes[5] = 1; // ELFDATA2LSB
        bytes[6] = 1; // EV_CURRENT
        bytes[7] = 0; // ELFOSABI_NONE

        // -----------------------------------------------------
        // ELF header
        // -----------------------------------------------------

        put_u16(&mut bytes, 16, 2); // ET_EXEC
        put_u16(&mut bytes, 18, 62); // EM_X86_64
        put_u32(&mut bytes, 20, 1); // EV_CURRENT

        put_u64(&mut bytes, 24, 0x400000); // e_entry
        put_u64(&mut bytes, 32, phoff as u64);
        put_u64(&mut bytes, 40, shoff as u64);

        put_u32(&mut bytes, 48, 0); // e_flags

        put_u16(&mut bytes, 52, 64); // e_ehsize
        put_u16(&mut bytes, 54, ELF64_PROGRAM_HEADER_SIZE as u16);
        put_u16(&mut bytes, 56, count as u16);
        put_u16(&mut bytes, 58, ELF64_SECTION_HEADER_SIZE as u16);
        put_u16(&mut bytes, 60, 1); // e_shnum
        put_u16(&mut bytes, 62, 0); // e_shstrndx

        // -----------------------------------------------------
        // Program header table
        // -----------------------------------------------------

        for i in 0..count {
            let ph = phoff + i * ELF64_PROGRAM_HEADER_SIZE;

            let p_vaddr = 0x400000 + data_offset as u64 + (i as u64 * 0x1000);

            put_u32(&mut bytes, ph, PT_LOAD);
            put_u32(&mut bytes, ph + 4, PF_R | PF_X);

            put_u64(&mut bytes, ph + 8, data_offset as u64);
            put_u64(&mut bytes, ph + 16, p_vaddr);
            put_u64(&mut bytes, ph + 24, p_vaddr);

            put_u64(&mut bytes, ph + 32, 16); // p_filesz
            put_u64(&mut bytes, ph + 40, 16); // p_memsz
            put_u64(&mut bytes, ph + 48, 0x1000); // p_align
        }

        // -----------------------------------------------------
        // Section header table
        // -----------------------------------------------------

        let sh = shoff;

        put_u32(&mut bytes, sh, 0); // sh_name
        put_u32(&mut bytes, sh + 4, SHT_PROGBITS);

        put_u64(&mut bytes, sh + 8, 6); // SHF_ALLOC | SHF_EXECINSTR

        let sh_addr = 0x400000 + data_offset as u64;

        put_u64(&mut bytes, sh + 16, sh_addr);
        put_u64(&mut bytes, sh + 24, data_offset as u64);
        put_u64(&mut bytes, sh + 32, 16);

        put_u32(&mut bytes, sh + 40, 0);
        put_u32(&mut bytes, sh + 44, 0);

        put_u64(&mut bytes, sh + 48, 16);
        put_u64(&mut bytes, sh + 56, 0);

        bytes
    }

    // ---------------------------------------------------------
    // ELF header tests
    // ---------------------------------------------------------

    #[test]
    fn parses_valid_elf64() {
        let bytes = valid_elf64(1);

        let elf = Elf::parse(&bytes).expect("valid ELF64 should parse");

        assert_eq!(elf.sections.len(), 1);

        let section = &elf.sections[0];

        assert_eq!(section.header.sh_name, 0);
        assert_eq!(section.header.sh_type, SHT_PROGBITS);
        assert_eq!(section.header.sh_flags, 6);
        assert_eq!(section.header.sh_addr, 0x401000);
        assert_eq!(section.header.sh_offset, 0x1000);
        assert_eq!(section.header.sh_size, 16);
        assert_eq!(section.header.sh_addralign, 16);

        assert_eq!(elf.programs.len(), 1);
    }

    #[test]
    fn rejects_truncated_elf_header() {
        for len in 0..ELF64_HEADER_SIZE {
            let bytes = vec![0u8; len];

            assert!(
                Elf::parse(&bytes).is_err(),
                "accepted truncated ELF header: {len} bytes"
            );
        }
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = valid_elf64(1);

        bytes[0] = 0;

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_elf_class() {
        let mut bytes = valid_elf64(1);

        bytes[4] = 0xFF;

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_unsupported_endianness() {
        let mut bytes = valid_elf64(1);

        bytes[5] = 0;
        assert!(Elf::parse(&bytes).is_err());

        bytes[5] = 3;
        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_ident_version() {
        let mut bytes = valid_elf64(1);

        bytes[6] = 0;

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_nonzero_ident_padding() {
        for offset in 9..16 {
            let mut bytes = valid_elf64(1);

            bytes[offset] = 1;

            assert!(
                Elf::parse(&bytes).is_err(),
                "accepted nonzero EI_PAD at offset {offset}"
            );
        }
    }

    // ---------------------------------------------------------
    // Section header tests
    // ---------------------------------------------------------

    #[test]
    fn rejects_section_table_out_of_bounds() {
        let mut bytes = valid_elf64(1);

        put_u64(&mut bytes, 40, 0x10000);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_truncated_section_table() {
        let mut bytes = valid_elf64(1);

        // Section table begins at 64 + 56.
        bytes.truncate(64 + 56 + ELF64_SECTION_HEADER_SIZE - 1);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_section_entry_size() {
        let mut bytes = valid_elf64(1);

        put_u16(&mut bytes, 58, 32);

        assert!(Elf::parse(&bytes).is_err());
    }

    // ---------------------------------------------------------
    // Program header tests
    // ---------------------------------------------------------

    #[test]
    fn parses_valid_program_header() {
        let bytes = valid_elf64(1);

        let elf = Elf::parse(&bytes).expect("valid program header should parse");

        assert_eq!(elf.programs.len(), 1);

        let ph = &elf.programs[0];

        assert_eq!(ph.p_type, PT_LOAD);
        assert_eq!(ph.p_flags, PF_R | PF_X);
        assert_eq!(ph.p_offset, 0x1000);
        assert_eq!(ph.p_vaddr, 0x401000);
        assert_eq!(ph.p_paddr, 0x401000);
        assert_eq!(ph.p_filesz, 16);
        assert_eq!(ph.p_memsz, 16);
        assert_eq!(ph.p_align, 0x1000);
    }

    #[test]
    fn parses_multiple_program_headers() {
        let bytes = valid_elf64(2);

        let elf = Elf::parse(&bytes).expect("multiple program headers should parse");

        assert_eq!(elf.programs.len(), 2);

        let first = &elf.programs[0];
        let second = &elf.programs[1];

        assert_eq!(first.p_type, PT_LOAD);
        assert_eq!(second.p_type, PT_LOAD);

        assert_eq!(first.p_vaddr, 0x401000);
        assert_eq!(second.p_vaddr, 0x402000);
    }

    #[test]
    fn parses_elf_without_program_headers() {
        let bytes = valid_elf64(0);

        let elf = Elf::parse(&bytes).expect("ELF without program headers should parse");

        assert!(elf.programs.is_empty());
    }

    #[test]
    fn rejects_program_header_table_out_of_bounds() {
        let mut bytes = valid_elf64(1);

        put_u64(&mut bytes, 32, 0x10000);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_truncated_program_header_table() {
        let mut bytes = valid_elf64(1);

        // Keep the section table intact but move the program
        // header table to an invalid location.
        let invalid_phoff = bytes.len() - 1;

        put_u64(&mut bytes, 32, invalid_phoff as u64);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_program_header_entry_size() {
        let mut bytes = valid_elf64(1);

        put_u16(&mut bytes, 54, 32);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_zero_program_header_entry_size() {
        let mut bytes = valid_elf64(1);

        put_u16(&mut bytes, 54, 0);

        assert!(Elf::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_program_header_count_exceeding_bounds() {
        let mut bytes = valid_elf64(1);

        put_u16(&mut bytes, 56, 100);

        assert!(Elf::parse(&bytes).is_err());
    }

    // ---------------------------------------------------------
    // Parser safety tests
    // ---------------------------------------------------------

    #[test]
    fn parser_never_panics_on_truncated_input() {
        let bytes = valid_elf64(1);

        for len in 0..bytes.len() {
            let result = std::panic::catch_unwind(|| {
                let _ = Elf::parse(&bytes[..len]);
            });

            assert!(result.is_ok(), "parser panicked with input length {len}");
        }
    }
}
