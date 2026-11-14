use crate::{
    elf::types::{
        EI_CLASS, EI_DATA, EI_NIDENT, EI_PAD, EI_VERSION, ELF_MAGIC, ELF64_HDRLEN, ELFCLASS32,
        ELFCLASSNONE, ELFDATANONE, EV_CURRENT, Elf64Ehdr, ObjectType,
    },
    error::ElfError,
};

pub struct ElfImage {
    header: Elf64Ehdr,
}

impl ElfImage {
    pub fn parse(bytes: impl AsRef<[u8]>) -> Result<Self, ElfError> {
        let bytes = bytes.as_ref();

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

        if e_ident[0..4] != ELF_MAGIC {
            return Err(ElfError::InvalidMagic);
        }

        let e_class = e_ident[EI_CLASS];

        // do not support elf32 will see about this later
        if e_class == ELFCLASSNONE || e_class == ELFCLASS32 {
            return Err(ElfError::UnsupportedClass(e_class));
        }

        let ei_data = e_ident[EI_DATA];

        if ei_data == ELFDATANONE {
            return Err(ElfError::UnsupportedEndian(0));
        }

        if e_ident[EI_VERSION] != EV_CURRENT {
            return Err(ElfError::UnsupportedVersion(bytes[EI_VERSION] as u32));
        }

        if e_ident[EI_PAD..EI_NIDENT].iter().any(|&b| b != 0) {
            return Err(ElfError::InvalidHeader("EI_PAD must be zero"));
        }

        // after EI_NIDENT
        let rest = &bytes[EI_NIDENT..];

        let e_type = ObjectType::try_from(u16::from_le_bytes(rest[0..2].try_into().unwrap()))?;
        let e_machine = u32::from_le_bytes(rest[2..4].try_into().unwrap());
        let e_version = u32::from_le_bytes(rest[4..8].try_into().unwrap());

        if e_version != EV_CURRENT as u32 {
            return Err(ElfError::UnsupportedVersion(e_version));
        }

        // this can be zero if there no entry point
        let e_entry = u64::from_le_bytes(rest[8..16].try_into().unwrap());
        let e_phoff = u64::from_le_bytes(rest[16..24].try_into().unwrap());
        let e_shoff = u64::from_le_bytes(rest[24..32].try_into().unwrap());
        let e_flags = u32::from_le_bytes(rest[32..36].try_into().unwrap());
        let e_ehsize = u16::from_le_bytes(rest[36..38].try_into().unwrap());
        let e_phentsize = u16::from_le_bytes(rest[38..40].try_into().unwrap());
        let e_phnum = u16::from_le_bytes(rest[40..42].try_into().unwrap());
        let e_shentsize = u16::from_le_bytes(rest[42..44].try_into().unwrap());
        let e_shnum = u16::from_le_bytes(rest[44..46].try_into().unwrap());
        let e_shstrndx = u16::from_le_bytes(rest[46..48].try_into().unwrap());

        todo!()
    }
}
