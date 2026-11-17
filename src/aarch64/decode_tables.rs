// @generated from Arm's encodingindex.xml, do not edit

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RootClass {
    Reserved,
    Sme,
    Sve,
    Unallocated,
    Dpimm,
    Control,
    Dpreg,
    SimdDp,
    Ldst,
}

#[inline(always)]
pub const fn root_index(insn: u32) -> usize {
    (((insn >> 31) & 0x1) << 4 | (insn >> 25) & 0xf) as usize
}

pub static DECODE_ROOT: [RootClass; 32] = [
    RootClass::Reserved,    /* 00000 */
    RootClass::Unallocated, /* 00001 */
    RootClass::Sve,         /* 00010 */
    RootClass::Unallocated, /* 00011 */
    RootClass::Ldst,        /* 00100 */
    RootClass::Dpreg,       /* 00101 */
    RootClass::Ldst,        /* 00110 */
    RootClass::SimdDp,      /* 00111 */
    RootClass::Dpimm,       /* 01000 */
    RootClass::Dpimm,       /* 01001 */
    RootClass::Control,     /* 01010 */
    RootClass::Control,     /* 01011 */
    RootClass::Ldst,        /* 01100 */
    RootClass::Dpreg,       /* 01101 */
    RootClass::Ldst,        /* 01110 */
    RootClass::SimdDp,      /* 01111 */
    RootClass::Sme,         /* 10000 */
    RootClass::Unallocated, /* 10001 */
    RootClass::Sve,         /* 10010 */
    RootClass::Unallocated, /* 10011 */
    RootClass::Ldst,        /* 10100 */
    RootClass::Dpreg,       /* 10101 */
    RootClass::Ldst,        /* 10110 */
    RootClass::SimdDp,      /* 10111 */
    RootClass::Dpimm,       /* 11000 */
    RootClass::Dpimm,       /* 11001 */
    RootClass::Control,     /* 11010 */
    RootClass::Control,     /* 11011 */
    RootClass::Ldst,        /* 11100 */
    RootClass::Dpreg,       /* 11101 */
    RootClass::Ldst,        /* 11110 */
    RootClass::SimdDp,      /* 11111 */
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpimmClass {
    Dp1srcImm,
    Pcreladdr,
    AddsubImm,
    AddsubImmtags,
    MinmaxImm,
    LogImm,
    Movewide,
    Bitfield,
    Extract,
}

#[inline(always)]
pub const fn dpimm_index(insn: u32) -> usize {
    (((insn >> 29) & 0x3) << 4 | (insn >> 22) & 0xf) as usize
}

pub static DECODE_DPIMM: [DpimmClass; 64] = [
    DpimmClass::Pcreladdr,     /* 000000 */
    DpimmClass::Pcreladdr,     /* 000001 */
    DpimmClass::Pcreladdr,     /* 000010 */
    DpimmClass::Pcreladdr,     /* 000011 */
    DpimmClass::AddsubImm,     /* 000100 */
    DpimmClass::AddsubImm,     /* 000101 */
    DpimmClass::AddsubImmtags, /* 000110 */
    DpimmClass::MinmaxImm,     /* 000111 */
    DpimmClass::LogImm,        /* 001000 */
    DpimmClass::LogImm,        /* 001001 */
    DpimmClass::Movewide,      /* 001010 */
    DpimmClass::Movewide,      /* 001011 */
    DpimmClass::Bitfield,      /* 001100 */
    DpimmClass::Bitfield,      /* 001101 */
    DpimmClass::Extract,       /* 001110 */
    DpimmClass::Extract,       /* 001111 */
    DpimmClass::Pcreladdr,     /* 010000 */
    DpimmClass::Pcreladdr,     /* 010001 */
    DpimmClass::Pcreladdr,     /* 010010 */
    DpimmClass::Pcreladdr,     /* 010011 */
    DpimmClass::AddsubImm,     /* 010100 */
    DpimmClass::AddsubImm,     /* 010101 */
    DpimmClass::AddsubImmtags, /* 010110 */
    DpimmClass::MinmaxImm,     /* 010111 */
    DpimmClass::LogImm,        /* 011000 */
    DpimmClass::LogImm,        /* 011001 */
    DpimmClass::Movewide,      /* 011010 */
    DpimmClass::Movewide,      /* 011011 */
    DpimmClass::Bitfield,      /* 011100 */
    DpimmClass::Bitfield,      /* 011101 */
    DpimmClass::Extract,       /* 011110 */
    DpimmClass::Extract,       /* 011111 */
    DpimmClass::Pcreladdr,     /* 100000 */
    DpimmClass::Pcreladdr,     /* 100001 */
    DpimmClass::Pcreladdr,     /* 100010 */
    DpimmClass::Pcreladdr,     /* 100011 */
    DpimmClass::AddsubImm,     /* 100100 */
    DpimmClass::AddsubImm,     /* 100101 */
    DpimmClass::AddsubImmtags, /* 100110 */
    DpimmClass::MinmaxImm,     /* 100111 */
    DpimmClass::LogImm,        /* 101000 */
    DpimmClass::LogImm,        /* 101001 */
    DpimmClass::Movewide,      /* 101010 */
    DpimmClass::Movewide,      /* 101011 */
    DpimmClass::Bitfield,      /* 101100 */
    DpimmClass::Bitfield,      /* 101101 */
    DpimmClass::Extract,       /* 101110 */
    DpimmClass::Extract,       /* 101111 */
    DpimmClass::Pcreladdr,     /* 110000 */
    DpimmClass::Pcreladdr,     /* 110001 */
    DpimmClass::Pcreladdr,     /* 110010 */
    DpimmClass::Pcreladdr,     /* 110011 */
    DpimmClass::AddsubImm,     /* 110100 */
    DpimmClass::AddsubImm,     /* 110101 */
    DpimmClass::AddsubImmtags, /* 110110 */
    DpimmClass::MinmaxImm,     /* 110111 */
    DpimmClass::LogImm,        /* 111000 */
    DpimmClass::LogImm,        /* 111001 */
    DpimmClass::Movewide,      /* 111010 */
    DpimmClass::Movewide,      /* 111011 */
    DpimmClass::Bitfield,      /* 111100 */
    DpimmClass::Bitfield,      /* 111101 */
    DpimmClass::Dp1srcImm,     /* 111110 */
    DpimmClass::Dp1srcImm,     /* 111111 */
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlClass {
    Condbranch,
    Miscbranch,
    CompbranchRegs2,
    Unallocated,
    Exception,
    Extendedhints,
    Systeminstrswithreg,
    Hints,
    Barriers,
    Pstate,
    Systeminstrs,
    Systemmove,
    Syspairinstrs,
    Systemmovepr,
    BranchReg,
    BranchImm,
    Compbranch,
    Testbranch,
    CompbranchRegs,
    CompbranchImm,
}

#[inline(always)]
pub const fn control_index(insn: u32) -> usize {
    (((insn >> 29) & 0x7) << 19 | ((insn >> 12) & 0x3fff) << 5 | insn & 0x1f) as usize
}

static CONTROL_CLASSES: [ControlClass; 20] = [
    ControlClass::Condbranch,
    ControlClass::Miscbranch,
    ControlClass::CompbranchRegs2,
    ControlClass::Unallocated,
    ControlClass::Exception,
    ControlClass::Extendedhints,
    ControlClass::Systeminstrswithreg,
    ControlClass::Hints,
    ControlClass::Barriers,
    ControlClass::Pstate,
    ControlClass::Systeminstrs,
    ControlClass::Systemmove,
    ControlClass::Syspairinstrs,
    ControlClass::Systemmovepr,
    ControlClass::BranchReg,
    ControlClass::BranchImm,
    ControlClass::Compbranch,
    ControlClass::Testbranch,
    ControlClass::CompbranchRegs,
    ControlClass::CompbranchImm,
];

static CONTROL_L0: [[u16; 256]; 1] = [[
    32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
    32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
    32783, 32783, 32783, 32783, 32783, 32783, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
    32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32785, 32785, 32785, 32785,
    32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32769, 32769, 32769, 32769, 32769, 32769,
    32769, 32769, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
    32771, 32771, 32771, 32771, 32771, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 32771,
    32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
    32771, 32771, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
    32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
    32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32784, 32784, 32784, 32784, 32784,
    32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32785, 32785,
    32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
    32785, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 2, 3, 4, 4, 32771, 32771, 32771,
    32771, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
    32782, 32782, 32782, 32782, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 32771, 32771,
    32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
    32771,
]];

static CONTROL_L1: [[u16; 256]; 6] = [
    [
        32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770,
        32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786,
        32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770,
        32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771,
        32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786,
        32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770,
        32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771,
        32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770,
        32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770,
        32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786,
        32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770,
        32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771,
        32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786,
        32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770,
        32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771,
        32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770,
        32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770,
        32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786,
        32771, 32771, 32770, 32770, 32770, 32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770,
        32770, 32786, 32786, 32771, 32771, 32770, 32770, 32770, 32770,
    ],
    [
        32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787,
        32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787,
        32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771,
        32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771,
        32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787,
        32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787,
        32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771,
        32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771,
        32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787,
        32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787,
        32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771,
        32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771,
        32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787,
        32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787,
        32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771,
        32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771,
        32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787,
        32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787,
        32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771,
        32771, 32787, 32787, 32771, 32771, 32787, 32787, 32771, 32771,
    ],
    [
        32771, 0, 1, 32771, 32771, 32771, 32771, 32771, 32771, 0, 1, 32771, 32771, 32771, 32771,
        32771, 32771, 0, 1, 32771, 32771, 32771, 32771, 32771, 2, 3, 1, 32771, 32771, 32771, 32771,
        32771, 32771, 0, 1, 32771, 32771, 32771, 32771, 32771, 32771, 0, 1, 32771, 32771, 32771,
        32771, 32771, 32771, 0, 1, 32771, 32771, 32771, 32771, 32771, 32771, 0, 1, 32771, 32771,
        32771, 32771, 32771, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
    ],
    [
        32771, 0, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 0, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 0, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 0, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 0, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 0,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 0, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 0, 32771, 32771, 32771, 32771, 32771, 32771, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778,
        32778, 32778, 32778, 32778, 32778, 32778, 32778, 32778, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779, 32779,
        32779, 32779, 32779, 32779, 32779, 32779,
    ],
    [
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
    ],
    [
        32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771,
        32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786,
        32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771,
        32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771,
        32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786,
        32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771,
        32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771,
        32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771,
        32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771,
        32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786,
        32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771,
        32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771,
        32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786,
        32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771,
        32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771,
        32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771,
        32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771,
        32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786,
        32771, 32771, 32771, 32771, 32771, 32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771,
        32771, 32786, 32786, 32771, 32771, 32771, 32771, 32771, 32771,
    ],
];

static CONTROL_L2: [[u16; 64]; 4] = [
    [
        32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773,
        32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773,
        32773, 32773, 32773, 32773, 32773, 32773, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
    ],
    [
        32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777,
        32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777, 32777,
        32777, 32777, 32777, 32777, 32777, 32777, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
    ],
    [
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771, 32771,
        32771, 32771, 32771, 32771, 32771, 32771, 32774, 32774, 32774, 32774, 32774, 32774, 32774,
        32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774,
        32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774, 32774,
    ],
    [
        32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773,
        32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773,
        32773, 32773, 32773, 32773, 32773, 32775, 32776, 32776, 32776, 32776, 32776, 32776, 32776,
        32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776,
        32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776, 32776,
    ],
];

#[inline]
pub fn decode_control(insn: u32) -> ControlClass {
    let i = control_index(insn);
    let mut e: u16 = 0;
    e = CONTROL_L0[e as usize][(i >> 14) & 0xff];
    if e & 0x8000 != 0 {
        return CONTROL_CLASSES[(e & 0x7fff) as usize];
    }
    e = CONTROL_L1[e as usize][(i >> 6) & 0xff];
    if e & 0x8000 != 0 {
        return CONTROL_CLASSES[(e & 0x7fff) as usize];
    }
    e = CONTROL_L2[e as usize][i & 0x3f];
    CONTROL_CLASSES[(e & 0x7fff) as usize]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpregClass {
    Dp2src,
    Dp1src,
    LogShift,
    AddsubShift,
    AddsubExt,
    AddsubCarry,
    AddsubPt,
    Unallocated,
    Rmif,
    Setf,
    CondcmpReg,
    CondcmpImm,
    Condsel,
    Dp3src,
}

#[inline(always)]
pub const fn dpreg_index(insn: u32) -> usize {
    (((insn >> 30) & 0x1) << 11
        | ((insn >> 28) & 0x1) << 10
        | ((insn >> 21) & 0xf) << 6
        | (insn >> 10) & 0x3f) as usize
}

pub static DECODE_DPREG: [DpregClass; 4096] = [
    DpregClass::LogShift,    /* 000000000000 */
    DpregClass::LogShift,    /* 000000000001 */
    DpregClass::LogShift,    /* 000000000010 */
    DpregClass::LogShift,    /* 000000000011 */
    DpregClass::LogShift,    /* 000000000100 */
    DpregClass::LogShift,    /* 000000000101 */
    DpregClass::LogShift,    /* 000000000110 */
    DpregClass::LogShift,    /* 000000000111 */
    DpregClass::LogShift,    /* 000000001000 */
    DpregClass::LogShift,    /* 000000001001 */
    DpregClass::LogShift,    /* 000000001010 */
    DpregClass::LogShift,    /* 000000001011 */
    DpregClass::LogShift,    /* 000000001100 */
    DpregClass::LogShift,    /* 000000001101 */
    DpregClass::LogShift,    /* 000000001110 */
    DpregClass::LogShift,    /* 000000001111 */
    DpregClass::LogShift,    /* 000000010000 */
    DpregClass::LogShift,    /* 000000010001 */
    DpregClass::LogShift,    /* 000000010010 */
    DpregClass::LogShift,    /* 000000010011 */
    DpregClass::LogShift,    /* 000000010100 */
    DpregClass::LogShift,    /* 000000010101 */
    DpregClass::LogShift,    /* 000000010110 */
    DpregClass::LogShift,    /* 000000010111 */
    DpregClass::LogShift,    /* 000000011000 */
    DpregClass::LogShift,    /* 000000011001 */
    DpregClass::LogShift,    /* 000000011010 */
    DpregClass::LogShift,    /* 000000011011 */
    DpregClass::LogShift,    /* 000000011100 */
    DpregClass::LogShift,    /* 000000011101 */
    DpregClass::LogShift,    /* 000000011110 */
    DpregClass::LogShift,    /* 000000011111 */
    DpregClass::LogShift,    /* 000000100000 */
    DpregClass::LogShift,    /* 000000100001 */
    DpregClass::LogShift,    /* 000000100010 */
    DpregClass::LogShift,    /* 000000100011 */
    DpregClass::LogShift,    /* 000000100100 */
    DpregClass::LogShift,    /* 000000100101 */
    DpregClass::LogShift,    /* 000000100110 */
    DpregClass::LogShift,    /* 000000100111 */
    DpregClass::LogShift,    /* 000000101000 */
    DpregClass::LogShift,    /* 000000101001 */
    DpregClass::LogShift,    /* 000000101010 */
    DpregClass::LogShift,    /* 000000101011 */
    DpregClass::LogShift,    /* 000000101100 */
    DpregClass::LogShift,    /* 000000101101 */
    DpregClass::LogShift,    /* 000000101110 */
    DpregClass::LogShift,    /* 000000101111 */
    DpregClass::LogShift,    /* 000000110000 */
    DpregClass::LogShift,    /* 000000110001 */
    DpregClass::LogShift,    /* 000000110010 */
    DpregClass::LogShift,    /* 000000110011 */
    DpregClass::LogShift,    /* 000000110100 */
    DpregClass::LogShift,    /* 000000110101 */
    DpregClass::LogShift,    /* 000000110110 */
    DpregClass::LogShift,    /* 000000110111 */
    DpregClass::LogShift,    /* 000000111000 */
    DpregClass::LogShift,    /* 000000111001 */
    DpregClass::LogShift,    /* 000000111010 */
    DpregClass::LogShift,    /* 000000111011 */
    DpregClass::LogShift,    /* 000000111100 */
    DpregClass::LogShift,    /* 000000111101 */
    DpregClass::LogShift,    /* 000000111110 */
    DpregClass::LogShift,    /* 000000111111 */
    DpregClass::LogShift,    /* 000001000000 */
    DpregClass::LogShift,    /* 000001000001 */
    DpregClass::LogShift,    /* 000001000010 */
    DpregClass::LogShift,    /* 000001000011 */
    DpregClass::LogShift,    /* 000001000100 */
    DpregClass::LogShift,    /* 000001000101 */
    DpregClass::LogShift,    /* 000001000110 */
    DpregClass::LogShift,    /* 000001000111 */
    DpregClass::LogShift,    /* 000001001000 */
    DpregClass::LogShift,    /* 000001001001 */
    DpregClass::LogShift,    /* 000001001010 */
    DpregClass::LogShift,    /* 000001001011 */
    DpregClass::LogShift,    /* 000001001100 */
    DpregClass::LogShift,    /* 000001001101 */
    DpregClass::LogShift,    /* 000001001110 */
    DpregClass::LogShift,    /* 000001001111 */
    DpregClass::LogShift,    /* 000001010000 */
    DpregClass::LogShift,    /* 000001010001 */
    DpregClass::LogShift,    /* 000001010010 */
    DpregClass::LogShift,    /* 000001010011 */
    DpregClass::LogShift,    /* 000001010100 */
    DpregClass::LogShift,    /* 000001010101 */
    DpregClass::LogShift,    /* 000001010110 */
    DpregClass::LogShift,    /* 000001010111 */
    DpregClass::LogShift,    /* 000001011000 */
    DpregClass::LogShift,    /* 000001011001 */
    DpregClass::LogShift,    /* 000001011010 */
    DpregClass::LogShift,    /* 000001011011 */
    DpregClass::LogShift,    /* 000001011100 */
    DpregClass::LogShift,    /* 000001011101 */
    DpregClass::LogShift,    /* 000001011110 */
    DpregClass::LogShift,    /* 000001011111 */
    DpregClass::LogShift,    /* 000001100000 */
    DpregClass::LogShift,    /* 000001100001 */
    DpregClass::LogShift,    /* 000001100010 */
    DpregClass::LogShift,    /* 000001100011 */
    DpregClass::LogShift,    /* 000001100100 */
    DpregClass::LogShift,    /* 000001100101 */
    DpregClass::LogShift,    /* 000001100110 */
    DpregClass::LogShift,    /* 000001100111 */
    DpregClass::LogShift,    /* 000001101000 */
    DpregClass::LogShift,    /* 000001101001 */
    DpregClass::LogShift,    /* 000001101010 */
    DpregClass::LogShift,    /* 000001101011 */
    DpregClass::LogShift,    /* 000001101100 */
    DpregClass::LogShift,    /* 000001101101 */
    DpregClass::LogShift,    /* 000001101110 */
    DpregClass::LogShift,    /* 000001101111 */
    DpregClass::LogShift,    /* 000001110000 */
    DpregClass::LogShift,    /* 000001110001 */
    DpregClass::LogShift,    /* 000001110010 */
    DpregClass::LogShift,    /* 000001110011 */
    DpregClass::LogShift,    /* 000001110100 */
    DpregClass::LogShift,    /* 000001110101 */
    DpregClass::LogShift,    /* 000001110110 */
    DpregClass::LogShift,    /* 000001110111 */
    DpregClass::LogShift,    /* 000001111000 */
    DpregClass::LogShift,    /* 000001111001 */
    DpregClass::LogShift,    /* 000001111010 */
    DpregClass::LogShift,    /* 000001111011 */
    DpregClass::LogShift,    /* 000001111100 */
    DpregClass::LogShift,    /* 000001111101 */
    DpregClass::LogShift,    /* 000001111110 */
    DpregClass::LogShift,    /* 000001111111 */
    DpregClass::LogShift,    /* 000010000000 */
    DpregClass::LogShift,    /* 000010000001 */
    DpregClass::LogShift,    /* 000010000010 */
    DpregClass::LogShift,    /* 000010000011 */
    DpregClass::LogShift,    /* 000010000100 */
    DpregClass::LogShift,    /* 000010000101 */
    DpregClass::LogShift,    /* 000010000110 */
    DpregClass::LogShift,    /* 000010000111 */
    DpregClass::LogShift,    /* 000010001000 */
    DpregClass::LogShift,    /* 000010001001 */
    DpregClass::LogShift,    /* 000010001010 */
    DpregClass::LogShift,    /* 000010001011 */
    DpregClass::LogShift,    /* 000010001100 */
    DpregClass::LogShift,    /* 000010001101 */
    DpregClass::LogShift,    /* 000010001110 */
    DpregClass::LogShift,    /* 000010001111 */
    DpregClass::LogShift,    /* 000010010000 */
    DpregClass::LogShift,    /* 000010010001 */
    DpregClass::LogShift,    /* 000010010010 */
    DpregClass::LogShift,    /* 000010010011 */
    DpregClass::LogShift,    /* 000010010100 */
    DpregClass::LogShift,    /* 000010010101 */
    DpregClass::LogShift,    /* 000010010110 */
    DpregClass::LogShift,    /* 000010010111 */
    DpregClass::LogShift,    /* 000010011000 */
    DpregClass::LogShift,    /* 000010011001 */
    DpregClass::LogShift,    /* 000010011010 */
    DpregClass::LogShift,    /* 000010011011 */
    DpregClass::LogShift,    /* 000010011100 */
    DpregClass::LogShift,    /* 000010011101 */
    DpregClass::LogShift,    /* 000010011110 */
    DpregClass::LogShift,    /* 000010011111 */
    DpregClass::LogShift,    /* 000010100000 */
    DpregClass::LogShift,    /* 000010100001 */
    DpregClass::LogShift,    /* 000010100010 */
    DpregClass::LogShift,    /* 000010100011 */
    DpregClass::LogShift,    /* 000010100100 */
    DpregClass::LogShift,    /* 000010100101 */
    DpregClass::LogShift,    /* 000010100110 */
    DpregClass::LogShift,    /* 000010100111 */
    DpregClass::LogShift,    /* 000010101000 */
    DpregClass::LogShift,    /* 000010101001 */
    DpregClass::LogShift,    /* 000010101010 */
    DpregClass::LogShift,    /* 000010101011 */
    DpregClass::LogShift,    /* 000010101100 */
    DpregClass::LogShift,    /* 000010101101 */
    DpregClass::LogShift,    /* 000010101110 */
    DpregClass::LogShift,    /* 000010101111 */
    DpregClass::LogShift,    /* 000010110000 */
    DpregClass::LogShift,    /* 000010110001 */
    DpregClass::LogShift,    /* 000010110010 */
    DpregClass::LogShift,    /* 000010110011 */
    DpregClass::LogShift,    /* 000010110100 */
    DpregClass::LogShift,    /* 000010110101 */
    DpregClass::LogShift,    /* 000010110110 */
    DpregClass::LogShift,    /* 000010110111 */
    DpregClass::LogShift,    /* 000010111000 */
    DpregClass::LogShift,    /* 000010111001 */
    DpregClass::LogShift,    /* 000010111010 */
    DpregClass::LogShift,    /* 000010111011 */
    DpregClass::LogShift,    /* 000010111100 */
    DpregClass::LogShift,    /* 000010111101 */
    DpregClass::LogShift,    /* 000010111110 */
    DpregClass::LogShift,    /* 000010111111 */
    DpregClass::LogShift,    /* 000011000000 */
    DpregClass::LogShift,    /* 000011000001 */
    DpregClass::LogShift,    /* 000011000010 */
    DpregClass::LogShift,    /* 000011000011 */
    DpregClass::LogShift,    /* 000011000100 */
    DpregClass::LogShift,    /* 000011000101 */
    DpregClass::LogShift,    /* 000011000110 */
    DpregClass::LogShift,    /* 000011000111 */
    DpregClass::LogShift,    /* 000011001000 */
    DpregClass::LogShift,    /* 000011001001 */
    DpregClass::LogShift,    /* 000011001010 */
    DpregClass::LogShift,    /* 000011001011 */
    DpregClass::LogShift,    /* 000011001100 */
    DpregClass::LogShift,    /* 000011001101 */
    DpregClass::LogShift,    /* 000011001110 */
    DpregClass::LogShift,    /* 000011001111 */
    DpregClass::LogShift,    /* 000011010000 */
    DpregClass::LogShift,    /* 000011010001 */
    DpregClass::LogShift,    /* 000011010010 */
    DpregClass::LogShift,    /* 000011010011 */
    DpregClass::LogShift,    /* 000011010100 */
    DpregClass::LogShift,    /* 000011010101 */
    DpregClass::LogShift,    /* 000011010110 */
    DpregClass::LogShift,    /* 000011010111 */
    DpregClass::LogShift,    /* 000011011000 */
    DpregClass::LogShift,    /* 000011011001 */
    DpregClass::LogShift,    /* 000011011010 */
    DpregClass::LogShift,    /* 000011011011 */
    DpregClass::LogShift,    /* 000011011100 */
    DpregClass::LogShift,    /* 000011011101 */
    DpregClass::LogShift,    /* 000011011110 */
    DpregClass::LogShift,    /* 000011011111 */
    DpregClass::LogShift,    /* 000011100000 */
    DpregClass::LogShift,    /* 000011100001 */
    DpregClass::LogShift,    /* 000011100010 */
    DpregClass::LogShift,    /* 000011100011 */
    DpregClass::LogShift,    /* 000011100100 */
    DpregClass::LogShift,    /* 000011100101 */
    DpregClass::LogShift,    /* 000011100110 */
    DpregClass::LogShift,    /* 000011100111 */
    DpregClass::LogShift,    /* 000011101000 */
    DpregClass::LogShift,    /* 000011101001 */
    DpregClass::LogShift,    /* 000011101010 */
    DpregClass::LogShift,    /* 000011101011 */
    DpregClass::LogShift,    /* 000011101100 */
    DpregClass::LogShift,    /* 000011101101 */
    DpregClass::LogShift,    /* 000011101110 */
    DpregClass::LogShift,    /* 000011101111 */
    DpregClass::LogShift,    /* 000011110000 */
    DpregClass::LogShift,    /* 000011110001 */
    DpregClass::LogShift,    /* 000011110010 */
    DpregClass::LogShift,    /* 000011110011 */
    DpregClass::LogShift,    /* 000011110100 */
    DpregClass::LogShift,    /* 000011110101 */
    DpregClass::LogShift,    /* 000011110110 */
    DpregClass::LogShift,    /* 000011110111 */
    DpregClass::LogShift,    /* 000011111000 */
    DpregClass::LogShift,    /* 000011111001 */
    DpregClass::LogShift,    /* 000011111010 */
    DpregClass::LogShift,    /* 000011111011 */
    DpregClass::LogShift,    /* 000011111100 */
    DpregClass::LogShift,    /* 000011111101 */
    DpregClass::LogShift,    /* 000011111110 */
    DpregClass::LogShift,    /* 000011111111 */
    DpregClass::LogShift,    /* 000100000000 */
    DpregClass::LogShift,    /* 000100000001 */
    DpregClass::LogShift,    /* 000100000010 */
    DpregClass::LogShift,    /* 000100000011 */
    DpregClass::LogShift,    /* 000100000100 */
    DpregClass::LogShift,    /* 000100000101 */
    DpregClass::LogShift,    /* 000100000110 */
    DpregClass::LogShift,    /* 000100000111 */
    DpregClass::LogShift,    /* 000100001000 */
    DpregClass::LogShift,    /* 000100001001 */
    DpregClass::LogShift,    /* 000100001010 */
    DpregClass::LogShift,    /* 000100001011 */
    DpregClass::LogShift,    /* 000100001100 */
    DpregClass::LogShift,    /* 000100001101 */
    DpregClass::LogShift,    /* 000100001110 */
    DpregClass::LogShift,    /* 000100001111 */
    DpregClass::LogShift,    /* 000100010000 */
    DpregClass::LogShift,    /* 000100010001 */
    DpregClass::LogShift,    /* 000100010010 */
    DpregClass::LogShift,    /* 000100010011 */
    DpregClass::LogShift,    /* 000100010100 */
    DpregClass::LogShift,    /* 000100010101 */
    DpregClass::LogShift,    /* 000100010110 */
    DpregClass::LogShift,    /* 000100010111 */
    DpregClass::LogShift,    /* 000100011000 */
    DpregClass::LogShift,    /* 000100011001 */
    DpregClass::LogShift,    /* 000100011010 */
    DpregClass::LogShift,    /* 000100011011 */
    DpregClass::LogShift,    /* 000100011100 */
    DpregClass::LogShift,    /* 000100011101 */
    DpregClass::LogShift,    /* 000100011110 */
    DpregClass::LogShift,    /* 000100011111 */
    DpregClass::LogShift,    /* 000100100000 */
    DpregClass::LogShift,    /* 000100100001 */
    DpregClass::LogShift,    /* 000100100010 */
    DpregClass::LogShift,    /* 000100100011 */
    DpregClass::LogShift,    /* 000100100100 */
    DpregClass::LogShift,    /* 000100100101 */
    DpregClass::LogShift,    /* 000100100110 */
    DpregClass::LogShift,    /* 000100100111 */
    DpregClass::LogShift,    /* 000100101000 */
    DpregClass::LogShift,    /* 000100101001 */
    DpregClass::LogShift,    /* 000100101010 */
    DpregClass::LogShift,    /* 000100101011 */
    DpregClass::LogShift,    /* 000100101100 */
    DpregClass::LogShift,    /* 000100101101 */
    DpregClass::LogShift,    /* 000100101110 */
    DpregClass::LogShift,    /* 000100101111 */
    DpregClass::LogShift,    /* 000100110000 */
    DpregClass::LogShift,    /* 000100110001 */
    DpregClass::LogShift,    /* 000100110010 */
    DpregClass::LogShift,    /* 000100110011 */
    DpregClass::LogShift,    /* 000100110100 */
    DpregClass::LogShift,    /* 000100110101 */
    DpregClass::LogShift,    /* 000100110110 */
    DpregClass::LogShift,    /* 000100110111 */
    DpregClass::LogShift,    /* 000100111000 */
    DpregClass::LogShift,    /* 000100111001 */
    DpregClass::LogShift,    /* 000100111010 */
    DpregClass::LogShift,    /* 000100111011 */
    DpregClass::LogShift,    /* 000100111100 */
    DpregClass::LogShift,    /* 000100111101 */
    DpregClass::LogShift,    /* 000100111110 */
    DpregClass::LogShift,    /* 000100111111 */
    DpregClass::LogShift,    /* 000101000000 */
    DpregClass::LogShift,    /* 000101000001 */
    DpregClass::LogShift,    /* 000101000010 */
    DpregClass::LogShift,    /* 000101000011 */
    DpregClass::LogShift,    /* 000101000100 */
    DpregClass::LogShift,    /* 000101000101 */
    DpregClass::LogShift,    /* 000101000110 */
    DpregClass::LogShift,    /* 000101000111 */
    DpregClass::LogShift,    /* 000101001000 */
    DpregClass::LogShift,    /* 000101001001 */
    DpregClass::LogShift,    /* 000101001010 */
    DpregClass::LogShift,    /* 000101001011 */
    DpregClass::LogShift,    /* 000101001100 */
    DpregClass::LogShift,    /* 000101001101 */
    DpregClass::LogShift,    /* 000101001110 */
    DpregClass::LogShift,    /* 000101001111 */
    DpregClass::LogShift,    /* 000101010000 */
    DpregClass::LogShift,    /* 000101010001 */
    DpregClass::LogShift,    /* 000101010010 */
    DpregClass::LogShift,    /* 000101010011 */
    DpregClass::LogShift,    /* 000101010100 */
    DpregClass::LogShift,    /* 000101010101 */
    DpregClass::LogShift,    /* 000101010110 */
    DpregClass::LogShift,    /* 000101010111 */
    DpregClass::LogShift,    /* 000101011000 */
    DpregClass::LogShift,    /* 000101011001 */
    DpregClass::LogShift,    /* 000101011010 */
    DpregClass::LogShift,    /* 000101011011 */
    DpregClass::LogShift,    /* 000101011100 */
    DpregClass::LogShift,    /* 000101011101 */
    DpregClass::LogShift,    /* 000101011110 */
    DpregClass::LogShift,    /* 000101011111 */
    DpregClass::LogShift,    /* 000101100000 */
    DpregClass::LogShift,    /* 000101100001 */
    DpregClass::LogShift,    /* 000101100010 */
    DpregClass::LogShift,    /* 000101100011 */
    DpregClass::LogShift,    /* 000101100100 */
    DpregClass::LogShift,    /* 000101100101 */
    DpregClass::LogShift,    /* 000101100110 */
    DpregClass::LogShift,    /* 000101100111 */
    DpregClass::LogShift,    /* 000101101000 */
    DpregClass::LogShift,    /* 000101101001 */
    DpregClass::LogShift,    /* 000101101010 */
    DpregClass::LogShift,    /* 000101101011 */
    DpregClass::LogShift,    /* 000101101100 */
    DpregClass::LogShift,    /* 000101101101 */
    DpregClass::LogShift,    /* 000101101110 */
    DpregClass::LogShift,    /* 000101101111 */
    DpregClass::LogShift,    /* 000101110000 */
    DpregClass::LogShift,    /* 000101110001 */
    DpregClass::LogShift,    /* 000101110010 */
    DpregClass::LogShift,    /* 000101110011 */
    DpregClass::LogShift,    /* 000101110100 */
    DpregClass::LogShift,    /* 000101110101 */
    DpregClass::LogShift,    /* 000101110110 */
    DpregClass::LogShift,    /* 000101110111 */
    DpregClass::LogShift,    /* 000101111000 */
    DpregClass::LogShift,    /* 000101111001 */
    DpregClass::LogShift,    /* 000101111010 */
    DpregClass::LogShift,    /* 000101111011 */
    DpregClass::LogShift,    /* 000101111100 */
    DpregClass::LogShift,    /* 000101111101 */
    DpregClass::LogShift,    /* 000101111110 */
    DpregClass::LogShift,    /* 000101111111 */
    DpregClass::LogShift,    /* 000110000000 */
    DpregClass::LogShift,    /* 000110000001 */
    DpregClass::LogShift,    /* 000110000010 */
    DpregClass::LogShift,    /* 000110000011 */
    DpregClass::LogShift,    /* 000110000100 */
    DpregClass::LogShift,    /* 000110000101 */
    DpregClass::LogShift,    /* 000110000110 */
    DpregClass::LogShift,    /* 000110000111 */
    DpregClass::LogShift,    /* 000110001000 */
    DpregClass::LogShift,    /* 000110001001 */
    DpregClass::LogShift,    /* 000110001010 */
    DpregClass::LogShift,    /* 000110001011 */
    DpregClass::LogShift,    /* 000110001100 */
    DpregClass::LogShift,    /* 000110001101 */
    DpregClass::LogShift,    /* 000110001110 */
    DpregClass::LogShift,    /* 000110001111 */
    DpregClass::LogShift,    /* 000110010000 */
    DpregClass::LogShift,    /* 000110010001 */
    DpregClass::LogShift,    /* 000110010010 */
    DpregClass::LogShift,    /* 000110010011 */
    DpregClass::LogShift,    /* 000110010100 */
    DpregClass::LogShift,    /* 000110010101 */
    DpregClass::LogShift,    /* 000110010110 */
    DpregClass::LogShift,    /* 000110010111 */
    DpregClass::LogShift,    /* 000110011000 */
    DpregClass::LogShift,    /* 000110011001 */
    DpregClass::LogShift,    /* 000110011010 */
    DpregClass::LogShift,    /* 000110011011 */
    DpregClass::LogShift,    /* 000110011100 */
    DpregClass::LogShift,    /* 000110011101 */
    DpregClass::LogShift,    /* 000110011110 */
    DpregClass::LogShift,    /* 000110011111 */
    DpregClass::LogShift,    /* 000110100000 */
    DpregClass::LogShift,    /* 000110100001 */
    DpregClass::LogShift,    /* 000110100010 */
    DpregClass::LogShift,    /* 000110100011 */
    DpregClass::LogShift,    /* 000110100100 */
    DpregClass::LogShift,    /* 000110100101 */
    DpregClass::LogShift,    /* 000110100110 */
    DpregClass::LogShift,    /* 000110100111 */
    DpregClass::LogShift,    /* 000110101000 */
    DpregClass::LogShift,    /* 000110101001 */
    DpregClass::LogShift,    /* 000110101010 */
    DpregClass::LogShift,    /* 000110101011 */
    DpregClass::LogShift,    /* 000110101100 */
    DpregClass::LogShift,    /* 000110101101 */
    DpregClass::LogShift,    /* 000110101110 */
    DpregClass::LogShift,    /* 000110101111 */
    DpregClass::LogShift,    /* 000110110000 */
    DpregClass::LogShift,    /* 000110110001 */
    DpregClass::LogShift,    /* 000110110010 */
    DpregClass::LogShift,    /* 000110110011 */
    DpregClass::LogShift,    /* 000110110100 */
    DpregClass::LogShift,    /* 000110110101 */
    DpregClass::LogShift,    /* 000110110110 */
    DpregClass::LogShift,    /* 000110110111 */
    DpregClass::LogShift,    /* 000110111000 */
    DpregClass::LogShift,    /* 000110111001 */
    DpregClass::LogShift,    /* 000110111010 */
    DpregClass::LogShift,    /* 000110111011 */
    DpregClass::LogShift,    /* 000110111100 */
    DpregClass::LogShift,    /* 000110111101 */
    DpregClass::LogShift,    /* 000110111110 */
    DpregClass::LogShift,    /* 000110111111 */
    DpregClass::LogShift,    /* 000111000000 */
    DpregClass::LogShift,    /* 000111000001 */
    DpregClass::LogShift,    /* 000111000010 */
    DpregClass::LogShift,    /* 000111000011 */
    DpregClass::LogShift,    /* 000111000100 */
    DpregClass::LogShift,    /* 000111000101 */
    DpregClass::LogShift,    /* 000111000110 */
    DpregClass::LogShift,    /* 000111000111 */
    DpregClass::LogShift,    /* 000111001000 */
    DpregClass::LogShift,    /* 000111001001 */
    DpregClass::LogShift,    /* 000111001010 */
    DpregClass::LogShift,    /* 000111001011 */
    DpregClass::LogShift,    /* 000111001100 */
    DpregClass::LogShift,    /* 000111001101 */
    DpregClass::LogShift,    /* 000111001110 */
    DpregClass::LogShift,    /* 000111001111 */
    DpregClass::LogShift,    /* 000111010000 */
    DpregClass::LogShift,    /* 000111010001 */
    DpregClass::LogShift,    /* 000111010010 */
    DpregClass::LogShift,    /* 000111010011 */
    DpregClass::LogShift,    /* 000111010100 */
    DpregClass::LogShift,    /* 000111010101 */
    DpregClass::LogShift,    /* 000111010110 */
    DpregClass::LogShift,    /* 000111010111 */
    DpregClass::LogShift,    /* 000111011000 */
    DpregClass::LogShift,    /* 000111011001 */
    DpregClass::LogShift,    /* 000111011010 */
    DpregClass::LogShift,    /* 000111011011 */
    DpregClass::LogShift,    /* 000111011100 */
    DpregClass::LogShift,    /* 000111011101 */
    DpregClass::LogShift,    /* 000111011110 */
    DpregClass::LogShift,    /* 000111011111 */
    DpregClass::LogShift,    /* 000111100000 */
    DpregClass::LogShift,    /* 000111100001 */
    DpregClass::LogShift,    /* 000111100010 */
    DpregClass::LogShift,    /* 000111100011 */
    DpregClass::LogShift,    /* 000111100100 */
    DpregClass::LogShift,    /* 000111100101 */
    DpregClass::LogShift,    /* 000111100110 */
    DpregClass::LogShift,    /* 000111100111 */
    DpregClass::LogShift,    /* 000111101000 */
    DpregClass::LogShift,    /* 000111101001 */
    DpregClass::LogShift,    /* 000111101010 */
    DpregClass::LogShift,    /* 000111101011 */
    DpregClass::LogShift,    /* 000111101100 */
    DpregClass::LogShift,    /* 000111101101 */
    DpregClass::LogShift,    /* 000111101110 */
    DpregClass::LogShift,    /* 000111101111 */
    DpregClass::LogShift,    /* 000111110000 */
    DpregClass::LogShift,    /* 000111110001 */
    DpregClass::LogShift,    /* 000111110010 */
    DpregClass::LogShift,    /* 000111110011 */
    DpregClass::LogShift,    /* 000111110100 */
    DpregClass::LogShift,    /* 000111110101 */
    DpregClass::LogShift,    /* 000111110110 */
    DpregClass::LogShift,    /* 000111110111 */
    DpregClass::LogShift,    /* 000111111000 */
    DpregClass::LogShift,    /* 000111111001 */
    DpregClass::LogShift,    /* 000111111010 */
    DpregClass::LogShift,    /* 000111111011 */
    DpregClass::LogShift,    /* 000111111100 */
    DpregClass::LogShift,    /* 000111111101 */
    DpregClass::LogShift,    /* 000111111110 */
    DpregClass::LogShift,    /* 000111111111 */
    DpregClass::AddsubShift, /* 001000000000 */
    DpregClass::AddsubShift, /* 001000000001 */
    DpregClass::AddsubShift, /* 001000000010 */
    DpregClass::AddsubShift, /* 001000000011 */
    DpregClass::AddsubShift, /* 001000000100 */
    DpregClass::AddsubShift, /* 001000000101 */
    DpregClass::AddsubShift, /* 001000000110 */
    DpregClass::AddsubShift, /* 001000000111 */
    DpregClass::AddsubShift, /* 001000001000 */
    DpregClass::AddsubShift, /* 001000001001 */
    DpregClass::AddsubShift, /* 001000001010 */
    DpregClass::AddsubShift, /* 001000001011 */
    DpregClass::AddsubShift, /* 001000001100 */
    DpregClass::AddsubShift, /* 001000001101 */
    DpregClass::AddsubShift, /* 001000001110 */
    DpregClass::AddsubShift, /* 001000001111 */
    DpregClass::AddsubShift, /* 001000010000 */
    DpregClass::AddsubShift, /* 001000010001 */
    DpregClass::AddsubShift, /* 001000010010 */
    DpregClass::AddsubShift, /* 001000010011 */
    DpregClass::AddsubShift, /* 001000010100 */
    DpregClass::AddsubShift, /* 001000010101 */
    DpregClass::AddsubShift, /* 001000010110 */
    DpregClass::AddsubShift, /* 001000010111 */
    DpregClass::AddsubShift, /* 001000011000 */
    DpregClass::AddsubShift, /* 001000011001 */
    DpregClass::AddsubShift, /* 001000011010 */
    DpregClass::AddsubShift, /* 001000011011 */
    DpregClass::AddsubShift, /* 001000011100 */
    DpregClass::AddsubShift, /* 001000011101 */
    DpregClass::AddsubShift, /* 001000011110 */
    DpregClass::AddsubShift, /* 001000011111 */
    DpregClass::AddsubShift, /* 001000100000 */
    DpregClass::AddsubShift, /* 001000100001 */
    DpregClass::AddsubShift, /* 001000100010 */
    DpregClass::AddsubShift, /* 001000100011 */
    DpregClass::AddsubShift, /* 001000100100 */
    DpregClass::AddsubShift, /* 001000100101 */
    DpregClass::AddsubShift, /* 001000100110 */
    DpregClass::AddsubShift, /* 001000100111 */
    DpregClass::AddsubShift, /* 001000101000 */
    DpregClass::AddsubShift, /* 001000101001 */
    DpregClass::AddsubShift, /* 001000101010 */
    DpregClass::AddsubShift, /* 001000101011 */
    DpregClass::AddsubShift, /* 001000101100 */
    DpregClass::AddsubShift, /* 001000101101 */
    DpregClass::AddsubShift, /* 001000101110 */
    DpregClass::AddsubShift, /* 001000101111 */
    DpregClass::AddsubShift, /* 001000110000 */
    DpregClass::AddsubShift, /* 001000110001 */
    DpregClass::AddsubShift, /* 001000110010 */
    DpregClass::AddsubShift, /* 001000110011 */
    DpregClass::AddsubShift, /* 001000110100 */
    DpregClass::AddsubShift, /* 001000110101 */
    DpregClass::AddsubShift, /* 001000110110 */
    DpregClass::AddsubShift, /* 001000110111 */
    DpregClass::AddsubShift, /* 001000111000 */
    DpregClass::AddsubShift, /* 001000111001 */
    DpregClass::AddsubShift, /* 001000111010 */
    DpregClass::AddsubShift, /* 001000111011 */
    DpregClass::AddsubShift, /* 001000111100 */
    DpregClass::AddsubShift, /* 001000111101 */
    DpregClass::AddsubShift, /* 001000111110 */
    DpregClass::AddsubShift, /* 001000111111 */
    DpregClass::AddsubExt,   /* 001001000000 */
    DpregClass::AddsubExt,   /* 001001000001 */
    DpregClass::AddsubExt,   /* 001001000010 */
    DpregClass::AddsubExt,   /* 001001000011 */
    DpregClass::AddsubExt,   /* 001001000100 */
    DpregClass::AddsubExt,   /* 001001000101 */
    DpregClass::AddsubExt,   /* 001001000110 */
    DpregClass::AddsubExt,   /* 001001000111 */
    DpregClass::AddsubExt,   /* 001001001000 */
    DpregClass::AddsubExt,   /* 001001001001 */
    DpregClass::AddsubExt,   /* 001001001010 */
    DpregClass::AddsubExt,   /* 001001001011 */
    DpregClass::AddsubExt,   /* 001001001100 */
    DpregClass::AddsubExt,   /* 001001001101 */
    DpregClass::AddsubExt,   /* 001001001110 */
    DpregClass::AddsubExt,   /* 001001001111 */
    DpregClass::AddsubExt,   /* 001001010000 */
    DpregClass::AddsubExt,   /* 001001010001 */
    DpregClass::AddsubExt,   /* 001001010010 */
    DpregClass::AddsubExt,   /* 001001010011 */
    DpregClass::AddsubExt,   /* 001001010100 */
    DpregClass::AddsubExt,   /* 001001010101 */
    DpregClass::AddsubExt,   /* 001001010110 */
    DpregClass::AddsubExt,   /* 001001010111 */
    DpregClass::AddsubExt,   /* 001001011000 */
    DpregClass::AddsubExt,   /* 001001011001 */
    DpregClass::AddsubExt,   /* 001001011010 */
    DpregClass::AddsubExt,   /* 001001011011 */
    DpregClass::AddsubExt,   /* 001001011100 */
    DpregClass::AddsubExt,   /* 001001011101 */
    DpregClass::AddsubExt,   /* 001001011110 */
    DpregClass::AddsubExt,   /* 001001011111 */
    DpregClass::AddsubExt,   /* 001001100000 */
    DpregClass::AddsubExt,   /* 001001100001 */
    DpregClass::AddsubExt,   /* 001001100010 */
    DpregClass::AddsubExt,   /* 001001100011 */
    DpregClass::AddsubExt,   /* 001001100100 */
    DpregClass::AddsubExt,   /* 001001100101 */
    DpregClass::AddsubExt,   /* 001001100110 */
    DpregClass::AddsubExt,   /* 001001100111 */
    DpregClass::AddsubExt,   /* 001001101000 */
    DpregClass::AddsubExt,   /* 001001101001 */
    DpregClass::AddsubExt,   /* 001001101010 */
    DpregClass::AddsubExt,   /* 001001101011 */
    DpregClass::AddsubExt,   /* 001001101100 */
    DpregClass::AddsubExt,   /* 001001101101 */
    DpregClass::AddsubExt,   /* 001001101110 */
    DpregClass::AddsubExt,   /* 001001101111 */
    DpregClass::AddsubExt,   /* 001001110000 */
    DpregClass::AddsubExt,   /* 001001110001 */
    DpregClass::AddsubExt,   /* 001001110010 */
    DpregClass::AddsubExt,   /* 001001110011 */
    DpregClass::AddsubExt,   /* 001001110100 */
    DpregClass::AddsubExt,   /* 001001110101 */
    DpregClass::AddsubExt,   /* 001001110110 */
    DpregClass::AddsubExt,   /* 001001110111 */
    DpregClass::AddsubExt,   /* 001001111000 */
    DpregClass::AddsubExt,   /* 001001111001 */
    DpregClass::AddsubExt,   /* 001001111010 */
    DpregClass::AddsubExt,   /* 001001111011 */
    DpregClass::AddsubExt,   /* 001001111100 */
    DpregClass::AddsubExt,   /* 001001111101 */
    DpregClass::AddsubExt,   /* 001001111110 */
    DpregClass::AddsubExt,   /* 001001111111 */
    DpregClass::AddsubShift, /* 001010000000 */
    DpregClass::AddsubShift, /* 001010000001 */
    DpregClass::AddsubShift, /* 001010000010 */
    DpregClass::AddsubShift, /* 001010000011 */
    DpregClass::AddsubShift, /* 001010000100 */
    DpregClass::AddsubShift, /* 001010000101 */
    DpregClass::AddsubShift, /* 001010000110 */
    DpregClass::AddsubShift, /* 001010000111 */
    DpregClass::AddsubShift, /* 001010001000 */
    DpregClass::AddsubShift, /* 001010001001 */
    DpregClass::AddsubShift, /* 001010001010 */
    DpregClass::AddsubShift, /* 001010001011 */
    DpregClass::AddsubShift, /* 001010001100 */
    DpregClass::AddsubShift, /* 001010001101 */
    DpregClass::AddsubShift, /* 001010001110 */
    DpregClass::AddsubShift, /* 001010001111 */
    DpregClass::AddsubShift, /* 001010010000 */
    DpregClass::AddsubShift, /* 001010010001 */
    DpregClass::AddsubShift, /* 001010010010 */
    DpregClass::AddsubShift, /* 001010010011 */
    DpregClass::AddsubShift, /* 001010010100 */
    DpregClass::AddsubShift, /* 001010010101 */
    DpregClass::AddsubShift, /* 001010010110 */
    DpregClass::AddsubShift, /* 001010010111 */
    DpregClass::AddsubShift, /* 001010011000 */
    DpregClass::AddsubShift, /* 001010011001 */
    DpregClass::AddsubShift, /* 001010011010 */
    DpregClass::AddsubShift, /* 001010011011 */
    DpregClass::AddsubShift, /* 001010011100 */
    DpregClass::AddsubShift, /* 001010011101 */
    DpregClass::AddsubShift, /* 001010011110 */
    DpregClass::AddsubShift, /* 001010011111 */
    DpregClass::AddsubShift, /* 001010100000 */
    DpregClass::AddsubShift, /* 001010100001 */
    DpregClass::AddsubShift, /* 001010100010 */
    DpregClass::AddsubShift, /* 001010100011 */
    DpregClass::AddsubShift, /* 001010100100 */
    DpregClass::AddsubShift, /* 001010100101 */
    DpregClass::AddsubShift, /* 001010100110 */
    DpregClass::AddsubShift, /* 001010100111 */
    DpregClass::AddsubShift, /* 001010101000 */
    DpregClass::AddsubShift, /* 001010101001 */
    DpregClass::AddsubShift, /* 001010101010 */
    DpregClass::AddsubShift, /* 001010101011 */
    DpregClass::AddsubShift, /* 001010101100 */
    DpregClass::AddsubShift, /* 001010101101 */
    DpregClass::AddsubShift, /* 001010101110 */
    DpregClass::AddsubShift, /* 001010101111 */
    DpregClass::AddsubShift, /* 001010110000 */
    DpregClass::AddsubShift, /* 001010110001 */
    DpregClass::AddsubShift, /* 001010110010 */
    DpregClass::AddsubShift, /* 001010110011 */
    DpregClass::AddsubShift, /* 001010110100 */
    DpregClass::AddsubShift, /* 001010110101 */
    DpregClass::AddsubShift, /* 001010110110 */
    DpregClass::AddsubShift, /* 001010110111 */
    DpregClass::AddsubShift, /* 001010111000 */
    DpregClass::AddsubShift, /* 001010111001 */
    DpregClass::AddsubShift, /* 001010111010 */
    DpregClass::AddsubShift, /* 001010111011 */
    DpregClass::AddsubShift, /* 001010111100 */
    DpregClass::AddsubShift, /* 001010111101 */
    DpregClass::AddsubShift, /* 001010111110 */
    DpregClass::AddsubShift, /* 001010111111 */
    DpregClass::AddsubExt,   /* 001011000000 */
    DpregClass::AddsubExt,   /* 001011000001 */
    DpregClass::AddsubExt,   /* 001011000010 */
    DpregClass::AddsubExt,   /* 001011000011 */
    DpregClass::AddsubExt,   /* 001011000100 */
    DpregClass::AddsubExt,   /* 001011000101 */
    DpregClass::AddsubExt,   /* 001011000110 */
    DpregClass::AddsubExt,   /* 001011000111 */
    DpregClass::AddsubExt,   /* 001011001000 */
    DpregClass::AddsubExt,   /* 001011001001 */
    DpregClass::AddsubExt,   /* 001011001010 */
    DpregClass::AddsubExt,   /* 001011001011 */
    DpregClass::AddsubExt,   /* 001011001100 */
    DpregClass::AddsubExt,   /* 001011001101 */
    DpregClass::AddsubExt,   /* 001011001110 */
    DpregClass::AddsubExt,   /* 001011001111 */
    DpregClass::AddsubExt,   /* 001011010000 */
    DpregClass::AddsubExt,   /* 001011010001 */
    DpregClass::AddsubExt,   /* 001011010010 */
    DpregClass::AddsubExt,   /* 001011010011 */
    DpregClass::AddsubExt,   /* 001011010100 */
    DpregClass::AddsubExt,   /* 001011010101 */
    DpregClass::AddsubExt,   /* 001011010110 */
    DpregClass::AddsubExt,   /* 001011010111 */
    DpregClass::AddsubExt,   /* 001011011000 */
    DpregClass::AddsubExt,   /* 001011011001 */
    DpregClass::AddsubExt,   /* 001011011010 */
    DpregClass::AddsubExt,   /* 001011011011 */
    DpregClass::AddsubExt,   /* 001011011100 */
    DpregClass::AddsubExt,   /* 001011011101 */
    DpregClass::AddsubExt,   /* 001011011110 */
    DpregClass::AddsubExt,   /* 001011011111 */
    DpregClass::AddsubExt,   /* 001011100000 */
    DpregClass::AddsubExt,   /* 001011100001 */
    DpregClass::AddsubExt,   /* 001011100010 */
    DpregClass::AddsubExt,   /* 001011100011 */
    DpregClass::AddsubExt,   /* 001011100100 */
    DpregClass::AddsubExt,   /* 001011100101 */
    DpregClass::AddsubExt,   /* 001011100110 */
    DpregClass::AddsubExt,   /* 001011100111 */
    DpregClass::AddsubExt,   /* 001011101000 */
    DpregClass::AddsubExt,   /* 001011101001 */
    DpregClass::AddsubExt,   /* 001011101010 */
    DpregClass::AddsubExt,   /* 001011101011 */
    DpregClass::AddsubExt,   /* 001011101100 */
    DpregClass::AddsubExt,   /* 001011101101 */
    DpregClass::AddsubExt,   /* 001011101110 */
    DpregClass::AddsubExt,   /* 001011101111 */
    DpregClass::AddsubExt,   /* 001011110000 */
    DpregClass::AddsubExt,   /* 001011110001 */
    DpregClass::AddsubExt,   /* 001011110010 */
    DpregClass::AddsubExt,   /* 001011110011 */
    DpregClass::AddsubExt,   /* 001011110100 */
    DpregClass::AddsubExt,   /* 001011110101 */
    DpregClass::AddsubExt,   /* 001011110110 */
    DpregClass::AddsubExt,   /* 001011110111 */
    DpregClass::AddsubExt,   /* 001011111000 */
    DpregClass::AddsubExt,   /* 001011111001 */
    DpregClass::AddsubExt,   /* 001011111010 */
    DpregClass::AddsubExt,   /* 001011111011 */
    DpregClass::AddsubExt,   /* 001011111100 */
    DpregClass::AddsubExt,   /* 001011111101 */
    DpregClass::AddsubExt,   /* 001011111110 */
    DpregClass::AddsubExt,   /* 001011111111 */
    DpregClass::AddsubShift, /* 001100000000 */
    DpregClass::AddsubShift, /* 001100000001 */
    DpregClass::AddsubShift, /* 001100000010 */
    DpregClass::AddsubShift, /* 001100000011 */
    DpregClass::AddsubShift, /* 001100000100 */
    DpregClass::AddsubShift, /* 001100000101 */
    DpregClass::AddsubShift, /* 001100000110 */
    DpregClass::AddsubShift, /* 001100000111 */
    DpregClass::AddsubShift, /* 001100001000 */
    DpregClass::AddsubShift, /* 001100001001 */
    DpregClass::AddsubShift, /* 001100001010 */
    DpregClass::AddsubShift, /* 001100001011 */
    DpregClass::AddsubShift, /* 001100001100 */
    DpregClass::AddsubShift, /* 001100001101 */
    DpregClass::AddsubShift, /* 001100001110 */
    DpregClass::AddsubShift, /* 001100001111 */
    DpregClass::AddsubShift, /* 001100010000 */
    DpregClass::AddsubShift, /* 001100010001 */
    DpregClass::AddsubShift, /* 001100010010 */
    DpregClass::AddsubShift, /* 001100010011 */
    DpregClass::AddsubShift, /* 001100010100 */
    DpregClass::AddsubShift, /* 001100010101 */
    DpregClass::AddsubShift, /* 001100010110 */
    DpregClass::AddsubShift, /* 001100010111 */
    DpregClass::AddsubShift, /* 001100011000 */
    DpregClass::AddsubShift, /* 001100011001 */
    DpregClass::AddsubShift, /* 001100011010 */
    DpregClass::AddsubShift, /* 001100011011 */
    DpregClass::AddsubShift, /* 001100011100 */
    DpregClass::AddsubShift, /* 001100011101 */
    DpregClass::AddsubShift, /* 001100011110 */
    DpregClass::AddsubShift, /* 001100011111 */
    DpregClass::AddsubShift, /* 001100100000 */
    DpregClass::AddsubShift, /* 001100100001 */
    DpregClass::AddsubShift, /* 001100100010 */
    DpregClass::AddsubShift, /* 001100100011 */
    DpregClass::AddsubShift, /* 001100100100 */
    DpregClass::AddsubShift, /* 001100100101 */
    DpregClass::AddsubShift, /* 001100100110 */
    DpregClass::AddsubShift, /* 001100100111 */
    DpregClass::AddsubShift, /* 001100101000 */
    DpregClass::AddsubShift, /* 001100101001 */
    DpregClass::AddsubShift, /* 001100101010 */
    DpregClass::AddsubShift, /* 001100101011 */
    DpregClass::AddsubShift, /* 001100101100 */
    DpregClass::AddsubShift, /* 001100101101 */
    DpregClass::AddsubShift, /* 001100101110 */
    DpregClass::AddsubShift, /* 001100101111 */
    DpregClass::AddsubShift, /* 001100110000 */
    DpregClass::AddsubShift, /* 001100110001 */
    DpregClass::AddsubShift, /* 001100110010 */
    DpregClass::AddsubShift, /* 001100110011 */
    DpregClass::AddsubShift, /* 001100110100 */
    DpregClass::AddsubShift, /* 001100110101 */
    DpregClass::AddsubShift, /* 001100110110 */
    DpregClass::AddsubShift, /* 001100110111 */
    DpregClass::AddsubShift, /* 001100111000 */
    DpregClass::AddsubShift, /* 001100111001 */
    DpregClass::AddsubShift, /* 001100111010 */
    DpregClass::AddsubShift, /* 001100111011 */
    DpregClass::AddsubShift, /* 001100111100 */
    DpregClass::AddsubShift, /* 001100111101 */
    DpregClass::AddsubShift, /* 001100111110 */
    DpregClass::AddsubShift, /* 001100111111 */
    DpregClass::AddsubExt,   /* 001101000000 */
    DpregClass::AddsubExt,   /* 001101000001 */
    DpregClass::AddsubExt,   /* 001101000010 */
    DpregClass::AddsubExt,   /* 001101000011 */
    DpregClass::AddsubExt,   /* 001101000100 */
    DpregClass::AddsubExt,   /* 001101000101 */
    DpregClass::AddsubExt,   /* 001101000110 */
    DpregClass::AddsubExt,   /* 001101000111 */
    DpregClass::AddsubExt,   /* 001101001000 */
    DpregClass::AddsubExt,   /* 001101001001 */
    DpregClass::AddsubExt,   /* 001101001010 */
    DpregClass::AddsubExt,   /* 001101001011 */
    DpregClass::AddsubExt,   /* 001101001100 */
    DpregClass::AddsubExt,   /* 001101001101 */
    DpregClass::AddsubExt,   /* 001101001110 */
    DpregClass::AddsubExt,   /* 001101001111 */
    DpregClass::AddsubExt,   /* 001101010000 */
    DpregClass::AddsubExt,   /* 001101010001 */
    DpregClass::AddsubExt,   /* 001101010010 */
    DpregClass::AddsubExt,   /* 001101010011 */
    DpregClass::AddsubExt,   /* 001101010100 */
    DpregClass::AddsubExt,   /* 001101010101 */
    DpregClass::AddsubExt,   /* 001101010110 */
    DpregClass::AddsubExt,   /* 001101010111 */
    DpregClass::AddsubExt,   /* 001101011000 */
    DpregClass::AddsubExt,   /* 001101011001 */
    DpregClass::AddsubExt,   /* 001101011010 */
    DpregClass::AddsubExt,   /* 001101011011 */
    DpregClass::AddsubExt,   /* 001101011100 */
    DpregClass::AddsubExt,   /* 001101011101 */
    DpregClass::AddsubExt,   /* 001101011110 */
    DpregClass::AddsubExt,   /* 001101011111 */
    DpregClass::AddsubExt,   /* 001101100000 */
    DpregClass::AddsubExt,   /* 001101100001 */
    DpregClass::AddsubExt,   /* 001101100010 */
    DpregClass::AddsubExt,   /* 001101100011 */
    DpregClass::AddsubExt,   /* 001101100100 */
    DpregClass::AddsubExt,   /* 001101100101 */
    DpregClass::AddsubExt,   /* 001101100110 */
    DpregClass::AddsubExt,   /* 001101100111 */
    DpregClass::AddsubExt,   /* 001101101000 */
    DpregClass::AddsubExt,   /* 001101101001 */
    DpregClass::AddsubExt,   /* 001101101010 */
    DpregClass::AddsubExt,   /* 001101101011 */
    DpregClass::AddsubExt,   /* 001101101100 */
    DpregClass::AddsubExt,   /* 001101101101 */
    DpregClass::AddsubExt,   /* 001101101110 */
    DpregClass::AddsubExt,   /* 001101101111 */
    DpregClass::AddsubExt,   /* 001101110000 */
    DpregClass::AddsubExt,   /* 001101110001 */
    DpregClass::AddsubExt,   /* 001101110010 */
    DpregClass::AddsubExt,   /* 001101110011 */
    DpregClass::AddsubExt,   /* 001101110100 */
    DpregClass::AddsubExt,   /* 001101110101 */
    DpregClass::AddsubExt,   /* 001101110110 */
    DpregClass::AddsubExt,   /* 001101110111 */
    DpregClass::AddsubExt,   /* 001101111000 */
    DpregClass::AddsubExt,   /* 001101111001 */
    DpregClass::AddsubExt,   /* 001101111010 */
    DpregClass::AddsubExt,   /* 001101111011 */
    DpregClass::AddsubExt,   /* 001101111100 */
    DpregClass::AddsubExt,   /* 001101111101 */
    DpregClass::AddsubExt,   /* 001101111110 */
    DpregClass::AddsubExt,   /* 001101111111 */
    DpregClass::AddsubShift, /* 001110000000 */
    DpregClass::AddsubShift, /* 001110000001 */
    DpregClass::AddsubShift, /* 001110000010 */
    DpregClass::AddsubShift, /* 001110000011 */
    DpregClass::AddsubShift, /* 001110000100 */
    DpregClass::AddsubShift, /* 001110000101 */
    DpregClass::AddsubShift, /* 001110000110 */
    DpregClass::AddsubShift, /* 001110000111 */
    DpregClass::AddsubShift, /* 001110001000 */
    DpregClass::AddsubShift, /* 001110001001 */
    DpregClass::AddsubShift, /* 001110001010 */
    DpregClass::AddsubShift, /* 001110001011 */
    DpregClass::AddsubShift, /* 001110001100 */
    DpregClass::AddsubShift, /* 001110001101 */
    DpregClass::AddsubShift, /* 001110001110 */
    DpregClass::AddsubShift, /* 001110001111 */
    DpregClass::AddsubShift, /* 001110010000 */
    DpregClass::AddsubShift, /* 001110010001 */
    DpregClass::AddsubShift, /* 001110010010 */
    DpregClass::AddsubShift, /* 001110010011 */
    DpregClass::AddsubShift, /* 001110010100 */
    DpregClass::AddsubShift, /* 001110010101 */
    DpregClass::AddsubShift, /* 001110010110 */
    DpregClass::AddsubShift, /* 001110010111 */
    DpregClass::AddsubShift, /* 001110011000 */
    DpregClass::AddsubShift, /* 001110011001 */
    DpregClass::AddsubShift, /* 001110011010 */
    DpregClass::AddsubShift, /* 001110011011 */
    DpregClass::AddsubShift, /* 001110011100 */
    DpregClass::AddsubShift, /* 001110011101 */
    DpregClass::AddsubShift, /* 001110011110 */
    DpregClass::AddsubShift, /* 001110011111 */
    DpregClass::AddsubShift, /* 001110100000 */
    DpregClass::AddsubShift, /* 001110100001 */
    DpregClass::AddsubShift, /* 001110100010 */
    DpregClass::AddsubShift, /* 001110100011 */
    DpregClass::AddsubShift, /* 001110100100 */
    DpregClass::AddsubShift, /* 001110100101 */
    DpregClass::AddsubShift, /* 001110100110 */
    DpregClass::AddsubShift, /* 001110100111 */
    DpregClass::AddsubShift, /* 001110101000 */
    DpregClass::AddsubShift, /* 001110101001 */
    DpregClass::AddsubShift, /* 001110101010 */
    DpregClass::AddsubShift, /* 001110101011 */
    DpregClass::AddsubShift, /* 001110101100 */
    DpregClass::AddsubShift, /* 001110101101 */
    DpregClass::AddsubShift, /* 001110101110 */
    DpregClass::AddsubShift, /* 001110101111 */
    DpregClass::AddsubShift, /* 001110110000 */
    DpregClass::AddsubShift, /* 001110110001 */
    DpregClass::AddsubShift, /* 001110110010 */
    DpregClass::AddsubShift, /* 001110110011 */
    DpregClass::AddsubShift, /* 001110110100 */
    DpregClass::AddsubShift, /* 001110110101 */
    DpregClass::AddsubShift, /* 001110110110 */
    DpregClass::AddsubShift, /* 001110110111 */
    DpregClass::AddsubShift, /* 001110111000 */
    DpregClass::AddsubShift, /* 001110111001 */
    DpregClass::AddsubShift, /* 001110111010 */
    DpregClass::AddsubShift, /* 001110111011 */
    DpregClass::AddsubShift, /* 001110111100 */
    DpregClass::AddsubShift, /* 001110111101 */
    DpregClass::AddsubShift, /* 001110111110 */
    DpregClass::AddsubShift, /* 001110111111 */
    DpregClass::AddsubExt,   /* 001111000000 */
    DpregClass::AddsubExt,   /* 001111000001 */
    DpregClass::AddsubExt,   /* 001111000010 */
    DpregClass::AddsubExt,   /* 001111000011 */
    DpregClass::AddsubExt,   /* 001111000100 */
    DpregClass::AddsubExt,   /* 001111000101 */
    DpregClass::AddsubExt,   /* 001111000110 */
    DpregClass::AddsubExt,   /* 001111000111 */
    DpregClass::AddsubExt,   /* 001111001000 */
    DpregClass::AddsubExt,   /* 001111001001 */
    DpregClass::AddsubExt,   /* 001111001010 */
    DpregClass::AddsubExt,   /* 001111001011 */
    DpregClass::AddsubExt,   /* 001111001100 */
    DpregClass::AddsubExt,   /* 001111001101 */
    DpregClass::AddsubExt,   /* 001111001110 */
    DpregClass::AddsubExt,   /* 001111001111 */
    DpregClass::AddsubExt,   /* 001111010000 */
    DpregClass::AddsubExt,   /* 001111010001 */
    DpregClass::AddsubExt,   /* 001111010010 */
    DpregClass::AddsubExt,   /* 001111010011 */
    DpregClass::AddsubExt,   /* 001111010100 */
    DpregClass::AddsubExt,   /* 001111010101 */
    DpregClass::AddsubExt,   /* 001111010110 */
    DpregClass::AddsubExt,   /* 001111010111 */
    DpregClass::AddsubExt,   /* 001111011000 */
    DpregClass::AddsubExt,   /* 001111011001 */
    DpregClass::AddsubExt,   /* 001111011010 */
    DpregClass::AddsubExt,   /* 001111011011 */
    DpregClass::AddsubExt,   /* 001111011100 */
    DpregClass::AddsubExt,   /* 001111011101 */
    DpregClass::AddsubExt,   /* 001111011110 */
    DpregClass::AddsubExt,   /* 001111011111 */
    DpregClass::AddsubExt,   /* 001111100000 */
    DpregClass::AddsubExt,   /* 001111100001 */
    DpregClass::AddsubExt,   /* 001111100010 */
    DpregClass::AddsubExt,   /* 001111100011 */
    DpregClass::AddsubExt,   /* 001111100100 */
    DpregClass::AddsubExt,   /* 001111100101 */
    DpregClass::AddsubExt,   /* 001111100110 */
    DpregClass::AddsubExt,   /* 001111100111 */
    DpregClass::AddsubExt,   /* 001111101000 */
    DpregClass::AddsubExt,   /* 001111101001 */
    DpregClass::AddsubExt,   /* 001111101010 */
    DpregClass::AddsubExt,   /* 001111101011 */
    DpregClass::AddsubExt,   /* 001111101100 */
    DpregClass::AddsubExt,   /* 001111101101 */
    DpregClass::AddsubExt,   /* 001111101110 */
    DpregClass::AddsubExt,   /* 001111101111 */
    DpregClass::AddsubExt,   /* 001111110000 */
    DpregClass::AddsubExt,   /* 001111110001 */
    DpregClass::AddsubExt,   /* 001111110010 */
    DpregClass::AddsubExt,   /* 001111110011 */
    DpregClass::AddsubExt,   /* 001111110100 */
    DpregClass::AddsubExt,   /* 001111110101 */
    DpregClass::AddsubExt,   /* 001111110110 */
    DpregClass::AddsubExt,   /* 001111110111 */
    DpregClass::AddsubExt,   /* 001111111000 */
    DpregClass::AddsubExt,   /* 001111111001 */
    DpregClass::AddsubExt,   /* 001111111010 */
    DpregClass::AddsubExt,   /* 001111111011 */
    DpregClass::AddsubExt,   /* 001111111100 */
    DpregClass::AddsubExt,   /* 001111111101 */
    DpregClass::AddsubExt,   /* 001111111110 */
    DpregClass::AddsubExt,   /* 001111111111 */
    DpregClass::AddsubCarry, /* 010000000000 */
    DpregClass::Rmif,        /* 010000000001 */
    DpregClass::Setf,        /* 010000000010 */
    DpregClass::Unallocated, /* 010000000011 */
    DpregClass::Unallocated, /* 010000000100 */
    DpregClass::Unallocated, /* 010000000101 */
    DpregClass::Unallocated, /* 010000000110 */
    DpregClass::Unallocated, /* 010000000111 */
    DpregClass::AddsubPt,    /* 010000001000 */
    DpregClass::AddsubPt,    /* 010000001001 */
    DpregClass::AddsubPt,    /* 010000001010 */
    DpregClass::AddsubPt,    /* 010000001011 */
    DpregClass::AddsubPt,    /* 010000001100 */
    DpregClass::AddsubPt,    /* 010000001101 */
    DpregClass::AddsubPt,    /* 010000001110 */
    DpregClass::AddsubPt,    /* 010000001111 */
    DpregClass::Unallocated, /* 010000010000 */
    DpregClass::Unallocated, /* 010000010001 */
    DpregClass::Setf,        /* 010000010010 */
    DpregClass::Unallocated, /* 010000010011 */
    DpregClass::Unallocated, /* 010000010100 */
    DpregClass::Unallocated, /* 010000010101 */
    DpregClass::Unallocated, /* 010000010110 */
    DpregClass::Unallocated, /* 010000010111 */
    DpregClass::Unallocated, /* 010000011000 */
    DpregClass::Unallocated, /* 010000011001 */
    DpregClass::Unallocated, /* 010000011010 */
    DpregClass::Unallocated, /* 010000011011 */
    DpregClass::Unallocated, /* 010000011100 */
    DpregClass::Unallocated, /* 010000011101 */
    DpregClass::Unallocated, /* 010000011110 */
    DpregClass::Unallocated, /* 010000011111 */
    DpregClass::Unallocated, /* 010000100000 */
    DpregClass::Rmif,        /* 010000100001 */
    DpregClass::Setf,        /* 010000100010 */
    DpregClass::Unallocated, /* 010000100011 */
    DpregClass::Unallocated, /* 010000100100 */
    DpregClass::Unallocated, /* 010000100101 */
    DpregClass::Unallocated, /* 010000100110 */
    DpregClass::Unallocated, /* 010000100111 */
    DpregClass::Unallocated, /* 010000101000 */
    DpregClass::Unallocated, /* 010000101001 */
    DpregClass::Unallocated, /* 010000101010 */
    DpregClass::Unallocated, /* 010000101011 */
    DpregClass::Unallocated, /* 010000101100 */
    DpregClass::Unallocated, /* 010000101101 */
    DpregClass::Unallocated, /* 010000101110 */
    DpregClass::Unallocated, /* 010000101111 */
    DpregClass::Unallocated, /* 010000110000 */
    DpregClass::Unallocated, /* 010000110001 */
    DpregClass::Setf,        /* 010000110010 */
    DpregClass::Unallocated, /* 010000110011 */
    DpregClass::Unallocated, /* 010000110100 */
    DpregClass::Unallocated, /* 010000110101 */
    DpregClass::Unallocated, /* 010000110110 */
    DpregClass::Unallocated, /* 010000110111 */
    DpregClass::Unallocated, /* 010000111000 */
    DpregClass::Unallocated, /* 010000111001 */
    DpregClass::Unallocated, /* 010000111010 */
    DpregClass::Unallocated, /* 010000111011 */
    DpregClass::Unallocated, /* 010000111100 */
    DpregClass::Unallocated, /* 010000111101 */
    DpregClass::Unallocated, /* 010000111110 */
    DpregClass::Unallocated, /* 010000111111 */
    DpregClass::Unallocated, /* 010001000000 */
    DpregClass::Unallocated, /* 010001000001 */
    DpregClass::Unallocated, /* 010001000010 */
    DpregClass::Unallocated, /* 010001000011 */
    DpregClass::Unallocated, /* 010001000100 */
    DpregClass::Unallocated, /* 010001000101 */
    DpregClass::Unallocated, /* 010001000110 */
    DpregClass::Unallocated, /* 010001000111 */
    DpregClass::Unallocated, /* 010001001000 */
    DpregClass::Unallocated, /* 010001001001 */
    DpregClass::Unallocated, /* 010001001010 */
    DpregClass::Unallocated, /* 010001001011 */
    DpregClass::Unallocated, /* 010001001100 */
    DpregClass::Unallocated, /* 010001001101 */
    DpregClass::Unallocated, /* 010001001110 */
    DpregClass::Unallocated, /* 010001001111 */
    DpregClass::Unallocated, /* 010001010000 */
    DpregClass::Unallocated, /* 010001010001 */
    DpregClass::Unallocated, /* 010001010010 */
    DpregClass::Unallocated, /* 010001010011 */
    DpregClass::Unallocated, /* 010001010100 */
    DpregClass::Unallocated, /* 010001010101 */
    DpregClass::Unallocated, /* 010001010110 */
    DpregClass::Unallocated, /* 010001010111 */
    DpregClass::Unallocated, /* 010001011000 */
    DpregClass::Unallocated, /* 010001011001 */
    DpregClass::Unallocated, /* 010001011010 */
    DpregClass::Unallocated, /* 010001011011 */
    DpregClass::Unallocated, /* 010001011100 */
    DpregClass::Unallocated, /* 010001011101 */
    DpregClass::Unallocated, /* 010001011110 */
    DpregClass::Unallocated, /* 010001011111 */
    DpregClass::Unallocated, /* 010001100000 */
    DpregClass::Unallocated, /* 010001100001 */
    DpregClass::Unallocated, /* 010001100010 */
    DpregClass::Unallocated, /* 010001100011 */
    DpregClass::Unallocated, /* 010001100100 */
    DpregClass::Unallocated, /* 010001100101 */
    DpregClass::Unallocated, /* 010001100110 */
    DpregClass::Unallocated, /* 010001100111 */
    DpregClass::Unallocated, /* 010001101000 */
    DpregClass::Unallocated, /* 010001101001 */
    DpregClass::Unallocated, /* 010001101010 */
    DpregClass::Unallocated, /* 010001101011 */
    DpregClass::Unallocated, /* 010001101100 */
    DpregClass::Unallocated, /* 010001101101 */
    DpregClass::Unallocated, /* 010001101110 */
    DpregClass::Unallocated, /* 010001101111 */
    DpregClass::Unallocated, /* 010001110000 */
    DpregClass::Unallocated, /* 010001110001 */
    DpregClass::Unallocated, /* 010001110010 */
    DpregClass::Unallocated, /* 010001110011 */
    DpregClass::Unallocated, /* 010001110100 */
    DpregClass::Unallocated, /* 010001110101 */
    DpregClass::Unallocated, /* 010001110110 */
    DpregClass::Unallocated, /* 010001110111 */
    DpregClass::Unallocated, /* 010001111000 */
    DpregClass::Unallocated, /* 010001111001 */
    DpregClass::Unallocated, /* 010001111010 */
    DpregClass::Unallocated, /* 010001111011 */
    DpregClass::Unallocated, /* 010001111100 */
    DpregClass::Unallocated, /* 010001111101 */
    DpregClass::Unallocated, /* 010001111110 */
    DpregClass::Unallocated, /* 010001111111 */
    DpregClass::CondcmpReg,  /* 010010000000 */
    DpregClass::CondcmpReg,  /* 010010000001 */
    DpregClass::CondcmpImm,  /* 010010000010 */
    DpregClass::CondcmpImm,  /* 010010000011 */
    DpregClass::CondcmpReg,  /* 010010000100 */
    DpregClass::CondcmpReg,  /* 010010000101 */
    DpregClass::CondcmpImm,  /* 010010000110 */
    DpregClass::CondcmpImm,  /* 010010000111 */
    DpregClass::CondcmpReg,  /* 010010001000 */
    DpregClass::CondcmpReg,  /* 010010001001 */
    DpregClass::CondcmpImm,  /* 010010001010 */
    DpregClass::CondcmpImm,  /* 010010001011 */
    DpregClass::CondcmpReg,  /* 010010001100 */
    DpregClass::CondcmpReg,  /* 010010001101 */
    DpregClass::CondcmpImm,  /* 010010001110 */
    DpregClass::CondcmpImm,  /* 010010001111 */
    DpregClass::CondcmpReg,  /* 010010010000 */
    DpregClass::CondcmpReg,  /* 010010010001 */
    DpregClass::CondcmpImm,  /* 010010010010 */
    DpregClass::CondcmpImm,  /* 010010010011 */
    DpregClass::CondcmpReg,  /* 010010010100 */
    DpregClass::CondcmpReg,  /* 010010010101 */
    DpregClass::CondcmpImm,  /* 010010010110 */
    DpregClass::CondcmpImm,  /* 010010010111 */
    DpregClass::CondcmpReg,  /* 010010011000 */
    DpregClass::CondcmpReg,  /* 010010011001 */
    DpregClass::CondcmpImm,  /* 010010011010 */
    DpregClass::CondcmpImm,  /* 010010011011 */
    DpregClass::CondcmpReg,  /* 010010011100 */
    DpregClass::CondcmpReg,  /* 010010011101 */
    DpregClass::CondcmpImm,  /* 010010011110 */
    DpregClass::CondcmpImm,  /* 010010011111 */
    DpregClass::CondcmpReg,  /* 010010100000 */
    DpregClass::CondcmpReg,  /* 010010100001 */
    DpregClass::CondcmpImm,  /* 010010100010 */
    DpregClass::CondcmpImm,  /* 010010100011 */
    DpregClass::CondcmpReg,  /* 010010100100 */
    DpregClass::CondcmpReg,  /* 010010100101 */
    DpregClass::CondcmpImm,  /* 010010100110 */
    DpregClass::CondcmpImm,  /* 010010100111 */
    DpregClass::CondcmpReg,  /* 010010101000 */
    DpregClass::CondcmpReg,  /* 010010101001 */
    DpregClass::CondcmpImm,  /* 010010101010 */
    DpregClass::CondcmpImm,  /* 010010101011 */
    DpregClass::CondcmpReg,  /* 010010101100 */
    DpregClass::CondcmpReg,  /* 010010101101 */
    DpregClass::CondcmpImm,  /* 010010101110 */
    DpregClass::CondcmpImm,  /* 010010101111 */
    DpregClass::CondcmpReg,  /* 010010110000 */
    DpregClass::CondcmpReg,  /* 010010110001 */
    DpregClass::CondcmpImm,  /* 010010110010 */
    DpregClass::CondcmpImm,  /* 010010110011 */
    DpregClass::CondcmpReg,  /* 010010110100 */
    DpregClass::CondcmpReg,  /* 010010110101 */
    DpregClass::CondcmpImm,  /* 010010110110 */
    DpregClass::CondcmpImm,  /* 010010110111 */
    DpregClass::CondcmpReg,  /* 010010111000 */
    DpregClass::CondcmpReg,  /* 010010111001 */
    DpregClass::CondcmpImm,  /* 010010111010 */
    DpregClass::CondcmpImm,  /* 010010111011 */
    DpregClass::CondcmpReg,  /* 010010111100 */
    DpregClass::CondcmpReg,  /* 010010111101 */
    DpregClass::CondcmpImm,  /* 010010111110 */
    DpregClass::CondcmpImm,  /* 010010111111 */
    DpregClass::Unallocated, /* 010011000000 */
    DpregClass::Unallocated, /* 010011000001 */
    DpregClass::Unallocated, /* 010011000010 */
    DpregClass::Unallocated, /* 010011000011 */
    DpregClass::Unallocated, /* 010011000100 */
    DpregClass::Unallocated, /* 010011000101 */
    DpregClass::Unallocated, /* 010011000110 */
    DpregClass::Unallocated, /* 010011000111 */
    DpregClass::Unallocated, /* 010011001000 */
    DpregClass::Unallocated, /* 010011001001 */
    DpregClass::Unallocated, /* 010011001010 */
    DpregClass::Unallocated, /* 010011001011 */
    DpregClass::Unallocated, /* 010011001100 */
    DpregClass::Unallocated, /* 010011001101 */
    DpregClass::Unallocated, /* 010011001110 */
    DpregClass::Unallocated, /* 010011001111 */
    DpregClass::Unallocated, /* 010011010000 */
    DpregClass::Unallocated, /* 010011010001 */
    DpregClass::Unallocated, /* 010011010010 */
    DpregClass::Unallocated, /* 010011010011 */
    DpregClass::Unallocated, /* 010011010100 */
    DpregClass::Unallocated, /* 010011010101 */
    DpregClass::Unallocated, /* 010011010110 */
    DpregClass::Unallocated, /* 010011010111 */
    DpregClass::Unallocated, /* 010011011000 */
    DpregClass::Unallocated, /* 010011011001 */
    DpregClass::Unallocated, /* 010011011010 */
    DpregClass::Unallocated, /* 010011011011 */
    DpregClass::Unallocated, /* 010011011100 */
    DpregClass::Unallocated, /* 010011011101 */
    DpregClass::Unallocated, /* 010011011110 */
    DpregClass::Unallocated, /* 010011011111 */
    DpregClass::Unallocated, /* 010011100000 */
    DpregClass::Unallocated, /* 010011100001 */
    DpregClass::Unallocated, /* 010011100010 */
    DpregClass::Unallocated, /* 010011100011 */
    DpregClass::Unallocated, /* 010011100100 */
    DpregClass::Unallocated, /* 010011100101 */
    DpregClass::Unallocated, /* 010011100110 */
    DpregClass::Unallocated, /* 010011100111 */
    DpregClass::Unallocated, /* 010011101000 */
    DpregClass::Unallocated, /* 010011101001 */
    DpregClass::Unallocated, /* 010011101010 */
    DpregClass::Unallocated, /* 010011101011 */
    DpregClass::Unallocated, /* 010011101100 */
    DpregClass::Unallocated, /* 010011101101 */
    DpregClass::Unallocated, /* 010011101110 */
    DpregClass::Unallocated, /* 010011101111 */
    DpregClass::Unallocated, /* 010011110000 */
    DpregClass::Unallocated, /* 010011110001 */
    DpregClass::Unallocated, /* 010011110010 */
    DpregClass::Unallocated, /* 010011110011 */
    DpregClass::Unallocated, /* 010011110100 */
    DpregClass::Unallocated, /* 010011110101 */
    DpregClass::Unallocated, /* 010011110110 */
    DpregClass::Unallocated, /* 010011110111 */
    DpregClass::Unallocated, /* 010011111000 */
    DpregClass::Unallocated, /* 010011111001 */
    DpregClass::Unallocated, /* 010011111010 */
    DpregClass::Unallocated, /* 010011111011 */
    DpregClass::Unallocated, /* 010011111100 */
    DpregClass::Unallocated, /* 010011111101 */
    DpregClass::Unallocated, /* 010011111110 */
    DpregClass::Unallocated, /* 010011111111 */
    DpregClass::Condsel,     /* 010100000000 */
    DpregClass::Condsel,     /* 010100000001 */
    DpregClass::Condsel,     /* 010100000010 */
    DpregClass::Condsel,     /* 010100000011 */
    DpregClass::Condsel,     /* 010100000100 */
    DpregClass::Condsel,     /* 010100000101 */
    DpregClass::Condsel,     /* 010100000110 */
    DpregClass::Condsel,     /* 010100000111 */
    DpregClass::Condsel,     /* 010100001000 */
    DpregClass::Condsel,     /* 010100001001 */
    DpregClass::Condsel,     /* 010100001010 */
    DpregClass::Condsel,     /* 010100001011 */
    DpregClass::Condsel,     /* 010100001100 */
    DpregClass::Condsel,     /* 010100001101 */
    DpregClass::Condsel,     /* 010100001110 */
    DpregClass::Condsel,     /* 010100001111 */
    DpregClass::Condsel,     /* 010100010000 */
    DpregClass::Condsel,     /* 010100010001 */
    DpregClass::Condsel,     /* 010100010010 */
    DpregClass::Condsel,     /* 010100010011 */
    DpregClass::Condsel,     /* 010100010100 */
    DpregClass::Condsel,     /* 010100010101 */
    DpregClass::Condsel,     /* 010100010110 */
    DpregClass::Condsel,     /* 010100010111 */
    DpregClass::Condsel,     /* 010100011000 */
    DpregClass::Condsel,     /* 010100011001 */
    DpregClass::Condsel,     /* 010100011010 */
    DpregClass::Condsel,     /* 010100011011 */
    DpregClass::Condsel,     /* 010100011100 */
    DpregClass::Condsel,     /* 010100011101 */
    DpregClass::Condsel,     /* 010100011110 */
    DpregClass::Condsel,     /* 010100011111 */
    DpregClass::Condsel,     /* 010100100000 */
    DpregClass::Condsel,     /* 010100100001 */
    DpregClass::Condsel,     /* 010100100010 */
    DpregClass::Condsel,     /* 010100100011 */
    DpregClass::Condsel,     /* 010100100100 */
    DpregClass::Condsel,     /* 010100100101 */
    DpregClass::Condsel,     /* 010100100110 */
    DpregClass::Condsel,     /* 010100100111 */
    DpregClass::Condsel,     /* 010100101000 */
    DpregClass::Condsel,     /* 010100101001 */
    DpregClass::Condsel,     /* 010100101010 */
    DpregClass::Condsel,     /* 010100101011 */
    DpregClass::Condsel,     /* 010100101100 */
    DpregClass::Condsel,     /* 010100101101 */
    DpregClass::Condsel,     /* 010100101110 */
    DpregClass::Condsel,     /* 010100101111 */
    DpregClass::Condsel,     /* 010100110000 */
    DpregClass::Condsel,     /* 010100110001 */
    DpregClass::Condsel,     /* 010100110010 */
    DpregClass::Condsel,     /* 010100110011 */
    DpregClass::Condsel,     /* 010100110100 */
    DpregClass::Condsel,     /* 010100110101 */
    DpregClass::Condsel,     /* 010100110110 */
    DpregClass::Condsel,     /* 010100110111 */
    DpregClass::Condsel,     /* 010100111000 */
    DpregClass::Condsel,     /* 010100111001 */
    DpregClass::Condsel,     /* 010100111010 */
    DpregClass::Condsel,     /* 010100111011 */
    DpregClass::Condsel,     /* 010100111100 */
    DpregClass::Condsel,     /* 010100111101 */
    DpregClass::Condsel,     /* 010100111110 */
    DpregClass::Condsel,     /* 010100111111 */
    DpregClass::Unallocated, /* 010101000000 */
    DpregClass::Unallocated, /* 010101000001 */
    DpregClass::Unallocated, /* 010101000010 */
    DpregClass::Unallocated, /* 010101000011 */
    DpregClass::Unallocated, /* 010101000100 */
    DpregClass::Unallocated, /* 010101000101 */
    DpregClass::Unallocated, /* 010101000110 */
    DpregClass::Unallocated, /* 010101000111 */
    DpregClass::Unallocated, /* 010101001000 */
    DpregClass::Unallocated, /* 010101001001 */
    DpregClass::Unallocated, /* 010101001010 */
    DpregClass::Unallocated, /* 010101001011 */
    DpregClass::Unallocated, /* 010101001100 */
    DpregClass::Unallocated, /* 010101001101 */
    DpregClass::Unallocated, /* 010101001110 */
    DpregClass::Unallocated, /* 010101001111 */
    DpregClass::Unallocated, /* 010101010000 */
    DpregClass::Unallocated, /* 010101010001 */
    DpregClass::Unallocated, /* 010101010010 */
    DpregClass::Unallocated, /* 010101010011 */
    DpregClass::Unallocated, /* 010101010100 */
    DpregClass::Unallocated, /* 010101010101 */
    DpregClass::Unallocated, /* 010101010110 */
    DpregClass::Unallocated, /* 010101010111 */
    DpregClass::Unallocated, /* 010101011000 */
    DpregClass::Unallocated, /* 010101011001 */
    DpregClass::Unallocated, /* 010101011010 */
    DpregClass::Unallocated, /* 010101011011 */
    DpregClass::Unallocated, /* 010101011100 */
    DpregClass::Unallocated, /* 010101011101 */
    DpregClass::Unallocated, /* 010101011110 */
    DpregClass::Unallocated, /* 010101011111 */
    DpregClass::Unallocated, /* 010101100000 */
    DpregClass::Unallocated, /* 010101100001 */
    DpregClass::Unallocated, /* 010101100010 */
    DpregClass::Unallocated, /* 010101100011 */
    DpregClass::Unallocated, /* 010101100100 */
    DpregClass::Unallocated, /* 010101100101 */
    DpregClass::Unallocated, /* 010101100110 */
    DpregClass::Unallocated, /* 010101100111 */
    DpregClass::Unallocated, /* 010101101000 */
    DpregClass::Unallocated, /* 010101101001 */
    DpregClass::Unallocated, /* 010101101010 */
    DpregClass::Unallocated, /* 010101101011 */
    DpregClass::Unallocated, /* 010101101100 */
    DpregClass::Unallocated, /* 010101101101 */
    DpregClass::Unallocated, /* 010101101110 */
    DpregClass::Unallocated, /* 010101101111 */
    DpregClass::Unallocated, /* 010101110000 */
    DpregClass::Unallocated, /* 010101110001 */
    DpregClass::Unallocated, /* 010101110010 */
    DpregClass::Unallocated, /* 010101110011 */
    DpregClass::Unallocated, /* 010101110100 */
    DpregClass::Unallocated, /* 010101110101 */
    DpregClass::Unallocated, /* 010101110110 */
    DpregClass::Unallocated, /* 010101110111 */
    DpregClass::Unallocated, /* 010101111000 */
    DpregClass::Unallocated, /* 010101111001 */
    DpregClass::Unallocated, /* 010101111010 */
    DpregClass::Unallocated, /* 010101111011 */
    DpregClass::Unallocated, /* 010101111100 */
    DpregClass::Unallocated, /* 010101111101 */
    DpregClass::Unallocated, /* 010101111110 */
    DpregClass::Unallocated, /* 010101111111 */
    DpregClass::Dp2src,      /* 010110000000 */
    DpregClass::Dp2src,      /* 010110000001 */
    DpregClass::Dp2src,      /* 010110000010 */
    DpregClass::Dp2src,      /* 010110000011 */
    DpregClass::Dp2src,      /* 010110000100 */
    DpregClass::Dp2src,      /* 010110000101 */
    DpregClass::Dp2src,      /* 010110000110 */
    DpregClass::Dp2src,      /* 010110000111 */
    DpregClass::Dp2src,      /* 010110001000 */
    DpregClass::Dp2src,      /* 010110001001 */
    DpregClass::Dp2src,      /* 010110001010 */
    DpregClass::Dp2src,      /* 010110001011 */
    DpregClass::Dp2src,      /* 010110001100 */
    DpregClass::Dp2src,      /* 010110001101 */
    DpregClass::Dp2src,      /* 010110001110 */
    DpregClass::Dp2src,      /* 010110001111 */
    DpregClass::Dp2src,      /* 010110010000 */
    DpregClass::Dp2src,      /* 010110010001 */
    DpregClass::Dp2src,      /* 010110010010 */
    DpregClass::Dp2src,      /* 010110010011 */
    DpregClass::Dp2src,      /* 010110010100 */
    DpregClass::Dp2src,      /* 010110010101 */
    DpregClass::Dp2src,      /* 010110010110 */
    DpregClass::Dp2src,      /* 010110010111 */
    DpregClass::Dp2src,      /* 010110011000 */
    DpregClass::Dp2src,      /* 010110011001 */
    DpregClass::Dp2src,      /* 010110011010 */
    DpregClass::Dp2src,      /* 010110011011 */
    DpregClass::Dp2src,      /* 010110011100 */
    DpregClass::Dp2src,      /* 010110011101 */
    DpregClass::Dp2src,      /* 010110011110 */
    DpregClass::Dp2src,      /* 010110011111 */
    DpregClass::Dp2src,      /* 010110100000 */
    DpregClass::Dp2src,      /* 010110100001 */
    DpregClass::Dp2src,      /* 010110100010 */
    DpregClass::Dp2src,      /* 010110100011 */
    DpregClass::Dp2src,      /* 010110100100 */
    DpregClass::Dp2src,      /* 010110100101 */
    DpregClass::Dp2src,      /* 010110100110 */
    DpregClass::Dp2src,      /* 010110100111 */
    DpregClass::Dp2src,      /* 010110101000 */
    DpregClass::Dp2src,      /* 010110101001 */
    DpregClass::Dp2src,      /* 010110101010 */
    DpregClass::Dp2src,      /* 010110101011 */
    DpregClass::Dp2src,      /* 010110101100 */
    DpregClass::Dp2src,      /* 010110101101 */
    DpregClass::Dp2src,      /* 010110101110 */
    DpregClass::Dp2src,      /* 010110101111 */
    DpregClass::Dp2src,      /* 010110110000 */
    DpregClass::Dp2src,      /* 010110110001 */
    DpregClass::Dp2src,      /* 010110110010 */
    DpregClass::Dp2src,      /* 010110110011 */
    DpregClass::Dp2src,      /* 010110110100 */
    DpregClass::Dp2src,      /* 010110110101 */
    DpregClass::Dp2src,      /* 010110110110 */
    DpregClass::Dp2src,      /* 010110110111 */
    DpregClass::Dp2src,      /* 010110111000 */
    DpregClass::Dp2src,      /* 010110111001 */
    DpregClass::Dp2src,      /* 010110111010 */
    DpregClass::Dp2src,      /* 010110111011 */
    DpregClass::Dp2src,      /* 010110111100 */
    DpregClass::Dp2src,      /* 010110111101 */
    DpregClass::Dp2src,      /* 010110111110 */
    DpregClass::Dp2src,      /* 010110111111 */
    DpregClass::Unallocated, /* 010111000000 */
    DpregClass::Unallocated, /* 010111000001 */
    DpregClass::Unallocated, /* 010111000010 */
    DpregClass::Unallocated, /* 010111000011 */
    DpregClass::Unallocated, /* 010111000100 */
    DpregClass::Unallocated, /* 010111000101 */
    DpregClass::Unallocated, /* 010111000110 */
    DpregClass::Unallocated, /* 010111000111 */
    DpregClass::Unallocated, /* 010111001000 */
    DpregClass::Unallocated, /* 010111001001 */
    DpregClass::Unallocated, /* 010111001010 */
    DpregClass::Unallocated, /* 010111001011 */
    DpregClass::Unallocated, /* 010111001100 */
    DpregClass::Unallocated, /* 010111001101 */
    DpregClass::Unallocated, /* 010111001110 */
    DpregClass::Unallocated, /* 010111001111 */
    DpregClass::Unallocated, /* 010111010000 */
    DpregClass::Unallocated, /* 010111010001 */
    DpregClass::Unallocated, /* 010111010010 */
    DpregClass::Unallocated, /* 010111010011 */
    DpregClass::Unallocated, /* 010111010100 */
    DpregClass::Unallocated, /* 010111010101 */
    DpregClass::Unallocated, /* 010111010110 */
    DpregClass::Unallocated, /* 010111010111 */
    DpregClass::Unallocated, /* 010111011000 */
    DpregClass::Unallocated, /* 010111011001 */
    DpregClass::Unallocated, /* 010111011010 */
    DpregClass::Unallocated, /* 010111011011 */
    DpregClass::Unallocated, /* 010111011100 */
    DpregClass::Unallocated, /* 010111011101 */
    DpregClass::Unallocated, /* 010111011110 */
    DpregClass::Unallocated, /* 010111011111 */
    DpregClass::Unallocated, /* 010111100000 */
    DpregClass::Unallocated, /* 010111100001 */
    DpregClass::Unallocated, /* 010111100010 */
    DpregClass::Unallocated, /* 010111100011 */
    DpregClass::Unallocated, /* 010111100100 */
    DpregClass::Unallocated, /* 010111100101 */
    DpregClass::Unallocated, /* 010111100110 */
    DpregClass::Unallocated, /* 010111100111 */
    DpregClass::Unallocated, /* 010111101000 */
    DpregClass::Unallocated, /* 010111101001 */
    DpregClass::Unallocated, /* 010111101010 */
    DpregClass::Unallocated, /* 010111101011 */
    DpregClass::Unallocated, /* 010111101100 */
    DpregClass::Unallocated, /* 010111101101 */
    DpregClass::Unallocated, /* 010111101110 */
    DpregClass::Unallocated, /* 010111101111 */
    DpregClass::Unallocated, /* 010111110000 */
    DpregClass::Unallocated, /* 010111110001 */
    DpregClass::Unallocated, /* 010111110010 */
    DpregClass::Unallocated, /* 010111110011 */
    DpregClass::Unallocated, /* 010111110100 */
    DpregClass::Unallocated, /* 010111110101 */
    DpregClass::Unallocated, /* 010111110110 */
    DpregClass::Unallocated, /* 010111110111 */
    DpregClass::Unallocated, /* 010111111000 */
    DpregClass::Unallocated, /* 010111111001 */
    DpregClass::Unallocated, /* 010111111010 */
    DpregClass::Unallocated, /* 010111111011 */
    DpregClass::Unallocated, /* 010111111100 */
    DpregClass::Unallocated, /* 010111111101 */
    DpregClass::Unallocated, /* 010111111110 */
    DpregClass::Unallocated, /* 010111111111 */
    DpregClass::Dp3src,      /* 011000000000 */
    DpregClass::Dp3src,      /* 011000000001 */
    DpregClass::Dp3src,      /* 011000000010 */
    DpregClass::Dp3src,      /* 011000000011 */
    DpregClass::Dp3src,      /* 011000000100 */
    DpregClass::Dp3src,      /* 011000000101 */
    DpregClass::Dp3src,      /* 011000000110 */
    DpregClass::Dp3src,      /* 011000000111 */
    DpregClass::Dp3src,      /* 011000001000 */
    DpregClass::Dp3src,      /* 011000001001 */
    DpregClass::Dp3src,      /* 011000001010 */
    DpregClass::Dp3src,      /* 011000001011 */
    DpregClass::Dp3src,      /* 011000001100 */
    DpregClass::Dp3src,      /* 011000001101 */
    DpregClass::Dp3src,      /* 011000001110 */
    DpregClass::Dp3src,      /* 011000001111 */
    DpregClass::Dp3src,      /* 011000010000 */
    DpregClass::Dp3src,      /* 011000010001 */
    DpregClass::Dp3src,      /* 011000010010 */
    DpregClass::Dp3src,      /* 011000010011 */
    DpregClass::Dp3src,      /* 011000010100 */
    DpregClass::Dp3src,      /* 011000010101 */
    DpregClass::Dp3src,      /* 011000010110 */
    DpregClass::Dp3src,      /* 011000010111 */
    DpregClass::Dp3src,      /* 011000011000 */
    DpregClass::Dp3src,      /* 011000011001 */
    DpregClass::Dp3src,      /* 011000011010 */
    DpregClass::Dp3src,      /* 011000011011 */
    DpregClass::Dp3src,      /* 011000011100 */
    DpregClass::Dp3src,      /* 011000011101 */
    DpregClass::Dp3src,      /* 011000011110 */
    DpregClass::Dp3src,      /* 011000011111 */
    DpregClass::Dp3src,      /* 011000100000 */
    DpregClass::Dp3src,      /* 011000100001 */
    DpregClass::Dp3src,      /* 011000100010 */
    DpregClass::Dp3src,      /* 011000100011 */
    DpregClass::Dp3src,      /* 011000100100 */
    DpregClass::Dp3src,      /* 011000100101 */
    DpregClass::Dp3src,      /* 011000100110 */
    DpregClass::Dp3src,      /* 011000100111 */
    DpregClass::Dp3src,      /* 011000101000 */
    DpregClass::Dp3src,      /* 011000101001 */
    DpregClass::Dp3src,      /* 011000101010 */
    DpregClass::Dp3src,      /* 011000101011 */
    DpregClass::Dp3src,      /* 011000101100 */
    DpregClass::Dp3src,      /* 011000101101 */
    DpregClass::Dp3src,      /* 011000101110 */
    DpregClass::Dp3src,      /* 011000101111 */
    DpregClass::Dp3src,      /* 011000110000 */
    DpregClass::Dp3src,      /* 011000110001 */
    DpregClass::Dp3src,      /* 011000110010 */
    DpregClass::Dp3src,      /* 011000110011 */
    DpregClass::Dp3src,      /* 011000110100 */
    DpregClass::Dp3src,      /* 011000110101 */
    DpregClass::Dp3src,      /* 011000110110 */
    DpregClass::Dp3src,      /* 011000110111 */
    DpregClass::Dp3src,      /* 011000111000 */
    DpregClass::Dp3src,      /* 011000111001 */
    DpregClass::Dp3src,      /* 011000111010 */
    DpregClass::Dp3src,      /* 011000111011 */
    DpregClass::Dp3src,      /* 011000111100 */
    DpregClass::Dp3src,      /* 011000111101 */
    DpregClass::Dp3src,      /* 011000111110 */
    DpregClass::Dp3src,      /* 011000111111 */
    DpregClass::Dp3src,      /* 011001000000 */
    DpregClass::Dp3src,      /* 011001000001 */
    DpregClass::Dp3src,      /* 011001000010 */
    DpregClass::Dp3src,      /* 011001000011 */
    DpregClass::Dp3src,      /* 011001000100 */
    DpregClass::Dp3src,      /* 011001000101 */
    DpregClass::Dp3src,      /* 011001000110 */
    DpregClass::Dp3src,      /* 011001000111 */
    DpregClass::Dp3src,      /* 011001001000 */
    DpregClass::Dp3src,      /* 011001001001 */
    DpregClass::Dp3src,      /* 011001001010 */
    DpregClass::Dp3src,      /* 011001001011 */
    DpregClass::Dp3src,      /* 011001001100 */
    DpregClass::Dp3src,      /* 011001001101 */
    DpregClass::Dp3src,      /* 011001001110 */
    DpregClass::Dp3src,      /* 011001001111 */
    DpregClass::Dp3src,      /* 011001010000 */
    DpregClass::Dp3src,      /* 011001010001 */
    DpregClass::Dp3src,      /* 011001010010 */
    DpregClass::Dp3src,      /* 011001010011 */
    DpregClass::Dp3src,      /* 011001010100 */
    DpregClass::Dp3src,      /* 011001010101 */
    DpregClass::Dp3src,      /* 011001010110 */
    DpregClass::Dp3src,      /* 011001010111 */
    DpregClass::Dp3src,      /* 011001011000 */
    DpregClass::Dp3src,      /* 011001011001 */
    DpregClass::Dp3src,      /* 011001011010 */
    DpregClass::Dp3src,      /* 011001011011 */
    DpregClass::Dp3src,      /* 011001011100 */
    DpregClass::Dp3src,      /* 011001011101 */
    DpregClass::Dp3src,      /* 011001011110 */
    DpregClass::Dp3src,      /* 011001011111 */
    DpregClass::Dp3src,      /* 011001100000 */
    DpregClass::Dp3src,      /* 011001100001 */
    DpregClass::Dp3src,      /* 011001100010 */
    DpregClass::Dp3src,      /* 011001100011 */
    DpregClass::Dp3src,      /* 011001100100 */
    DpregClass::Dp3src,      /* 011001100101 */
    DpregClass::Dp3src,      /* 011001100110 */
    DpregClass::Dp3src,      /* 011001100111 */
    DpregClass::Dp3src,      /* 011001101000 */
    DpregClass::Dp3src,      /* 011001101001 */
    DpregClass::Dp3src,      /* 011001101010 */
    DpregClass::Dp3src,      /* 011001101011 */
    DpregClass::Dp3src,      /* 011001101100 */
    DpregClass::Dp3src,      /* 011001101101 */
    DpregClass::Dp3src,      /* 011001101110 */
    DpregClass::Dp3src,      /* 011001101111 */
    DpregClass::Dp3src,      /* 011001110000 */
    DpregClass::Dp3src,      /* 011001110001 */
    DpregClass::Dp3src,      /* 011001110010 */
    DpregClass::Dp3src,      /* 011001110011 */
    DpregClass::Dp3src,      /* 011001110100 */
    DpregClass::Dp3src,      /* 011001110101 */
    DpregClass::Dp3src,      /* 011001110110 */
    DpregClass::Dp3src,      /* 011001110111 */
    DpregClass::Dp3src,      /* 011001111000 */
    DpregClass::Dp3src,      /* 011001111001 */
    DpregClass::Dp3src,      /* 011001111010 */
    DpregClass::Dp3src,      /* 011001111011 */
    DpregClass::Dp3src,      /* 011001111100 */
    DpregClass::Dp3src,      /* 011001111101 */
    DpregClass::Dp3src,      /* 011001111110 */
    DpregClass::Dp3src,      /* 011001111111 */
    DpregClass::Dp3src,      /* 011010000000 */
    DpregClass::Dp3src,      /* 011010000001 */
    DpregClass::Dp3src,      /* 011010000010 */
    DpregClass::Dp3src,      /* 011010000011 */
    DpregClass::Dp3src,      /* 011010000100 */
    DpregClass::Dp3src,      /* 011010000101 */
    DpregClass::Dp3src,      /* 011010000110 */
    DpregClass::Dp3src,      /* 011010000111 */
    DpregClass::Dp3src,      /* 011010001000 */
    DpregClass::Dp3src,      /* 011010001001 */
    DpregClass::Dp3src,      /* 011010001010 */
    DpregClass::Dp3src,      /* 011010001011 */
    DpregClass::Dp3src,      /* 011010001100 */
    DpregClass::Dp3src,      /* 011010001101 */
    DpregClass::Dp3src,      /* 011010001110 */
    DpregClass::Dp3src,      /* 011010001111 */
    DpregClass::Dp3src,      /* 011010010000 */
    DpregClass::Dp3src,      /* 011010010001 */
    DpregClass::Dp3src,      /* 011010010010 */
    DpregClass::Dp3src,      /* 011010010011 */
    DpregClass::Dp3src,      /* 011010010100 */
    DpregClass::Dp3src,      /* 011010010101 */
    DpregClass::Dp3src,      /* 011010010110 */
    DpregClass::Dp3src,      /* 011010010111 */
    DpregClass::Dp3src,      /* 011010011000 */
    DpregClass::Dp3src,      /* 011010011001 */
    DpregClass::Dp3src,      /* 011010011010 */
    DpregClass::Dp3src,      /* 011010011011 */
    DpregClass::Dp3src,      /* 011010011100 */
    DpregClass::Dp3src,      /* 011010011101 */
    DpregClass::Dp3src,      /* 011010011110 */
    DpregClass::Dp3src,      /* 011010011111 */
    DpregClass::Dp3src,      /* 011010100000 */
    DpregClass::Dp3src,      /* 011010100001 */
    DpregClass::Dp3src,      /* 011010100010 */
    DpregClass::Dp3src,      /* 011010100011 */
    DpregClass::Dp3src,      /* 011010100100 */
    DpregClass::Dp3src,      /* 011010100101 */
    DpregClass::Dp3src,      /* 011010100110 */
    DpregClass::Dp3src,      /* 011010100111 */
    DpregClass::Dp3src,      /* 011010101000 */
    DpregClass::Dp3src,      /* 011010101001 */
    DpregClass::Dp3src,      /* 011010101010 */
    DpregClass::Dp3src,      /* 011010101011 */
    DpregClass::Dp3src,      /* 011010101100 */
    DpregClass::Dp3src,      /* 011010101101 */
    DpregClass::Dp3src,      /* 011010101110 */
    DpregClass::Dp3src,      /* 011010101111 */
    DpregClass::Dp3src,      /* 011010110000 */
    DpregClass::Dp3src,      /* 011010110001 */
    DpregClass::Dp3src,      /* 011010110010 */
    DpregClass::Dp3src,      /* 011010110011 */
    DpregClass::Dp3src,      /* 011010110100 */
    DpregClass::Dp3src,      /* 011010110101 */
    DpregClass::Dp3src,      /* 011010110110 */
    DpregClass::Dp3src,      /* 011010110111 */
    DpregClass::Dp3src,      /* 011010111000 */
    DpregClass::Dp3src,      /* 011010111001 */
    DpregClass::Dp3src,      /* 011010111010 */
    DpregClass::Dp3src,      /* 011010111011 */
    DpregClass::Dp3src,      /* 011010111100 */
    DpregClass::Dp3src,      /* 011010111101 */
    DpregClass::Dp3src,      /* 011010111110 */
    DpregClass::Dp3src,      /* 011010111111 */
    DpregClass::Dp3src,      /* 011011000000 */
    DpregClass::Dp3src,      /* 011011000001 */
    DpregClass::Dp3src,      /* 011011000010 */
    DpregClass::Dp3src,      /* 011011000011 */
    DpregClass::Dp3src,      /* 011011000100 */
    DpregClass::Dp3src,      /* 011011000101 */
    DpregClass::Dp3src,      /* 011011000110 */
    DpregClass::Dp3src,      /* 011011000111 */
    DpregClass::Dp3src,      /* 011011001000 */
    DpregClass::Dp3src,      /* 011011001001 */
    DpregClass::Dp3src,      /* 011011001010 */
    DpregClass::Dp3src,      /* 011011001011 */
    DpregClass::Dp3src,      /* 011011001100 */
    DpregClass::Dp3src,      /* 011011001101 */
    DpregClass::Dp3src,      /* 011011001110 */
    DpregClass::Dp3src,      /* 011011001111 */
    DpregClass::Dp3src,      /* 011011010000 */
    DpregClass::Dp3src,      /* 011011010001 */
    DpregClass::Dp3src,      /* 011011010010 */
    DpregClass::Dp3src,      /* 011011010011 */
    DpregClass::Dp3src,      /* 011011010100 */
    DpregClass::Dp3src,      /* 011011010101 */
    DpregClass::Dp3src,      /* 011011010110 */
    DpregClass::Dp3src,      /* 011011010111 */
    DpregClass::Dp3src,      /* 011011011000 */
    DpregClass::Dp3src,      /* 011011011001 */
    DpregClass::Dp3src,      /* 011011011010 */
    DpregClass::Dp3src,      /* 011011011011 */
    DpregClass::Dp3src,      /* 011011011100 */
    DpregClass::Dp3src,      /* 011011011101 */
    DpregClass::Dp3src,      /* 011011011110 */
    DpregClass::Dp3src,      /* 011011011111 */
    DpregClass::Dp3src,      /* 011011100000 */
    DpregClass::Dp3src,      /* 011011100001 */
    DpregClass::Dp3src,      /* 011011100010 */
    DpregClass::Dp3src,      /* 011011100011 */
    DpregClass::Dp3src,      /* 011011100100 */
    DpregClass::Dp3src,      /* 011011100101 */
    DpregClass::Dp3src,      /* 011011100110 */
    DpregClass::Dp3src,      /* 011011100111 */
    DpregClass::Dp3src,      /* 011011101000 */
    DpregClass::Dp3src,      /* 011011101001 */
    DpregClass::Dp3src,      /* 011011101010 */
    DpregClass::Dp3src,      /* 011011101011 */
    DpregClass::Dp3src,      /* 011011101100 */
    DpregClass::Dp3src,      /* 011011101101 */
    DpregClass::Dp3src,      /* 011011101110 */
    DpregClass::Dp3src,      /* 011011101111 */
    DpregClass::Dp3src,      /* 011011110000 */
    DpregClass::Dp3src,      /* 011011110001 */
    DpregClass::Dp3src,      /* 011011110010 */
    DpregClass::Dp3src,      /* 011011110011 */
    DpregClass::Dp3src,      /* 011011110100 */
    DpregClass::Dp3src,      /* 011011110101 */
    DpregClass::Dp3src,      /* 011011110110 */
    DpregClass::Dp3src,      /* 011011110111 */
    DpregClass::Dp3src,      /* 011011111000 */
    DpregClass::Dp3src,      /* 011011111001 */
    DpregClass::Dp3src,      /* 011011111010 */
    DpregClass::Dp3src,      /* 011011111011 */
    DpregClass::Dp3src,      /* 011011111100 */
    DpregClass::Dp3src,      /* 011011111101 */
    DpregClass::Dp3src,      /* 011011111110 */
    DpregClass::Dp3src,      /* 011011111111 */
    DpregClass::Dp3src,      /* 011100000000 */
    DpregClass::Dp3src,      /* 011100000001 */
    DpregClass::Dp3src,      /* 011100000010 */
    DpregClass::Dp3src,      /* 011100000011 */
    DpregClass::Dp3src,      /* 011100000100 */
    DpregClass::Dp3src,      /* 011100000101 */
    DpregClass::Dp3src,      /* 011100000110 */
    DpregClass::Dp3src,      /* 011100000111 */
    DpregClass::Dp3src,      /* 011100001000 */
    DpregClass::Dp3src,      /* 011100001001 */
    DpregClass::Dp3src,      /* 011100001010 */
    DpregClass::Dp3src,      /* 011100001011 */
    DpregClass::Dp3src,      /* 011100001100 */
    DpregClass::Dp3src,      /* 011100001101 */
    DpregClass::Dp3src,      /* 011100001110 */
    DpregClass::Dp3src,      /* 011100001111 */
    DpregClass::Dp3src,      /* 011100010000 */
    DpregClass::Dp3src,      /* 011100010001 */
    DpregClass::Dp3src,      /* 011100010010 */
    DpregClass::Dp3src,      /* 011100010011 */
    DpregClass::Dp3src,      /* 011100010100 */
    DpregClass::Dp3src,      /* 011100010101 */
    DpregClass::Dp3src,      /* 011100010110 */
    DpregClass::Dp3src,      /* 011100010111 */
    DpregClass::Dp3src,      /* 011100011000 */
    DpregClass::Dp3src,      /* 011100011001 */
    DpregClass::Dp3src,      /* 011100011010 */
    DpregClass::Dp3src,      /* 011100011011 */
    DpregClass::Dp3src,      /* 011100011100 */
    DpregClass::Dp3src,      /* 011100011101 */
    DpregClass::Dp3src,      /* 011100011110 */
    DpregClass::Dp3src,      /* 011100011111 */
    DpregClass::Dp3src,      /* 011100100000 */
    DpregClass::Dp3src,      /* 011100100001 */
    DpregClass::Dp3src,      /* 011100100010 */
    DpregClass::Dp3src,      /* 011100100011 */
    DpregClass::Dp3src,      /* 011100100100 */
    DpregClass::Dp3src,      /* 011100100101 */
    DpregClass::Dp3src,      /* 011100100110 */
    DpregClass::Dp3src,      /* 011100100111 */
    DpregClass::Dp3src,      /* 011100101000 */
    DpregClass::Dp3src,      /* 011100101001 */
    DpregClass::Dp3src,      /* 011100101010 */
    DpregClass::Dp3src,      /* 011100101011 */
    DpregClass::Dp3src,      /* 011100101100 */
    DpregClass::Dp3src,      /* 011100101101 */
    DpregClass::Dp3src,      /* 011100101110 */
    DpregClass::Dp3src,      /* 011100101111 */
    DpregClass::Dp3src,      /* 011100110000 */
    DpregClass::Dp3src,      /* 011100110001 */
    DpregClass::Dp3src,      /* 011100110010 */
    DpregClass::Dp3src,      /* 011100110011 */
    DpregClass::Dp3src,      /* 011100110100 */
    DpregClass::Dp3src,      /* 011100110101 */
    DpregClass::Dp3src,      /* 011100110110 */
    DpregClass::Dp3src,      /* 011100110111 */
    DpregClass::Dp3src,      /* 011100111000 */
    DpregClass::Dp3src,      /* 011100111001 */
    DpregClass::Dp3src,      /* 011100111010 */
    DpregClass::Dp3src,      /* 011100111011 */
    DpregClass::Dp3src,      /* 011100111100 */
    DpregClass::Dp3src,      /* 011100111101 */
    DpregClass::Dp3src,      /* 011100111110 */
    DpregClass::Dp3src,      /* 011100111111 */
    DpregClass::Dp3src,      /* 011101000000 */
    DpregClass::Dp3src,      /* 011101000001 */
    DpregClass::Dp3src,      /* 011101000010 */
    DpregClass::Dp3src,      /* 011101000011 */
    DpregClass::Dp3src,      /* 011101000100 */
    DpregClass::Dp3src,      /* 011101000101 */
    DpregClass::Dp3src,      /* 011101000110 */
    DpregClass::Dp3src,      /* 011101000111 */
    DpregClass::Dp3src,      /* 011101001000 */
    DpregClass::Dp3src,      /* 011101001001 */
    DpregClass::Dp3src,      /* 011101001010 */
    DpregClass::Dp3src,      /* 011101001011 */
    DpregClass::Dp3src,      /* 011101001100 */
    DpregClass::Dp3src,      /* 011101001101 */
    DpregClass::Dp3src,      /* 011101001110 */
    DpregClass::Dp3src,      /* 011101001111 */
    DpregClass::Dp3src,      /* 011101010000 */
    DpregClass::Dp3src,      /* 011101010001 */
    DpregClass::Dp3src,      /* 011101010010 */
    DpregClass::Dp3src,      /* 011101010011 */
    DpregClass::Dp3src,      /* 011101010100 */
    DpregClass::Dp3src,      /* 011101010101 */
    DpregClass::Dp3src,      /* 011101010110 */
    DpregClass::Dp3src,      /* 011101010111 */
    DpregClass::Dp3src,      /* 011101011000 */
    DpregClass::Dp3src,      /* 011101011001 */
    DpregClass::Dp3src,      /* 011101011010 */
    DpregClass::Dp3src,      /* 011101011011 */
    DpregClass::Dp3src,      /* 011101011100 */
    DpregClass::Dp3src,      /* 011101011101 */
    DpregClass::Dp3src,      /* 011101011110 */
    DpregClass::Dp3src,      /* 011101011111 */
    DpregClass::Dp3src,      /* 011101100000 */
    DpregClass::Dp3src,      /* 011101100001 */
    DpregClass::Dp3src,      /* 011101100010 */
    DpregClass::Dp3src,      /* 011101100011 */
    DpregClass::Dp3src,      /* 011101100100 */
    DpregClass::Dp3src,      /* 011101100101 */
    DpregClass::Dp3src,      /* 011101100110 */
    DpregClass::Dp3src,      /* 011101100111 */
    DpregClass::Dp3src,      /* 011101101000 */
    DpregClass::Dp3src,      /* 011101101001 */
    DpregClass::Dp3src,      /* 011101101010 */
    DpregClass::Dp3src,      /* 011101101011 */
    DpregClass::Dp3src,      /* 011101101100 */
    DpregClass::Dp3src,      /* 011101101101 */
    DpregClass::Dp3src,      /* 011101101110 */
    DpregClass::Dp3src,      /* 011101101111 */
    DpregClass::Dp3src,      /* 011101110000 */
    DpregClass::Dp3src,      /* 011101110001 */
    DpregClass::Dp3src,      /* 011101110010 */
    DpregClass::Dp3src,      /* 011101110011 */
    DpregClass::Dp3src,      /* 011101110100 */
    DpregClass::Dp3src,      /* 011101110101 */
    DpregClass::Dp3src,      /* 011101110110 */
    DpregClass::Dp3src,      /* 011101110111 */
    DpregClass::Dp3src,      /* 011101111000 */
    DpregClass::Dp3src,      /* 011101111001 */
    DpregClass::Dp3src,      /* 011101111010 */
    DpregClass::Dp3src,      /* 011101111011 */
    DpregClass::Dp3src,      /* 011101111100 */
    DpregClass::Dp3src,      /* 011101111101 */
    DpregClass::Dp3src,      /* 011101111110 */
    DpregClass::Dp3src,      /* 011101111111 */
    DpregClass::Dp3src,      /* 011110000000 */
    DpregClass::Dp3src,      /* 011110000001 */
    DpregClass::Dp3src,      /* 011110000010 */
    DpregClass::Dp3src,      /* 011110000011 */
    DpregClass::Dp3src,      /* 011110000100 */
    DpregClass::Dp3src,      /* 011110000101 */
    DpregClass::Dp3src,      /* 011110000110 */
    DpregClass::Dp3src,      /* 011110000111 */
    DpregClass::Dp3src,      /* 011110001000 */
    DpregClass::Dp3src,      /* 011110001001 */
    DpregClass::Dp3src,      /* 011110001010 */
    DpregClass::Dp3src,      /* 011110001011 */
    DpregClass::Dp3src,      /* 011110001100 */
    DpregClass::Dp3src,      /* 011110001101 */
    DpregClass::Dp3src,      /* 011110001110 */
    DpregClass::Dp3src,      /* 011110001111 */
    DpregClass::Dp3src,      /* 011110010000 */
    DpregClass::Dp3src,      /* 011110010001 */
    DpregClass::Dp3src,      /* 011110010010 */
    DpregClass::Dp3src,      /* 011110010011 */
    DpregClass::Dp3src,      /* 011110010100 */
    DpregClass::Dp3src,      /* 011110010101 */
    DpregClass::Dp3src,      /* 011110010110 */
    DpregClass::Dp3src,      /* 011110010111 */
    DpregClass::Dp3src,      /* 011110011000 */
    DpregClass::Dp3src,      /* 011110011001 */
    DpregClass::Dp3src,      /* 011110011010 */
    DpregClass::Dp3src,      /* 011110011011 */
    DpregClass::Dp3src,      /* 011110011100 */
    DpregClass::Dp3src,      /* 011110011101 */
    DpregClass::Dp3src,      /* 011110011110 */
    DpregClass::Dp3src,      /* 011110011111 */
    DpregClass::Dp3src,      /* 011110100000 */
    DpregClass::Dp3src,      /* 011110100001 */
    DpregClass::Dp3src,      /* 011110100010 */
    DpregClass::Dp3src,      /* 011110100011 */
    DpregClass::Dp3src,      /* 011110100100 */
    DpregClass::Dp3src,      /* 011110100101 */
    DpregClass::Dp3src,      /* 011110100110 */
    DpregClass::Dp3src,      /* 011110100111 */
    DpregClass::Dp3src,      /* 011110101000 */
    DpregClass::Dp3src,      /* 011110101001 */
    DpregClass::Dp3src,      /* 011110101010 */
    DpregClass::Dp3src,      /* 011110101011 */
    DpregClass::Dp3src,      /* 011110101100 */
    DpregClass::Dp3src,      /* 011110101101 */
    DpregClass::Dp3src,      /* 011110101110 */
    DpregClass::Dp3src,      /* 011110101111 */
    DpregClass::Dp3src,      /* 011110110000 */
    DpregClass::Dp3src,      /* 011110110001 */
    DpregClass::Dp3src,      /* 011110110010 */
    DpregClass::Dp3src,      /* 011110110011 */
    DpregClass::Dp3src,      /* 011110110100 */
    DpregClass::Dp3src,      /* 011110110101 */
    DpregClass::Dp3src,      /* 011110110110 */
    DpregClass::Dp3src,      /* 011110110111 */
    DpregClass::Dp3src,      /* 011110111000 */
    DpregClass::Dp3src,      /* 011110111001 */
    DpregClass::Dp3src,      /* 011110111010 */
    DpregClass::Dp3src,      /* 011110111011 */
    DpregClass::Dp3src,      /* 011110111100 */
    DpregClass::Dp3src,      /* 011110111101 */
    DpregClass::Dp3src,      /* 011110111110 */
    DpregClass::Dp3src,      /* 011110111111 */
    DpregClass::Dp3src,      /* 011111000000 */
    DpregClass::Dp3src,      /* 011111000001 */
    DpregClass::Dp3src,      /* 011111000010 */
    DpregClass::Dp3src,      /* 011111000011 */
    DpregClass::Dp3src,      /* 011111000100 */
    DpregClass::Dp3src,      /* 011111000101 */
    DpregClass::Dp3src,      /* 011111000110 */
    DpregClass::Dp3src,      /* 011111000111 */
    DpregClass::Dp3src,      /* 011111001000 */
    DpregClass::Dp3src,      /* 011111001001 */
    DpregClass::Dp3src,      /* 011111001010 */
    DpregClass::Dp3src,      /* 011111001011 */
    DpregClass::Dp3src,      /* 011111001100 */
    DpregClass::Dp3src,      /* 011111001101 */
    DpregClass::Dp3src,      /* 011111001110 */
    DpregClass::Dp3src,      /* 011111001111 */
    DpregClass::Dp3src,      /* 011111010000 */
    DpregClass::Dp3src,      /* 011111010001 */
    DpregClass::Dp3src,      /* 011111010010 */
    DpregClass::Dp3src,      /* 011111010011 */
    DpregClass::Dp3src,      /* 011111010100 */
    DpregClass::Dp3src,      /* 011111010101 */
    DpregClass::Dp3src,      /* 011111010110 */
    DpregClass::Dp3src,      /* 011111010111 */
    DpregClass::Dp3src,      /* 011111011000 */
    DpregClass::Dp3src,      /* 011111011001 */
    DpregClass::Dp3src,      /* 011111011010 */
    DpregClass::Dp3src,      /* 011111011011 */
    DpregClass::Dp3src,      /* 011111011100 */
    DpregClass::Dp3src,      /* 011111011101 */
    DpregClass::Dp3src,      /* 011111011110 */
    DpregClass::Dp3src,      /* 011111011111 */
    DpregClass::Dp3src,      /* 011111100000 */
    DpregClass::Dp3src,      /* 011111100001 */
    DpregClass::Dp3src,      /* 011111100010 */
    DpregClass::Dp3src,      /* 011111100011 */
    DpregClass::Dp3src,      /* 011111100100 */
    DpregClass::Dp3src,      /* 011111100101 */
    DpregClass::Dp3src,      /* 011111100110 */
    DpregClass::Dp3src,      /* 011111100111 */
    DpregClass::Dp3src,      /* 011111101000 */
    DpregClass::Dp3src,      /* 011111101001 */
    DpregClass::Dp3src,      /* 011111101010 */
    DpregClass::Dp3src,      /* 011111101011 */
    DpregClass::Dp3src,      /* 011111101100 */
    DpregClass::Dp3src,      /* 011111101101 */
    DpregClass::Dp3src,      /* 011111101110 */
    DpregClass::Dp3src,      /* 011111101111 */
    DpregClass::Dp3src,      /* 011111110000 */
    DpregClass::Dp3src,      /* 011111110001 */
    DpregClass::Dp3src,      /* 011111110010 */
    DpregClass::Dp3src,      /* 011111110011 */
    DpregClass::Dp3src,      /* 011111110100 */
    DpregClass::Dp3src,      /* 011111110101 */
    DpregClass::Dp3src,      /* 011111110110 */
    DpregClass::Dp3src,      /* 011111110111 */
    DpregClass::Dp3src,      /* 011111111000 */
    DpregClass::Dp3src,      /* 011111111001 */
    DpregClass::Dp3src,      /* 011111111010 */
    DpregClass::Dp3src,      /* 011111111011 */
    DpregClass::Dp3src,      /* 011111111100 */
    DpregClass::Dp3src,      /* 011111111101 */
    DpregClass::Dp3src,      /* 011111111110 */
    DpregClass::Dp3src,      /* 011111111111 */
    DpregClass::LogShift,    /* 100000000000 */
    DpregClass::LogShift,    /* 100000000001 */
    DpregClass::LogShift,    /* 100000000010 */
    DpregClass::LogShift,    /* 100000000011 */
    DpregClass::LogShift,    /* 100000000100 */
    DpregClass::LogShift,    /* 100000000101 */
    DpregClass::LogShift,    /* 100000000110 */
    DpregClass::LogShift,    /* 100000000111 */
    DpregClass::LogShift,    /* 100000001000 */
    DpregClass::LogShift,    /* 100000001001 */
    DpregClass::LogShift,    /* 100000001010 */
    DpregClass::LogShift,    /* 100000001011 */
    DpregClass::LogShift,    /* 100000001100 */
    DpregClass::LogShift,    /* 100000001101 */
    DpregClass::LogShift,    /* 100000001110 */
    DpregClass::LogShift,    /* 100000001111 */
    DpregClass::LogShift,    /* 100000010000 */
    DpregClass::LogShift,    /* 100000010001 */
    DpregClass::LogShift,    /* 100000010010 */
    DpregClass::LogShift,    /* 100000010011 */
    DpregClass::LogShift,    /* 100000010100 */
    DpregClass::LogShift,    /* 100000010101 */
    DpregClass::LogShift,    /* 100000010110 */
    DpregClass::LogShift,    /* 100000010111 */
    DpregClass::LogShift,    /* 100000011000 */
    DpregClass::LogShift,    /* 100000011001 */
    DpregClass::LogShift,    /* 100000011010 */
    DpregClass::LogShift,    /* 100000011011 */
    DpregClass::LogShift,    /* 100000011100 */
    DpregClass::LogShift,    /* 100000011101 */
    DpregClass::LogShift,    /* 100000011110 */
    DpregClass::LogShift,    /* 100000011111 */
    DpregClass::LogShift,    /* 100000100000 */
    DpregClass::LogShift,    /* 100000100001 */
    DpregClass::LogShift,    /* 100000100010 */
    DpregClass::LogShift,    /* 100000100011 */
    DpregClass::LogShift,    /* 100000100100 */
    DpregClass::LogShift,    /* 100000100101 */
    DpregClass::LogShift,    /* 100000100110 */
    DpregClass::LogShift,    /* 100000100111 */
    DpregClass::LogShift,    /* 100000101000 */
    DpregClass::LogShift,    /* 100000101001 */
    DpregClass::LogShift,    /* 100000101010 */
    DpregClass::LogShift,    /* 100000101011 */
    DpregClass::LogShift,    /* 100000101100 */
    DpregClass::LogShift,    /* 100000101101 */
    DpregClass::LogShift,    /* 100000101110 */
    DpregClass::LogShift,    /* 100000101111 */
    DpregClass::LogShift,    /* 100000110000 */
    DpregClass::LogShift,    /* 100000110001 */
    DpregClass::LogShift,    /* 100000110010 */
    DpregClass::LogShift,    /* 100000110011 */
    DpregClass::LogShift,    /* 100000110100 */
    DpregClass::LogShift,    /* 100000110101 */
    DpregClass::LogShift,    /* 100000110110 */
    DpregClass::LogShift,    /* 100000110111 */
    DpregClass::LogShift,    /* 100000111000 */
    DpregClass::LogShift,    /* 100000111001 */
    DpregClass::LogShift,    /* 100000111010 */
    DpregClass::LogShift,    /* 100000111011 */
    DpregClass::LogShift,    /* 100000111100 */
    DpregClass::LogShift,    /* 100000111101 */
    DpregClass::LogShift,    /* 100000111110 */
    DpregClass::LogShift,    /* 100000111111 */
    DpregClass::LogShift,    /* 100001000000 */
    DpregClass::LogShift,    /* 100001000001 */
    DpregClass::LogShift,    /* 100001000010 */
    DpregClass::LogShift,    /* 100001000011 */
    DpregClass::LogShift,    /* 100001000100 */
    DpregClass::LogShift,    /* 100001000101 */
    DpregClass::LogShift,    /* 100001000110 */
    DpregClass::LogShift,    /* 100001000111 */
    DpregClass::LogShift,    /* 100001001000 */
    DpregClass::LogShift,    /* 100001001001 */
    DpregClass::LogShift,    /* 100001001010 */
    DpregClass::LogShift,    /* 100001001011 */
    DpregClass::LogShift,    /* 100001001100 */
    DpregClass::LogShift,    /* 100001001101 */
    DpregClass::LogShift,    /* 100001001110 */
    DpregClass::LogShift,    /* 100001001111 */
    DpregClass::LogShift,    /* 100001010000 */
    DpregClass::LogShift,    /* 100001010001 */
    DpregClass::LogShift,    /* 100001010010 */
    DpregClass::LogShift,    /* 100001010011 */
    DpregClass::LogShift,    /* 100001010100 */
    DpregClass::LogShift,    /* 100001010101 */
    DpregClass::LogShift,    /* 100001010110 */
    DpregClass::LogShift,    /* 100001010111 */
    DpregClass::LogShift,    /* 100001011000 */
    DpregClass::LogShift,    /* 100001011001 */
    DpregClass::LogShift,    /* 100001011010 */
    DpregClass::LogShift,    /* 100001011011 */
    DpregClass::LogShift,    /* 100001011100 */
    DpregClass::LogShift,    /* 100001011101 */
    DpregClass::LogShift,    /* 100001011110 */
    DpregClass::LogShift,    /* 100001011111 */
    DpregClass::LogShift,    /* 100001100000 */
    DpregClass::LogShift,    /* 100001100001 */
    DpregClass::LogShift,    /* 100001100010 */
    DpregClass::LogShift,    /* 100001100011 */
    DpregClass::LogShift,    /* 100001100100 */
    DpregClass::LogShift,    /* 100001100101 */
    DpregClass::LogShift,    /* 100001100110 */
    DpregClass::LogShift,    /* 100001100111 */
    DpregClass::LogShift,    /* 100001101000 */
    DpregClass::LogShift,    /* 100001101001 */
    DpregClass::LogShift,    /* 100001101010 */
    DpregClass::LogShift,    /* 100001101011 */
    DpregClass::LogShift,    /* 100001101100 */
    DpregClass::LogShift,    /* 100001101101 */
    DpregClass::LogShift,    /* 100001101110 */
    DpregClass::LogShift,    /* 100001101111 */
    DpregClass::LogShift,    /* 100001110000 */
    DpregClass::LogShift,    /* 100001110001 */
    DpregClass::LogShift,    /* 100001110010 */
    DpregClass::LogShift,    /* 100001110011 */
    DpregClass::LogShift,    /* 100001110100 */
    DpregClass::LogShift,    /* 100001110101 */
    DpregClass::LogShift,    /* 100001110110 */
    DpregClass::LogShift,    /* 100001110111 */
    DpregClass::LogShift,    /* 100001111000 */
    DpregClass::LogShift,    /* 100001111001 */
    DpregClass::LogShift,    /* 100001111010 */
    DpregClass::LogShift,    /* 100001111011 */
    DpregClass::LogShift,    /* 100001111100 */
    DpregClass::LogShift,    /* 100001111101 */
    DpregClass::LogShift,    /* 100001111110 */
    DpregClass::LogShift,    /* 100001111111 */
    DpregClass::LogShift,    /* 100010000000 */
    DpregClass::LogShift,    /* 100010000001 */
    DpregClass::LogShift,    /* 100010000010 */
    DpregClass::LogShift,    /* 100010000011 */
    DpregClass::LogShift,    /* 100010000100 */
    DpregClass::LogShift,    /* 100010000101 */
    DpregClass::LogShift,    /* 100010000110 */
    DpregClass::LogShift,    /* 100010000111 */
    DpregClass::LogShift,    /* 100010001000 */
    DpregClass::LogShift,    /* 100010001001 */
    DpregClass::LogShift,    /* 100010001010 */
    DpregClass::LogShift,    /* 100010001011 */
    DpregClass::LogShift,    /* 100010001100 */
    DpregClass::LogShift,    /* 100010001101 */
    DpregClass::LogShift,    /* 100010001110 */
    DpregClass::LogShift,    /* 100010001111 */
    DpregClass::LogShift,    /* 100010010000 */
    DpregClass::LogShift,    /* 100010010001 */
    DpregClass::LogShift,    /* 100010010010 */
    DpregClass::LogShift,    /* 100010010011 */
    DpregClass::LogShift,    /* 100010010100 */
    DpregClass::LogShift,    /* 100010010101 */
    DpregClass::LogShift,    /* 100010010110 */
    DpregClass::LogShift,    /* 100010010111 */
    DpregClass::LogShift,    /* 100010011000 */
    DpregClass::LogShift,    /* 100010011001 */
    DpregClass::LogShift,    /* 100010011010 */
    DpregClass::LogShift,    /* 100010011011 */
    DpregClass::LogShift,    /* 100010011100 */
    DpregClass::LogShift,    /* 100010011101 */
    DpregClass::LogShift,    /* 100010011110 */
    DpregClass::LogShift,    /* 100010011111 */
    DpregClass::LogShift,    /* 100010100000 */
    DpregClass::LogShift,    /* 100010100001 */
    DpregClass::LogShift,    /* 100010100010 */
    DpregClass::LogShift,    /* 100010100011 */
    DpregClass::LogShift,    /* 100010100100 */
    DpregClass::LogShift,    /* 100010100101 */
    DpregClass::LogShift,    /* 100010100110 */
    DpregClass::LogShift,    /* 100010100111 */
    DpregClass::LogShift,    /* 100010101000 */
    DpregClass::LogShift,    /* 100010101001 */
    DpregClass::LogShift,    /* 100010101010 */
    DpregClass::LogShift,    /* 100010101011 */
    DpregClass::LogShift,    /* 100010101100 */
    DpregClass::LogShift,    /* 100010101101 */
    DpregClass::LogShift,    /* 100010101110 */
    DpregClass::LogShift,    /* 100010101111 */
    DpregClass::LogShift,    /* 100010110000 */
    DpregClass::LogShift,    /* 100010110001 */
    DpregClass::LogShift,    /* 100010110010 */
    DpregClass::LogShift,    /* 100010110011 */
    DpregClass::LogShift,    /* 100010110100 */
    DpregClass::LogShift,    /* 100010110101 */
    DpregClass::LogShift,    /* 100010110110 */
    DpregClass::LogShift,    /* 100010110111 */
    DpregClass::LogShift,    /* 100010111000 */
    DpregClass::LogShift,    /* 100010111001 */
    DpregClass::LogShift,    /* 100010111010 */
    DpregClass::LogShift,    /* 100010111011 */
    DpregClass::LogShift,    /* 100010111100 */
    DpregClass::LogShift,    /* 100010111101 */
    DpregClass::LogShift,    /* 100010111110 */
    DpregClass::LogShift,    /* 100010111111 */
    DpregClass::LogShift,    /* 100011000000 */
    DpregClass::LogShift,    /* 100011000001 */
    DpregClass::LogShift,    /* 100011000010 */
    DpregClass::LogShift,    /* 100011000011 */
    DpregClass::LogShift,    /* 100011000100 */
    DpregClass::LogShift,    /* 100011000101 */
    DpregClass::LogShift,    /* 100011000110 */
    DpregClass::LogShift,    /* 100011000111 */
    DpregClass::LogShift,    /* 100011001000 */
    DpregClass::LogShift,    /* 100011001001 */
    DpregClass::LogShift,    /* 100011001010 */
    DpregClass::LogShift,    /* 100011001011 */
    DpregClass::LogShift,    /* 100011001100 */
    DpregClass::LogShift,    /* 100011001101 */
    DpregClass::LogShift,    /* 100011001110 */
    DpregClass::LogShift,    /* 100011001111 */
    DpregClass::LogShift,    /* 100011010000 */
    DpregClass::LogShift,    /* 100011010001 */
    DpregClass::LogShift,    /* 100011010010 */
    DpregClass::LogShift,    /* 100011010011 */
    DpregClass::LogShift,    /* 100011010100 */
    DpregClass::LogShift,    /* 100011010101 */
    DpregClass::LogShift,    /* 100011010110 */
    DpregClass::LogShift,    /* 100011010111 */
    DpregClass::LogShift,    /* 100011011000 */
    DpregClass::LogShift,    /* 100011011001 */
    DpregClass::LogShift,    /* 100011011010 */
    DpregClass::LogShift,    /* 100011011011 */
    DpregClass::LogShift,    /* 100011011100 */
    DpregClass::LogShift,    /* 100011011101 */
    DpregClass::LogShift,    /* 100011011110 */
    DpregClass::LogShift,    /* 100011011111 */
    DpregClass::LogShift,    /* 100011100000 */
    DpregClass::LogShift,    /* 100011100001 */
    DpregClass::LogShift,    /* 100011100010 */
    DpregClass::LogShift,    /* 100011100011 */
    DpregClass::LogShift,    /* 100011100100 */
    DpregClass::LogShift,    /* 100011100101 */
    DpregClass::LogShift,    /* 100011100110 */
    DpregClass::LogShift,    /* 100011100111 */
    DpregClass::LogShift,    /* 100011101000 */
    DpregClass::LogShift,    /* 100011101001 */
    DpregClass::LogShift,    /* 100011101010 */
    DpregClass::LogShift,    /* 100011101011 */
    DpregClass::LogShift,    /* 100011101100 */
    DpregClass::LogShift,    /* 100011101101 */
    DpregClass::LogShift,    /* 100011101110 */
    DpregClass::LogShift,    /* 100011101111 */
    DpregClass::LogShift,    /* 100011110000 */
    DpregClass::LogShift,    /* 100011110001 */
    DpregClass::LogShift,    /* 100011110010 */
    DpregClass::LogShift,    /* 100011110011 */
    DpregClass::LogShift,    /* 100011110100 */
    DpregClass::LogShift,    /* 100011110101 */
    DpregClass::LogShift,    /* 100011110110 */
    DpregClass::LogShift,    /* 100011110111 */
    DpregClass::LogShift,    /* 100011111000 */
    DpregClass::LogShift,    /* 100011111001 */
    DpregClass::LogShift,    /* 100011111010 */
    DpregClass::LogShift,    /* 100011111011 */
    DpregClass::LogShift,    /* 100011111100 */
    DpregClass::LogShift,    /* 100011111101 */
    DpregClass::LogShift,    /* 100011111110 */
    DpregClass::LogShift,    /* 100011111111 */
    DpregClass::LogShift,    /* 100100000000 */
    DpregClass::LogShift,    /* 100100000001 */
    DpregClass::LogShift,    /* 100100000010 */
    DpregClass::LogShift,    /* 100100000011 */
    DpregClass::LogShift,    /* 100100000100 */
    DpregClass::LogShift,    /* 100100000101 */
    DpregClass::LogShift,    /* 100100000110 */
    DpregClass::LogShift,    /* 100100000111 */
    DpregClass::LogShift,    /* 100100001000 */
    DpregClass::LogShift,    /* 100100001001 */
    DpregClass::LogShift,    /* 100100001010 */
    DpregClass::LogShift,    /* 100100001011 */
    DpregClass::LogShift,    /* 100100001100 */
    DpregClass::LogShift,    /* 100100001101 */
    DpregClass::LogShift,    /* 100100001110 */
    DpregClass::LogShift,    /* 100100001111 */
    DpregClass::LogShift,    /* 100100010000 */
    DpregClass::LogShift,    /* 100100010001 */
    DpregClass::LogShift,    /* 100100010010 */
    DpregClass::LogShift,    /* 100100010011 */
    DpregClass::LogShift,    /* 100100010100 */
    DpregClass::LogShift,    /* 100100010101 */
    DpregClass::LogShift,    /* 100100010110 */
    DpregClass::LogShift,    /* 100100010111 */
    DpregClass::LogShift,    /* 100100011000 */
    DpregClass::LogShift,    /* 100100011001 */
    DpregClass::LogShift,    /* 100100011010 */
    DpregClass::LogShift,    /* 100100011011 */
    DpregClass::LogShift,    /* 100100011100 */
    DpregClass::LogShift,    /* 100100011101 */
    DpregClass::LogShift,    /* 100100011110 */
    DpregClass::LogShift,    /* 100100011111 */
    DpregClass::LogShift,    /* 100100100000 */
    DpregClass::LogShift,    /* 100100100001 */
    DpregClass::LogShift,    /* 100100100010 */
    DpregClass::LogShift,    /* 100100100011 */
    DpregClass::LogShift,    /* 100100100100 */
    DpregClass::LogShift,    /* 100100100101 */
    DpregClass::LogShift,    /* 100100100110 */
    DpregClass::LogShift,    /* 100100100111 */
    DpregClass::LogShift,    /* 100100101000 */
    DpregClass::LogShift,    /* 100100101001 */
    DpregClass::LogShift,    /* 100100101010 */
    DpregClass::LogShift,    /* 100100101011 */
    DpregClass::LogShift,    /* 100100101100 */
    DpregClass::LogShift,    /* 100100101101 */
    DpregClass::LogShift,    /* 100100101110 */
    DpregClass::LogShift,    /* 100100101111 */
    DpregClass::LogShift,    /* 100100110000 */
    DpregClass::LogShift,    /* 100100110001 */
    DpregClass::LogShift,    /* 100100110010 */
    DpregClass::LogShift,    /* 100100110011 */
    DpregClass::LogShift,    /* 100100110100 */
    DpregClass::LogShift,    /* 100100110101 */
    DpregClass::LogShift,    /* 100100110110 */
    DpregClass::LogShift,    /* 100100110111 */
    DpregClass::LogShift,    /* 100100111000 */
    DpregClass::LogShift,    /* 100100111001 */
    DpregClass::LogShift,    /* 100100111010 */
    DpregClass::LogShift,    /* 100100111011 */
    DpregClass::LogShift,    /* 100100111100 */
    DpregClass::LogShift,    /* 100100111101 */
    DpregClass::LogShift,    /* 100100111110 */
    DpregClass::LogShift,    /* 100100111111 */
    DpregClass::LogShift,    /* 100101000000 */
    DpregClass::LogShift,    /* 100101000001 */
    DpregClass::LogShift,    /* 100101000010 */
    DpregClass::LogShift,    /* 100101000011 */
    DpregClass::LogShift,    /* 100101000100 */
    DpregClass::LogShift,    /* 100101000101 */
    DpregClass::LogShift,    /* 100101000110 */
    DpregClass::LogShift,    /* 100101000111 */
    DpregClass::LogShift,    /* 100101001000 */
    DpregClass::LogShift,    /* 100101001001 */
    DpregClass::LogShift,    /* 100101001010 */
    DpregClass::LogShift,    /* 100101001011 */
    DpregClass::LogShift,    /* 100101001100 */
    DpregClass::LogShift,    /* 100101001101 */
    DpregClass::LogShift,    /* 100101001110 */
    DpregClass::LogShift,    /* 100101001111 */
    DpregClass::LogShift,    /* 100101010000 */
    DpregClass::LogShift,    /* 100101010001 */
    DpregClass::LogShift,    /* 100101010010 */
    DpregClass::LogShift,    /* 100101010011 */
    DpregClass::LogShift,    /* 100101010100 */
    DpregClass::LogShift,    /* 100101010101 */
    DpregClass::LogShift,    /* 100101010110 */
    DpregClass::LogShift,    /* 100101010111 */
    DpregClass::LogShift,    /* 100101011000 */
    DpregClass::LogShift,    /* 100101011001 */
    DpregClass::LogShift,    /* 100101011010 */
    DpregClass::LogShift,    /* 100101011011 */
    DpregClass::LogShift,    /* 100101011100 */
    DpregClass::LogShift,    /* 100101011101 */
    DpregClass::LogShift,    /* 100101011110 */
    DpregClass::LogShift,    /* 100101011111 */
    DpregClass::LogShift,    /* 100101100000 */
    DpregClass::LogShift,    /* 100101100001 */
    DpregClass::LogShift,    /* 100101100010 */
    DpregClass::LogShift,    /* 100101100011 */
    DpregClass::LogShift,    /* 100101100100 */
    DpregClass::LogShift,    /* 100101100101 */
    DpregClass::LogShift,    /* 100101100110 */
    DpregClass::LogShift,    /* 100101100111 */
    DpregClass::LogShift,    /* 100101101000 */
    DpregClass::LogShift,    /* 100101101001 */
    DpregClass::LogShift,    /* 100101101010 */
    DpregClass::LogShift,    /* 100101101011 */
    DpregClass::LogShift,    /* 100101101100 */
    DpregClass::LogShift,    /* 100101101101 */
    DpregClass::LogShift,    /* 100101101110 */
    DpregClass::LogShift,    /* 100101101111 */
    DpregClass::LogShift,    /* 100101110000 */
    DpregClass::LogShift,    /* 100101110001 */
    DpregClass::LogShift,    /* 100101110010 */
    DpregClass::LogShift,    /* 100101110011 */
    DpregClass::LogShift,    /* 100101110100 */
    DpregClass::LogShift,    /* 100101110101 */
    DpregClass::LogShift,    /* 100101110110 */
    DpregClass::LogShift,    /* 100101110111 */
    DpregClass::LogShift,    /* 100101111000 */
    DpregClass::LogShift,    /* 100101111001 */
    DpregClass::LogShift,    /* 100101111010 */
    DpregClass::LogShift,    /* 100101111011 */
    DpregClass::LogShift,    /* 100101111100 */
    DpregClass::LogShift,    /* 100101111101 */
    DpregClass::LogShift,    /* 100101111110 */
    DpregClass::LogShift,    /* 100101111111 */
    DpregClass::LogShift,    /* 100110000000 */
    DpregClass::LogShift,    /* 100110000001 */
    DpregClass::LogShift,    /* 100110000010 */
    DpregClass::LogShift,    /* 100110000011 */
    DpregClass::LogShift,    /* 100110000100 */
    DpregClass::LogShift,    /* 100110000101 */
    DpregClass::LogShift,    /* 100110000110 */
    DpregClass::LogShift,    /* 100110000111 */
    DpregClass::LogShift,    /* 100110001000 */
    DpregClass::LogShift,    /* 100110001001 */
    DpregClass::LogShift,    /* 100110001010 */
    DpregClass::LogShift,    /* 100110001011 */
    DpregClass::LogShift,    /* 100110001100 */
    DpregClass::LogShift,    /* 100110001101 */
    DpregClass::LogShift,    /* 100110001110 */
    DpregClass::LogShift,    /* 100110001111 */
    DpregClass::LogShift,    /* 100110010000 */
    DpregClass::LogShift,    /* 100110010001 */
    DpregClass::LogShift,    /* 100110010010 */
    DpregClass::LogShift,    /* 100110010011 */
    DpregClass::LogShift,    /* 100110010100 */
    DpregClass::LogShift,    /* 100110010101 */
    DpregClass::LogShift,    /* 100110010110 */
    DpregClass::LogShift,    /* 100110010111 */
    DpregClass::LogShift,    /* 100110011000 */
    DpregClass::LogShift,    /* 100110011001 */
    DpregClass::LogShift,    /* 100110011010 */
    DpregClass::LogShift,    /* 100110011011 */
    DpregClass::LogShift,    /* 100110011100 */
    DpregClass::LogShift,    /* 100110011101 */
    DpregClass::LogShift,    /* 100110011110 */
    DpregClass::LogShift,    /* 100110011111 */
    DpregClass::LogShift,    /* 100110100000 */
    DpregClass::LogShift,    /* 100110100001 */
    DpregClass::LogShift,    /* 100110100010 */
    DpregClass::LogShift,    /* 100110100011 */
    DpregClass::LogShift,    /* 100110100100 */
    DpregClass::LogShift,    /* 100110100101 */
    DpregClass::LogShift,    /* 100110100110 */
    DpregClass::LogShift,    /* 100110100111 */
    DpregClass::LogShift,    /* 100110101000 */
    DpregClass::LogShift,    /* 100110101001 */
    DpregClass::LogShift,    /* 100110101010 */
    DpregClass::LogShift,    /* 100110101011 */
    DpregClass::LogShift,    /* 100110101100 */
    DpregClass::LogShift,    /* 100110101101 */
    DpregClass::LogShift,    /* 100110101110 */
    DpregClass::LogShift,    /* 100110101111 */
    DpregClass::LogShift,    /* 100110110000 */
    DpregClass::LogShift,    /* 100110110001 */
    DpregClass::LogShift,    /* 100110110010 */
    DpregClass::LogShift,    /* 100110110011 */
    DpregClass::LogShift,    /* 100110110100 */
    DpregClass::LogShift,    /* 100110110101 */
    DpregClass::LogShift,    /* 100110110110 */
    DpregClass::LogShift,    /* 100110110111 */
    DpregClass::LogShift,    /* 100110111000 */
    DpregClass::LogShift,    /* 100110111001 */
    DpregClass::LogShift,    /* 100110111010 */
    DpregClass::LogShift,    /* 100110111011 */
    DpregClass::LogShift,    /* 100110111100 */
    DpregClass::LogShift,    /* 100110111101 */
    DpregClass::LogShift,    /* 100110111110 */
    DpregClass::LogShift,    /* 100110111111 */
    DpregClass::LogShift,    /* 100111000000 */
    DpregClass::LogShift,    /* 100111000001 */
    DpregClass::LogShift,    /* 100111000010 */
    DpregClass::LogShift,    /* 100111000011 */
    DpregClass::LogShift,    /* 100111000100 */
    DpregClass::LogShift,    /* 100111000101 */
    DpregClass::LogShift,    /* 100111000110 */
    DpregClass::LogShift,    /* 100111000111 */
    DpregClass::LogShift,    /* 100111001000 */
    DpregClass::LogShift,    /* 100111001001 */
    DpregClass::LogShift,    /* 100111001010 */
    DpregClass::LogShift,    /* 100111001011 */
    DpregClass::LogShift,    /* 100111001100 */
    DpregClass::LogShift,    /* 100111001101 */
    DpregClass::LogShift,    /* 100111001110 */
    DpregClass::LogShift,    /* 100111001111 */
    DpregClass::LogShift,    /* 100111010000 */
    DpregClass::LogShift,    /* 100111010001 */
    DpregClass::LogShift,    /* 100111010010 */
    DpregClass::LogShift,    /* 100111010011 */
    DpregClass::LogShift,    /* 100111010100 */
    DpregClass::LogShift,    /* 100111010101 */
    DpregClass::LogShift,    /* 100111010110 */
    DpregClass::LogShift,    /* 100111010111 */
    DpregClass::LogShift,    /* 100111011000 */
    DpregClass::LogShift,    /* 100111011001 */
    DpregClass::LogShift,    /* 100111011010 */
    DpregClass::LogShift,    /* 100111011011 */
    DpregClass::LogShift,    /* 100111011100 */
    DpregClass::LogShift,    /* 100111011101 */
    DpregClass::LogShift,    /* 100111011110 */
    DpregClass::LogShift,    /* 100111011111 */
    DpregClass::LogShift,    /* 100111100000 */
    DpregClass::LogShift,    /* 100111100001 */
    DpregClass::LogShift,    /* 100111100010 */
    DpregClass::LogShift,    /* 100111100011 */
    DpregClass::LogShift,    /* 100111100100 */
    DpregClass::LogShift,    /* 100111100101 */
    DpregClass::LogShift,    /* 100111100110 */
    DpregClass::LogShift,    /* 100111100111 */
    DpregClass::LogShift,    /* 100111101000 */
    DpregClass::LogShift,    /* 100111101001 */
    DpregClass::LogShift,    /* 100111101010 */
    DpregClass::LogShift,    /* 100111101011 */
    DpregClass::LogShift,    /* 100111101100 */
    DpregClass::LogShift,    /* 100111101101 */
    DpregClass::LogShift,    /* 100111101110 */
    DpregClass::LogShift,    /* 100111101111 */
    DpregClass::LogShift,    /* 100111110000 */
    DpregClass::LogShift,    /* 100111110001 */
    DpregClass::LogShift,    /* 100111110010 */
    DpregClass::LogShift,    /* 100111110011 */
    DpregClass::LogShift,    /* 100111110100 */
    DpregClass::LogShift,    /* 100111110101 */
    DpregClass::LogShift,    /* 100111110110 */
    DpregClass::LogShift,    /* 100111110111 */
    DpregClass::LogShift,    /* 100111111000 */
    DpregClass::LogShift,    /* 100111111001 */
    DpregClass::LogShift,    /* 100111111010 */
    DpregClass::LogShift,    /* 100111111011 */
    DpregClass::LogShift,    /* 100111111100 */
    DpregClass::LogShift,    /* 100111111101 */
    DpregClass::LogShift,    /* 100111111110 */
    DpregClass::LogShift,    /* 100111111111 */
    DpregClass::AddsubShift, /* 101000000000 */
    DpregClass::AddsubShift, /* 101000000001 */
    DpregClass::AddsubShift, /* 101000000010 */
    DpregClass::AddsubShift, /* 101000000011 */
    DpregClass::AddsubShift, /* 101000000100 */
    DpregClass::AddsubShift, /* 101000000101 */
    DpregClass::AddsubShift, /* 101000000110 */
    DpregClass::AddsubShift, /* 101000000111 */
    DpregClass::AddsubShift, /* 101000001000 */
    DpregClass::AddsubShift, /* 101000001001 */
    DpregClass::AddsubShift, /* 101000001010 */
    DpregClass::AddsubShift, /* 101000001011 */
    DpregClass::AddsubShift, /* 101000001100 */
    DpregClass::AddsubShift, /* 101000001101 */
    DpregClass::AddsubShift, /* 101000001110 */
    DpregClass::AddsubShift, /* 101000001111 */
    DpregClass::AddsubShift, /* 101000010000 */
    DpregClass::AddsubShift, /* 101000010001 */
    DpregClass::AddsubShift, /* 101000010010 */
    DpregClass::AddsubShift, /* 101000010011 */
    DpregClass::AddsubShift, /* 101000010100 */
    DpregClass::AddsubShift, /* 101000010101 */
    DpregClass::AddsubShift, /* 101000010110 */
    DpregClass::AddsubShift, /* 101000010111 */
    DpregClass::AddsubShift, /* 101000011000 */
    DpregClass::AddsubShift, /* 101000011001 */
    DpregClass::AddsubShift, /* 101000011010 */
    DpregClass::AddsubShift, /* 101000011011 */
    DpregClass::AddsubShift, /* 101000011100 */
    DpregClass::AddsubShift, /* 101000011101 */
    DpregClass::AddsubShift, /* 101000011110 */
    DpregClass::AddsubShift, /* 101000011111 */
    DpregClass::AddsubShift, /* 101000100000 */
    DpregClass::AddsubShift, /* 101000100001 */
    DpregClass::AddsubShift, /* 101000100010 */
    DpregClass::AddsubShift, /* 101000100011 */
    DpregClass::AddsubShift, /* 101000100100 */
    DpregClass::AddsubShift, /* 101000100101 */
    DpregClass::AddsubShift, /* 101000100110 */
    DpregClass::AddsubShift, /* 101000100111 */
    DpregClass::AddsubShift, /* 101000101000 */
    DpregClass::AddsubShift, /* 101000101001 */
    DpregClass::AddsubShift, /* 101000101010 */
    DpregClass::AddsubShift, /* 101000101011 */
    DpregClass::AddsubShift, /* 101000101100 */
    DpregClass::AddsubShift, /* 101000101101 */
    DpregClass::AddsubShift, /* 101000101110 */
    DpregClass::AddsubShift, /* 101000101111 */
    DpregClass::AddsubShift, /* 101000110000 */
    DpregClass::AddsubShift, /* 101000110001 */
    DpregClass::AddsubShift, /* 101000110010 */
    DpregClass::AddsubShift, /* 101000110011 */
    DpregClass::AddsubShift, /* 101000110100 */
    DpregClass::AddsubShift, /* 101000110101 */
    DpregClass::AddsubShift, /* 101000110110 */
    DpregClass::AddsubShift, /* 101000110111 */
    DpregClass::AddsubShift, /* 101000111000 */
    DpregClass::AddsubShift, /* 101000111001 */
    DpregClass::AddsubShift, /* 101000111010 */
    DpregClass::AddsubShift, /* 101000111011 */
    DpregClass::AddsubShift, /* 101000111100 */
    DpregClass::AddsubShift, /* 101000111101 */
    DpregClass::AddsubShift, /* 101000111110 */
    DpregClass::AddsubShift, /* 101000111111 */
    DpregClass::AddsubExt,   /* 101001000000 */
    DpregClass::AddsubExt,   /* 101001000001 */
    DpregClass::AddsubExt,   /* 101001000010 */
    DpregClass::AddsubExt,   /* 101001000011 */
    DpregClass::AddsubExt,   /* 101001000100 */
    DpregClass::AddsubExt,   /* 101001000101 */
    DpregClass::AddsubExt,   /* 101001000110 */
    DpregClass::AddsubExt,   /* 101001000111 */
    DpregClass::AddsubExt,   /* 101001001000 */
    DpregClass::AddsubExt,   /* 101001001001 */
    DpregClass::AddsubExt,   /* 101001001010 */
    DpregClass::AddsubExt,   /* 101001001011 */
    DpregClass::AddsubExt,   /* 101001001100 */
    DpregClass::AddsubExt,   /* 101001001101 */
    DpregClass::AddsubExt,   /* 101001001110 */
    DpregClass::AddsubExt,   /* 101001001111 */
    DpregClass::AddsubExt,   /* 101001010000 */
    DpregClass::AddsubExt,   /* 101001010001 */
    DpregClass::AddsubExt,   /* 101001010010 */
    DpregClass::AddsubExt,   /* 101001010011 */
    DpregClass::AddsubExt,   /* 101001010100 */
    DpregClass::AddsubExt,   /* 101001010101 */
    DpregClass::AddsubExt,   /* 101001010110 */
    DpregClass::AddsubExt,   /* 101001010111 */
    DpregClass::AddsubExt,   /* 101001011000 */
    DpregClass::AddsubExt,   /* 101001011001 */
    DpregClass::AddsubExt,   /* 101001011010 */
    DpregClass::AddsubExt,   /* 101001011011 */
    DpregClass::AddsubExt,   /* 101001011100 */
    DpregClass::AddsubExt,   /* 101001011101 */
    DpregClass::AddsubExt,   /* 101001011110 */
    DpregClass::AddsubExt,   /* 101001011111 */
    DpregClass::AddsubExt,   /* 101001100000 */
    DpregClass::AddsubExt,   /* 101001100001 */
    DpregClass::AddsubExt,   /* 101001100010 */
    DpregClass::AddsubExt,   /* 101001100011 */
    DpregClass::AddsubExt,   /* 101001100100 */
    DpregClass::AddsubExt,   /* 101001100101 */
    DpregClass::AddsubExt,   /* 101001100110 */
    DpregClass::AddsubExt,   /* 101001100111 */
    DpregClass::AddsubExt,   /* 101001101000 */
    DpregClass::AddsubExt,   /* 101001101001 */
    DpregClass::AddsubExt,   /* 101001101010 */
    DpregClass::AddsubExt,   /* 101001101011 */
    DpregClass::AddsubExt,   /* 101001101100 */
    DpregClass::AddsubExt,   /* 101001101101 */
    DpregClass::AddsubExt,   /* 101001101110 */
    DpregClass::AddsubExt,   /* 101001101111 */
    DpregClass::AddsubExt,   /* 101001110000 */
    DpregClass::AddsubExt,   /* 101001110001 */
    DpregClass::AddsubExt,   /* 101001110010 */
    DpregClass::AddsubExt,   /* 101001110011 */
    DpregClass::AddsubExt,   /* 101001110100 */
    DpregClass::AddsubExt,   /* 101001110101 */
    DpregClass::AddsubExt,   /* 101001110110 */
    DpregClass::AddsubExt,   /* 101001110111 */
    DpregClass::AddsubExt,   /* 101001111000 */
    DpregClass::AddsubExt,   /* 101001111001 */
    DpregClass::AddsubExt,   /* 101001111010 */
    DpregClass::AddsubExt,   /* 101001111011 */
    DpregClass::AddsubExt,   /* 101001111100 */
    DpregClass::AddsubExt,   /* 101001111101 */
    DpregClass::AddsubExt,   /* 101001111110 */
    DpregClass::AddsubExt,   /* 101001111111 */
    DpregClass::AddsubShift, /* 101010000000 */
    DpregClass::AddsubShift, /* 101010000001 */
    DpregClass::AddsubShift, /* 101010000010 */
    DpregClass::AddsubShift, /* 101010000011 */
    DpregClass::AddsubShift, /* 101010000100 */
    DpregClass::AddsubShift, /* 101010000101 */
    DpregClass::AddsubShift, /* 101010000110 */
    DpregClass::AddsubShift, /* 101010000111 */
    DpregClass::AddsubShift, /* 101010001000 */
    DpregClass::AddsubShift, /* 101010001001 */
    DpregClass::AddsubShift, /* 101010001010 */
    DpregClass::AddsubShift, /* 101010001011 */
    DpregClass::AddsubShift, /* 101010001100 */
    DpregClass::AddsubShift, /* 101010001101 */
    DpregClass::AddsubShift, /* 101010001110 */
    DpregClass::AddsubShift, /* 101010001111 */
    DpregClass::AddsubShift, /* 101010010000 */
    DpregClass::AddsubShift, /* 101010010001 */
    DpregClass::AddsubShift, /* 101010010010 */
    DpregClass::AddsubShift, /* 101010010011 */
    DpregClass::AddsubShift, /* 101010010100 */
    DpregClass::AddsubShift, /* 101010010101 */
    DpregClass::AddsubShift, /* 101010010110 */
    DpregClass::AddsubShift, /* 101010010111 */
    DpregClass::AddsubShift, /* 101010011000 */
    DpregClass::AddsubShift, /* 101010011001 */
    DpregClass::AddsubShift, /* 101010011010 */
    DpregClass::AddsubShift, /* 101010011011 */
    DpregClass::AddsubShift, /* 101010011100 */
    DpregClass::AddsubShift, /* 101010011101 */
    DpregClass::AddsubShift, /* 101010011110 */
    DpregClass::AddsubShift, /* 101010011111 */
    DpregClass::AddsubShift, /* 101010100000 */
    DpregClass::AddsubShift, /* 101010100001 */
    DpregClass::AddsubShift, /* 101010100010 */
    DpregClass::AddsubShift, /* 101010100011 */
    DpregClass::AddsubShift, /* 101010100100 */
    DpregClass::AddsubShift, /* 101010100101 */
    DpregClass::AddsubShift, /* 101010100110 */
    DpregClass::AddsubShift, /* 101010100111 */
    DpregClass::AddsubShift, /* 101010101000 */
    DpregClass::AddsubShift, /* 101010101001 */
    DpregClass::AddsubShift, /* 101010101010 */
    DpregClass::AddsubShift, /* 101010101011 */
    DpregClass::AddsubShift, /* 101010101100 */
    DpregClass::AddsubShift, /* 101010101101 */
    DpregClass::AddsubShift, /* 101010101110 */
    DpregClass::AddsubShift, /* 101010101111 */
    DpregClass::AddsubShift, /* 101010110000 */
    DpregClass::AddsubShift, /* 101010110001 */
    DpregClass::AddsubShift, /* 101010110010 */
    DpregClass::AddsubShift, /* 101010110011 */
    DpregClass::AddsubShift, /* 101010110100 */
    DpregClass::AddsubShift, /* 101010110101 */
    DpregClass::AddsubShift, /* 101010110110 */
    DpregClass::AddsubShift, /* 101010110111 */
    DpregClass::AddsubShift, /* 101010111000 */
    DpregClass::AddsubShift, /* 101010111001 */
    DpregClass::AddsubShift, /* 101010111010 */
    DpregClass::AddsubShift, /* 101010111011 */
    DpregClass::AddsubShift, /* 101010111100 */
    DpregClass::AddsubShift, /* 101010111101 */
    DpregClass::AddsubShift, /* 101010111110 */
    DpregClass::AddsubShift, /* 101010111111 */
    DpregClass::AddsubExt,   /* 101011000000 */
    DpregClass::AddsubExt,   /* 101011000001 */
    DpregClass::AddsubExt,   /* 101011000010 */
    DpregClass::AddsubExt,   /* 101011000011 */
    DpregClass::AddsubExt,   /* 101011000100 */
    DpregClass::AddsubExt,   /* 101011000101 */
    DpregClass::AddsubExt,   /* 101011000110 */
    DpregClass::AddsubExt,   /* 101011000111 */
    DpregClass::AddsubExt,   /* 101011001000 */
    DpregClass::AddsubExt,   /* 101011001001 */
    DpregClass::AddsubExt,   /* 101011001010 */
    DpregClass::AddsubExt,   /* 101011001011 */
    DpregClass::AddsubExt,   /* 101011001100 */
    DpregClass::AddsubExt,   /* 101011001101 */
    DpregClass::AddsubExt,   /* 101011001110 */
    DpregClass::AddsubExt,   /* 101011001111 */
    DpregClass::AddsubExt,   /* 101011010000 */
    DpregClass::AddsubExt,   /* 101011010001 */
    DpregClass::AddsubExt,   /* 101011010010 */
    DpregClass::AddsubExt,   /* 101011010011 */
    DpregClass::AddsubExt,   /* 101011010100 */
    DpregClass::AddsubExt,   /* 101011010101 */
    DpregClass::AddsubExt,   /* 101011010110 */
    DpregClass::AddsubExt,   /* 101011010111 */
    DpregClass::AddsubExt,   /* 101011011000 */
    DpregClass::AddsubExt,   /* 101011011001 */
    DpregClass::AddsubExt,   /* 101011011010 */
    DpregClass::AddsubExt,   /* 101011011011 */
    DpregClass::AddsubExt,   /* 101011011100 */
    DpregClass::AddsubExt,   /* 101011011101 */
    DpregClass::AddsubExt,   /* 101011011110 */
    DpregClass::AddsubExt,   /* 101011011111 */
    DpregClass::AddsubExt,   /* 101011100000 */
    DpregClass::AddsubExt,   /* 101011100001 */
    DpregClass::AddsubExt,   /* 101011100010 */
    DpregClass::AddsubExt,   /* 101011100011 */
    DpregClass::AddsubExt,   /* 101011100100 */
    DpregClass::AddsubExt,   /* 101011100101 */
    DpregClass::AddsubExt,   /* 101011100110 */
    DpregClass::AddsubExt,   /* 101011100111 */
    DpregClass::AddsubExt,   /* 101011101000 */
    DpregClass::AddsubExt,   /* 101011101001 */
    DpregClass::AddsubExt,   /* 101011101010 */
    DpregClass::AddsubExt,   /* 101011101011 */
    DpregClass::AddsubExt,   /* 101011101100 */
    DpregClass::AddsubExt,   /* 101011101101 */
    DpregClass::AddsubExt,   /* 101011101110 */
    DpregClass::AddsubExt,   /* 101011101111 */
    DpregClass::AddsubExt,   /* 101011110000 */
    DpregClass::AddsubExt,   /* 101011110001 */
    DpregClass::AddsubExt,   /* 101011110010 */
    DpregClass::AddsubExt,   /* 101011110011 */
    DpregClass::AddsubExt,   /* 101011110100 */
    DpregClass::AddsubExt,   /* 101011110101 */
    DpregClass::AddsubExt,   /* 101011110110 */
    DpregClass::AddsubExt,   /* 101011110111 */
    DpregClass::AddsubExt,   /* 101011111000 */
    DpregClass::AddsubExt,   /* 101011111001 */
    DpregClass::AddsubExt,   /* 101011111010 */
    DpregClass::AddsubExt,   /* 101011111011 */
    DpregClass::AddsubExt,   /* 101011111100 */
    DpregClass::AddsubExt,   /* 101011111101 */
    DpregClass::AddsubExt,   /* 101011111110 */
    DpregClass::AddsubExt,   /* 101011111111 */
    DpregClass::AddsubShift, /* 101100000000 */
    DpregClass::AddsubShift, /* 101100000001 */
    DpregClass::AddsubShift, /* 101100000010 */
    DpregClass::AddsubShift, /* 101100000011 */
    DpregClass::AddsubShift, /* 101100000100 */
    DpregClass::AddsubShift, /* 101100000101 */
    DpregClass::AddsubShift, /* 101100000110 */
    DpregClass::AddsubShift, /* 101100000111 */
    DpregClass::AddsubShift, /* 101100001000 */
    DpregClass::AddsubShift, /* 101100001001 */
    DpregClass::AddsubShift, /* 101100001010 */
    DpregClass::AddsubShift, /* 101100001011 */
    DpregClass::AddsubShift, /* 101100001100 */
    DpregClass::AddsubShift, /* 101100001101 */
    DpregClass::AddsubShift, /* 101100001110 */
    DpregClass::AddsubShift, /* 101100001111 */
    DpregClass::AddsubShift, /* 101100010000 */
    DpregClass::AddsubShift, /* 101100010001 */
    DpregClass::AddsubShift, /* 101100010010 */
    DpregClass::AddsubShift, /* 101100010011 */
    DpregClass::AddsubShift, /* 101100010100 */
    DpregClass::AddsubShift, /* 101100010101 */
    DpregClass::AddsubShift, /* 101100010110 */
    DpregClass::AddsubShift, /* 101100010111 */
    DpregClass::AddsubShift, /* 101100011000 */
    DpregClass::AddsubShift, /* 101100011001 */
    DpregClass::AddsubShift, /* 101100011010 */
    DpregClass::AddsubShift, /* 101100011011 */
    DpregClass::AddsubShift, /* 101100011100 */
    DpregClass::AddsubShift, /* 101100011101 */
    DpregClass::AddsubShift, /* 101100011110 */
    DpregClass::AddsubShift, /* 101100011111 */
    DpregClass::AddsubShift, /* 101100100000 */
    DpregClass::AddsubShift, /* 101100100001 */
    DpregClass::AddsubShift, /* 101100100010 */
    DpregClass::AddsubShift, /* 101100100011 */
    DpregClass::AddsubShift, /* 101100100100 */
    DpregClass::AddsubShift, /* 101100100101 */
    DpregClass::AddsubShift, /* 101100100110 */
    DpregClass::AddsubShift, /* 101100100111 */
    DpregClass::AddsubShift, /* 101100101000 */
    DpregClass::AddsubShift, /* 101100101001 */
    DpregClass::AddsubShift, /* 101100101010 */
    DpregClass::AddsubShift, /* 101100101011 */
    DpregClass::AddsubShift, /* 101100101100 */
    DpregClass::AddsubShift, /* 101100101101 */
    DpregClass::AddsubShift, /* 101100101110 */
    DpregClass::AddsubShift, /* 101100101111 */
    DpregClass::AddsubShift, /* 101100110000 */
    DpregClass::AddsubShift, /* 101100110001 */
    DpregClass::AddsubShift, /* 101100110010 */
    DpregClass::AddsubShift, /* 101100110011 */
    DpregClass::AddsubShift, /* 101100110100 */
    DpregClass::AddsubShift, /* 101100110101 */
    DpregClass::AddsubShift, /* 101100110110 */
    DpregClass::AddsubShift, /* 101100110111 */
    DpregClass::AddsubShift, /* 101100111000 */
    DpregClass::AddsubShift, /* 101100111001 */
    DpregClass::AddsubShift, /* 101100111010 */
    DpregClass::AddsubShift, /* 101100111011 */
    DpregClass::AddsubShift, /* 101100111100 */
    DpregClass::AddsubShift, /* 101100111101 */
    DpregClass::AddsubShift, /* 101100111110 */
    DpregClass::AddsubShift, /* 101100111111 */
    DpregClass::AddsubExt,   /* 101101000000 */
    DpregClass::AddsubExt,   /* 101101000001 */
    DpregClass::AddsubExt,   /* 101101000010 */
    DpregClass::AddsubExt,   /* 101101000011 */
    DpregClass::AddsubExt,   /* 101101000100 */
    DpregClass::AddsubExt,   /* 101101000101 */
    DpregClass::AddsubExt,   /* 101101000110 */
    DpregClass::AddsubExt,   /* 101101000111 */
    DpregClass::AddsubExt,   /* 101101001000 */
    DpregClass::AddsubExt,   /* 101101001001 */
    DpregClass::AddsubExt,   /* 101101001010 */
    DpregClass::AddsubExt,   /* 101101001011 */
    DpregClass::AddsubExt,   /* 101101001100 */
    DpregClass::AddsubExt,   /* 101101001101 */
    DpregClass::AddsubExt,   /* 101101001110 */
    DpregClass::AddsubExt,   /* 101101001111 */
    DpregClass::AddsubExt,   /* 101101010000 */
    DpregClass::AddsubExt,   /* 101101010001 */
    DpregClass::AddsubExt,   /* 101101010010 */
    DpregClass::AddsubExt,   /* 101101010011 */
    DpregClass::AddsubExt,   /* 101101010100 */
    DpregClass::AddsubExt,   /* 101101010101 */
    DpregClass::AddsubExt,   /* 101101010110 */
    DpregClass::AddsubExt,   /* 101101010111 */
    DpregClass::AddsubExt,   /* 101101011000 */
    DpregClass::AddsubExt,   /* 101101011001 */
    DpregClass::AddsubExt,   /* 101101011010 */
    DpregClass::AddsubExt,   /* 101101011011 */
    DpregClass::AddsubExt,   /* 101101011100 */
    DpregClass::AddsubExt,   /* 101101011101 */
    DpregClass::AddsubExt,   /* 101101011110 */
    DpregClass::AddsubExt,   /* 101101011111 */
    DpregClass::AddsubExt,   /* 101101100000 */
    DpregClass::AddsubExt,   /* 101101100001 */
    DpregClass::AddsubExt,   /* 101101100010 */
    DpregClass::AddsubExt,   /* 101101100011 */
    DpregClass::AddsubExt,   /* 101101100100 */
    DpregClass::AddsubExt,   /* 101101100101 */
    DpregClass::AddsubExt,   /* 101101100110 */
    DpregClass::AddsubExt,   /* 101101100111 */
    DpregClass::AddsubExt,   /* 101101101000 */
    DpregClass::AddsubExt,   /* 101101101001 */
    DpregClass::AddsubExt,   /* 101101101010 */
    DpregClass::AddsubExt,   /* 101101101011 */
    DpregClass::AddsubExt,   /* 101101101100 */
    DpregClass::AddsubExt,   /* 101101101101 */
    DpregClass::AddsubExt,   /* 101101101110 */
    DpregClass::AddsubExt,   /* 101101101111 */
    DpregClass::AddsubExt,   /* 101101110000 */
    DpregClass::AddsubExt,   /* 101101110001 */
    DpregClass::AddsubExt,   /* 101101110010 */
    DpregClass::AddsubExt,   /* 101101110011 */
    DpregClass::AddsubExt,   /* 101101110100 */
    DpregClass::AddsubExt,   /* 101101110101 */
    DpregClass::AddsubExt,   /* 101101110110 */
    DpregClass::AddsubExt,   /* 101101110111 */
    DpregClass::AddsubExt,   /* 101101111000 */
    DpregClass::AddsubExt,   /* 101101111001 */
    DpregClass::AddsubExt,   /* 101101111010 */
    DpregClass::AddsubExt,   /* 101101111011 */
    DpregClass::AddsubExt,   /* 101101111100 */
    DpregClass::AddsubExt,   /* 101101111101 */
    DpregClass::AddsubExt,   /* 101101111110 */
    DpregClass::AddsubExt,   /* 101101111111 */
    DpregClass::AddsubShift, /* 101110000000 */
    DpregClass::AddsubShift, /* 101110000001 */
    DpregClass::AddsubShift, /* 101110000010 */
    DpregClass::AddsubShift, /* 101110000011 */
    DpregClass::AddsubShift, /* 101110000100 */
    DpregClass::AddsubShift, /* 101110000101 */
    DpregClass::AddsubShift, /* 101110000110 */
    DpregClass::AddsubShift, /* 101110000111 */
    DpregClass::AddsubShift, /* 101110001000 */
    DpregClass::AddsubShift, /* 101110001001 */
    DpregClass::AddsubShift, /* 101110001010 */
    DpregClass::AddsubShift, /* 101110001011 */
    DpregClass::AddsubShift, /* 101110001100 */
    DpregClass::AddsubShift, /* 101110001101 */
    DpregClass::AddsubShift, /* 101110001110 */
    DpregClass::AddsubShift, /* 101110001111 */
    DpregClass::AddsubShift, /* 101110010000 */
    DpregClass::AddsubShift, /* 101110010001 */
    DpregClass::AddsubShift, /* 101110010010 */
    DpregClass::AddsubShift, /* 101110010011 */
    DpregClass::AddsubShift, /* 101110010100 */
    DpregClass::AddsubShift, /* 101110010101 */
    DpregClass::AddsubShift, /* 101110010110 */
    DpregClass::AddsubShift, /* 101110010111 */
    DpregClass::AddsubShift, /* 101110011000 */
    DpregClass::AddsubShift, /* 101110011001 */
    DpregClass::AddsubShift, /* 101110011010 */
    DpregClass::AddsubShift, /* 101110011011 */
    DpregClass::AddsubShift, /* 101110011100 */
    DpregClass::AddsubShift, /* 101110011101 */
    DpregClass::AddsubShift, /* 101110011110 */
    DpregClass::AddsubShift, /* 101110011111 */
    DpregClass::AddsubShift, /* 101110100000 */
    DpregClass::AddsubShift, /* 101110100001 */
    DpregClass::AddsubShift, /* 101110100010 */
    DpregClass::AddsubShift, /* 101110100011 */
    DpregClass::AddsubShift, /* 101110100100 */
    DpregClass::AddsubShift, /* 101110100101 */
    DpregClass::AddsubShift, /* 101110100110 */
    DpregClass::AddsubShift, /* 101110100111 */
    DpregClass::AddsubShift, /* 101110101000 */
    DpregClass::AddsubShift, /* 101110101001 */
    DpregClass::AddsubShift, /* 101110101010 */
    DpregClass::AddsubShift, /* 101110101011 */
    DpregClass::AddsubShift, /* 101110101100 */
    DpregClass::AddsubShift, /* 101110101101 */
    DpregClass::AddsubShift, /* 101110101110 */
    DpregClass::AddsubShift, /* 101110101111 */
    DpregClass::AddsubShift, /* 101110110000 */
    DpregClass::AddsubShift, /* 101110110001 */
    DpregClass::AddsubShift, /* 101110110010 */
    DpregClass::AddsubShift, /* 101110110011 */
    DpregClass::AddsubShift, /* 101110110100 */
    DpregClass::AddsubShift, /* 101110110101 */
    DpregClass::AddsubShift, /* 101110110110 */
    DpregClass::AddsubShift, /* 101110110111 */
    DpregClass::AddsubShift, /* 101110111000 */
    DpregClass::AddsubShift, /* 101110111001 */
    DpregClass::AddsubShift, /* 101110111010 */
    DpregClass::AddsubShift, /* 101110111011 */
    DpregClass::AddsubShift, /* 101110111100 */
    DpregClass::AddsubShift, /* 101110111101 */
    DpregClass::AddsubShift, /* 101110111110 */
    DpregClass::AddsubShift, /* 101110111111 */
    DpregClass::AddsubExt,   /* 101111000000 */
    DpregClass::AddsubExt,   /* 101111000001 */
    DpregClass::AddsubExt,   /* 101111000010 */
    DpregClass::AddsubExt,   /* 101111000011 */
    DpregClass::AddsubExt,   /* 101111000100 */
    DpregClass::AddsubExt,   /* 101111000101 */
    DpregClass::AddsubExt,   /* 101111000110 */
    DpregClass::AddsubExt,   /* 101111000111 */
    DpregClass::AddsubExt,   /* 101111001000 */
    DpregClass::AddsubExt,   /* 101111001001 */
    DpregClass::AddsubExt,   /* 101111001010 */
    DpregClass::AddsubExt,   /* 101111001011 */
    DpregClass::AddsubExt,   /* 101111001100 */
    DpregClass::AddsubExt,   /* 101111001101 */
    DpregClass::AddsubExt,   /* 101111001110 */
    DpregClass::AddsubExt,   /* 101111001111 */
    DpregClass::AddsubExt,   /* 101111010000 */
    DpregClass::AddsubExt,   /* 101111010001 */
    DpregClass::AddsubExt,   /* 101111010010 */
    DpregClass::AddsubExt,   /* 101111010011 */
    DpregClass::AddsubExt,   /* 101111010100 */
    DpregClass::AddsubExt,   /* 101111010101 */
    DpregClass::AddsubExt,   /* 101111010110 */
    DpregClass::AddsubExt,   /* 101111010111 */
    DpregClass::AddsubExt,   /* 101111011000 */
    DpregClass::AddsubExt,   /* 101111011001 */
    DpregClass::AddsubExt,   /* 101111011010 */
    DpregClass::AddsubExt,   /* 101111011011 */
    DpregClass::AddsubExt,   /* 101111011100 */
    DpregClass::AddsubExt,   /* 101111011101 */
    DpregClass::AddsubExt,   /* 101111011110 */
    DpregClass::AddsubExt,   /* 101111011111 */
    DpregClass::AddsubExt,   /* 101111100000 */
    DpregClass::AddsubExt,   /* 101111100001 */
    DpregClass::AddsubExt,   /* 101111100010 */
    DpregClass::AddsubExt,   /* 101111100011 */
    DpregClass::AddsubExt,   /* 101111100100 */
    DpregClass::AddsubExt,   /* 101111100101 */
    DpregClass::AddsubExt,   /* 101111100110 */
    DpregClass::AddsubExt,   /* 101111100111 */
    DpregClass::AddsubExt,   /* 101111101000 */
    DpregClass::AddsubExt,   /* 101111101001 */
    DpregClass::AddsubExt,   /* 101111101010 */
    DpregClass::AddsubExt,   /* 101111101011 */
    DpregClass::AddsubExt,   /* 101111101100 */
    DpregClass::AddsubExt,   /* 101111101101 */
    DpregClass::AddsubExt,   /* 101111101110 */
    DpregClass::AddsubExt,   /* 101111101111 */
    DpregClass::AddsubExt,   /* 101111110000 */
    DpregClass::AddsubExt,   /* 101111110001 */
    DpregClass::AddsubExt,   /* 101111110010 */
    DpregClass::AddsubExt,   /* 101111110011 */
    DpregClass::AddsubExt,   /* 101111110100 */
    DpregClass::AddsubExt,   /* 101111110101 */
    DpregClass::AddsubExt,   /* 101111110110 */
    DpregClass::AddsubExt,   /* 101111110111 */
    DpregClass::AddsubExt,   /* 101111111000 */
    DpregClass::AddsubExt,   /* 101111111001 */
    DpregClass::AddsubExt,   /* 101111111010 */
    DpregClass::AddsubExt,   /* 101111111011 */
    DpregClass::AddsubExt,   /* 101111111100 */
    DpregClass::AddsubExt,   /* 101111111101 */
    DpregClass::AddsubExt,   /* 101111111110 */
    DpregClass::AddsubExt,   /* 101111111111 */
    DpregClass::AddsubCarry, /* 110000000000 */
    DpregClass::Rmif,        /* 110000000001 */
    DpregClass::Setf,        /* 110000000010 */
    DpregClass::Unallocated, /* 110000000011 */
    DpregClass::Unallocated, /* 110000000100 */
    DpregClass::Unallocated, /* 110000000101 */
    DpregClass::Unallocated, /* 110000000110 */
    DpregClass::Unallocated, /* 110000000111 */
    DpregClass::AddsubPt,    /* 110000001000 */
    DpregClass::AddsubPt,    /* 110000001001 */
    DpregClass::AddsubPt,    /* 110000001010 */
    DpregClass::AddsubPt,    /* 110000001011 */
    DpregClass::AddsubPt,    /* 110000001100 */
    DpregClass::AddsubPt,    /* 110000001101 */
    DpregClass::AddsubPt,    /* 110000001110 */
    DpregClass::AddsubPt,    /* 110000001111 */
    DpregClass::Unallocated, /* 110000010000 */
    DpregClass::Unallocated, /* 110000010001 */
    DpregClass::Setf,        /* 110000010010 */
    DpregClass::Unallocated, /* 110000010011 */
    DpregClass::Unallocated, /* 110000010100 */
    DpregClass::Unallocated, /* 110000010101 */
    DpregClass::Unallocated, /* 110000010110 */
    DpregClass::Unallocated, /* 110000010111 */
    DpregClass::Unallocated, /* 110000011000 */
    DpregClass::Unallocated, /* 110000011001 */
    DpregClass::Unallocated, /* 110000011010 */
    DpregClass::Unallocated, /* 110000011011 */
    DpregClass::Unallocated, /* 110000011100 */
    DpregClass::Unallocated, /* 110000011101 */
    DpregClass::Unallocated, /* 110000011110 */
    DpregClass::Unallocated, /* 110000011111 */
    DpregClass::Unallocated, /* 110000100000 */
    DpregClass::Rmif,        /* 110000100001 */
    DpregClass::Setf,        /* 110000100010 */
    DpregClass::Unallocated, /* 110000100011 */
    DpregClass::Unallocated, /* 110000100100 */
    DpregClass::Unallocated, /* 110000100101 */
    DpregClass::Unallocated, /* 110000100110 */
    DpregClass::Unallocated, /* 110000100111 */
    DpregClass::Unallocated, /* 110000101000 */
    DpregClass::Unallocated, /* 110000101001 */
    DpregClass::Unallocated, /* 110000101010 */
    DpregClass::Unallocated, /* 110000101011 */
    DpregClass::Unallocated, /* 110000101100 */
    DpregClass::Unallocated, /* 110000101101 */
    DpregClass::Unallocated, /* 110000101110 */
    DpregClass::Unallocated, /* 110000101111 */
    DpregClass::Unallocated, /* 110000110000 */
    DpregClass::Unallocated, /* 110000110001 */
    DpregClass::Setf,        /* 110000110010 */
    DpregClass::Unallocated, /* 110000110011 */
    DpregClass::Unallocated, /* 110000110100 */
    DpregClass::Unallocated, /* 110000110101 */
    DpregClass::Unallocated, /* 110000110110 */
    DpregClass::Unallocated, /* 110000110111 */
    DpregClass::Unallocated, /* 110000111000 */
    DpregClass::Unallocated, /* 110000111001 */
    DpregClass::Unallocated, /* 110000111010 */
    DpregClass::Unallocated, /* 110000111011 */
    DpregClass::Unallocated, /* 110000111100 */
    DpregClass::Unallocated, /* 110000111101 */
    DpregClass::Unallocated, /* 110000111110 */
    DpregClass::Unallocated, /* 110000111111 */
    DpregClass::Unallocated, /* 110001000000 */
    DpregClass::Unallocated, /* 110001000001 */
    DpregClass::Unallocated, /* 110001000010 */
    DpregClass::Unallocated, /* 110001000011 */
    DpregClass::Unallocated, /* 110001000100 */
    DpregClass::Unallocated, /* 110001000101 */
    DpregClass::Unallocated, /* 110001000110 */
    DpregClass::Unallocated, /* 110001000111 */
    DpregClass::Unallocated, /* 110001001000 */
    DpregClass::Unallocated, /* 110001001001 */
    DpregClass::Unallocated, /* 110001001010 */
    DpregClass::Unallocated, /* 110001001011 */
    DpregClass::Unallocated, /* 110001001100 */
    DpregClass::Unallocated, /* 110001001101 */
    DpregClass::Unallocated, /* 110001001110 */
    DpregClass::Unallocated, /* 110001001111 */
    DpregClass::Unallocated, /* 110001010000 */
    DpregClass::Unallocated, /* 110001010001 */
    DpregClass::Unallocated, /* 110001010010 */
    DpregClass::Unallocated, /* 110001010011 */
    DpregClass::Unallocated, /* 110001010100 */
    DpregClass::Unallocated, /* 110001010101 */
    DpregClass::Unallocated, /* 110001010110 */
    DpregClass::Unallocated, /* 110001010111 */
    DpregClass::Unallocated, /* 110001011000 */
    DpregClass::Unallocated, /* 110001011001 */
    DpregClass::Unallocated, /* 110001011010 */
    DpregClass::Unallocated, /* 110001011011 */
    DpregClass::Unallocated, /* 110001011100 */
    DpregClass::Unallocated, /* 110001011101 */
    DpregClass::Unallocated, /* 110001011110 */
    DpregClass::Unallocated, /* 110001011111 */
    DpregClass::Unallocated, /* 110001100000 */
    DpregClass::Unallocated, /* 110001100001 */
    DpregClass::Unallocated, /* 110001100010 */
    DpregClass::Unallocated, /* 110001100011 */
    DpregClass::Unallocated, /* 110001100100 */
    DpregClass::Unallocated, /* 110001100101 */
    DpregClass::Unallocated, /* 110001100110 */
    DpregClass::Unallocated, /* 110001100111 */
    DpregClass::Unallocated, /* 110001101000 */
    DpregClass::Unallocated, /* 110001101001 */
    DpregClass::Unallocated, /* 110001101010 */
    DpregClass::Unallocated, /* 110001101011 */
    DpregClass::Unallocated, /* 110001101100 */
    DpregClass::Unallocated, /* 110001101101 */
    DpregClass::Unallocated, /* 110001101110 */
    DpregClass::Unallocated, /* 110001101111 */
    DpregClass::Unallocated, /* 110001110000 */
    DpregClass::Unallocated, /* 110001110001 */
    DpregClass::Unallocated, /* 110001110010 */
    DpregClass::Unallocated, /* 110001110011 */
    DpregClass::Unallocated, /* 110001110100 */
    DpregClass::Unallocated, /* 110001110101 */
    DpregClass::Unallocated, /* 110001110110 */
    DpregClass::Unallocated, /* 110001110111 */
    DpregClass::Unallocated, /* 110001111000 */
    DpregClass::Unallocated, /* 110001111001 */
    DpregClass::Unallocated, /* 110001111010 */
    DpregClass::Unallocated, /* 110001111011 */
    DpregClass::Unallocated, /* 110001111100 */
    DpregClass::Unallocated, /* 110001111101 */
    DpregClass::Unallocated, /* 110001111110 */
    DpregClass::Unallocated, /* 110001111111 */
    DpregClass::CondcmpReg,  /* 110010000000 */
    DpregClass::CondcmpReg,  /* 110010000001 */
    DpregClass::CondcmpImm,  /* 110010000010 */
    DpregClass::CondcmpImm,  /* 110010000011 */
    DpregClass::CondcmpReg,  /* 110010000100 */
    DpregClass::CondcmpReg,  /* 110010000101 */
    DpregClass::CondcmpImm,  /* 110010000110 */
    DpregClass::CondcmpImm,  /* 110010000111 */
    DpregClass::CondcmpReg,  /* 110010001000 */
    DpregClass::CondcmpReg,  /* 110010001001 */
    DpregClass::CondcmpImm,  /* 110010001010 */
    DpregClass::CondcmpImm,  /* 110010001011 */
    DpregClass::CondcmpReg,  /* 110010001100 */
    DpregClass::CondcmpReg,  /* 110010001101 */
    DpregClass::CondcmpImm,  /* 110010001110 */
    DpregClass::CondcmpImm,  /* 110010001111 */
    DpregClass::CondcmpReg,  /* 110010010000 */
    DpregClass::CondcmpReg,  /* 110010010001 */
    DpregClass::CondcmpImm,  /* 110010010010 */
    DpregClass::CondcmpImm,  /* 110010010011 */
    DpregClass::CondcmpReg,  /* 110010010100 */
    DpregClass::CondcmpReg,  /* 110010010101 */
    DpregClass::CondcmpImm,  /* 110010010110 */
    DpregClass::CondcmpImm,  /* 110010010111 */
    DpregClass::CondcmpReg,  /* 110010011000 */
    DpregClass::CondcmpReg,  /* 110010011001 */
    DpregClass::CondcmpImm,  /* 110010011010 */
    DpregClass::CondcmpImm,  /* 110010011011 */
    DpregClass::CondcmpReg,  /* 110010011100 */
    DpregClass::CondcmpReg,  /* 110010011101 */
    DpregClass::CondcmpImm,  /* 110010011110 */
    DpregClass::CondcmpImm,  /* 110010011111 */
    DpregClass::CondcmpReg,  /* 110010100000 */
    DpregClass::CondcmpReg,  /* 110010100001 */
    DpregClass::CondcmpImm,  /* 110010100010 */
    DpregClass::CondcmpImm,  /* 110010100011 */
    DpregClass::CondcmpReg,  /* 110010100100 */
    DpregClass::CondcmpReg,  /* 110010100101 */
    DpregClass::CondcmpImm,  /* 110010100110 */
    DpregClass::CondcmpImm,  /* 110010100111 */
    DpregClass::CondcmpReg,  /* 110010101000 */
    DpregClass::CondcmpReg,  /* 110010101001 */
    DpregClass::CondcmpImm,  /* 110010101010 */
    DpregClass::CondcmpImm,  /* 110010101011 */
    DpregClass::CondcmpReg,  /* 110010101100 */
    DpregClass::CondcmpReg,  /* 110010101101 */
    DpregClass::CondcmpImm,  /* 110010101110 */
    DpregClass::CondcmpImm,  /* 110010101111 */
    DpregClass::CondcmpReg,  /* 110010110000 */
    DpregClass::CondcmpReg,  /* 110010110001 */
    DpregClass::CondcmpImm,  /* 110010110010 */
    DpregClass::CondcmpImm,  /* 110010110011 */
    DpregClass::CondcmpReg,  /* 110010110100 */
    DpregClass::CondcmpReg,  /* 110010110101 */
    DpregClass::CondcmpImm,  /* 110010110110 */
    DpregClass::CondcmpImm,  /* 110010110111 */
    DpregClass::CondcmpReg,  /* 110010111000 */
    DpregClass::CondcmpReg,  /* 110010111001 */
    DpregClass::CondcmpImm,  /* 110010111010 */
    DpregClass::CondcmpImm,  /* 110010111011 */
    DpregClass::CondcmpReg,  /* 110010111100 */
    DpregClass::CondcmpReg,  /* 110010111101 */
    DpregClass::CondcmpImm,  /* 110010111110 */
    DpregClass::CondcmpImm,  /* 110010111111 */
    DpregClass::Unallocated, /* 110011000000 */
    DpregClass::Unallocated, /* 110011000001 */
    DpregClass::Unallocated, /* 110011000010 */
    DpregClass::Unallocated, /* 110011000011 */
    DpregClass::Unallocated, /* 110011000100 */
    DpregClass::Unallocated, /* 110011000101 */
    DpregClass::Unallocated, /* 110011000110 */
    DpregClass::Unallocated, /* 110011000111 */
    DpregClass::Unallocated, /* 110011001000 */
    DpregClass::Unallocated, /* 110011001001 */
    DpregClass::Unallocated, /* 110011001010 */
    DpregClass::Unallocated, /* 110011001011 */
    DpregClass::Unallocated, /* 110011001100 */
    DpregClass::Unallocated, /* 110011001101 */
    DpregClass::Unallocated, /* 110011001110 */
    DpregClass::Unallocated, /* 110011001111 */
    DpregClass::Unallocated, /* 110011010000 */
    DpregClass::Unallocated, /* 110011010001 */
    DpregClass::Unallocated, /* 110011010010 */
    DpregClass::Unallocated, /* 110011010011 */
    DpregClass::Unallocated, /* 110011010100 */
    DpregClass::Unallocated, /* 110011010101 */
    DpregClass::Unallocated, /* 110011010110 */
    DpregClass::Unallocated, /* 110011010111 */
    DpregClass::Unallocated, /* 110011011000 */
    DpregClass::Unallocated, /* 110011011001 */
    DpregClass::Unallocated, /* 110011011010 */
    DpregClass::Unallocated, /* 110011011011 */
    DpregClass::Unallocated, /* 110011011100 */
    DpregClass::Unallocated, /* 110011011101 */
    DpregClass::Unallocated, /* 110011011110 */
    DpregClass::Unallocated, /* 110011011111 */
    DpregClass::Unallocated, /* 110011100000 */
    DpregClass::Unallocated, /* 110011100001 */
    DpregClass::Unallocated, /* 110011100010 */
    DpregClass::Unallocated, /* 110011100011 */
    DpregClass::Unallocated, /* 110011100100 */
    DpregClass::Unallocated, /* 110011100101 */
    DpregClass::Unallocated, /* 110011100110 */
    DpregClass::Unallocated, /* 110011100111 */
    DpregClass::Unallocated, /* 110011101000 */
    DpregClass::Unallocated, /* 110011101001 */
    DpregClass::Unallocated, /* 110011101010 */
    DpregClass::Unallocated, /* 110011101011 */
    DpregClass::Unallocated, /* 110011101100 */
    DpregClass::Unallocated, /* 110011101101 */
    DpregClass::Unallocated, /* 110011101110 */
    DpregClass::Unallocated, /* 110011101111 */
    DpregClass::Unallocated, /* 110011110000 */
    DpregClass::Unallocated, /* 110011110001 */
    DpregClass::Unallocated, /* 110011110010 */
    DpregClass::Unallocated, /* 110011110011 */
    DpregClass::Unallocated, /* 110011110100 */
    DpregClass::Unallocated, /* 110011110101 */
    DpregClass::Unallocated, /* 110011110110 */
    DpregClass::Unallocated, /* 110011110111 */
    DpregClass::Unallocated, /* 110011111000 */
    DpregClass::Unallocated, /* 110011111001 */
    DpregClass::Unallocated, /* 110011111010 */
    DpregClass::Unallocated, /* 110011111011 */
    DpregClass::Unallocated, /* 110011111100 */
    DpregClass::Unallocated, /* 110011111101 */
    DpregClass::Unallocated, /* 110011111110 */
    DpregClass::Unallocated, /* 110011111111 */
    DpregClass::Condsel,     /* 110100000000 */
    DpregClass::Condsel,     /* 110100000001 */
    DpregClass::Condsel,     /* 110100000010 */
    DpregClass::Condsel,     /* 110100000011 */
    DpregClass::Condsel,     /* 110100000100 */
    DpregClass::Condsel,     /* 110100000101 */
    DpregClass::Condsel,     /* 110100000110 */
    DpregClass::Condsel,     /* 110100000111 */
    DpregClass::Condsel,     /* 110100001000 */
    DpregClass::Condsel,     /* 110100001001 */
    DpregClass::Condsel,     /* 110100001010 */
    DpregClass::Condsel,     /* 110100001011 */
    DpregClass::Condsel,     /* 110100001100 */
    DpregClass::Condsel,     /* 110100001101 */
    DpregClass::Condsel,     /* 110100001110 */
    DpregClass::Condsel,     /* 110100001111 */
    DpregClass::Condsel,     /* 110100010000 */
    DpregClass::Condsel,     /* 110100010001 */
    DpregClass::Condsel,     /* 110100010010 */
    DpregClass::Condsel,     /* 110100010011 */
    DpregClass::Condsel,     /* 110100010100 */
    DpregClass::Condsel,     /* 110100010101 */
    DpregClass::Condsel,     /* 110100010110 */
    DpregClass::Condsel,     /* 110100010111 */
    DpregClass::Condsel,     /* 110100011000 */
    DpregClass::Condsel,     /* 110100011001 */
    DpregClass::Condsel,     /* 110100011010 */
    DpregClass::Condsel,     /* 110100011011 */
    DpregClass::Condsel,     /* 110100011100 */
    DpregClass::Condsel,     /* 110100011101 */
    DpregClass::Condsel,     /* 110100011110 */
    DpregClass::Condsel,     /* 110100011111 */
    DpregClass::Condsel,     /* 110100100000 */
    DpregClass::Condsel,     /* 110100100001 */
    DpregClass::Condsel,     /* 110100100010 */
    DpregClass::Condsel,     /* 110100100011 */
    DpregClass::Condsel,     /* 110100100100 */
    DpregClass::Condsel,     /* 110100100101 */
    DpregClass::Condsel,     /* 110100100110 */
    DpregClass::Condsel,     /* 110100100111 */
    DpregClass::Condsel,     /* 110100101000 */
    DpregClass::Condsel,     /* 110100101001 */
    DpregClass::Condsel,     /* 110100101010 */
    DpregClass::Condsel,     /* 110100101011 */
    DpregClass::Condsel,     /* 110100101100 */
    DpregClass::Condsel,     /* 110100101101 */
    DpregClass::Condsel,     /* 110100101110 */
    DpregClass::Condsel,     /* 110100101111 */
    DpregClass::Condsel,     /* 110100110000 */
    DpregClass::Condsel,     /* 110100110001 */
    DpregClass::Condsel,     /* 110100110010 */
    DpregClass::Condsel,     /* 110100110011 */
    DpregClass::Condsel,     /* 110100110100 */
    DpregClass::Condsel,     /* 110100110101 */
    DpregClass::Condsel,     /* 110100110110 */
    DpregClass::Condsel,     /* 110100110111 */
    DpregClass::Condsel,     /* 110100111000 */
    DpregClass::Condsel,     /* 110100111001 */
    DpregClass::Condsel,     /* 110100111010 */
    DpregClass::Condsel,     /* 110100111011 */
    DpregClass::Condsel,     /* 110100111100 */
    DpregClass::Condsel,     /* 110100111101 */
    DpregClass::Condsel,     /* 110100111110 */
    DpregClass::Condsel,     /* 110100111111 */
    DpregClass::Unallocated, /* 110101000000 */
    DpregClass::Unallocated, /* 110101000001 */
    DpregClass::Unallocated, /* 110101000010 */
    DpregClass::Unallocated, /* 110101000011 */
    DpregClass::Unallocated, /* 110101000100 */
    DpregClass::Unallocated, /* 110101000101 */
    DpregClass::Unallocated, /* 110101000110 */
    DpregClass::Unallocated, /* 110101000111 */
    DpregClass::Unallocated, /* 110101001000 */
    DpregClass::Unallocated, /* 110101001001 */
    DpregClass::Unallocated, /* 110101001010 */
    DpregClass::Unallocated, /* 110101001011 */
    DpregClass::Unallocated, /* 110101001100 */
    DpregClass::Unallocated, /* 110101001101 */
    DpregClass::Unallocated, /* 110101001110 */
    DpregClass::Unallocated, /* 110101001111 */
    DpregClass::Unallocated, /* 110101010000 */
    DpregClass::Unallocated, /* 110101010001 */
    DpregClass::Unallocated, /* 110101010010 */
    DpregClass::Unallocated, /* 110101010011 */
    DpregClass::Unallocated, /* 110101010100 */
    DpregClass::Unallocated, /* 110101010101 */
    DpregClass::Unallocated, /* 110101010110 */
    DpregClass::Unallocated, /* 110101010111 */
    DpregClass::Unallocated, /* 110101011000 */
    DpregClass::Unallocated, /* 110101011001 */
    DpregClass::Unallocated, /* 110101011010 */
    DpregClass::Unallocated, /* 110101011011 */
    DpregClass::Unallocated, /* 110101011100 */
    DpregClass::Unallocated, /* 110101011101 */
    DpregClass::Unallocated, /* 110101011110 */
    DpregClass::Unallocated, /* 110101011111 */
    DpregClass::Unallocated, /* 110101100000 */
    DpregClass::Unallocated, /* 110101100001 */
    DpregClass::Unallocated, /* 110101100010 */
    DpregClass::Unallocated, /* 110101100011 */
    DpregClass::Unallocated, /* 110101100100 */
    DpregClass::Unallocated, /* 110101100101 */
    DpregClass::Unallocated, /* 110101100110 */
    DpregClass::Unallocated, /* 110101100111 */
    DpregClass::Unallocated, /* 110101101000 */
    DpregClass::Unallocated, /* 110101101001 */
    DpregClass::Unallocated, /* 110101101010 */
    DpregClass::Unallocated, /* 110101101011 */
    DpregClass::Unallocated, /* 110101101100 */
    DpregClass::Unallocated, /* 110101101101 */
    DpregClass::Unallocated, /* 110101101110 */
    DpregClass::Unallocated, /* 110101101111 */
    DpregClass::Unallocated, /* 110101110000 */
    DpregClass::Unallocated, /* 110101110001 */
    DpregClass::Unallocated, /* 110101110010 */
    DpregClass::Unallocated, /* 110101110011 */
    DpregClass::Unallocated, /* 110101110100 */
    DpregClass::Unallocated, /* 110101110101 */
    DpregClass::Unallocated, /* 110101110110 */
    DpregClass::Unallocated, /* 110101110111 */
    DpregClass::Unallocated, /* 110101111000 */
    DpregClass::Unallocated, /* 110101111001 */
    DpregClass::Unallocated, /* 110101111010 */
    DpregClass::Unallocated, /* 110101111011 */
    DpregClass::Unallocated, /* 110101111100 */
    DpregClass::Unallocated, /* 110101111101 */
    DpregClass::Unallocated, /* 110101111110 */
    DpregClass::Unallocated, /* 110101111111 */
    DpregClass::Dp1src,      /* 110110000000 */
    DpregClass::Dp1src,      /* 110110000001 */
    DpregClass::Dp1src,      /* 110110000010 */
    DpregClass::Dp1src,      /* 110110000011 */
    DpregClass::Dp1src,      /* 110110000100 */
    DpregClass::Dp1src,      /* 110110000101 */
    DpregClass::Dp1src,      /* 110110000110 */
    DpregClass::Dp1src,      /* 110110000111 */
    DpregClass::Dp1src,      /* 110110001000 */
    DpregClass::Dp1src,      /* 110110001001 */
    DpregClass::Dp1src,      /* 110110001010 */
    DpregClass::Dp1src,      /* 110110001011 */
    DpregClass::Dp1src,      /* 110110001100 */
    DpregClass::Dp1src,      /* 110110001101 */
    DpregClass::Dp1src,      /* 110110001110 */
    DpregClass::Dp1src,      /* 110110001111 */
    DpregClass::Dp1src,      /* 110110010000 */
    DpregClass::Dp1src,      /* 110110010001 */
    DpregClass::Dp1src,      /* 110110010010 */
    DpregClass::Dp1src,      /* 110110010011 */
    DpregClass::Dp1src,      /* 110110010100 */
    DpregClass::Dp1src,      /* 110110010101 */
    DpregClass::Dp1src,      /* 110110010110 */
    DpregClass::Dp1src,      /* 110110010111 */
    DpregClass::Dp1src,      /* 110110011000 */
    DpregClass::Dp1src,      /* 110110011001 */
    DpregClass::Dp1src,      /* 110110011010 */
    DpregClass::Dp1src,      /* 110110011011 */
    DpregClass::Dp1src,      /* 110110011100 */
    DpregClass::Dp1src,      /* 110110011101 */
    DpregClass::Dp1src,      /* 110110011110 */
    DpregClass::Dp1src,      /* 110110011111 */
    DpregClass::Dp1src,      /* 110110100000 */
    DpregClass::Dp1src,      /* 110110100001 */
    DpregClass::Dp1src,      /* 110110100010 */
    DpregClass::Dp1src,      /* 110110100011 */
    DpregClass::Dp1src,      /* 110110100100 */
    DpregClass::Dp1src,      /* 110110100101 */
    DpregClass::Dp1src,      /* 110110100110 */
    DpregClass::Dp1src,      /* 110110100111 */
    DpregClass::Dp1src,      /* 110110101000 */
    DpregClass::Dp1src,      /* 110110101001 */
    DpregClass::Dp1src,      /* 110110101010 */
    DpregClass::Dp1src,      /* 110110101011 */
    DpregClass::Dp1src,      /* 110110101100 */
    DpregClass::Dp1src,      /* 110110101101 */
    DpregClass::Dp1src,      /* 110110101110 */
    DpregClass::Dp1src,      /* 110110101111 */
    DpregClass::Dp1src,      /* 110110110000 */
    DpregClass::Dp1src,      /* 110110110001 */
    DpregClass::Dp1src,      /* 110110110010 */
    DpregClass::Dp1src,      /* 110110110011 */
    DpregClass::Dp1src,      /* 110110110100 */
    DpregClass::Dp1src,      /* 110110110101 */
    DpregClass::Dp1src,      /* 110110110110 */
    DpregClass::Dp1src,      /* 110110110111 */
    DpregClass::Dp1src,      /* 110110111000 */
    DpregClass::Dp1src,      /* 110110111001 */
    DpregClass::Dp1src,      /* 110110111010 */
    DpregClass::Dp1src,      /* 110110111011 */
    DpregClass::Dp1src,      /* 110110111100 */
    DpregClass::Dp1src,      /* 110110111101 */
    DpregClass::Dp1src,      /* 110110111110 */
    DpregClass::Dp1src,      /* 110110111111 */
    DpregClass::Unallocated, /* 110111000000 */
    DpregClass::Unallocated, /* 110111000001 */
    DpregClass::Unallocated, /* 110111000010 */
    DpregClass::Unallocated, /* 110111000011 */
    DpregClass::Unallocated, /* 110111000100 */
    DpregClass::Unallocated, /* 110111000101 */
    DpregClass::Unallocated, /* 110111000110 */
    DpregClass::Unallocated, /* 110111000111 */
    DpregClass::Unallocated, /* 110111001000 */
    DpregClass::Unallocated, /* 110111001001 */
    DpregClass::Unallocated, /* 110111001010 */
    DpregClass::Unallocated, /* 110111001011 */
    DpregClass::Unallocated, /* 110111001100 */
    DpregClass::Unallocated, /* 110111001101 */
    DpregClass::Unallocated, /* 110111001110 */
    DpregClass::Unallocated, /* 110111001111 */
    DpregClass::Unallocated, /* 110111010000 */
    DpregClass::Unallocated, /* 110111010001 */
    DpregClass::Unallocated, /* 110111010010 */
    DpregClass::Unallocated, /* 110111010011 */
    DpregClass::Unallocated, /* 110111010100 */
    DpregClass::Unallocated, /* 110111010101 */
    DpregClass::Unallocated, /* 110111010110 */
    DpregClass::Unallocated, /* 110111010111 */
    DpregClass::Unallocated, /* 110111011000 */
    DpregClass::Unallocated, /* 110111011001 */
    DpregClass::Unallocated, /* 110111011010 */
    DpregClass::Unallocated, /* 110111011011 */
    DpregClass::Unallocated, /* 110111011100 */
    DpregClass::Unallocated, /* 110111011101 */
    DpregClass::Unallocated, /* 110111011110 */
    DpregClass::Unallocated, /* 110111011111 */
    DpregClass::Unallocated, /* 110111100000 */
    DpregClass::Unallocated, /* 110111100001 */
    DpregClass::Unallocated, /* 110111100010 */
    DpregClass::Unallocated, /* 110111100011 */
    DpregClass::Unallocated, /* 110111100100 */
    DpregClass::Unallocated, /* 110111100101 */
    DpregClass::Unallocated, /* 110111100110 */
    DpregClass::Unallocated, /* 110111100111 */
    DpregClass::Unallocated, /* 110111101000 */
    DpregClass::Unallocated, /* 110111101001 */
    DpregClass::Unallocated, /* 110111101010 */
    DpregClass::Unallocated, /* 110111101011 */
    DpregClass::Unallocated, /* 110111101100 */
    DpregClass::Unallocated, /* 110111101101 */
    DpregClass::Unallocated, /* 110111101110 */
    DpregClass::Unallocated, /* 110111101111 */
    DpregClass::Unallocated, /* 110111110000 */
    DpregClass::Unallocated, /* 110111110001 */
    DpregClass::Unallocated, /* 110111110010 */
    DpregClass::Unallocated, /* 110111110011 */
    DpregClass::Unallocated, /* 110111110100 */
    DpregClass::Unallocated, /* 110111110101 */
    DpregClass::Unallocated, /* 110111110110 */
    DpregClass::Unallocated, /* 110111110111 */
    DpregClass::Unallocated, /* 110111111000 */
    DpregClass::Unallocated, /* 110111111001 */
    DpregClass::Unallocated, /* 110111111010 */
    DpregClass::Unallocated, /* 110111111011 */
    DpregClass::Unallocated, /* 110111111100 */
    DpregClass::Unallocated, /* 110111111101 */
    DpregClass::Unallocated, /* 110111111110 */
    DpregClass::Unallocated, /* 110111111111 */
    DpregClass::Dp3src,      /* 111000000000 */
    DpregClass::Dp3src,      /* 111000000001 */
    DpregClass::Dp3src,      /* 111000000010 */
    DpregClass::Dp3src,      /* 111000000011 */
    DpregClass::Dp3src,      /* 111000000100 */
    DpregClass::Dp3src,      /* 111000000101 */
    DpregClass::Dp3src,      /* 111000000110 */
    DpregClass::Dp3src,      /* 111000000111 */
    DpregClass::Dp3src,      /* 111000001000 */
    DpregClass::Dp3src,      /* 111000001001 */
    DpregClass::Dp3src,      /* 111000001010 */
    DpregClass::Dp3src,      /* 111000001011 */
    DpregClass::Dp3src,      /* 111000001100 */
    DpregClass::Dp3src,      /* 111000001101 */
    DpregClass::Dp3src,      /* 111000001110 */
    DpregClass::Dp3src,      /* 111000001111 */
    DpregClass::Dp3src,      /* 111000010000 */
    DpregClass::Dp3src,      /* 111000010001 */
    DpregClass::Dp3src,      /* 111000010010 */
    DpregClass::Dp3src,      /* 111000010011 */
    DpregClass::Dp3src,      /* 111000010100 */
    DpregClass::Dp3src,      /* 111000010101 */
    DpregClass::Dp3src,      /* 111000010110 */
    DpregClass::Dp3src,      /* 111000010111 */
    DpregClass::Dp3src,      /* 111000011000 */
    DpregClass::Dp3src,      /* 111000011001 */
    DpregClass::Dp3src,      /* 111000011010 */
    DpregClass::Dp3src,      /* 111000011011 */
    DpregClass::Dp3src,      /* 111000011100 */
    DpregClass::Dp3src,      /* 111000011101 */
    DpregClass::Dp3src,      /* 111000011110 */
    DpregClass::Dp3src,      /* 111000011111 */
    DpregClass::Dp3src,      /* 111000100000 */
    DpregClass::Dp3src,      /* 111000100001 */
    DpregClass::Dp3src,      /* 111000100010 */
    DpregClass::Dp3src,      /* 111000100011 */
    DpregClass::Dp3src,      /* 111000100100 */
    DpregClass::Dp3src,      /* 111000100101 */
    DpregClass::Dp3src,      /* 111000100110 */
    DpregClass::Dp3src,      /* 111000100111 */
    DpregClass::Dp3src,      /* 111000101000 */
    DpregClass::Dp3src,      /* 111000101001 */
    DpregClass::Dp3src,      /* 111000101010 */
    DpregClass::Dp3src,      /* 111000101011 */
    DpregClass::Dp3src,      /* 111000101100 */
    DpregClass::Dp3src,      /* 111000101101 */
    DpregClass::Dp3src,      /* 111000101110 */
    DpregClass::Dp3src,      /* 111000101111 */
    DpregClass::Dp3src,      /* 111000110000 */
    DpregClass::Dp3src,      /* 111000110001 */
    DpregClass::Dp3src,      /* 111000110010 */
    DpregClass::Dp3src,      /* 111000110011 */
    DpregClass::Dp3src,      /* 111000110100 */
    DpregClass::Dp3src,      /* 111000110101 */
    DpregClass::Dp3src,      /* 111000110110 */
    DpregClass::Dp3src,      /* 111000110111 */
    DpregClass::Dp3src,      /* 111000111000 */
    DpregClass::Dp3src,      /* 111000111001 */
    DpregClass::Dp3src,      /* 111000111010 */
    DpregClass::Dp3src,      /* 111000111011 */
    DpregClass::Dp3src,      /* 111000111100 */
    DpregClass::Dp3src,      /* 111000111101 */
    DpregClass::Dp3src,      /* 111000111110 */
    DpregClass::Dp3src,      /* 111000111111 */
    DpregClass::Dp3src,      /* 111001000000 */
    DpregClass::Dp3src,      /* 111001000001 */
    DpregClass::Dp3src,      /* 111001000010 */
    DpregClass::Dp3src,      /* 111001000011 */
    DpregClass::Dp3src,      /* 111001000100 */
    DpregClass::Dp3src,      /* 111001000101 */
    DpregClass::Dp3src,      /* 111001000110 */
    DpregClass::Dp3src,      /* 111001000111 */
    DpregClass::Dp3src,      /* 111001001000 */
    DpregClass::Dp3src,      /* 111001001001 */
    DpregClass::Dp3src,      /* 111001001010 */
    DpregClass::Dp3src,      /* 111001001011 */
    DpregClass::Dp3src,      /* 111001001100 */
    DpregClass::Dp3src,      /* 111001001101 */
    DpregClass::Dp3src,      /* 111001001110 */
    DpregClass::Dp3src,      /* 111001001111 */
    DpregClass::Dp3src,      /* 111001010000 */
    DpregClass::Dp3src,      /* 111001010001 */
    DpregClass::Dp3src,      /* 111001010010 */
    DpregClass::Dp3src,      /* 111001010011 */
    DpregClass::Dp3src,      /* 111001010100 */
    DpregClass::Dp3src,      /* 111001010101 */
    DpregClass::Dp3src,      /* 111001010110 */
    DpregClass::Dp3src,      /* 111001010111 */
    DpregClass::Dp3src,      /* 111001011000 */
    DpregClass::Dp3src,      /* 111001011001 */
    DpregClass::Dp3src,      /* 111001011010 */
    DpregClass::Dp3src,      /* 111001011011 */
    DpregClass::Dp3src,      /* 111001011100 */
    DpregClass::Dp3src,      /* 111001011101 */
    DpregClass::Dp3src,      /* 111001011110 */
    DpregClass::Dp3src,      /* 111001011111 */
    DpregClass::Dp3src,      /* 111001100000 */
    DpregClass::Dp3src,      /* 111001100001 */
    DpregClass::Dp3src,      /* 111001100010 */
    DpregClass::Dp3src,      /* 111001100011 */
    DpregClass::Dp3src,      /* 111001100100 */
    DpregClass::Dp3src,      /* 111001100101 */
    DpregClass::Dp3src,      /* 111001100110 */
    DpregClass::Dp3src,      /* 111001100111 */
    DpregClass::Dp3src,      /* 111001101000 */
    DpregClass::Dp3src,      /* 111001101001 */
    DpregClass::Dp3src,      /* 111001101010 */
    DpregClass::Dp3src,      /* 111001101011 */
    DpregClass::Dp3src,      /* 111001101100 */
    DpregClass::Dp3src,      /* 111001101101 */
    DpregClass::Dp3src,      /* 111001101110 */
    DpregClass::Dp3src,      /* 111001101111 */
    DpregClass::Dp3src,      /* 111001110000 */
    DpregClass::Dp3src,      /* 111001110001 */
    DpregClass::Dp3src,      /* 111001110010 */
    DpregClass::Dp3src,      /* 111001110011 */
    DpregClass::Dp3src,      /* 111001110100 */
    DpregClass::Dp3src,      /* 111001110101 */
    DpregClass::Dp3src,      /* 111001110110 */
    DpregClass::Dp3src,      /* 111001110111 */
    DpregClass::Dp3src,      /* 111001111000 */
    DpregClass::Dp3src,      /* 111001111001 */
    DpregClass::Dp3src,      /* 111001111010 */
    DpregClass::Dp3src,      /* 111001111011 */
    DpregClass::Dp3src,      /* 111001111100 */
    DpregClass::Dp3src,      /* 111001111101 */
    DpregClass::Dp3src,      /* 111001111110 */
    DpregClass::Dp3src,      /* 111001111111 */
    DpregClass::Dp3src,      /* 111010000000 */
    DpregClass::Dp3src,      /* 111010000001 */
    DpregClass::Dp3src,      /* 111010000010 */
    DpregClass::Dp3src,      /* 111010000011 */
    DpregClass::Dp3src,      /* 111010000100 */
    DpregClass::Dp3src,      /* 111010000101 */
    DpregClass::Dp3src,      /* 111010000110 */
    DpregClass::Dp3src,      /* 111010000111 */
    DpregClass::Dp3src,      /* 111010001000 */
    DpregClass::Dp3src,      /* 111010001001 */
    DpregClass::Dp3src,      /* 111010001010 */
    DpregClass::Dp3src,      /* 111010001011 */
    DpregClass::Dp3src,      /* 111010001100 */
    DpregClass::Dp3src,      /* 111010001101 */
    DpregClass::Dp3src,      /* 111010001110 */
    DpregClass::Dp3src,      /* 111010001111 */
    DpregClass::Dp3src,      /* 111010010000 */
    DpregClass::Dp3src,      /* 111010010001 */
    DpregClass::Dp3src,      /* 111010010010 */
    DpregClass::Dp3src,      /* 111010010011 */
    DpregClass::Dp3src,      /* 111010010100 */
    DpregClass::Dp3src,      /* 111010010101 */
    DpregClass::Dp3src,      /* 111010010110 */
    DpregClass::Dp3src,      /* 111010010111 */
    DpregClass::Dp3src,      /* 111010011000 */
    DpregClass::Dp3src,      /* 111010011001 */
    DpregClass::Dp3src,      /* 111010011010 */
    DpregClass::Dp3src,      /* 111010011011 */
    DpregClass::Dp3src,      /* 111010011100 */
    DpregClass::Dp3src,      /* 111010011101 */
    DpregClass::Dp3src,      /* 111010011110 */
    DpregClass::Dp3src,      /* 111010011111 */
    DpregClass::Dp3src,      /* 111010100000 */
    DpregClass::Dp3src,      /* 111010100001 */
    DpregClass::Dp3src,      /* 111010100010 */
    DpregClass::Dp3src,      /* 111010100011 */
    DpregClass::Dp3src,      /* 111010100100 */
    DpregClass::Dp3src,      /* 111010100101 */
    DpregClass::Dp3src,      /* 111010100110 */
    DpregClass::Dp3src,      /* 111010100111 */
    DpregClass::Dp3src,      /* 111010101000 */
    DpregClass::Dp3src,      /* 111010101001 */
    DpregClass::Dp3src,      /* 111010101010 */
    DpregClass::Dp3src,      /* 111010101011 */
    DpregClass::Dp3src,      /* 111010101100 */
    DpregClass::Dp3src,      /* 111010101101 */
    DpregClass::Dp3src,      /* 111010101110 */
    DpregClass::Dp3src,      /* 111010101111 */
    DpregClass::Dp3src,      /* 111010110000 */
    DpregClass::Dp3src,      /* 111010110001 */
    DpregClass::Dp3src,      /* 111010110010 */
    DpregClass::Dp3src,      /* 111010110011 */
    DpregClass::Dp3src,      /* 111010110100 */
    DpregClass::Dp3src,      /* 111010110101 */
    DpregClass::Dp3src,      /* 111010110110 */
    DpregClass::Dp3src,      /* 111010110111 */
    DpregClass::Dp3src,      /* 111010111000 */
    DpregClass::Dp3src,      /* 111010111001 */
    DpregClass::Dp3src,      /* 111010111010 */
    DpregClass::Dp3src,      /* 111010111011 */
    DpregClass::Dp3src,      /* 111010111100 */
    DpregClass::Dp3src,      /* 111010111101 */
    DpregClass::Dp3src,      /* 111010111110 */
    DpregClass::Dp3src,      /* 111010111111 */
    DpregClass::Dp3src,      /* 111011000000 */
    DpregClass::Dp3src,      /* 111011000001 */
    DpregClass::Dp3src,      /* 111011000010 */
    DpregClass::Dp3src,      /* 111011000011 */
    DpregClass::Dp3src,      /* 111011000100 */
    DpregClass::Dp3src,      /* 111011000101 */
    DpregClass::Dp3src,      /* 111011000110 */
    DpregClass::Dp3src,      /* 111011000111 */
    DpregClass::Dp3src,      /* 111011001000 */
    DpregClass::Dp3src,      /* 111011001001 */
    DpregClass::Dp3src,      /* 111011001010 */
    DpregClass::Dp3src,      /* 111011001011 */
    DpregClass::Dp3src,      /* 111011001100 */
    DpregClass::Dp3src,      /* 111011001101 */
    DpregClass::Dp3src,      /* 111011001110 */
    DpregClass::Dp3src,      /* 111011001111 */
    DpregClass::Dp3src,      /* 111011010000 */
    DpregClass::Dp3src,      /* 111011010001 */
    DpregClass::Dp3src,      /* 111011010010 */
    DpregClass::Dp3src,      /* 111011010011 */
    DpregClass::Dp3src,      /* 111011010100 */
    DpregClass::Dp3src,      /* 111011010101 */
    DpregClass::Dp3src,      /* 111011010110 */
    DpregClass::Dp3src,      /* 111011010111 */
    DpregClass::Dp3src,      /* 111011011000 */
    DpregClass::Dp3src,      /* 111011011001 */
    DpregClass::Dp3src,      /* 111011011010 */
    DpregClass::Dp3src,      /* 111011011011 */
    DpregClass::Dp3src,      /* 111011011100 */
    DpregClass::Dp3src,      /* 111011011101 */
    DpregClass::Dp3src,      /* 111011011110 */
    DpregClass::Dp3src,      /* 111011011111 */
    DpregClass::Dp3src,      /* 111011100000 */
    DpregClass::Dp3src,      /* 111011100001 */
    DpregClass::Dp3src,      /* 111011100010 */
    DpregClass::Dp3src,      /* 111011100011 */
    DpregClass::Dp3src,      /* 111011100100 */
    DpregClass::Dp3src,      /* 111011100101 */
    DpregClass::Dp3src,      /* 111011100110 */
    DpregClass::Dp3src,      /* 111011100111 */
    DpregClass::Dp3src,      /* 111011101000 */
    DpregClass::Dp3src,      /* 111011101001 */
    DpregClass::Dp3src,      /* 111011101010 */
    DpregClass::Dp3src,      /* 111011101011 */
    DpregClass::Dp3src,      /* 111011101100 */
    DpregClass::Dp3src,      /* 111011101101 */
    DpregClass::Dp3src,      /* 111011101110 */
    DpregClass::Dp3src,      /* 111011101111 */
    DpregClass::Dp3src,      /* 111011110000 */
    DpregClass::Dp3src,      /* 111011110001 */
    DpregClass::Dp3src,      /* 111011110010 */
    DpregClass::Dp3src,      /* 111011110011 */
    DpregClass::Dp3src,      /* 111011110100 */
    DpregClass::Dp3src,      /* 111011110101 */
    DpregClass::Dp3src,      /* 111011110110 */
    DpregClass::Dp3src,      /* 111011110111 */
    DpregClass::Dp3src,      /* 111011111000 */
    DpregClass::Dp3src,      /* 111011111001 */
    DpregClass::Dp3src,      /* 111011111010 */
    DpregClass::Dp3src,      /* 111011111011 */
    DpregClass::Dp3src,      /* 111011111100 */
    DpregClass::Dp3src,      /* 111011111101 */
    DpregClass::Dp3src,      /* 111011111110 */
    DpregClass::Dp3src,      /* 111011111111 */
    DpregClass::Dp3src,      /* 111100000000 */
    DpregClass::Dp3src,      /* 111100000001 */
    DpregClass::Dp3src,      /* 111100000010 */
    DpregClass::Dp3src,      /* 111100000011 */
    DpregClass::Dp3src,      /* 111100000100 */
    DpregClass::Dp3src,      /* 111100000101 */
    DpregClass::Dp3src,      /* 111100000110 */
    DpregClass::Dp3src,      /* 111100000111 */
    DpregClass::Dp3src,      /* 111100001000 */
    DpregClass::Dp3src,      /* 111100001001 */
    DpregClass::Dp3src,      /* 111100001010 */
    DpregClass::Dp3src,      /* 111100001011 */
    DpregClass::Dp3src,      /* 111100001100 */
    DpregClass::Dp3src,      /* 111100001101 */
    DpregClass::Dp3src,      /* 111100001110 */
    DpregClass::Dp3src,      /* 111100001111 */
    DpregClass::Dp3src,      /* 111100010000 */
    DpregClass::Dp3src,      /* 111100010001 */
    DpregClass::Dp3src,      /* 111100010010 */
    DpregClass::Dp3src,      /* 111100010011 */
    DpregClass::Dp3src,      /* 111100010100 */
    DpregClass::Dp3src,      /* 111100010101 */
    DpregClass::Dp3src,      /* 111100010110 */
    DpregClass::Dp3src,      /* 111100010111 */
    DpregClass::Dp3src,      /* 111100011000 */
    DpregClass::Dp3src,      /* 111100011001 */
    DpregClass::Dp3src,      /* 111100011010 */
    DpregClass::Dp3src,      /* 111100011011 */
    DpregClass::Dp3src,      /* 111100011100 */
    DpregClass::Dp3src,      /* 111100011101 */
    DpregClass::Dp3src,      /* 111100011110 */
    DpregClass::Dp3src,      /* 111100011111 */
    DpregClass::Dp3src,      /* 111100100000 */
    DpregClass::Dp3src,      /* 111100100001 */
    DpregClass::Dp3src,      /* 111100100010 */
    DpregClass::Dp3src,      /* 111100100011 */
    DpregClass::Dp3src,      /* 111100100100 */
    DpregClass::Dp3src,      /* 111100100101 */
    DpregClass::Dp3src,      /* 111100100110 */
    DpregClass::Dp3src,      /* 111100100111 */
    DpregClass::Dp3src,      /* 111100101000 */
    DpregClass::Dp3src,      /* 111100101001 */
    DpregClass::Dp3src,      /* 111100101010 */
    DpregClass::Dp3src,      /* 111100101011 */
    DpregClass::Dp3src,      /* 111100101100 */
    DpregClass::Dp3src,      /* 111100101101 */
    DpregClass::Dp3src,      /* 111100101110 */
    DpregClass::Dp3src,      /* 111100101111 */
    DpregClass::Dp3src,      /* 111100110000 */
    DpregClass::Dp3src,      /* 111100110001 */
    DpregClass::Dp3src,      /* 111100110010 */
    DpregClass::Dp3src,      /* 111100110011 */
    DpregClass::Dp3src,      /* 111100110100 */
    DpregClass::Dp3src,      /* 111100110101 */
    DpregClass::Dp3src,      /* 111100110110 */
    DpregClass::Dp3src,      /* 111100110111 */
    DpregClass::Dp3src,      /* 111100111000 */
    DpregClass::Dp3src,      /* 111100111001 */
    DpregClass::Dp3src,      /* 111100111010 */
    DpregClass::Dp3src,      /* 111100111011 */
    DpregClass::Dp3src,      /* 111100111100 */
    DpregClass::Dp3src,      /* 111100111101 */
    DpregClass::Dp3src,      /* 111100111110 */
    DpregClass::Dp3src,      /* 111100111111 */
    DpregClass::Dp3src,      /* 111101000000 */
    DpregClass::Dp3src,      /* 111101000001 */
    DpregClass::Dp3src,      /* 111101000010 */
    DpregClass::Dp3src,      /* 111101000011 */
    DpregClass::Dp3src,      /* 111101000100 */
    DpregClass::Dp3src,      /* 111101000101 */
    DpregClass::Dp3src,      /* 111101000110 */
    DpregClass::Dp3src,      /* 111101000111 */
    DpregClass::Dp3src,      /* 111101001000 */
    DpregClass::Dp3src,      /* 111101001001 */
    DpregClass::Dp3src,      /* 111101001010 */
    DpregClass::Dp3src,      /* 111101001011 */
    DpregClass::Dp3src,      /* 111101001100 */
    DpregClass::Dp3src,      /* 111101001101 */
    DpregClass::Dp3src,      /* 111101001110 */
    DpregClass::Dp3src,      /* 111101001111 */
    DpregClass::Dp3src,      /* 111101010000 */
    DpregClass::Dp3src,      /* 111101010001 */
    DpregClass::Dp3src,      /* 111101010010 */
    DpregClass::Dp3src,      /* 111101010011 */
    DpregClass::Dp3src,      /* 111101010100 */
    DpregClass::Dp3src,      /* 111101010101 */
    DpregClass::Dp3src,      /* 111101010110 */
    DpregClass::Dp3src,      /* 111101010111 */
    DpregClass::Dp3src,      /* 111101011000 */
    DpregClass::Dp3src,      /* 111101011001 */
    DpregClass::Dp3src,      /* 111101011010 */
    DpregClass::Dp3src,      /* 111101011011 */
    DpregClass::Dp3src,      /* 111101011100 */
    DpregClass::Dp3src,      /* 111101011101 */
    DpregClass::Dp3src,      /* 111101011110 */
    DpregClass::Dp3src,      /* 111101011111 */
    DpregClass::Dp3src,      /* 111101100000 */
    DpregClass::Dp3src,      /* 111101100001 */
    DpregClass::Dp3src,      /* 111101100010 */
    DpregClass::Dp3src,      /* 111101100011 */
    DpregClass::Dp3src,      /* 111101100100 */
    DpregClass::Dp3src,      /* 111101100101 */
    DpregClass::Dp3src,      /* 111101100110 */
    DpregClass::Dp3src,      /* 111101100111 */
    DpregClass::Dp3src,      /* 111101101000 */
    DpregClass::Dp3src,      /* 111101101001 */
    DpregClass::Dp3src,      /* 111101101010 */
    DpregClass::Dp3src,      /* 111101101011 */
    DpregClass::Dp3src,      /* 111101101100 */
    DpregClass::Dp3src,      /* 111101101101 */
    DpregClass::Dp3src,      /* 111101101110 */
    DpregClass::Dp3src,      /* 111101101111 */
    DpregClass::Dp3src,      /* 111101110000 */
    DpregClass::Dp3src,      /* 111101110001 */
    DpregClass::Dp3src,      /* 111101110010 */
    DpregClass::Dp3src,      /* 111101110011 */
    DpregClass::Dp3src,      /* 111101110100 */
    DpregClass::Dp3src,      /* 111101110101 */
    DpregClass::Dp3src,      /* 111101110110 */
    DpregClass::Dp3src,      /* 111101110111 */
    DpregClass::Dp3src,      /* 111101111000 */
    DpregClass::Dp3src,      /* 111101111001 */
    DpregClass::Dp3src,      /* 111101111010 */
    DpregClass::Dp3src,      /* 111101111011 */
    DpregClass::Dp3src,      /* 111101111100 */
    DpregClass::Dp3src,      /* 111101111101 */
    DpregClass::Dp3src,      /* 111101111110 */
    DpregClass::Dp3src,      /* 111101111111 */
    DpregClass::Dp3src,      /* 111110000000 */
    DpregClass::Dp3src,      /* 111110000001 */
    DpregClass::Dp3src,      /* 111110000010 */
    DpregClass::Dp3src,      /* 111110000011 */
    DpregClass::Dp3src,      /* 111110000100 */
    DpregClass::Dp3src,      /* 111110000101 */
    DpregClass::Dp3src,      /* 111110000110 */
    DpregClass::Dp3src,      /* 111110000111 */
    DpregClass::Dp3src,      /* 111110001000 */
    DpregClass::Dp3src,      /* 111110001001 */
    DpregClass::Dp3src,      /* 111110001010 */
    DpregClass::Dp3src,      /* 111110001011 */
    DpregClass::Dp3src,      /* 111110001100 */
    DpregClass::Dp3src,      /* 111110001101 */
    DpregClass::Dp3src,      /* 111110001110 */
    DpregClass::Dp3src,      /* 111110001111 */
    DpregClass::Dp3src,      /* 111110010000 */
    DpregClass::Dp3src,      /* 111110010001 */
    DpregClass::Dp3src,      /* 111110010010 */
    DpregClass::Dp3src,      /* 111110010011 */
    DpregClass::Dp3src,      /* 111110010100 */
    DpregClass::Dp3src,      /* 111110010101 */
    DpregClass::Dp3src,      /* 111110010110 */
    DpregClass::Dp3src,      /* 111110010111 */
    DpregClass::Dp3src,      /* 111110011000 */
    DpregClass::Dp3src,      /* 111110011001 */
    DpregClass::Dp3src,      /* 111110011010 */
    DpregClass::Dp3src,      /* 111110011011 */
    DpregClass::Dp3src,      /* 111110011100 */
    DpregClass::Dp3src,      /* 111110011101 */
    DpregClass::Dp3src,      /* 111110011110 */
    DpregClass::Dp3src,      /* 111110011111 */
    DpregClass::Dp3src,      /* 111110100000 */
    DpregClass::Dp3src,      /* 111110100001 */
    DpregClass::Dp3src,      /* 111110100010 */
    DpregClass::Dp3src,      /* 111110100011 */
    DpregClass::Dp3src,      /* 111110100100 */
    DpregClass::Dp3src,      /* 111110100101 */
    DpregClass::Dp3src,      /* 111110100110 */
    DpregClass::Dp3src,      /* 111110100111 */
    DpregClass::Dp3src,      /* 111110101000 */
    DpregClass::Dp3src,      /* 111110101001 */
    DpregClass::Dp3src,      /* 111110101010 */
    DpregClass::Dp3src,      /* 111110101011 */
    DpregClass::Dp3src,      /* 111110101100 */
    DpregClass::Dp3src,      /* 111110101101 */
    DpregClass::Dp3src,      /* 111110101110 */
    DpregClass::Dp3src,      /* 111110101111 */
    DpregClass::Dp3src,      /* 111110110000 */
    DpregClass::Dp3src,      /* 111110110001 */
    DpregClass::Dp3src,      /* 111110110010 */
    DpregClass::Dp3src,      /* 111110110011 */
    DpregClass::Dp3src,      /* 111110110100 */
    DpregClass::Dp3src,      /* 111110110101 */
    DpregClass::Dp3src,      /* 111110110110 */
    DpregClass::Dp3src,      /* 111110110111 */
    DpregClass::Dp3src,      /* 111110111000 */
    DpregClass::Dp3src,      /* 111110111001 */
    DpregClass::Dp3src,      /* 111110111010 */
    DpregClass::Dp3src,      /* 111110111011 */
    DpregClass::Dp3src,      /* 111110111100 */
    DpregClass::Dp3src,      /* 111110111101 */
    DpregClass::Dp3src,      /* 111110111110 */
    DpregClass::Dp3src,      /* 111110111111 */
    DpregClass::Dp3src,      /* 111111000000 */
    DpregClass::Dp3src,      /* 111111000001 */
    DpregClass::Dp3src,      /* 111111000010 */
    DpregClass::Dp3src,      /* 111111000011 */
    DpregClass::Dp3src,      /* 111111000100 */
    DpregClass::Dp3src,      /* 111111000101 */
    DpregClass::Dp3src,      /* 111111000110 */
    DpregClass::Dp3src,      /* 111111000111 */
    DpregClass::Dp3src,      /* 111111001000 */
    DpregClass::Dp3src,      /* 111111001001 */
    DpregClass::Dp3src,      /* 111111001010 */
    DpregClass::Dp3src,      /* 111111001011 */
    DpregClass::Dp3src,      /* 111111001100 */
    DpregClass::Dp3src,      /* 111111001101 */
    DpregClass::Dp3src,      /* 111111001110 */
    DpregClass::Dp3src,      /* 111111001111 */
    DpregClass::Dp3src,      /* 111111010000 */
    DpregClass::Dp3src,      /* 111111010001 */
    DpregClass::Dp3src,      /* 111111010010 */
    DpregClass::Dp3src,      /* 111111010011 */
    DpregClass::Dp3src,      /* 111111010100 */
    DpregClass::Dp3src,      /* 111111010101 */
    DpregClass::Dp3src,      /* 111111010110 */
    DpregClass::Dp3src,      /* 111111010111 */
    DpregClass::Dp3src,      /* 111111011000 */
    DpregClass::Dp3src,      /* 111111011001 */
    DpregClass::Dp3src,      /* 111111011010 */
    DpregClass::Dp3src,      /* 111111011011 */
    DpregClass::Dp3src,      /* 111111011100 */
    DpregClass::Dp3src,      /* 111111011101 */
    DpregClass::Dp3src,      /* 111111011110 */
    DpregClass::Dp3src,      /* 111111011111 */
    DpregClass::Dp3src,      /* 111111100000 */
    DpregClass::Dp3src,      /* 111111100001 */
    DpregClass::Dp3src,      /* 111111100010 */
    DpregClass::Dp3src,      /* 111111100011 */
    DpregClass::Dp3src,      /* 111111100100 */
    DpregClass::Dp3src,      /* 111111100101 */
    DpregClass::Dp3src,      /* 111111100110 */
    DpregClass::Dp3src,      /* 111111100111 */
    DpregClass::Dp3src,      /* 111111101000 */
    DpregClass::Dp3src,      /* 111111101001 */
    DpregClass::Dp3src,      /* 111111101010 */
    DpregClass::Dp3src,      /* 111111101011 */
    DpregClass::Dp3src,      /* 111111101100 */
    DpregClass::Dp3src,      /* 111111101101 */
    DpregClass::Dp3src,      /* 111111101110 */
    DpregClass::Dp3src,      /* 111111101111 */
    DpregClass::Dp3src,      /* 111111110000 */
    DpregClass::Dp3src,      /* 111111110001 */
    DpregClass::Dp3src,      /* 111111110010 */
    DpregClass::Dp3src,      /* 111111110011 */
    DpregClass::Dp3src,      /* 111111110100 */
    DpregClass::Dp3src,      /* 111111110101 */
    DpregClass::Dp3src,      /* 111111110110 */
    DpregClass::Dp3src,      /* 111111110111 */
    DpregClass::Dp3src,      /* 111111111000 */
    DpregClass::Dp3src,      /* 111111111001 */
    DpregClass::Dp3src,      /* 111111111010 */
    DpregClass::Dp3src,      /* 111111111011 */
    DpregClass::Dp3src,      /* 111111111100 */
    DpregClass::Dp3src,      /* 111111111101 */
    DpregClass::Dp3src,      /* 111111111110 */
    DpregClass::Dp3src,      /* 111111111111 */
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimdDpClass {
    Unallocated,
    Cryptoaes,
    Cryptosha3,
    Cryptosha2,
    Asisdone,
    Asisdsamefp16,
    Asisdmiscfp16,
    Asisdsame2,
    Asisdmisc,
    Asisdpair,
    Asisddiff,
    Asisdsame,
    Asisdshf,
    Asisdelem,
    Asimdtbl,
    Asimdperm,
    Asimdext,
    Asimdins,
    Asimdsamefp16,
    Asimdmiscfp16,
    Asimdsame2,
    Asimdmisc,
    Asimdall,
    Asimddiff,
    Asimdsame,
    Asimdimm,
    Asimdshf,
    Asimdelem,
    Crypto3Imm2,
    Cryptosha5123,
    Crypto4,
    Crypto3Imm6,
    Cryptosha5122,
    Float2fix,
    Float2int,
    Floatdp1,
    Floatcmp,
    Floatimm,
    Floatccmp,
    Floatdp2,
    Floatsel,
    Floatdp3,
}

#[inline(always)]
pub const fn simd_dp_index(insn: u32) -> usize {
    (((insn >> 28) & 0xf) << 15
        | ((insn >> 23) & 0x3) << 13
        | ((insn >> 19) & 0xf) << 9
        | (insn >> 10) & 0x1ff) as usize
}

static SIMD_DP_CLASSES: [SimdDpClass; 42] = [
    SimdDpClass::Unallocated,
    SimdDpClass::Cryptoaes,
    SimdDpClass::Cryptosha3,
    SimdDpClass::Cryptosha2,
    SimdDpClass::Asisdone,
    SimdDpClass::Asisdsamefp16,
    SimdDpClass::Asisdmiscfp16,
    SimdDpClass::Asisdsame2,
    SimdDpClass::Asisdmisc,
    SimdDpClass::Asisdpair,
    SimdDpClass::Asisddiff,
    SimdDpClass::Asisdsame,
    SimdDpClass::Asisdshf,
    SimdDpClass::Asisdelem,
    SimdDpClass::Asimdtbl,
    SimdDpClass::Asimdperm,
    SimdDpClass::Asimdext,
    SimdDpClass::Asimdins,
    SimdDpClass::Asimdsamefp16,
    SimdDpClass::Asimdmiscfp16,
    SimdDpClass::Asimdsame2,
    SimdDpClass::Asimdmisc,
    SimdDpClass::Asimdall,
    SimdDpClass::Asimddiff,
    SimdDpClass::Asimdsame,
    SimdDpClass::Asimdimm,
    SimdDpClass::Asimdshf,
    SimdDpClass::Asimdelem,
    SimdDpClass::Crypto3Imm2,
    SimdDpClass::Cryptosha5123,
    SimdDpClass::Crypto4,
    SimdDpClass::Crypto3Imm6,
    SimdDpClass::Cryptosha5122,
    SimdDpClass::Float2fix,
    SimdDpClass::Float2int,
    SimdDpClass::Floatdp1,
    SimdDpClass::Floatcmp,
    SimdDpClass::Floatimm,
    SimdDpClass::Floatccmp,
    SimdDpClass::Floatdp2,
    SimdDpClass::Floatsel,
    SimdDpClass::Floatdp3,
];

static SIMD_DP_L0: [[u16; 256]; 1] = [[
    0, 1, 2, 3, 4, 1, 2, 3, 5, 6, 6, 6, 7, 7, 7, 7, 32801, 8, 32801, 8, 32801, 8, 32801, 8, 32809,
    32809, 32809, 32809, 32809, 32809, 32809, 32809, 9, 1, 10, 3, 11, 1, 10, 3, 5, 6, 6, 6, 7, 7,
    7, 7, 32801, 8, 32801, 8, 32801, 8, 32801, 8, 32809, 32809, 32809, 32809, 32809, 32809, 32809,
    32809, 0, 12, 2, 13, 4, 12, 2, 13, 5, 6, 6, 6, 7, 7, 7, 7, 14, 15, 16, 17, 18, 15, 16, 17, 19,
    19, 19, 19, 20, 20, 20, 20, 9, 1, 10, 3, 11, 1, 10, 3, 5, 6, 6, 6, 7, 7, 7, 7, 21, 22, 23, 24,
    25, 22, 23, 24, 19, 19, 19, 19, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32801, 8, 32801, 8,
    32801, 8, 32801, 8, 32809, 32809, 32809, 32809, 32809, 32809, 32809, 32809, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32801, 8, 32801, 8, 32801, 8, 32801, 8, 32809, 32809, 32809, 32809, 32809, 32809, 32809,
    32809, 26, 26, 27, 28, 32799, 32768, 29, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
]];

static SIMD_DP_L1: [[u16; 256]; 30] = [
    [
        0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
        1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
        1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1,
        1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
        1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0,
        0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    ],
    [
        5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1,
        1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6,
        1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5,
        6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1,
        5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1,
        1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6,
        1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5,
        6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1,
        5, 5, 6, 6, 1, 1, 1, 1, 5, 5, 6, 6, 1, 1, 1, 1,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    ],
    [
        6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1,
        1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6,
        1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6,
        6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1,
        6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1,
        1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6,
        1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6,
        6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1,
        6, 6, 6, 6, 1, 1, 1, 1, 6, 6, 6, 6, 1, 1, 1, 1,
    ],
    [
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ],
    [
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ],
    [
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10,
    ],
    [
        11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13,
        12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12,
        13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14,
        12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12,
        14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13,
        12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12,
        13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11,
        12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12,
        11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13,
        12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12,
        13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14, 12, 13, 12, 11, 12, 13, 12, 14,
        12, 13, 12,
    ],
    [
        15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15,
        15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15,
        1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1,
        1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1,
        15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15,
        15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15,
        1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1,
        1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1,
        15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15,
        15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1, 15, 15, 15, 15, 1, 1, 1, 1,
    ],
    [
        16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16,
        17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17,
        1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1,
        1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1,
        16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16,
        17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17,
        1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1,
        1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1,
        16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16,
        17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1, 16, 16, 17, 17, 1, 1, 1, 1,
    ],
    [
        17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17,
        17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17,
        1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1,
        1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1,
        17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17,
        17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17,
        1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1,
        1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1,
        17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17,
        17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1, 17, 17, 17, 17, 1, 1, 1, 1,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    ],
    [
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    ],
    [
        19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20,
        20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20,
        20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20,
        20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19,
        20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19,
        19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19,
        19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19,
        19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20,
        19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20,
        20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20,
        20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20, 20, 20, 20, 19, 19, 19, 19, 20,
        20, 20, 20,
    ],
    [
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23,
        23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22,
    ],
    [
        25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20,
        20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20,
        20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20,
        20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26,
        20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26,
        26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25,
        26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25,
        25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20,
        25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20,
        20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20,
        20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20, 20, 20, 20, 25, 25, 26, 26, 20,
        20, 20, 20,
    ],
    [
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23,
        23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
        27, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22,
    ],
    [
        26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20,
        20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20,
        20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20,
        20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26,
        20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26,
        26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26,
        26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26,
        26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20,
        26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20,
        20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20,
        20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20, 20, 20, 20, 26, 26, 26, 26, 20,
        20, 20, 20,
    ],
    [
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
        28, 28, 28,
    ],
    [
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29, 29,
        29, 29, 29,
    ],
    [
        30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20,
        20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20,
        20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20,
        20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30,
        20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30,
        30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30,
        30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30,
        30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20,
        30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20,
        20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20,
        20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20, 20, 20, 20, 30, 30, 30, 30, 20,
        20, 20, 20,
    ],
    [
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22,
    ],
    [
        31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768,
        32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20,
        20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31,
        32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768,
        20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20,
        31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768,
        32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20,
        20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31,
        32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768,
        20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20,
        31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768,
        32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20,
        20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31, 32768, 32768, 20, 20, 20, 20, 31, 31,
        32768, 32768, 20, 20, 20, 20,
    ],
    [
        21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
        24, 24, 24, 24, 24, 24, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27, 27,
        27, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22,
    ],
    [
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
        32768, 32768, 32768, 32768, 20, 20, 20, 20, 32768, 32768, 32768, 32768, 20, 20, 20, 20,
    ],
    [
        32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768,
        32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798,
        32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768,
        32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798,
        32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798,
        32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768,
        32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798,
        32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768,
        32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768,
        32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798,
        32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768,
        32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798,
        32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798,
        32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768,
        32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798,
        32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768,
        32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768,
        32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798,
        32798, 32798, 32768, 32768, 32768, 32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768,
        32768, 32798, 32798, 32798, 32798, 32768, 32768, 32768, 32768,
    ],
    [
        32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796,
        32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798,
        32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768,
        32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798,
        32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798,
        32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796,
        32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798,
        32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768,
        32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796,
        32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798,
        32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768,
        32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798,
        32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798,
        32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796,
        32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798,
        32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768,
        32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796,
        32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798,
        32798, 32798, 32796, 32796, 32768, 32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768,
        32768, 32798, 32798, 32798, 32798, 32796, 32796, 32768, 32768,
    ],
    [
        32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768,
        32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798,
        32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798,
        32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768,
        32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768,
        32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798,
        32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798,
        32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768,
        32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768,
        32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798,
        32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798,
        32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768,
        32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768,
        32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798,
        32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798,
        32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768,
        32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798, 32, 32768,
        32, 32768, 32798, 32798, 32798, 32798, 32, 32768, 32, 32768, 32798, 32798, 32798, 32798,
        32, 32768, 32, 32768,
    ],
    [
        32768, 32768, 32768, 32768, 33, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
];

static SIMD_DP_L2: [[u16; 8]; 34] = [
    [32782, 32785, 32783, 32785, 32782, 32785, 32783, 32785],
    [32768, 32788, 32768, 32788, 32768, 32788, 32768, 32788],
    [32791, 32792, 32789, 32792, 32791, 32792, 32789, 32792],
    [32791, 32792, 32768, 32792, 32791, 32792, 32768, 32792],
    [32791, 32792, 32790, 32792, 32791, 32792, 32790, 32792],
    [32782, 32786, 32783, 32786, 32782, 32786, 32783, 32786],
    [32782, 32768, 32783, 32768, 32782, 32768, 32783, 32768],
    [32791, 32792, 32787, 32792, 32791, 32792, 32787, 32792],
    [32795, 32793, 32795, 32793, 32795, 32793, 32795, 32793],
    [32795, 32794, 32795, 32794, 32795, 32794, 32795, 32794],
    [32795, 32768, 32795, 32768, 32795, 32768, 32795, 32768],
    [32802, 32806, 32807, 32808, 32805, 32806, 32807, 32808],
    [32804, 32806, 32807, 32808, 32805, 32806, 32807, 32808],
    [32803, 32806, 32807, 32808, 32805, 32806, 32807, 32808],
    [32768, 32806, 32807, 32808, 32805, 32806, 32807, 32808],
    [32784, 32785, 32784, 32785, 32784, 32785, 32784, 32785],
    [32784, 32786, 32784, 32786, 32784, 32786, 32784, 32786],
    [32784, 32768, 32784, 32768, 32784, 32768, 32784, 32768],
    [32791, 32792, 32769, 32792, 32791, 32792, 32769, 32792],
    [32770, 32772, 32768, 32772, 32770, 32772, 32768, 32772],
    [32768, 32775, 32768, 32775, 32768, 32775, 32768, 32775],
    [32778, 32779, 32776, 32779, 32778, 32779, 32776, 32779],
    [32778, 32779, 32768, 32779, 32778, 32779, 32768, 32779],
    [32778, 32779, 32771, 32779, 32778, 32779, 32771, 32779],
    [32778, 32779, 32777, 32779, 32778, 32779, 32777, 32779],
    [32770, 32773, 32768, 32773, 32770, 32773, 32768, 32773],
    [32770, 32768, 32768, 32768, 32770, 32768, 32768, 32768],
    [32778, 32779, 32774, 32779, 32778, 32779, 32774, 32779],
    [32781, 32780, 32781, 32780, 32781, 32780, 32781, 32780],
    [32781, 32768, 32781, 32768, 32781, 32768, 32781, 32768],
    [32768, 32772, 32768, 32772, 32768, 32772, 32768, 32772],
    [32768, 32773, 32768, 32773, 32768, 32773, 32768, 32773],
    [32797, 32797, 32797, 32797, 32768, 32768, 32768, 32768],
    [32800, 32800, 32800, 32800, 32768, 32768, 32768, 32768],
];

#[inline]
pub fn decode_simd_dp(insn: u32) -> SimdDpClass {
    let i = simd_dp_index(insn);
    let mut e: u16 = 0;
    e = SIMD_DP_L0[e as usize][(i >> 11) & 0xff];
    if e & 0x8000 != 0 {
        return SIMD_DP_CLASSES[(e & 0x7fff) as usize];
    }
    e = SIMD_DP_L1[e as usize][(i >> 3) & 0xff];
    if e & 0x8000 != 0 {
        return SIMD_DP_CLASSES[(e & 0x7fff) as usize];
    }
    e = SIMD_DP_L2[e as usize][i & 0x7];
    SIMD_DP_CLASSES[(e & 0x7fff) as usize]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdstClass {
    Unallocated,
    Comswappr,
    ComswapprUnpriv,
    Asisdlse,
    Asisdlsep,
    Asisdlso,
    Asisdlsop,
    Rcwcomswap,
    Rcwcomswappr,
    Memop128,
    MemopUnpriv,
    LdstGcs,
    Ldsttags,
    Ldstexclp,
    LdstexclrUnpriv,
    ComswapUnpriv,
    Ldstexclr,
    Ldstord,
    Comswap,
    Ldiappstilp,
    LdapstlWriteback,
    LdapstlUnscaled,
    LdapstlSimd,
    Loadlit,
    Memcms,
    LdstnapairOffs,
    LdstpairPost,
    LdstpairOff,
    LdstpairPre,
    LdstUnscaled,
    LdstImmpost,
    LdstUnpriv,
    LdstImmpre,
    Memop,
    LdstRegoff,
    LdstPac,
    LdstPos,
}

#[inline(always)]
pub const fn ldst_index(insn: u32) -> usize {
    (((insn >> 28) & 0xf) << 16 | ((insn >> 26) & 0x1) << 15 | (insn >> 10) & 0x7fff) as usize
}

static LDST_CLASSES: [LdstClass; 37] = [
    LdstClass::Unallocated,
    LdstClass::Comswappr,
    LdstClass::ComswapprUnpriv,
    LdstClass::Asisdlse,
    LdstClass::Asisdlsep,
    LdstClass::Asisdlso,
    LdstClass::Asisdlsop,
    LdstClass::Rcwcomswap,
    LdstClass::Rcwcomswappr,
    LdstClass::Memop128,
    LdstClass::MemopUnpriv,
    LdstClass::LdstGcs,
    LdstClass::Ldsttags,
    LdstClass::Ldstexclp,
    LdstClass::LdstexclrUnpriv,
    LdstClass::ComswapUnpriv,
    LdstClass::Ldstexclr,
    LdstClass::Ldstord,
    LdstClass::Comswap,
    LdstClass::Ldiappstilp,
    LdstClass::LdapstlWriteback,
    LdstClass::LdapstlUnscaled,
    LdstClass::LdapstlSimd,
    LdstClass::Loadlit,
    LdstClass::Memcms,
    LdstClass::LdstnapairOffs,
    LdstClass::LdstpairPost,
    LdstClass::LdstpairOff,
    LdstClass::LdstpairPre,
    LdstClass::LdstUnscaled,
    LdstClass::LdstImmpost,
    LdstClass::LdstUnpriv,
    LdstClass::LdstImmpre,
    LdstClass::Memop,
    LdstClass::LdstRegoff,
    LdstClass::LdstPac,
    LdstClass::LdstPos,
];

static LDST_L0: [[u16; 256]; 1] = [[
    0, 0, 1, 1, 32768, 32768, 2, 2, 3, 3, 4, 4, 5, 5, 32774, 32774, 32791, 32791, 32791, 32791, 6,
    6, 7, 7, 32791, 32791, 32791, 32791, 8, 8, 8, 8, 32793, 32793, 32794, 32794, 32795, 32795,
    32796, 32796, 32793, 32793, 32794, 32794, 32795, 32795, 32796, 32796, 9, 9, 9, 9, 32804, 32804,
    32804, 32804, 9, 9, 9, 9, 32804, 32804, 32804, 32804, 0, 0, 1, 1, 32768, 32768, 2, 2, 3, 3, 4,
    4, 5, 5, 32774, 32774, 32791, 32791, 32791, 32791, 6, 6, 7, 7, 32791, 32791, 32791, 32791, 8,
    8, 8, 8, 32793, 32793, 32794, 32794, 32795, 32795, 32796, 32796, 32793, 32793, 32794, 32794,
    32795, 32795, 32796, 32796, 9, 9, 9, 9, 32804, 32804, 32804, 32804, 9, 9, 9, 9, 32804, 32804,
    32804, 32804, 10, 10, 1, 1, 11, 11, 12, 12, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32768, 32791, 32791, 32791, 32791, 13, 13, 14, 14, 32791, 32791, 32791, 32791, 8, 8, 8, 8,
    32793, 32793, 32794, 32794, 32795, 32795, 32796, 32796, 32793, 32793, 32794, 32794, 32795,
    32795, 32796, 32796, 9, 9, 9, 9, 32804, 32804, 32804, 32804, 9, 9, 9, 9, 32804, 32804, 32804,
    32804, 10, 10, 1, 1, 11, 11, 12, 12, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    32791, 32791, 32791, 32791, 15, 16, 17, 17, 32791, 32791, 32791, 32791, 8, 8, 8, 8, 32793,
    32793, 32794, 32794, 32795, 32795, 32796, 32796, 32793, 32793, 32794, 32794, 32795, 32795,
    32796, 32796, 9, 9, 9, 9, 32804, 32804, 32804, 32804, 9, 9, 9, 9, 32804, 32804, 32804, 32804,
]];

static LDST_L1: [[u16; 256]; 18] = [
    [
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
        32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769,
    ],
    [
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785,
        32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32785, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
        32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786, 32786,
    ],
    [
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770,
        32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32770, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        32771, 32771, 32771, 32771, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772,
        32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32772, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        32773, 32773, 32773, 32773, 32773, 32773, 32773, 32773, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32773, 32773,
        32773, 32773, 32773, 32773, 32773, 32773, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2,
        2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
        1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2,
        2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
        1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
    ],
    [
        3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2,
        2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
        1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2,
        2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
        1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 2,
    ],
    [
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768,
    ],
    [
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    ],
    [
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784,
        32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32784, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
        32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781, 32781,
    ],
    [
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782,
        32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32782, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783,
        32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32783, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768,
    ],
    [
        3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768, 32768,
        32768, 32768,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 8, 8, 0, 0, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780,
    ],
    [
        3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780, 32780,
        32780, 32780,
    ],
];

static LDST_L2: [[u16; 16]; 9] = [
    [
        32789, 32792, 32787, 32768, 32789, 32792, 32787, 32768, 32789, 32792, 32787, 32768, 32789,
        32792, 32787, 32768,
    ],
    [
        32777, 32778, 32775, 32776, 32777, 32778, 32768, 32768, 32777, 32778, 32768, 32768, 32777,
        32778, 32768, 32768,
    ],
    [
        32777, 32778, 32768, 32768, 32777, 32778, 32768, 32768, 32777, 32778, 32768, 32768, 32777,
        32778, 32768, 32768,
    ],
    [
        32789, 32792, 32788, 32768, 32789, 32792, 32768, 32768, 32789, 32792, 32768, 32768, 32789,
        32792, 32768, 32768,
    ],
    [
        32789, 32792, 32768, 32768, 32789, 32792, 32768, 32768, 32789, 32792, 32768, 32768, 32789,
        32792, 32768, 32768,
    ],
    [
        32768, 32792, 32790, 32768, 32768, 32792, 32790, 32768, 32768, 32792, 32790, 32768, 32768,
        32792, 32790, 32768,
    ],
    [
        32797, 32798, 32799, 32800, 32797, 32798, 32799, 32800, 32797, 32798, 32799, 32800, 32797,
        32798, 32799, 32800,
    ],
    [
        32801, 32803, 32802, 32803, 32801, 32803, 32802, 32803, 32801, 32803, 32802, 32803, 32801,
        32803, 32802, 32803,
    ],
    [
        32789, 32792, 32787, 32779, 32789, 32792, 32787, 32779, 32789, 32792, 32787, 32779, 32789,
        32792, 32787, 32779,
    ],
];

#[inline]
pub fn decode_ldst(insn: u32) -> LdstClass {
    let i = ldst_index(insn);
    let mut e: u16 = 0;
    e = LDST_L0[e as usize][(i >> 12) & 0xff];
    if e & 0x8000 != 0 {
        return LDST_CLASSES[(e & 0x7fff) as usize];
    }
    e = LDST_L1[e as usize][(i >> 4) & 0xff];
    if e & 0x8000 != 0 {
        return LDST_CLASSES[(e & 0x7fff) as usize];
    }
    e = LDST_L2[e as usize][i & 0xf];
    LDST_CLASSES[(e & 0x7fff) as usize]
}
