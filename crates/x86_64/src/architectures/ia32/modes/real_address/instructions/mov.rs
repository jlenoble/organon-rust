//! IA-32 real-address mode MOV and CMOVcc instructions
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.1.1*

use crate::registers::{ IA32Reg8, IA32Reg16 };

/// Trait encompassing all IA-32 real-address mode MOV family instructions
pub trait Mov<T> {
    /// generic real-address mode MOV instruction
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV*
    fn mov(dest: T, src: T) -> u16;
}

/// IA-32 real-address mode MOV family instruction implementations
pub struct MOV;

macro_rules! impl_instruction {
    ($mnemonics:ident, $op_code_base:literal, $arg_type:ty) => {
        impl Mov<$arg_type> for MOV {
            fn $mnemonics(dest: $arg_type, src: $arg_type)-> u16 {
                $op_code_base + (dest as u16) + ((src as u16) << 3)
            }
        }
    };
}

impl_instruction!(mov, 0x88c0, IA32Reg8);
impl_instruction!(mov, 0x89c0, IA32Reg16);

// The following tests follow *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Table 7.1*

#[test]
fn real_address_mode_mov_instructions_from_gp_to_memory() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_segment_to_memory() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_gp_to_gp() {
    use strum::IntoEnumIterator;

    let mut op_code: u16 = 0x88c0;

    for src in IA32Reg8::iter() {
        for dest in IA32Reg8::iter() {
            assert_eq!(MOV::mov(dest, src), op_code);
            op_code += 1;
        }
    }

    let mut op_code: u16 = 0x89c0;

    for src in IA32Reg16::iter() {
        for dest in IA32Reg16::iter() {
            assert_eq!(MOV::mov(dest, src), op_code);
            op_code += 1;
        }
    }
}

#[test]
fn real_address_mode_mov_instructions_from_memory_to_gp() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_memory_to_segment() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_gp_to_segment() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_segment_to_gp() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_gp_to_control() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_control_to_gp() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_gp_to_debug() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_debug_to_gp() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_immediate_to_gp() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
