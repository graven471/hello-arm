/*
* hello ARM
* EL = CPL e.g. EL0 is same as Ring 0
* supported data types are same as x86, 8 bits (byte), 16 bits (halfword),
* mov dst, src: this is same as Intel x86 destination first then source
*
* Every instruction is 32-bits :) no x86 modr/m mess
*
* 32 bits (word), 64 bits (doubleword) and 128 bits (quadword)
*
* floating-point data types are: Half-precision, Single-precision, Double-precision
* BFloat16, 8-bit-floating-point
*
* since we are targetting AArch64 A profile the general-purpose register file contains 64-bit registers
* Many instructions can access these registers as 64-bit registers, or as 32-bit registers using only the bottom 32 bits.
*
* The SIMD&FP register file contains 128-bit registers:
* While the AArch64 vector registers support 128-bit vectors, the effective vector length can be 64-bits or 128-bits
* depending on the A64 instruction encoding used
*
* The Arm memory model
* - Generating an exception on unaligned memory access.
* - Restricting access by applications to specific areas of memory.
* - Translating virtual addresses (VAs) provided by executing instructions to physical addresses (PAs).
* - Altering the interpretation of multi-byte data between big-endian and little-endian.
* - Controlling the order of accesses to memory.
* - Controlling caches and address translation structures.
* - Synchronizing access to shared memory by multiple PEs.
* - Barriers that control and prevent speculative access to memory. (i love this)
*
* Registers in AArch64 Execution state
*
* R0-R30
* 31 general-purpose registers, R0 to R30. Each can be accessed as:
*
* - A 64-bit general-purpose register named X0 to X30. (same as r prefix registers like rax, rcx rdx etc)
* - A 32-bit general-purpose register named W0 to W30. (same as e prefix register like eax, ecx etc)
*
* mov x0, #0x12345678
* mov w0, #100
*
* for W class registers bits 32:63 is zero only bits 0:31 is used
*
* The X30 general-purpose register is used as the procedure call link register.
*
* SP (same as rsp)
* A 64-bit dedicated Stack Pointer register. The least significant 32 bits of the
* stack pointer can be accessed using the register name WSP.
*
* PC (same as rip)
* A 64-bit Program Counter holding the address of the current instruction.
* Software cannot write directly to the PC. It can be updated only on a branch, exception entry or exception return.
*
* V0-V31
* 32 SIMD&FP registers, V0 to V31. Each can be accessed as:
*
* - A 128-bit register named Q0 to Q31.
* - A 64-bit register named D0 to D31.
* - A 32-bit register named S0 to S31.
* - A 16-bit register named H0 to H31.
* - An 8-bit register named B0 to B31.
* - A 128-bit vector of elements (xmm/ymm stuffs)
* - A 64-bit vector of elements
*
* Where the number of bits described by a register name does not occupy an entire SIMD&FP register,
* it refers to the least significant bits
*
* just like V0 and W0 its the same physical register we are just slicing
*
* Z0-Z31
* 32 SVE scalable vector registers, Z0 to Z31, of equal length. Each register can be accessed as:
*
* A configurable-length vector of elements. The length, VL, is a power of two,
* from a minimum of 128 bits to an IMPLEMENTATION DEFINED maximum no greater than 2048 bits
*
* A SIMD&FP register, as described in V0-V31. Bits[127:0] of each Zn register
* hold the correspondingly numbered V0-V31 SIMD&FP register
*
* P0-P15
* 16 SVE predicate registers, named P0 to P15. Each SVE predicate register holds one bit for each byte
* of an SVE scalar vector register.
*
* FFR
* The dedicated SVE First Fault Register that has the same size and format as the SVE predicate registers, P0-P15.
*
* ZA
* Architectural state capable of holding a two-dimensional array of bytes.
*
* ZT0
* A 512-bit SME2 lookup table register.
*
* Pseudocode description of registers in AArch64 state
*
* The setter or assignment accessor, for example “X{width}(num) = result”, is used for register writes.
*
* The getter or non-assignment accessor, “operand = X{width}(num)”, is used for register reads
*
* Read the specified register and put its value into operand.
*
* X{width}(num) = result -> Write result into the specified register
*
* add x0, x1, x2
* operand1 = X[64](1);
* operand2 = X[64](2);
* result = operand1 + operand2;
* X[64](0) = result;
*
* The num parameter specifies the register number to be accessed within the named register file,
* and width specifies the bit width of the register access.
*
* For the general-purpose register X{width}() accessor, register number 31 accesses the zero register
* which reads as zero and ignores writes.
*
* X: access the general-purpose register file.
* width: Number of bits we want to access.
* num: Register number.
*
* X[64](0) -> to access all bits of X0.
*
* For other register accessors, such as the SIMD&FP V{width}() accessors,
* there is no special interpretation of the register number.
*
* The SP{width}() accessors have no parameters and are used to read or write the currently
* selected 64-bit stack pointer at the current Exception level.
*
* The PC64{width}() accessor has no argument list and is used to read the 64-bit Program Counter.
*
* The Vpart{width}() accessors are used to perform a partial read or write of an Advanced SIMD vector register,
* using an additional part parameter to select which half of the vector register to access.
*
* The Z{width}(), V{width}(), and Vpart{width}() accessors read and write the same underlying vector register file.
*
* AArch64 calling convention (AAPCS64)
* Arguments: x0-x7 (first 8 args rest go on stack) similar to rdi/rsi/rdx/rcx/r8/r9
*
* Return value: x0 (or x0+x1 for 128-bit)
*
* Caller-saved(scratch): x0-x15, don't need to preserve these across calls
*
* Calle-saved: x19-x28: if my function uses these i must save/restore them
*
* x29: frame pointer
* x30: link register (return address, bl stores here)
* sp: stack pointer, must be 16-byte aligned at all time
*/

#![allow(unused)]

pub mod elf;
pub mod error;

use std::{cmp::Ordering, f64::consts::E, fs::File, io::Read};

use crate::{
    elf::types::{EI_CLASS, EI_VERSION, ELF_MAGIC, ELFCLASS64, EV_CURRENT},
    error::{AelfError, ElfError},
};

pub fn decode_elf(name: &str) -> Result<(), AelfError> {
    // check does file exists, then validate is this a valid elf
    // then check is machine == AArch64 then read header and validate
    // elf file format:

    let mut file = File::open(name)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    if data.len() < 64 {
        return Err(AelfError::Elf(ElfError::InvalidHeader(
            "a valid ELF64 file must be alteast 64-bytes",
        )));
    }

    let elf_hdr = &data[0..64];

    // ============== valid first 16-bytes EI_NIDENT =================
    if elf_hdr[0..4] != ELF_MAGIC {
        return Err(AelfError::Elf(ElfError::InvalidMagic));
    }

    if elf_hdr[EI_CLASS] != ELFCLASS64 {
        return Err(AelfError::Elf(ElfError::UnsupportedClass(
            elf_hdr[EI_CLASS],
        )));
    }

    if elf_hdr[EI_VERSION] != EV_CURRENT {
        return Err(AelfError::Elf(ElfError::UnsupportedVersion(
            elf_hdr[EI_VERSION] as u32,
        )));
    }

    Ok(())
}
