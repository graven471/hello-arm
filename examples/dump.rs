#![allow(unused)]
use std::{error::Error, fs, io::Read};

use arm_disas::elf::parser::Elf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open("target/debug/examples/dump")?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    let elf = Elf::parse(&contents)?;

    println!("{:x?}", elf.header);

    for section in &elf.programs {
        println!("name = {}, data len = {}", section.p_type, section.p_flags);
    }

    let sym = elf.symbol_table();

    // if sym.is_some() {
    //     let sym = sym.unwrap();
    //     for s in sym {
    //         let name = elf.resolve_symbol_name(&s);
    //         println!("{}", name);
    //     }
    // }

    Ok(())
}
