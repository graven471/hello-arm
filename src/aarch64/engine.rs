#![allow(unused)]

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
            /*
                how does this looks ?
                Assembly:   ADD X0, X1, X2
                Hex word:   0x8B020020
                Binary:     10001011000000100000000000100000
                bit:  31  30  29  28......24  23..22  21  20.....16  15........10  9....5  4....0
                    ┌───┬───┬───┬──────────┬───────┬───┬─────────┬────────────┬───────┬───────┐
                    │ 1 │ 0 │ 0 │  01011   │  00   │ 0 │  00010  │   000000   │ 00001 │ 00000 │
                    └───┴───┴───┴──────────┴───────┴───┴─────────┴────────────┴───────┴───────┘
                    sf  op   S   opcode    shift   0     Rm         imm6        Rn      Rd
                    │   │   │      │         │          │            │          │       │
                    │   │   │      │         │          │            │          │       └─ destination = reg 0  → X0
                    │   │   │      │         │          │            │          └───────── 1st source  = reg 1  → X1
                    │   │   │      │         │          │            └──────────────────── shift amount = 0
                    │   │   │      │         │          └───────────────────────────────── 2nd source  = reg 2  → X2
                    │   │   │      │         └──────────────────────────────────────────── shift type = LSL
                    │   │   │      └────────────────────────────────────────────────────── "this is add/sub shifted register"
                    │   │   └───────────────────────────────────────────────────────────── don't update flags (ADDS would set this)
                    │   └───────────────────────────────────────────────────────────────── 0 = ADD, 1 = SUB
                    └───────────────────────────────────────────────────────────────────── 1 = 64-bit (X regs), 0 = 32-bit (W regs)

            5-bit register operand fields at fixed positions within the instruction.
            For general-purpose register operands, the values 0-30 select one of 31 registers.

            For Advanced SIMD and floating-point register access and SVE scalable vector register access,
            a 5-bit register operand field selects one of 32 registers of the appropriate type.

            the manual says: All encodings that are not fully defined are described as unallocated.
            An attempt to execute an unallocated instruction is UNDEFINED

                             ┌─────────────────────────────┐
              32-bit word →  │ look ONLY at bits 28..25    │  (+ bit 31 for special cases)
                             └─────────────────────────────┘
                                       │
                 bits 28..25           ▼
                 ┌──────────┬──────────────────────────────────────────────┐
                 │  100x    │ Data Processing — IMMEDIATE  (ADD #imm, MOVZ…)│
                 │  101x    │ BRANCHES / EXCEPTIONS / SYSTEM (B, BL, RET…)  │
                 │  x101    │ Data Processing — REGISTER   (ADD X0,X1,X2…)  │
                 │  x111    │ FLOATING-POINT & SIMD        (FADD, ADD V0…)  │
                 │  x1x0    │ LOADS & STORES               (LDR, STR, LDP…) │
                 │  0000    │ Reserved/UDF (bit31=0) or SME (bit31=1)       │
                 │  0010    │ SVE (when bit31=1)                            │
                 └──────────┴──────────────────────────────────────────────┘
                                x = don't care, checked in priority order
                            */
            let word = u32::from_le_bytes(instruction.try_into().unwrap());

            // (note): refer to Arm A-profile A64 Instruction Set Architecture "Index by Encoding"
            let major_opcode = (word >> 25) & 0xF;

            // to extract bits:  value = (w >> lo) & ((1 << (hi - lo + 1)) - 1)
            // (todo): replace this with compile time lookup table
            match major_opcode {
                // (todo): bits 31..29 needs to be zero
                0b0 => {
                    //reserved
                    todo!()
                }

                // (todo): bits 31..29 needs to be 1 and add SME encodings
                0b0001 | 0b0011 => {
                    // unallocated
                    todo!()
                }

                0b0010 => {
                    // data processing
                    todo!()
                }

                0b1000 | 0b1001 => {
                    // data processing -- Immediate
                    // refer to Arm A-profile A64 Instruction Set Architecture "Index by Encoding" Data processing -- immediate
                    let op0 = ((word >> 29) & 0x03) as u8;
                    let op1 = ((word >> 22) & 0xf) as u8;

                    match (op0, op1) {
                        (0b11, 0b1110) | (0b11, 0b1111) => {
                            // data-processing (1 source immediate)
                            let sf = ((word >> 31) & 0x01) as u8;
                            let opc = ((word >> 21) & 0x03) as u8;
                            let imm = (word >> 5) as u16;
                            let rd = (word & 0x1F) as u8;

                            match (sf, opc, rd) {
                                (0b0, _, _) => {
                                    // unallocated
                                    todo!()
                                }

                                (0b1, 0b00, _) | (0b1, 0b01, _) if rd != 0b11111 => {
                                    todo!()
                                }

                                (0b1, 0b00, 0b11111) => {
                                    // AUTIASPPC
                                }

                                (0b1, 0b01, 0b11111) => {
                                    // 	AUTIBSPPC
                                }

                                (0b1, 10, _) | (0b1, 11, _) => {
                                    // unallocated
                                    todo!()
                                }

                                _ => todo!(),
                            }

                            todo!()
                        }

                        (_, 0b0000) | (_, 0b0011) => {
                            // PC-rel.addressing
                            todo!()
                        }

                        (_, 0b0100) | (_, 0b0101) => {
                            // add/subtract (immediate)
                            let sf = ((word >> 31) & 0x01) as u8;
                            let op = ((word >> 30) & 0x01) as u8;
                            let s = ((word >> 29) & 0x01) as u8;
                            let sh = ((word >> 22) & 0x01) as u8;
                            let imm12 = ((word >> 10) & 0xFFF) as u16;
                            let rn = ((word >> 5) & 0x1F) as u8;
                            let rd = (word & 0x1F) as u8;

                            match (sf, op, s) {
                                (0, 0, 0) => {
                                    // ADD (immediate) - 32-bit
                                    todo!()
                                }

                                (0, 0, 1) => {
                                    // ADDS (immediate) - 32-bit
                                    todo!()
                                }

                                (0, 1, 0) => {
                                    // SUB (immediate) - 32-bit
                                    todo!()
                                }

                                (0, 1, 1) => {
                                    // SUBS(immediate) - 32-bit
                                    todo!()
                                }

                                (1, 0, 0) => {
                                    // ADD (immediate) - 64-bit
                                    todo!()
                                }

                                (1, 0, 1) => {
                                    // ADDS (immediate) - 64-bit
                                    todo!()
                                }

                                (1, 1, 0) => {
                                    // SUB (immediate) - 64-bit
                                    todo!()
                                }

                                (1, 1, 1) => {
                                    // SUBS (immediate) - 64-bit
                                }

                                _ => unreachable!(),
                            }

                            todo!()
                        }

                        (_, 0b0110) => {
                            // add/subtract (immediate, with tags)
                            let sf = ((word >> 31) & 0x01) as u8;
                            let op = ((word >> 30) & 0x01) as u8;
                            let s = ((word >> 29) & 0x01) as u8;
                            let imm6 = ((word >> 16) & 0x3F) as u8;
                            let op3 = ((word >> 14) & 0x03) as u8;
                            let imm4 = ((word >> 10) & 0xF) as u8;
                            let rn = ((word >> 5) & 0x1F) as u8;
                            let rd = (word & 0xf) as u8;

                            match (sf, op, s) {
                                (0, _, _) => {
                                    // unallocated
                                    todo!()
                                }

                                (1, _, 1) => {
                                    // unallocated
                                    todo!()
                                }

                                (1, 0, 0) => {
                                    // ADDG
                                    todo!()
                                }

                                (1, 1, 0) => {
                                    // SUBG
                                    todo!()
                                }

                                _ => unreachable!(),
                            }

                            todo!()
                        }

                        (_, 0b0111) => {
                            // min/max (immediate, with tags)
                            let sf = ((word >> 31) & 0x01) as u8;
                            let op = ((word >> 30) & 0x01) as u8;
                            let s = ((word >> 29) & 0x01) as u8;
                            let opc = ((word >> 18) & 0xF) as u8;
                            let imm8 = ((word >> 10) & 0xFF) as u8;
                            let rn = ((word >> 5) & 0x1F) as u8;
                            let rd = (word & 0xf) as u8;

                            match (sf, op, s, opc) {
                                (_, 0b0, 0b0, _) if opc >> 2 != 0 => {
                                    // unallocated
                                    todo!()
                                }

                                (_, 0b0, 0b1, _) => {
                                    // unallocated
                                    todo!()
                                }

                                (_, 0b1, _, _) => {
                                    // unallocated
                                    todo!()
                                }

                                (0b0, 0b0, 0b0, 0b0000) => {
                                    // SMAX(immediate) - 32 bit
                                    todo!()
                                }

                                (0b0, 0b0, 0b0, 0b0001) => {
                                    // UMAX(immediate) - 32 bit
                                    todo!()
                                }

                                (0b0, 0b0, 0b0, 0b0010) => {
                                    // SMIN(immediate) - 32 bit
                                    todo!()
                                }

                                _ => unimplemented!(),
                            }

                            todo!()
                        }

                        (_, 0b1000) | (_, 0b1001) => {
                            // logical (immediate)
                            todo!()
                        }

                        (_, 0b1010) | (_, 0b1011) => {
                            // move wide (immediate)
                            todo!()
                        }

                        (_, 0b1100) | (_, 0b1101) => {
                            // bitfield
                            todo!()
                        }

                        (op, 0b1111) if op != 0b11 => {
                            // extract
                            todo!()
                        }

                        _ => todo!(),
                    }

                    todo!()
                }

                0b1010 | 0b1011 => {
                    // branches, exception generating and system insructions
                    todo!()
                }

                0b0101 | 0b1101 => {
                    // data processing -- register
                    todo!()
                }

                0b1111 | 0b0111 => {
                    // data processing -- scalar floating-point and advance SIMD
                    todo!()
                }

                0b0100 | 0b1110 => {
                    // loads and stores
                    todo!()
                }

                _ => todo!(),
            }
        }
    }
}
