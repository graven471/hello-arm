use thiserror::Error;

pub type Result<T> = std::result::Result<T, AelfError>;

#[derive(Debug, Error)]
pub enum AelfError {
    #[error(transparent)]
    Elf(#[from] ElfError),

    #[error(transparent)]
    Loader(#[from] LoaderError),

    #[error(transparent)]
    Relocation(#[from] RelocationError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ElfError {
    // I/O
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // ELF identification
    #[error("invalid ELF magic")]
    InvalidMagic,

    #[error("unsupported ELF class: {0}")]
    UnsupportedClass(u8),

    #[error("unsupported ELF endianness: {0}")]
    UnsupportedEndian(u8),

    #[error("unsupported ELF version: {0}")]
    UnsupportedVersion(u32),

    #[error("unsupported ELF type: {0:#x}")]
    UnsupportedType(u16),

    #[error("unsupported machine architecture: {0:#x}")]
    UnsupportedMachine(u16),

    // Parsing
    #[error("unexpected end of ELF file")]
    UnexpectedEof,

    #[error("invalid ELF header: {0}")]
    InvalidHeader(&'static str),

    #[error("invalid program header: {0}")]
    InvalidProgramHeader(&'static str),

    #[error("invalid section header: {0}")]
    InvalidSectionHeader(&'static str),

    #[error("invalid symbol table: {0}")]
    InvalidSymbolTable(&'static str),

    #[error("invalid string table")]
    InvalidStringTable,

    #[error("invalid relocation entry: {0}")]
    InvalidRelocation(&'static str),

    // Bounds and offsets
    #[error("offset out of bounds: offset={offset:#x}, size={size:#x}")]
    OutOfBounds { offset: u64, size: u64 },

    #[error("integer overflow while calculating ELF offsets")]
    IntegerOverflow,

    #[error("invalid alignment: {0}")]
    InvalidAlignment(u64),

    // Lookup
    #[error("section not found: {0}")]
    SectionNotFound(String),

    #[error("symbol not found: {0}")]
    SymbolNotFound(String),

    #[error("program header not found: index={0}")]
    ProgramHeaderNotFound(usize),

    // Features
    #[error("unsupported ELF feature: {0}")]
    UnsupportedFeature(&'static str),
}

#[derive(Debug, Error)]
pub enum RelocationError {
    #[error("unsupported relocation type: {0:#x}")]
    UnsupportedRelocationType(u32),

    #[error("invalid relocation entry: {0}")]
    InvalidRelocation(&'static str),

    #[error("relocation target out of bounds: offset={offset:#x}, size={size:#x}")]
    TargetOutOfBounds { offset: u64, size: u64 },

    #[error("relocation value overflow")]
    ValueOverflow,

    #[error("relocation address is out of range")]
    AddressOutOfRange,

    #[error("relocation alignment violation: address={address:#x}, alignment={alignment}")]
    AlignmentViolation { address: u64, alignment: u64 },

    #[error("unresolved symbol: {0}")]
    UnresolvedSymbol(String),

    #[error("symbol value is missing")]
    MissingSymbolValue,

    #[error("relocation requires a GOT entry")]
    MissingGotEntry,

    #[error("relocation requires a PLT entry")]
    MissingPltEntry,

    #[error("unsupported relocation operation: {0}")]
    UnsupportedOperation(&'static str),
}

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("invalid load segment: {0}")]
    InvalidLoadSegment(&'static str),

    #[error("segment memory allocation failed")]
    AllocationFailed,

    #[error("segment mapping failed: {0}")]
    MappingFailed(&'static str),

    #[error("invalid segment permissions: {0}")]
    InvalidPermissions(&'static str),

    #[error("segment address overflow")]
    AddressOverflow,

    #[error("segment is not properly aligned: address={address:#x}, alignment={alignment}")]
    AlignmentViolation { address: u64, alignment: u64 },

    #[error("overlapping load segments")]
    OverlappingSegments,

    #[error("entry point is outside executable memory: {0:#x}")]
    InvalidEntryPoint(u64),

    #[error("no loadable segments found")]
    NoLoadableSegments,

    #[error("unsupported loading feature: {0}")]
    UnsupportedFeature(&'static str),

    #[error("ELF image is not suitable for loading: {0}")]
    InvalidImage(&'static str),
}
