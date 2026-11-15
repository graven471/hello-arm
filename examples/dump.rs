use std::{error::Error, fs, io::Read};

use arm_disas::elf::parser::Elf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open("coreutils/cat")?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    let elf = Elf::parse(&contents)?;

    println!("{:x?}", elf.header);

    for section in elf.sections {
        println!("name = {}, data len = {}", section.name, section.data.len());
    }

    Ok(())
}
