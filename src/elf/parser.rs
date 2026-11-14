use crate::{
    elf::types::{
        EI_CLASS, EI_DATA, EI_NIDENT, EI_PAD, EI_VERSION, ELF_MAGIC, ELF64_HDRLEN, ELFCLASS32,
        ELFCLASS64, ELFCLASSNONE, ELFDATALSB, ELFDATAMSB, ELFDATANONE, EV_CURRENT, Elf64Ehdr,
        Elf64Shdr, ObjectType,
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
}

pub struct ElfImage {
    header: Elf64Ehdr,
    section: Vec<Elf64Shdr>,
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

        // do not support elf32 will see about this later
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

        // ========================== section header =================================

        // (todo): e_shoff can be zero in that case return empty section
        // (todo): process program header (if available)

        let sh_offset = elf_hdr.e_shoff as usize;
        let sh_size = elf_hdr.e_shnum as usize * elf_hdr.e_shentsize as usize;
        let num_entries = elf_hdr.e_shnum as usize;
        let entry_size = elf_hdr.e_shentsize as usize;

        if (entry_size != core::mem::size_of::<Elf64Shdr>()) {
            return Err(ElfError::InvalidSectionHeader(
                "entry size must be 64-bytes",
            ));
        }

        if num_entries == 0 {
            return Ok(Self {
                header: elf_hdr,
                section: vec![],
            });
        }

        let end = sh_offset
            .checked_add(sh_size)
            .ok_or(ElfError::IntegerOverflow)?;

        if end > bytes.len() {
            return Err(ElfError::InvalidSectionHeader(
                "Section header table extends beyond file",
            ));
        }

        let mut sec_headers: Vec<Elf64Shdr> = Vec::with_capacity(num_entries);

        sec_headers = (0..num_entries)
            .map(|i| {
                // 0 * entry_size, 1 * entry_size, 2 * entry_size ... num_entries * entry_size
                let start = sh_offset * i + entry_size;
                // 0..entry_size, entry_size..entry_size*2, entry_size*2...entry_size*3
                let entry = &bytes[start..start + entry_size];
                Elf64Shdr {
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
                }
            })
            .collect();

        return Ok(Self {
            header: elf_hdr,
            section: sec_headers,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::elf::types::ELF64_SECTION_HEADER_LEN;

    use super::*;

    // ELF64 header = 64 bytes.
    // ELF64 section header = 64 bytes.

    fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }

    /// Builds a minimal valid ELF64 little-endian executable
    /// containing one section header.
    fn valid_elf64() -> Vec<u8> {
        let mut bytes = vec![0u8; ELF64_HDRLEN + ELF64_SECTION_HEADER_LEN];

        // e_ident
        bytes[0..4].copy_from_slice(b"\x7FELF");
        bytes[4] = 2; // ELFCLASS64
        bytes[5] = 1; // ELFDATA2LSB
        bytes[6] = 1; // EV_CURRENT
        bytes[7] = 0; // ELFOSABI_NONE

        // ELF header
        put_u16(&mut bytes, 16, 2); // e_type = ET_EXEC
        put_u16(&mut bytes, 18, 62); // e_machine = EM_X86_64
        put_u32(&mut bytes, 20, 1); // e_version = EV_CURRENT

        put_u64(&mut bytes, 24, 0x400000); // e_entry
        put_u64(&mut bytes, 32, 0); // e_phoff
        put_u64(&mut bytes, 40, 64); // e_shoff
        put_u32(&mut bytes, 48, 0); // e_flags

        put_u16(&mut bytes, 52, 64); // e_ehsize
        put_u16(&mut bytes, 54, 0); // e_phentsize
        put_u16(&mut bytes, 56, 0); // e_phnum
        put_u16(&mut bytes, 58, 64); // e_shentsize
        put_u16(&mut bytes, 60, 1); // e_shnum
        put_u16(&mut bytes, 62, 0); // e_shstrndx

        // First section header: SHT_PROGBITS
        let sh = ELF64_HDRLEN;

        put_u32(&mut bytes, sh, 0); // sh_name
        put_u32(&mut bytes, sh + 4, 1); // sh_type = SHT_PROGBITS
        put_u64(&mut bytes, sh + 8, 6); // sh_flags = SHF_ALLOC | SHF_EXECINSTR
        put_u64(&mut bytes, sh + 16, 0x400100); // sh_addr
        put_u64(&mut bytes, sh + 24, 128); // sh_offset
        put_u64(&mut bytes, sh + 32, 16); // sh_size
        put_u32(&mut bytes, sh + 40, 0); // sh_link
        put_u32(&mut bytes, sh + 44, 0); // sh_info
        put_u64(&mut bytes, sh + 48, 16); // sh_addralign
        put_u64(&mut bytes, sh + 56, 0); // sh_entsize

        bytes
    }

    #[test]
    fn parses_valid_elf64() {
        let bytes = valid_elf64();

        let image = ElfImage::parse(&bytes).expect("valid ELF64 should parse successfully");

        assert_eq!(image.section.len(), 1);

        let section = &image.section[0];

        assert_eq!(section.sh_name, 0);
        assert_eq!(section.sh_type, 1);
        assert_eq!(section.sh_flags, 6);
        assert_eq!(section.sh_addr, 0x400100);
        assert_eq!(section.sh_offset, 128);
        assert_eq!(section.sh_size, 16);
        assert_eq!(section.sh_addralign, 16);
    }

    #[test]
    fn rejects_truncated_elf_header() {
        for len in 0..ELF64_HDRLEN {
            let bytes = vec![0u8; len];

            assert!(
                ElfImage::parse(&bytes).is_err(),
                "parser accepted truncated header of {len} bytes"
            );
        }
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = valid_elf64();

        bytes[0] = 0;

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_elf_class() {
        let mut bytes = valid_elf64();

        bytes[4] = 0xFF;

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_unsupported_endianness() {
        let mut bytes = valid_elf64();

        bytes[5] = 0; // ELFDATANONE

        assert!(ElfImage::parse(&bytes).is_err());

        bytes[5] = 3; // Invalid encoding

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_ident_version() {
        let mut bytes = valid_elf64();

        bytes[6] = 0;

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_nonzero_ident_padding() {
        let mut bytes = valid_elf64();

        bytes[9] = 1;

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_section_table_out_of_bounds() {
        let mut bytes = valid_elf64();

        // Section table starts beyond the end of the file.
        put_u64(&mut bytes, 40, 0x10000);

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_truncated_section_table() {
        let mut bytes = valid_elf64();

        // Remove one byte from the section header.
        bytes.pop();

        assert!(ElfImage::parse(&bytes).is_err());
    }

    #[test]
    fn rejects_invalid_section_entry_size() {
        let mut bytes = valid_elf64();

        // ELF64 section header must be at least 64 bytes.
        put_u16(&mut bytes, 58, 32);

        assert!(ElfImage::parse(&bytes).is_err());
    }
}
