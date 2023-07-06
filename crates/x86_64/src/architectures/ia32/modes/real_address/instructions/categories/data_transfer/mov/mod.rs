//! IA-32 real-address mode MOV and CMOVcc instructions
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.1.1*

use crate::registers::{ IA32Reg8, IA32Reg16 };
use super::super::super::operands::{ Imm8, Imm16 };

/// Trait encompasSIng all IA-32 real-address mode MOV family instructions
pub trait Mov<Dest, Src> {
    /// generic real-address mode MOV instruction
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV*
    fn mov(dest: Dest, src: Src) -> u32;
}

/// IA-32 real-address mode MOV family instruction implementations
pub struct MOV;

macro_rules! impl_instruction_from_gp_to_gp {
    ($mnemonics:ident, $op_code_base:literal, $arg_type:ty) => {
        impl Mov<$arg_type, $arg_type> for MOV {
            fn $mnemonics(dest: $arg_type, src: $arg_type)-> u32 {
                $op_code_base + (dest as u32) + ((src as u32) << 3)
            }
        }
    };
}

macro_rules! impl_instruction_from_imm_to_gp8 {
    ($mnemonics:ident, $op_code_base:literal, $arg_type:ty) => {
        impl Mov<$arg_type, Imm8> for MOV {
            fn $mnemonics(dest: $arg_type, src: Imm8)-> u32 {
                $op_code_base + (0x100 * (dest as u32)) + (src.0 as u32)
            }
        }
    };
}

macro_rules! impl_instruction_from_imm_to_gp16 {
    ($mnemonics:ident, $op_code_base:literal, $arg_type:ty) => {
        impl Mov<$arg_type, Imm16> for MOV {
            fn $mnemonics(dest: $arg_type, src: Imm16)-> u32 {
                $op_code_base + (0x10000 * (dest as u32)) + 0x80000 + (((src.0 & 0xff00) >> 8) as u32) + (((src.0 & 0xff) << 8) as u32)
            }
        }
    };
}

impl_instruction_from_gp_to_gp!(mov, 0x88c0, IA32Reg8);
impl_instruction_from_gp_to_gp!(mov, 0x89c0, IA32Reg16);

impl_instruction_from_imm_to_gp8!(mov, 0xb000, IA32Reg8);
impl_instruction_from_imm_to_gp16!(mov, 0xb00000, IA32Reg16);

#[cfg(test)]
mod tests;
