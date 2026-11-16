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
    pub p_type: u32,   /*  type of segment */
    pub p_flags: u32,  /*  segment attributes */
    pub p_offset: u64, /*  offset in file */
    pub p_vaddr: u64,  /*  virtual address in memory */
    pub p_paddr: u64,  /*  reserved */
    pub p_filesz: u64, /*  size of segment in file */
    pub p_memsz: u64,  /*  size of segment in memory */
    pub p_align: u64,  /*  alignment of segment */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Phdr>() == ELF64_PHDR_LEN,
    "ELF64 Program header must be 56-bytes"
);

pub mod program_header_type {
    /// Unused entry.
    pub const NULL: u32 = 0x00000000;

    /// Loadable segment.
    pub const LOAD: u32 = 0x00000001;

    /// Dynamic linking table.
    pub const DYNAMIC: u32 = 0x00000002;

    /// Program interpreter path name.
    pub const INTERP: u32 = 0x00000003;

    /// Note section.
    pub const NOTE: u32 = 0x00000004;

    /// Reserved.
    pub const SHLIB: u32 = 0x00000005;

    /// Program header table.
    pub const PHDR: u32 = 0x00000006;

    /// Environment-specific use.
    pub const LOOS: u32 = 0x60000000;
    pub const HIOS: u32 = 0x6FFFFFFF;

    /// Processor-specific use.
    pub const LOPROC: u32 = 0x70000000;
    pub const HIPROC: u32 = 0x7FFFFFFF;
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
    pub st_name: u32,  /* symbol name */
    pub st_info: u8,   /* type and binding attribute */
    pub st_other: u8,  /* reserved */
    pub st_shndx: u16, /* section table index */
    pub st_value: u64, /* symbol value */
    pub st_size: u64,  /* size of object (e.g., common) */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Sym>() == ELF64_SYM_LEN,
    "Elf64Sym must be 24-bytes"
);

pub mod st_info {
    /// symbol is unspecified
    pub const STT_NOTYPE: u8 = 0;
    /// symbol is associated with a data object
    pub const STT_OBJECT: u8 = 1;
    /// symbol is associated with a function or other executable code
    pub const STT_FUNC: u8 = 2;
    /// symbol is associated with a section
    pub const STT_SECTION: u8 = 3;
    /// symbol's name is file name
    pub const STT_FILE: u8 = 4;
    /// symbol is a common data object
    pub const STT_COMMON: u8 = 5;
    /// symbol is thread-local data object
    pub const STT_TLS: u8 = 6;
    /// numbers of defined types
    pub const STT_NUM: u8 = 7;
    /// start of os specific
    pub const STT_LOOS: u8 = 10;
    /// symbol is indirect code object
    pub const STT_GNU_IFUNC: u8 = 10;
    /// end of os specific
    pub const STT_HIOS: u8 = 10;
    /// start of processor specific
    pub const STT_LOPROC: u8 = 13;
    /// end of os specific
    pub const STT_HIPROC: u8 = 15;
}

pub const ELF64_DYN_HDR_LEN: usize = 16;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64Dyn {
    pub d_tag: i64, /* dynamic entry type */
    pub d_val: u64, /* integer value */
}

const _: () = assert!(
    core::mem::size_of::<Elf64Dyn>() == ELF64_DYN_HDR_LEN,
    "dynamic header len must be 16-bytes"
);

pub mod dt_tag {
    /// Marks the end of the dynamic array.
    pub const DT_NULL: i64 = 0;
    /// Name of a needed library.
    pub const DT_NEEDED: i64 = 1;
    /// Size in bytes of the PLT relocation entries.
    pub const DT_PLTRELSZ: i64 = 2;
    /// Address of the PLT/GOT.
    pub const DT_PLTGOT: i64 = 3;
    /// Address of the symbol hash table.
    pub const DT_HASH: i64 = 4;
    /// Address of the string table.
    pub const DT_STRTAB: i64 = 5;
    /// Address of the symbol table.
    pub const DT_SYMTAB: i64 = 6;
    /// Address of the RELA relocation table.
    pub const DT_RELA: i64 = 7;
    /// Size in bytes of the RELA relocation table.
    pub const DT_RELASZ: i64 = 8;
    /// Size in bytes of a RELA entry.
    pub const DT_RELAENT: i64 = 9;
    /// Size in bytes of the string table.
    pub const DT_STRSZ: i64 = 10;
    /// Size in bytes of a symbol table entry.
    pub const DT_SYMENT: i64 = 11;
    /// Address of the initialization function.
    pub const DT_INIT: i64 = 12;
    /// Address of the termination function.
    pub const DT_FINI: i64 = 13;
    /// Name of the shared object.
    pub const DT_SONAME: i64 = 14;
    /// Library search path (deprecated).
    pub const DT_RPATH: i64 = 15;
    /// Changes symbol resolution rules.
    pub const DT_SYMBOLIC: i64 = 16;
    /// Address of the REL relocation table.
    pub const DT_REL: i64 = 17;
    /// Size in bytes of the REL relocation table.
    pub const DT_RELSZ: i64 = 18;
    /// Size in bytes of a REL entry.
    pub const DT_RELENT: i64 = 19;
    /// Type of relocation entry used by PLT.
    pub const DT_PLTREL: i64 = 20;
    /// Address of the debugging information.
    pub const DT_DEBUG: i64 = 21;
    /// Indicates that the object contains text relocations.
    pub const DT_TEXTREL: i64 = 22;
    /// Address of the PLT relocation entries.
    pub const DT_JMPREL: i64 = 23;
    /// Process all relocations before transferring control.
    pub const DT_BIND_NOW: i64 = 24;
    /// Address of the initialization function array.
    pub const DT_INIT_ARRAY: i64 = 25;
    /// Address of the termination function array.
    pub const DT_FINI_ARRAY: i64 = 26;
    /// Size in bytes of the initialization function array.
    pub const DT_INIT_ARRAYSZ: i64 = 27;
    /// Size in bytes of the termination function array.
    pub const DT_FINI_ARRAYSZ: i64 = 28;
    /// Library search path.
    pub const DT_RUNPATH: i64 = 29;
    /// Flags.
    pub const DT_FLAGS: i64 = 30;

    /// Start of encoded range.
    pub const DT_ENCODING: i64 = 32;
    /// Address of the pre-initialization function array.
    pub const DT_PREINIT_ARRAY: i64 = 32;
    /// Size in bytes of the pre-initialization function array.
    pub const DT_PREINIT_ARRAYSZ: i64 = 33;
    /// Address of the extended symbol table section index array.
    pub const DT_SYMTAB_SHNDX: i64 = 34;
    /// Size in bytes of the RELR relocation table.
    pub const DT_RELRSZ: i64 = 35;
    /// Address of the RELR relocation table.
    pub const DT_RELR: i64 = 36;
    /// Size in bytes of a RELR entry.
    pub const DT_RELRENT: i64 = 37;

    /// Number of defined standard dynamic tags.
    pub const DT_NUM: i64 = 38;

    /// Start of OS-specific range.
    pub const DT_LOOS: i64 = 0x6000_000d;

    /// GNU-specific indirect function symbol table.
    pub const DT_GNU_HASH: i64 = 0x6fff_fef5;
    /// PLT entry address for TLS descriptor.
    pub const DT_TLSDESC_PLT: i64 = 0x6fff_fef6;
    /// GOT entry address for TLS descriptor.
    pub const DT_TLSDESC_GOT: i64 = 0x6fff_fef7;
    /// Address of conflict symbol table.
    pub const DT_GNU_CONFLICT: i64 = 0x6fff_fef8;
    /// Address of library list.
    pub const DT_GNU_LIBLIST: i64 = 0x6fff_fef9;
    /// Configuration file name.
    pub const DT_CONFIG: i64 = 0x6fff_fefa;
    /// Dependency auditing.
    pub const DT_DEPAUDIT: i64 = 0x6fff_fefb;
    /// Object auditing.
    pub const DT_AUDIT: i64 = 0x6fff_fefc;
    /// PLT padding.
    pub const DT_PLTPAD: i64 = 0x6fff_fefd;
    /// Address of the move table.
    pub const DT_MOVETAB: i64 = 0x6fff_fefe;
    /// Address of the syminfo table.
    pub const DT_SYMINFO: i64 = 0x6fff_feff;

    /// End of OS-specific range.
    pub const DT_HIOS: i64 = 0x6fff_f000;

    /// Address of the version symbol table.
    pub const DT_VERSYM: i64 = 0x6fff_fff0;
    /// Number of RELA relocations.
    pub const DT_RELACOUNT: i64 = 0x6fff_fff9;
    /// Number of REL relocations.
    pub const DT_RELCOUNT: i64 = 0x6fff_fffa;
    /// Flags.
    pub const DT_FLAGS_1: i64 = 0x6fff_fffb;
    /// Address of the version definition table.
    pub const DT_VERDEF: i64 = 0x6fff_fffc;
    /// Number of version definitions.
    pub const DT_VERDEFNUM: i64 = 0x6fff_fffd;
    /// Address of the version dependency table.
    pub const DT_VERNEED: i64 = 0x6fff_fffe;
    /// Number of version dependencies.
    pub const DT_VERNEEDNUM: i64 = 0x6fff_ffff;

    /// Start of processor-specific range.
    pub const DT_LOPROC: i64 = 0x7000_0000;
    /// End of processor-specific range.
    pub const DT_HIPROC: i64 = 0x7fff_ffff;
}
