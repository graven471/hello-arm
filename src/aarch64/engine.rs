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
    registers::{
        GpRegister, GpRegister32, GpRegister64, SimdFpRegister32, SimdFpRegister64,
        SimdFpRegister128,
    },
};

use core::fmt::Write;
use std::ptr::write;

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

    #[inline(always)]
    fn simd_fp_register_str(&self, opc: u8, idx: usize) -> &'static str {
        match opc {
            (0b00) => {
                // 32-bit variant
                SimdFpRegister32::from_repr(idx).unwrap().into()
            }
            (0b01) => {
                // 64-bit variant
                SimdFpRegister64::from_repr(idx).unwrap().into()
            }
            (0b10) => {
                // 128-bit variant
                SimdFpRegister128::from_repr(idx).unwrap().into()
            }
            _ => unreachable!(),
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
        let sf = ((inst >> 31) & 0x01) as u8; /* 0 = 32-bit GP register 1 = 64-bit */
        let op = ((inst >> 30) & 0x01) as u8;
        let s = ((inst >> 29) & 0x01) as u8;
        let sh = ((inst >> 22) & 0x01) as u8;
        let imm12 = ((inst >> 10) & 0xFFF) as u16;
        let rn = ((inst >> 5) & 0x1F) as u8; /* first source */
        let rd = (inst & 0x1F) as u8; /* destination */

        //add-sub
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

                    DpregClass::Dp1src => {
                        let sf = ((word >> 31) & 0x01) as u8;
                        let s = ((word >> 29) & 0x01) as u8;

                        let opcode2 = ((word >> 16) & 0x1F) as u8;
                        let opcode = ((word >> 10) & 0x3F) as u8;

                        let rn = ((word >> 5) & 0x1F) as u8;
                        let rd = (word & 0x1F) as u8;
                    }

                    DpregClass::LogShift => {
                        let sf = ((word >> 31) & 0x01) as u8;
                        let opc = ((word >> 29) & 0x3) as u8;
                        let shift = ((word >> 22) & 0x3) as u8;
                        let n = ((word >> 21) & 0x01) as u8;
                        let rm = ((word >> 16) & 0x1F) as u8;
                        let imm6 = ((word >> 10) & 0x3F) as u8;
                        let rn = ((word >> 5) & 0x1F) as u8;
                        let rd = (word & 0x1F) as u8;

                        match (sf, opc, n) {
                            // AND - {32,64} bit
                            (0b0, 0b00, 0b0) | (0b1, 0b00, 0b0) => {
                                // AND <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
                                // AND <Xd>, <Xn>, <Xm>{, <shift> #<amount>} sf == 1

                                if sf == 0 && ((imm6 >> 5) & 0x01) != 0 {
                                    // reference says: if sf == '0' && imm6[5] == '1' then EndOfDecode(Decode_UNDEF); end;
                                    writeln!(self.decoded_inst, "UNALLOCATED");
                                    continue;
                                }

                                let mnemonic = self.mnemonic_str(Mnemonic::And);
                                let src = self.gp_register_str(sf, rn as usize);
                                let src2 = self.gp_register_str(sf, rm as usize);
                                let dst = self.gp_register_str(sf, rd as usize);

                                write!(
                                    self.decoded_inst,
                                    "{} {}, {}, {}",
                                    mnemonic, dst, src, src2
                                );

                                match shift {
                                    0b00 => {
                                        write!(self.decoded_inst, ", LSL #{}", imm6);
                                    }
                                    0b01 => {
                                        write!(self.decoded_inst, ", LSR #{}", imm6);
                                    }
                                    0b10 => {
                                        write!(self.decoded_inst, ", ASR #{}", imm6);
                                    }
                                    0b11 => {
                                        write!(self.decoded_inst, ", ROR #{}", imm6);
                                    }
                                    _ => unreachable!(),
                                }

                                writeln!(self.decoded_inst, "");
                            }

                            (0b0, 0b01, 0b0) | (0b1, 0b01, 0b0) => {
                                // ORR <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
                                if sf == 0 && ((imm6 >> 5) & 0x01) != 0 {
                                    // reference says: if sf == '0' && imm6[5] == '1' then EndOfDecode(Decode_UNDEF); end;
                                    writeln!(self.decoded_inst, "UNALLOCATED");
                                    continue;
                                }

                                let dst = self.gp_register_str(sf, rd as usize);

                                if shift == 0b00 && imm6 == 0b0 && rn == 0b11111 {
                                    // alias condition MOV(reg)
                                    let mnemonic = self.mnemonic_str(Mnemonic::Mov);
                                    let src = self.gp_register_str(sf, rm as usize);
                                    writeln!(self.decoded_inst, "{} {}, {}", mnemonic, dst, src);
                                    continue;
                                }

                                let mnemonic = self.mnemonic_str(Mnemonic::Orr);
                                let src = self.gp_register_str(sf, rn as usize); // wn
                                let src2 = self.gp_register_str(sf, rm as usize); // wm

                                write!(
                                    self.decoded_inst,
                                    "{} {}, {}, {}",
                                    mnemonic, dst, src, src2
                                );

                                match shift {
                                    0b00 => {
                                        write!(self.decoded_inst, ", LSL #{}", imm6);
                                    }
                                    0b01 => {
                                        write!(self.decoded_inst, ", LSR #{}", imm6);
                                    }
                                    0b10 => {
                                        write!(self.decoded_inst, ", ASR #{}", imm6);
                                    }
                                    0b11 => {
                                        write!(self.decoded_inst, ", ROR #{}", imm6);
                                    }
                                    _ => unreachable!(),
                                }

                                writeln!(self.decoded_inst, "");
                            }

                            (0b0, 0b01, 0b0) | (0b1, 0b01, 0b1) => {
                                // ORN
                            }

                            (0b0, 0b10, 0b0) => {
                                // EOR
                            }

                            (0b0, 0b11, 0b0) | (0b1, 0b11, 0b0) => {
                                // ANDS
                            }

                            _ => {}
                        }
                    }
                    DpregClass::AddsubShift => {
                        let sf = ((word >> 31) & 0x01) as u8;
                        let op = ((word >> 30) & 0x01) as u8;
                        let s = ((word >> 29) & 0x01) as u8;
                        let shift = ((word >> 22) & 0x3) as u8;
                        let rm = ((word >> 16) & 0x1F) as u8;
                        let imm6 = ((word >> 10) & 0x3F) as u8;
                        let rn = ((word >> 5) & 0x1F) as u8;
                        let rd = (word & 0x1F) as u8;

                        match (sf, op, s) {
                            (0b0, 0b0, 0b0) | (0b1, 0b0, 0b0) => {
                                // ADD <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
                                let mnemonic = self.mnemonic_str(Mnemonic::Add);

                                if sf == 0b11 || (sf == 0b0 && ((imm6 >> 5) & 0x01) != 0) {
                                    writeln!(self.decoded_inst, "UNALLOCATED");
                                    continue;
                                }

                                let dst = self.gp_register_str(sf, rd as usize);
                                let src = self.gp_register_str(sf, rn as usize);
                                let src2 = self.gp_register_str(sf, rm as usize);

                                write!(
                                    self.decoded_inst,
                                    "{} {}, {}, {}",
                                    mnemonic, dst, src, src2
                                );

                                match shift {
                                    0b00 => {
                                        write!(self.decoded_inst, ", LSL #{}", imm6);
                                    }

                                    0b01 => {
                                        write!(self.decoded_inst, ", LSR #{}", imm6);
                                    }

                                    0b10 => {
                                        write!(self.decoded_inst, ", ASR #{}", imm6);
                                    }

                                    0b11 => {
                                        write!(self.decoded_inst, ", RESERVED #{}", imm6);
                                    }

                                    _ => unreachable!(),
                                }

                                writeln!(self.decoded_inst, "");
                            }

                            (0b0, 0b1, 0b0) | (0b1, 0b1, 0b0) => {
                                // SUB <Wd>, <Wn>, <Wm>{, <shift> #<amount>}
                                let sf = ((word >> 31) & 0x01) as u8;
                                let op = ((word >> 30) & 0x01) as u8;
                                let s = ((word >> 29) & 0x01) as u8;
                                let shift = ((word >> 22) & 0x3) as u8;
                                let rm = ((word >> 16) & 0x1F) as u8;
                                let imm6 = ((word >> 10) & 0x3F) as u8;
                                let rn = ((word >> 5) & 0x1F) as u8;
                                let rd = (word & 0x1F) as u8;

                                if sf == 0b11 || (sf == 0b0 && ((imm6 >> 5) & 0x01) != 0) {
                                    writeln!(self.decoded_inst, "UNALLOCATED");
                                    continue;
                                }

                                if rn == 0b11111 {
                                    // NEG <Wd>, <Wm>{, <shift> #<amount>}
                                    continue;
                                }

                                let mnemonic = self.mnemonic_str(Mnemonic::Sub);
                                let dst = self.gp_register_str(sf, rd as usize);
                                let src = self.gp_register_str(sf, rn as usize);
                                let src2 = self.gp_register_str(sf, rm as usize);

                                write!(
                                    self.decoded_inst,
                                    "{} {}, {}, {}",
                                    mnemonic, dst, src, src2
                                );

                                match shift {
                                    0b00 => {
                                        write!(self.decoded_inst, ", LSL #{}", imm6);
                                    }

                                    0b01 => {
                                        write!(self.decoded_inst, ", LSR #{}", imm6);
                                    }

                                    0b10 => {
                                        write!(self.decoded_inst, ", ASR #{}", imm6);
                                    }

                                    0b11 => {
                                        write!(self.decoded_inst, ", RESERVED #{}", imm6);
                                    }

                                    _ => unreachable!(),
                                }

                                writeln!(self.decoded_inst, "");
                            }

                            _ => {}
                        }
                    }

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
                    LdstClass::Loadlit => {
                        let opc = ((word >> 30) & 0x3) as u8;
                        let vr = ((word >> 26) & 0x01) as u8;
                        let imm19 = (word >> 5) & 0x7ffff;
                        let rt = (word & 0x1F) as u8;

                        let mnemonic = self.mnemonic_str(Mnemonic::Ldr);

                        match (opc, vr) {
                            (0b00, 0b0) | (0b01, 0b0) => {
                                // LDR <Wt>, <label>
                                // LDR <Xt>, <label> if opc == 01
                                let wt = self.gp_register_str(opc, rt as usize);
                                //  (manual): Is the program label from which the data is to be loaded.
                                //  Its offset from the address of this instruction, in the range +/-1MB,
                                //  is encoded as "imm19" times 4.
                                let label = imm19 * 4;
                                writeln!(self.decoded_inst, "{} {}, #0x{:X}", mnemonic, wt, label);
                            }

                            (0b00, 0b1) | (0b01, 0b1) | (0b10, 0b1) => {
                                // LDR (literal, SIMD&FP) 32-bit, 64-bit and 128-bit
                                // LDR <St>, <label>, LDR <Dt>, <label>, LDR <Qt>, <label>
                                let reg = self.simd_fp_register_str(opc, rt as usize);

                                let label = imm19 * 4;
                                writeln!(self.decoded_inst, "{} {}, 0x{:#X}", mnemonic, reg, label);
                            }

                            (0b10, 0b0) => {
                                // LDRSW literal
                            }

                            (0b11, 0b0) => {
                                // PRFM literal
                            }

                            _ => {}
                        }
                    }
                    LdstClass::Memcms => {}
                    LdstClass::LdstnapairOffs => {}
                    LdstClass::LdstpairPost => {}
                    LdstClass::LdstpairOff => {}
                    LdstClass::LdstpairPre => {}
                    LdstClass::LdstUnscaled => {}
                    LdstClass::LdstImmpost => {
                        // Load/store register (immediate post-indexed)
                        let size = ((word >> 30) & 0x3) as u8;
                        let vr = ((word >> 26) & 0x01) as u8;
                        let opc = ((word >> 22) & 0x3) as u8;
                        let imm9 = ((word >> 12) & 0x1FF) as u16;
                        let rn = ((word >> 5) & 0x1F) as u8;
                        let rt = (word & 0x1F) as u8;

                        match (size, vr, opc) {
                            (0b10, 0b0, 0b01) | (0b11, 0b0, 0b01) => {
                                // LDR (immediate) 32 and 64 bits
                                // LDR <Wt>, [<Xn|SP>], #<simm> if size == 10
                                // LDR <Xt>, [<Xn|SP>], #<simm> if size == 11

                                let simm = (((word >> 12) & 0x1FF) as i16) << 7 >> 7;

                                let mnemonic = self.mnemonic_str(Mnemonic::Ldr);

                                // {wt,xt}
                                let transferred_reg: &'static str = if size == 0b10 {
                                    GpRegister32::from_repr(rt as usize).unwrap().into()
                                } else if size == 0b11 {
                                    GpRegister64::from_repr(rt as usize).unwrap().into()
                                } else {
                                    ""
                                };

                                let xn: &'static str =
                                    GpRegister64::from_repr(rn as usize).unwrap().into();

                                writeln!(
                                    self.decoded_inst,
                                    "{} {}, [{}], #{}",
                                    mnemonic, transferred_reg, xn, simm
                                );
                            }

                            _ => {}
                        }
                    }
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
