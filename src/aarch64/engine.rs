#![allow(unused)]

use crate::aarch64::{
    decode_tables::{
        ControlClass::{self},
        DECODE_DPIMM, DECODE_DPREG, DECODE_ROOT,
        DpimmClass::{self},
        DpregClass::{self},
        LdstClass::{self},
        RootClass::{self},
        SimdDpClass::{self},
        decode_control, decode_ldst, decode_simd_dp, dpimm_index, dpreg_index, root_index,
    },
    mnemonic::Mnemonic,
    registers::{GpRegister, GpRegister32, GpRegister64},
};

use core::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Immediate {
    Imm8(u8),
    Imm12(u16),
    Imm16(u16),
    ShiftAmount(u8),  // Imms
    RotateAmount(u8), // Immr
    None,
}

struct Instruction {
    mnemonic: Mnemonic,
    dst: GpRegister,
    src: GpRegister,
    shift: Option<u8>,
}

pub struct Disassembler<'a> {
    instructions: &'a [u8],
    pub decoded_inst: String,
}

impl<'a> Disassembler<'a> {
    pub fn new(instructions: &'a [u8]) -> Self {
        Self {
            instructions,
            // use 50mb capacity initially
            decoded_inst: String::with_capacity(50_000),
        }
    }

    #[inline(always)]
    fn mnemonic_str(&self, mnemonic: Mnemonic) -> &'static str {
        mnemonic.into()
    }

    #[inline(always)]
    fn gp_register_str(&self, sf: u8, idx: usize) -> &'static str {
        if sf == 1 {
            GpRegister64::from_repr(idx).unwrap().into()
        } else {
            GpRegister32::from_repr(idx).unwrap().into()
        }
    }

    fn build_instruction(
        mnemonic: Mnemonic,
        dst: GpRegister,
        src: GpRegister,
        imm: Immediate,
        shift: u8,
    ) {
    }

    fn decode_add_sub(&mut self, inst: u32) {
        let sf = ((inst >> 31) & 0x01) as u8;
        let op = ((inst >> 30) & 0x01) as u8;
        let s = ((inst >> 29) & 0x01) as u8;
        let sh = ((inst >> 22) & 0x01) as u8;
        let imm12 = ((inst >> 10) & 0xFFF) as u16;
        let rn = ((inst >> 5) & 0x1F) as u8; /* first source */
        let rd = (inst & 0x1F) as u8; /* destination */

        match (sf, op, s) {
            (0b0, 0b0, 0b0) | (0b1, 0b0, 0b0) => {
                // Add imm - {32,64} bit
                let src = self.gp_register_str(sf, rn as usize);
                let dst = self.gp_register_str(sf, rd as usize);

                if (imm12 == 0b0 && (rd == 0b11111 || rn == 0b11111)) {
                    let mnemonic = self.mnemonic_str(Mnemonic::Mov);
                    writeln!(&mut self.decoded_inst, "{} {}, {}", mnemonic, dst, src);
                } else {
                    let mnemonic = self.mnemonic_str(Mnemonic::Add);

                    write!(
                        &mut self.decoded_inst,
                        "{} {}, {}, #{}",
                        mnemonic, dst, src, imm12
                    );

                    if sh == 0b1 {
                        write!(&mut self.decoded_inst, ", LSL #12");
                    }

                    writeln!(&mut self.decoded_inst, "");
                }
            }

            (0b0, 0b0, 0b1) | (0b1, 0b0, 0b1) => {
                // adds imm - {32, 64} bit
                let mnemonic = self.mnemonic_str(Mnemonic::Adds);
                let src = self.gp_register_str(sf, rn as usize);
                let dst = self.gp_register_str(sf, rd as usize);

                write!(
                    &mut self.decoded_inst,
                    "{} {}, {}, #{}",
                    mnemonic, dst, src, imm12
                );

                if sh == 0b1 {
                    write!(&mut self.decoded_inst, ", LSL #12");
                }

                writeln!(&mut self.decoded_inst, "");
            }

            (0b0, 0b1, 0b0) | (0b1, 0b1, 0b0) => {
                // sub imm - {32, 64} bit
                let mnemonic = self.mnemonic_str(Mnemonic::Sub);
                let src = self.gp_register_str(sf, rn as usize);
                let dst = self.gp_register_str(sf, rd as usize);

                write!(
                    &mut self.decoded_inst,
                    "{} {}, {}, #{}",
                    mnemonic, dst, src, imm12
                );

                if sh == 0b1 {
                    write!(&mut self.decoded_inst, ", LSL #12");
                }

                writeln!(&mut self.decoded_inst, "");
            }

            (0b0, 0b1, 0b1) | (0b1, 0b1, 0b1) => {
                // subs imm - {32, 64} bit
                let src = self.gp_register_str(sf, rn as usize);
                let dst = self.gp_register_str(sf, rd as usize);

                if rd == 0b11111 {
                    let mnemonic = self.mnemonic_str(Mnemonic::Cmp);
                    write!(&mut self.decoded_inst, "{} {}, #{}", mnemonic, src, imm12);
                } else {
                    let mnemonic = self.mnemonic_str(Mnemonic::Subs);
                    write!(
                        &mut self.decoded_inst,
                        "{} {}, {}, #{}",
                        mnemonic, dst, src, imm12
                    );
                }

                if sh == 0b1 {
                    write!(&mut self.decoded_inst, ", LSL #12");
                }

                writeln!(&mut self.decoded_inst, "");
            }

            _ => unreachable!(),
        }
    }

    pub fn decode(&mut self) {
        const INSTRUCTION_LENGTH: usize = 4;

        // how beautiful is this fixed 4-byte fixed i hate you x86-64
        for instruction in self.instructions.chunks_exact(INSTRUCTION_LENGTH) {
            let word = u32::from_le_bytes(instruction.try_into().unwrap());

            match DECODE_ROOT[root_index(word)] {
                RootClass::Reserved => {}

                RootClass::Sme => {}

                RootClass::Sve => {}

                RootClass::Unallocated => {}

                RootClass::Dpimm => match DECODE_DPIMM[dpimm_index(word)] {
                    DpimmClass::Dp1srcImm => {}
                    DpimmClass::Pcreladdr => {}
                    DpimmClass::AddsubImm => self.decode_add_sub(word),
                    DpimmClass::AddsubImmtags => {}
                    DpimmClass::MinmaxImm => {}
                    DpimmClass::LogImm => {}
                    DpimmClass::Movewide => {}
                    DpimmClass::Bitfield => {}
                    DpimmClass::Extract => {}
                },

                RootClass::Control => match decode_control(word) {
                    ControlClass::Condbranch => {}
                    ControlClass::Miscbranch => {}
                    ControlClass::CompbranchRegs2 => {}
                    ControlClass::Unallocated => {}
                    ControlClass::Exception => {}
                    ControlClass::Extendedhints => {}
                    ControlClass::Systeminstrswithreg => {}
                    ControlClass::Hints => {}
                    ControlClass::Barriers => {}
                    ControlClass::Pstate => {}
                    ControlClass::Systeminstrs => {}
                    ControlClass::Systemmove => {}
                    ControlClass::Syspairinstrs => {}
                    ControlClass::Systemmovepr => {}
                    ControlClass::BranchReg => {}
                    ControlClass::BranchImm => {}
                    ControlClass::Compbranch => {}
                    ControlClass::Testbranch => {}
                    ControlClass::CompbranchRegs => {}
                    ControlClass::CompbranchImm => {}
                },

                RootClass::Dpreg => match DECODE_DPREG[dpreg_index(word)] {
                    DpregClass::Dp2src => {}
                    DpregClass::Dp1src => {}
                    DpregClass::LogShift => {}
                    DpregClass::AddsubShift => {}
                    DpregClass::AddsubExt => {}
                    DpregClass::AddsubCarry => {}
                    DpregClass::AddsubPt => {}
                    DpregClass::Unallocated => {}
                    DpregClass::Rmif => {}
                    DpregClass::Setf => {}
                    DpregClass::CondcmpReg => {}
                    DpregClass::CondcmpImm => {}
                    DpregClass::Condsel => {}
                    DpregClass::Dp3src => {}
                },

                RootClass::SimdDp => match decode_simd_dp(word) {
                    SimdDpClass::Unallocated => {}
                    SimdDpClass::Cryptoaes => {}
                    SimdDpClass::Cryptosha3 => {}
                    SimdDpClass::Cryptosha2 => {}
                    SimdDpClass::Asisdone => {}
                    SimdDpClass::Asisdsamefp16 => {}
                    SimdDpClass::Asisdmiscfp16 => {}
                    SimdDpClass::Asisdsame2 => {}
                    SimdDpClass::Asisdmisc => {}
                    SimdDpClass::Asisdpair => {}
                    SimdDpClass::Asisddiff => {}
                    SimdDpClass::Asisdsame => {}
                    SimdDpClass::Asisdshf => {}
                    SimdDpClass::Asisdelem => {}
                    SimdDpClass::Asimdtbl => {}
                    SimdDpClass::Asimdperm => {}
                    SimdDpClass::Asimdext => {}
                    SimdDpClass::Asimdins => {}
                    SimdDpClass::Asimdsamefp16 => {}
                    SimdDpClass::Asimdmiscfp16 => {}
                    SimdDpClass::Asimdsame2 => {}
                    SimdDpClass::Asimdmisc => {}
                    SimdDpClass::Asimdall => {}
                    SimdDpClass::Asimddiff => {}
                    SimdDpClass::Asimdsame => {}
                    SimdDpClass::Asimdimm => {}
                    SimdDpClass::Asimdshf => {}
                    SimdDpClass::Asimdelem => {}
                    SimdDpClass::Crypto3Imm2 => {}
                    SimdDpClass::Cryptosha5123 => {}
                    SimdDpClass::Crypto4 => {}
                    SimdDpClass::Crypto3Imm6 => {}
                    SimdDpClass::Cryptosha5122 => {}
                    SimdDpClass::Float2fix => {}
                    SimdDpClass::Float2int => {}
                    SimdDpClass::Floatdp1 => {}
                    SimdDpClass::Floatcmp => {}
                    SimdDpClass::Floatimm => {}
                    SimdDpClass::Floatccmp => {}
                    SimdDpClass::Floatdp2 => {}
                    SimdDpClass::Floatsel => {}
                    SimdDpClass::Floatdp3 => {}
                },

                RootClass::Ldst => match decode_ldst(word) {
                    LdstClass::Unallocated => {}
                    LdstClass::Comswappr => {}
                    LdstClass::ComswapprUnpriv => {}
                    LdstClass::Asisdlse => {}
                    LdstClass::Asisdlsep => {}
                    LdstClass::Asisdlso => {}
                    LdstClass::Asisdlsop => {}
                    LdstClass::Rcwcomswap => {}
                    LdstClass::Rcwcomswappr => {}
                    LdstClass::Memop128 => {}
                    LdstClass::MemopUnpriv => {}
                    LdstClass::LdstGcs => {}
                    LdstClass::Ldsttags => {}
                    LdstClass::Ldstexclp => {}
                    LdstClass::LdstexclrUnpriv => {}
                    LdstClass::ComswapUnpriv => {}
                    LdstClass::Ldstexclr => {}
                    LdstClass::Ldstord => {}
                    LdstClass::Comswap => {}
                    LdstClass::Ldiappstilp => {}
                    LdstClass::LdapstlWriteback => {}
                    LdstClass::LdapstlUnscaled => {}
                    LdstClass::LdapstlSimd => {}
                    LdstClass::Loadlit => {}
                    LdstClass::Memcms => {}
                    LdstClass::LdstnapairOffs => {}
                    LdstClass::LdstpairPost => {}
                    LdstClass::LdstpairOff => {}
                    LdstClass::LdstpairPre => {}
                    LdstClass::LdstUnscaled => {}
                    LdstClass::LdstImmpost => {}
                    LdstClass::LdstUnpriv => {}
                    LdstClass::LdstImmpre => {}
                    LdstClass::Memop => {}
                    LdstClass::LdstRegoff => {}
                    LdstClass::LdstPac => {}
                    LdstClass::LdstPos => {}
                },
            }
        }
    }
}
