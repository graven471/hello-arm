#![allow(unused)]

use crate::aarch64::decode_tables::{
    ControlClass::{self, *},
    DECODE_DPIMM, DECODE_DPREG, DECODE_ROOT,
    DpimmClass::{self, *},
    DpregClass::{self, *},
    LdstClass::{self, *},
    RootClass::{self, *},
    SimdDpClass::{self, *},
    decode_control, decode_ldst, decode_simd_dp, dpimm_index, dpreg_index, root_index,
};

pub struct Disassembler<'a> {
    instructions: &'a [u8],
}

impl<'a> Disassembler<'a> {
    pub fn new(instructions: &'a [u8]) -> Self {
        Self { instructions }
    }

    pub fn decode(&self) {
        // each instruction is fixed 32-bits 4-bytes
        // read ix slide 4 bytes decode that instruction read again
        const INSTRUCTION_LENGTH: usize = 4;

        // The letter W denotes a general-purpose register holding a 32-bit word,
        // and X denotes a general-purpose register holding a 64-bit doubleword.

        // how beautiful is this fixed 4-byte fixed i hate you x86-64
        for instruction in self.instructions.chunks_exact(INSTRUCTION_LENGTH) {
            let word = u32::from_le_bytes(instruction.try_into().unwrap());

            match DECODE_ROOT[root_index(word)] {
                RootClass::Reserved => todo!(),

                RootClass::Sme => todo!(),

                RootClass::Sve => todo!(),

                RootClass::Unallocated => todo!(),

                RootClass::Dpimm => match DECODE_DPIMM[dpimm_index(word)] {
                    DpimmClass::Dp1srcImm => todo!(),
                    DpimmClass::Pcreladdr => todo!(),
                    DpimmClass::AddsubImm => todo!(),
                    DpimmClass::AddsubImmtags => todo!(),
                    DpimmClass::MinmaxImm => todo!(),
                    DpimmClass::LogImm => todo!(),
                    DpimmClass::Movewide => todo!(),
                    DpimmClass::Bitfield => todo!(),
                    DpimmClass::Extract => todo!(),
                },

                RootClass::Control => match decode_control(word) {
                    ControlClass::Condbranch => todo!(),
                    ControlClass::Miscbranch => todo!(),
                    ControlClass::CompbranchRegs2 => todo!(),
                    ControlClass::Unallocated => todo!(),
                    ControlClass::Exception => todo!(),
                    ControlClass::Extendedhints => todo!(),
                    ControlClass::Systeminstrswithreg => todo!(),
                    ControlClass::Hints => todo!(),
                    ControlClass::Barriers => todo!(),
                    ControlClass::Pstate => todo!(),
                    ControlClass::Systeminstrs => todo!(),
                    ControlClass::Systemmove => todo!(),
                    ControlClass::Syspairinstrs => todo!(),
                    ControlClass::Systemmovepr => todo!(),
                    ControlClass::BranchReg => todo!(),
                    ControlClass::BranchImm => todo!(),
                    ControlClass::Compbranch => todo!(),
                    ControlClass::Testbranch => todo!(),
                    ControlClass::CompbranchRegs => todo!(),
                    ControlClass::CompbranchImm => todo!(),
                },

                RootClass::Dpreg => match DECODE_DPREG[dpreg_index(word)] {
                    DpregClass::Dp2src => todo!(),
                    DpregClass::Dp1src => todo!(),
                    DpregClass::LogShift => todo!(),
                    DpregClass::AddsubShift => todo!(),
                    DpregClass::AddsubExt => todo!(),
                    DpregClass::AddsubCarry => todo!(),
                    DpregClass::AddsubPt => todo!(),
                    DpregClass::Unallocated => todo!(),
                    DpregClass::Rmif => todo!(),
                    DpregClass::Setf => todo!(),
                    DpregClass::CondcmpReg => todo!(),
                    DpregClass::CondcmpImm => todo!(),
                    DpregClass::Condsel => todo!(),
                    DpregClass::Dp3src => todo!(),
                },

                RootClass::SimdDp => match decode_simd_dp(word) {
                    SimdDpClass::Unallocated => todo!(),
                    Cryptoaes => todo!(),
                    Cryptosha3 => todo!(),
                    Cryptosha2 => todo!(),
                    Asisdone => todo!(),
                    Asisdsamefp16 => todo!(),
                    Asisdmiscfp16 => todo!(),
                    Asisdsame2 => todo!(),
                    Asisdmisc => todo!(),
                    Asisdpair => todo!(),
                    Asisddiff => todo!(),
                    Asisdsame => todo!(),
                    Asisdshf => todo!(),
                    Asisdelem => todo!(),
                    Asimdtbl => todo!(),
                    Asimdperm => todo!(),
                    Asimdext => todo!(),
                    Asimdins => todo!(),
                    Asimdsamefp16 => todo!(),
                    Asimdmiscfp16 => todo!(),
                    Asimdsame2 => todo!(),
                    Asimdmisc => todo!(),
                    Asimdall => todo!(),
                    Asimddiff => todo!(),
                    Asimdsame => todo!(),
                    Asimdimm => todo!(),
                    Asimdshf => todo!(),
                    Asimdelem => todo!(),
                    Crypto3Imm2 => todo!(),
                    Cryptosha5123 => todo!(),
                    Crypto4 => todo!(),
                    Crypto3Imm6 => todo!(),
                    Cryptosha5122 => todo!(),
                    Float2fix => todo!(),
                    Float2int => todo!(),
                    Floatdp1 => todo!(),
                    Floatcmp => todo!(),
                    Floatimm => todo!(),
                    Floatccmp => todo!(),
                    Floatdp2 => todo!(),
                    Floatsel => todo!(),
                    Floatdp3 => todo!(),
                },

                RootClass::Ldst => match decode_ldst(word) {
                    LdstClass::Unallocated => todo!(),
                    Comswappr => todo!(),
                    ComswapprUnpriv => todo!(),
                    Asisdlse => todo!(),
                    Asisdlsep => todo!(),
                    Asisdlso => todo!(),
                    Asisdlsop => todo!(),
                    Rcwcomswap => todo!(),
                    Rcwcomswappr => todo!(),
                    Memop128 => todo!(),
                    MemopUnpriv => todo!(),
                    LdstGcs => todo!(),
                    Ldsttags => todo!(),
                    Ldstexclp => todo!(),
                    LdstexclrUnpriv => todo!(),
                    ComswapUnpriv => todo!(),
                    Ldstexclr => todo!(),
                    Ldstord => todo!(),
                    Comswap => todo!(),
                    Ldiappstilp => todo!(),
                    LdapstlWriteback => todo!(),
                    LdapstlUnscaled => todo!(),
                    LdapstlSimd => todo!(),
                    Loadlit => todo!(),
                    Memcms => todo!(),
                    LdstnapairOffs => todo!(),
                    LdstpairPost => todo!(),
                    LdstpairOff => todo!(),
                    LdstpairPre => todo!(),
                    LdstUnscaled => todo!(),
                    LdstImmpost => todo!(),
                    LdstUnpriv => todo!(),
                    LdstImmpre => todo!(),
                    Memop => todo!(),
                    LdstRegoff => todo!(),
                    LdstPac => todo!(),
                    LdstPos => todo!(),
                },
            }
        }
    }
}
