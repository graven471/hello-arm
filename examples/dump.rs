#![allow(unused)]
use std::{error::Error, fs, io::Read};

use arm_disas::{aarch64::engine::Disassembler, elf::parser::Elf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open("target/debug/examples/dump")?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    let elf = Elf::parse(&contents)?;

    println!("{:x?}", elf.header);

    for section in &elf.sections {
        println!("name = {}, data len = {}", section.name, section.data.len());
    }

    let text_section = elf
        .sections
        .iter()
        .find(|&sec| sec.name == ".text")
        .expect("no .text section");

    let disassembler = Disassembler::new(text_section.data);
    disassembler.decode();

    Ok(())
}
