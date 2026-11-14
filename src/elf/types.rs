/* refer to glibc/elf/elf.h and ELF specs */

/*
 * the layout is defined like this
 * |-----------------------------|
 * |     ELF Header              |
 * |_____________________________|
 * | Program header table        |
 * | optional                    |
 * |_____________________________|
 * |     segment 1               |
 * |_____________________________|
 * |     segment 2               |
 * |_____________________________|
 * |    ....                     |
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

pub const EI_NIDENT: usize = 16;

#[repr(C)]
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
