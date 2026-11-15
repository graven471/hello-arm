/* refer to glibc/elf/elf.h and ELF specs */

/*
 * the layout is defined like this for Relocatable file
 * |-----------------------------|
 * |     ELF Header              |
 * |_____________________________|
 * | Program header table        |
 * |                             |
 * |_____________________________|
 * |     section 1               |
 * |_____________________________|
 * |      ........               |
 * |_____________________________|
 * |    section n                |
 * |_____________________________|
 * | Section Header table        |
 * |  optional                   |
 * |_____________________________|
 */

use crate::error::ElfError;

#[repr(transparent)]
pub struct Elf64Half(pub u16);
#[repr(transparent)]
pub struct Elf64Addr(pub u64);
#[repr(transparent)]
pub struct El64Off(pub u32);
#[repr(transparent)]
pub struct Elf64Word(pub u32);
#[repr(transparent)]
pub struct Elf64Section(pub u32);

pub const ELF64_HDRLEN: usize = 64;

pub const ELFMAG_0: u8 = 0x7f;
pub const ELFMAG_1: u8 = 0x45; // E
pub const ELFMAG_2: u8 = 0x4C; // L
pub const ELFMAG_3: u8 = 0x46; // F

/// \x7F E L F
pub const ELF_MAGIC: [u8; 4] = [ELFMAG_0, ELFMAG_1, ELFMAG_2, ELFMAG_3];

// file class byte index EI_NIDENT[4]
pub const EI_CLASS: usize = 4;

/// invalid class
pub const ELFCLASSNONE: u8 = 0;
/// 32-bit objects
pub const ELFCLASS32: u8 = 1;
/// 64-bit objects
pub const ELFCLASS64: u8 = 2;

/// data encoding byte index EI_NIDENT[5]
pub const EI_DATA: usize = 5;

/// invalid data encoding
pub const ELFDATANONE: u8 = 0;
/// object file data structures are little-endian
pub const ELFDATALSB: u8 = 1;
/// object file data structures are big-endian
pub const ELFDATAMSB: u8 = 2;

/// File version EI_NIDENT[6] value must be EvCurrent
pub const EI_VERSION: usize = 6;

/// OS/ABI identification
pub const EI_OSABI: usize = 7;

/// System V ABI
pub const ELFOSABI_SYSV: u8 = 0;
/// HP-UX operating system
pub const ELFOSABI_HPUX: u8 = 1;
/// NetBsd Operating system
pub const ELFOSABI_NETBSD: u8 = 2;
/// Linux operation system
pub const ELFOSABI_LINUX: u8 = 3;
/// ARM
pub const ELFOSABI_ARM: u8 = 97;
/// standalone (embedded) application
pub const ELFOSABI_STANDALONE: u8 = 255;

/// ABI version
pub const EI_ABIVERSION: usize = 8;

/// Start of padding bytes
pub const EI_PAD: usize = 9;

/// Invalid elf version
pub const EV_NONE: u8 = 0;
/// Current version
pub const EV_CURRENT: u8 = 1;
pub const EV_NUM: u8 = 2;

pub const SHN_UNDEF: u16 = 0;

pub const EI_NIDENT: usize = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; EI_NIDENT], // magic number and other info
    pub e_type: u16,              // object file type
    pub e_machine: u16,           // architecture
    pub e_version: u32,           // object file version
    pub e_entry: u64,             // entry point virtual address
    pub e_phoff: u64,             // program header table file offset
    pub e_shoff: u64,             // section header table file offset
    pub e_flags: u32,             // processor-specific flags
    pub e_ehsize: u16,            // elf header size in bytes
    pub e_phentsize: u16,         // program header table entry size
    pub e_phnum: u16,             // program header table entry count
    pub e_shentsize: u16,         // section header table entry size
    pub e_shnum: u16,             // section header table entry count
    pub e_shstrndx: u16,          // section header string table index
}

const _: () = assert!(
    core::mem::size_of::<Elf64Ehdr>() == ELF64_HDRLEN,
    "Elf64 header must be 64 bytes"
);

#[repr(u16)]
pub enum ObjectType {
    ETNone = 0,        // no file type
    ETRel = 1,         // Relocatable file
    ETExec = 2,        // executable file
    ETDyn = 3,         // shared object file
    ETCore = 4,        // core file
    ETLoproc = 0xff00, // processor-specific
    ETHiproc = 0xffff, // processor-specific
}

impl TryFrom<u16> for ObjectType {
    type Error = ElfError;

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == ObjectType::ETNone as u16 => Ok(ObjectType::ETNone),
            x if x == ObjectType::ETRel as u16 => Ok(ObjectType::ETRel),
            x if x == ObjectType::ETExec as u16 => Ok(ObjectType::ETExec),
            x if x == ObjectType::ETDyn as u16 => Ok(ObjectType::ETDyn),
            x if x == ObjectType::ETCore as u16 => Ok(ObjectType::ETCore),
            x if x == ObjectType::ETLoproc as u16 => Ok(ObjectType::ETLoproc),
            x if x == ObjectType::ETHiproc as u16 => Ok(ObjectType::ETHiproc),
            _ => Err(ElfError::UnsupportedType(value)),
        }
    }
}

#[repr(u16)]
pub enum ArchType {
    EMNone = 0,

    // x86 family
    EMX86_64 = 62,

    // ARM family
    EMAArch64 = 183,

    // RISC-V
    EMRiscV = 243,

    // PowerPC
    EMPPC64 = 21,

    // IBM System z
    EMS390 = 22,

    // SPARC
    EMSPARCV9 = 43,

    // Itanium
    EMIA64 = 50,

    // DEC Alpha
    EMAlpha = 41,

    // MIPS
    EMMips = 8,

    // LoongArch
    EMLoongArch = 258,

    // AMD GPU
    EMAMDGPU = 224,

    // NVIDIA CUDA
    EMNVidiaCUDA = 190,

    // OpenRISC (64-bit-capable implementations)
    EMOpenRISC = 92,
}

impl TryFrom<u16> for ArchType {
    type Error = ElfError;

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EMNone),
            41 => Ok(Self::EMAlpha),
            43 => Ok(Self::EMSPARCV9),
            62 => Ok(Self::EMX86_64),
            50 => Ok(Self::EMIA64),
            21 => Ok(Self::EMPPC64),
            22 => Ok(Self::EMS390),
            183 => Ok(Self::EMAArch64),
            8 => Ok(Self::EMMips),
            243 => Ok(Self::EMRiscV),
            258 => Ok(Self::EMLoongArch),
            _ => Err(ElfError::UnsupportedMachine(value)),
        }
    }
}

pub const ELF64_SECTION_HEADER_LEN: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Shdr {
    pub sh_name: u32,      /* Section name */
    pub sh_type: u32,      /* Section type */
    pub sh_flags: u64,     /* section attributes */
    pub sh_addr: u64,      /* virtual address in memory */
    pub sh_offset: u64,    /* Offset in file */
    pub sh_size: u64,      /* size of section */
    pub sh_link: u32,      /* link to other section */
    pub sh_info: u32,      /* Miscellaneous information */
    pub sh_addralign: u64, /* address alignment boundary */
    pub sh_entsize: u64,   /* size of entries, if section has table */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Shdr>() == ELF64_SECTION_HEADER_LEN,
    "Elf64 section header must be 64-bytes"
);

/// ELF64 Section Header Type (`sh_type`) constants
pub mod sh_type {
    /// Inactive section header (always index 0)
    pub const SHT_NULL: u32 = 0;
    /// Program-defined data (.text, .data, .rodata, etc.)
    pub const SHT_PROGBITS: u32 = 1;
    /// Static symbol table
    pub const SHT_SYMTAB: u32 = 2;
    /// String table
    pub const SHT_STRTAB: u32 = 3;
    /// Relocation entries with explicit addend
    pub const SHT_RELA: u32 = 4;
    /// Symbol hash table (SysV)
    pub const SHT_HASH: u32 = 5;
    /// Dynamic linking information
    pub const SHT_DYNAMIC: u32 = 6;
    /// Vendor/OS-specific notes
    pub const SHT_NOTE: u32 = 7;
    /// Occupies no file space; zero-filled at load (.bss)
    pub const SHT_NOBITS: u32 = 8;
    /// Relocation entries without explicit addend
    pub const SHT_REL: u32 = 9;
    /// Reserved / unspecified
    pub const SHT_SHLIB: u32 = 10;
    /// Dynamic symbol table
    pub const SHT_DYNSYM: u32 = 11;
    /// Array of constructor function pointers
    pub const SHT_INIT_ARRAY: u32 = 14;
    /// Array of destructor function pointers
    pub const SHT_FINI_ARRAY: u32 = 15;
    /// Pre-init array
    pub const SHT_PREINIT_ARRAY: u32 = 16;
    /// Section group
    pub const SHT_GROUP: u32 = 17;
    /// Extended section indices
    pub const SHT_SYMTAB_SHNDX: u32 = 18;
    /// Number of defined types (not a real type)
    pub const SHT_NUM: u32 = 19;

    /// GNU-style hash table
    pub const SHT_GNU_HASH: u32 = 0x6fff_fff6;
    /// GNU attributes
    pub const SHT_GNU_ATTRIBUTES: u32 = 0x6fff_fff5;
    /// GNU version definitions
    pub const SHT_GNU_VERDEF: u32 = 0x6fff_fffd;
    /// GNU version requirements
    pub const SHT_GNU_VERNEED: u32 = 0x6fff_fffe;
    /// GNU version symbol table
    pub const SHT_GNU_VERSYM: u32 = 0x6fff_ffff;

    /// Start of OS-specific range
    pub const SHT_LOOS: u32 = 0x6000_0000;
    /// End of OS-specific range
    pub const SHT_HIOS: u32 = 0x6fff_ffff;
    /// Start of processor-specific range
    pub const SHT_LOPROC: u32 = 0x7000_0000;
    /// End of processor-specific range
    pub const SHT_HIPROC: u32 = 0x7fff_ffff;
    /// Start of application-specific range
    pub const SHT_LOUSER: u32 = 0x8000_0000;
    /// End of application-specific range
    pub const SHT_HIUSER: u32 = 0xffff_ffff;
}

pub mod sh_flags {
    /// section contains writable data
    pub const SHF_WRITE: u64 = 0x01;
    /// section is allocated in memory image of program
    pub const SHF_ALLOC: u64 = 0x02;
    /// section contains executable instructions
    pub const SHF_EXECINSTR: u64 = 0x04;
    /// might be merged
    pub const SHF_MERGE: u64 = 0x10;
    /// contains null-terminated strings
    pub const SHF_STRINGS: u64 = 0x20;
    /// sh_info contains SHT index
    pub const SHF_INFO_LINK: u64 = 0x40;
    /// Preserve order after combining
    pub const SHF_LINK_ORDER: u64 = 0x80;
    /// Non-standard OS specific handling required
    pub const SHF_OS_NONCONFORMING: u64 = 0x100;
    /// Section is member of a group
    pub const SHF_GROUP: u64 = 0x200;
    /// Section holds thread-local data
    pub const SHF_TLS: u64 = 0x400;
    /// Environment-specific use
    pub const SHF_MASKOS: u64 = 0x0FF00000;
    /// Processor-specific use
    pub const SHF_MASKPROC: u64 = 0xF0000000;
    /// Special ordering requirement (Solaris)
    pub const SHF_ORDERED: u64 = 0x4000000;
    /// Section is excluded unless referenced or allocated (Solaris)
    pub const SHF_EXCLUDE: u64 = 0x8000000;
}

pub const ELF64_PHDR_LEN: usize = 56;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Phdr {
    pub p_type: u32,   /* 0 type of segment */
    pub p_flags: u32,  /* 4 segment attributes */
    pub p_offset: u64, /* 8 offset in file */
    pub p_vaddr: u64,  /* 16 virtual address in memory */
    pub p_paddr: u64,  /* 24 reserved */
    pub p_filesz: u64, /* 32 size of segment in file */
    pub p_memsz: u64,  /* 40 size of segment in memory */
    pub p_align: u64,  /* 48 alignment of segment */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Phdr>() == ELF64_PHDR_LEN,
    "ELF64 Program header must be 56-bytes"
);

pub enum ProgramHeaderType {
    /// unused entry
    Null,
    /// loadable segment
    Load,
    /// dynamic linking table
    Dynamic,
    /// program interpreter path name
    Interp,
    /// note section
    Note,
    /// Rserved
    Shlib,
    /// program header table
    Phdr,
    /// Environment-specific use
    Loos,
    Hios,
    /// Processor-specific use
    LoProc,
    HiProc,
    /// unknown
    Other(u32),
}

impl From<u32> for ProgramHeaderType {
    fn from(value: u32) -> Self {
        match value {
            0 => ProgramHeaderType::Null,
            1 => ProgramHeaderType::Load,
            2 => ProgramHeaderType::Dynamic,
            3 => ProgramHeaderType::Interp,
            4 => ProgramHeaderType::Note,
            5 => ProgramHeaderType::Shlib,
            6 => ProgramHeaderType::Phdr,
            0x60000000 => ProgramHeaderType::Loos,
            0x6FFFFFFF => ProgramHeaderType::Hios,
            0x70000000 => ProgramHeaderType::LoProc,
            0x7FFFFFFF => ProgramHeaderType::HiProc,
            _ => ProgramHeaderType::Other(value),
        }
    }
}

pub mod p_flags {
    /// Execute permission
    pub const PF_X: u32 = 0x01;
    /// Write permission
    pub const PF_W: u32 = 0x02;
    /// Read permission
    pub const PF_R: u32 = 0x04;
    /// These flag bits are reserved for environment-specific use
    pub const PF_MASKOS: u32 = 0x00FF0000;
    /// These flag bits are reserved for processor-specific use
    pub const PF_MASKPROC: u32 = 0xFF000000;
}

pub const ELF64_SYM_LEN: usize = 24;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Sym {
    st_name: u32,  /* symbol name */
    st_info: u8,   /* type and binding attribute */
    st_other: u8,  /* reserved */
    st_shndx: u16, /* section table index */
    st_value: u64, /* symbol value */
    st_size: u64,  /* size of object (e.g., common) */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Sym>() == ELF64_SYM_LEN,
    "Elf64Sym must be 24-bytes"
);
