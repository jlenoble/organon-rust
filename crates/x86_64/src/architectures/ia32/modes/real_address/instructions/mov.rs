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

#[cfg(test)]
use IA32Reg8::{ AL, CL, DL, BL, AH, CH, DH, BH };
#[cfg(test)]
use IA32Reg16::{ AX, CX, DX, BX, SP, BP, SI, DI };

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_gp_to_memory() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_segment_to_memory() {
    unimplemented!();
}

#[test]
fn real_address_mode_mov_instructions_from_gp_to_gp() {
    assert_eq!(0x88c0, MOV::mov(AL, AL));
    assert_eq!(0x88c1, MOV::mov(CL, AL));
    assert_eq!(0x88c2, MOV::mov(DL, AL));
    assert_eq!(0x88c3, MOV::mov(BL, AL));
    assert_eq!(0x88c4, MOV::mov(AH, AL));
    assert_eq!(0x88c5, MOV::mov(CH, AL));
    assert_eq!(0x88c6, MOV::mov(DH, AL));
    assert_eq!(0x88c7, MOV::mov(BH, AL));
    assert_eq!(0x88c8, MOV::mov(AL, CL));
    assert_eq!(0x88c9, MOV::mov(CL, CL));
    assert_eq!(0x88ca, MOV::mov(DL, CL));
    assert_eq!(0x88cb, MOV::mov(BL, CL));
    assert_eq!(0x88cc, MOV::mov(AH, CL));
    assert_eq!(0x88cd, MOV::mov(CH, CL));
    assert_eq!(0x88ce, MOV::mov(DH, CL));
    assert_eq!(0x88cf, MOV::mov(BH, CL));
    assert_eq!(0x88d0, MOV::mov(AL, DL));
    assert_eq!(0x88d1, MOV::mov(CL, DL));
    assert_eq!(0x88d2, MOV::mov(DL, DL));
    assert_eq!(0x88d3, MOV::mov(BL, DL));
    assert_eq!(0x88d4, MOV::mov(AH, DL));
    assert_eq!(0x88d5, MOV::mov(CH, DL));
    assert_eq!(0x88d6, MOV::mov(DH, DL));
    assert_eq!(0x88d7, MOV::mov(BH, DL));
    assert_eq!(0x88d8, MOV::mov(AL, BL));
    assert_eq!(0x88d9, MOV::mov(CL, BL));
    assert_eq!(0x88da, MOV::mov(DL, BL));
    assert_eq!(0x88db, MOV::mov(BL, BL));
    assert_eq!(0x88dc, MOV::mov(AH, BL));
    assert_eq!(0x88dd, MOV::mov(CH, BL));
    assert_eq!(0x88de, MOV::mov(DH, BL));
    assert_eq!(0x88df, MOV::mov(BH, BL));
    assert_eq!(0x88e0, MOV::mov(AL, AH));
    assert_eq!(0x88e1, MOV::mov(CL, AH));
    assert_eq!(0x88e2, MOV::mov(DL, AH));
    assert_eq!(0x88e3, MOV::mov(BL, AH));
    assert_eq!(0x88e4, MOV::mov(AH, AH));
    assert_eq!(0x88e5, MOV::mov(CH, AH));
    assert_eq!(0x88e6, MOV::mov(DH, AH));
    assert_eq!(0x88e7, MOV::mov(BH, AH));
    assert_eq!(0x88e8, MOV::mov(AL, CH));
    assert_eq!(0x88e9, MOV::mov(CL, CH));
    assert_eq!(0x88ea, MOV::mov(DL, CH));
    assert_eq!(0x88eb, MOV::mov(BL, CH));
    assert_eq!(0x88ec, MOV::mov(AH, CH));
    assert_eq!(0x88ed, MOV::mov(CH, CH));
    assert_eq!(0x88ee, MOV::mov(DH, CH));
    assert_eq!(0x88ef, MOV::mov(BH, CH));
    assert_eq!(0x88f0, MOV::mov(AL, DH));
    assert_eq!(0x88f1, MOV::mov(CL, DH));
    assert_eq!(0x88f2, MOV::mov(DL, DH));
    assert_eq!(0x88f3, MOV::mov(BL, DH));
    assert_eq!(0x88f4, MOV::mov(AH, DH));
    assert_eq!(0x88f5, MOV::mov(CH, DH));
    assert_eq!(0x88f6, MOV::mov(DH, DH));
    assert_eq!(0x88f7, MOV::mov(BH, DH));
    assert_eq!(0x88f8, MOV::mov(AL, BH));
    assert_eq!(0x88f9, MOV::mov(CL, BH));
    assert_eq!(0x88fa, MOV::mov(DL, BH));
    assert_eq!(0x88fb, MOV::mov(BL, BH));
    assert_eq!(0x88fc, MOV::mov(AH, BH));
    assert_eq!(0x88fd, MOV::mov(CH, BH));
    assert_eq!(0x88fe, MOV::mov(DH, BH));
    assert_eq!(0x88ff, MOV::mov(BH, BH));

    assert_eq!(0x89c0, MOV::mov(AX, AX));
    assert_eq!(0x89c1, MOV::mov(CX, AX));
    assert_eq!(0x89c2, MOV::mov(DX, AX));
    assert_eq!(0x89c3, MOV::mov(BX, AX));
    assert_eq!(0x89c4, MOV::mov(SP, AX));
    assert_eq!(0x89c5, MOV::mov(BP, AX));
    assert_eq!(0x89c6, MOV::mov(SI, AX));
    assert_eq!(0x89c7, MOV::mov(DI, AX));
    assert_eq!(0x89c8, MOV::mov(AX, CX));
    assert_eq!(0x89c9, MOV::mov(CX, CX));
    assert_eq!(0x89ca, MOV::mov(DX, CX));
    assert_eq!(0x89cb, MOV::mov(BX, CX));
    assert_eq!(0x89cc, MOV::mov(SP, CX));
    assert_eq!(0x89cd, MOV::mov(BP, CX));
    assert_eq!(0x89ce, MOV::mov(SI, CX));
    assert_eq!(0x89cf, MOV::mov(DI, CX));
    assert_eq!(0x89d0, MOV::mov(AX, DX));
    assert_eq!(0x89d1, MOV::mov(CX, DX));
    assert_eq!(0x89d2, MOV::mov(DX, DX));
    assert_eq!(0x89d3, MOV::mov(BX, DX));
    assert_eq!(0x89d4, MOV::mov(SP, DX));
    assert_eq!(0x89d5, MOV::mov(BP, DX));
    assert_eq!(0x89d6, MOV::mov(SI, DX));
    assert_eq!(0x89d7, MOV::mov(DI, DX));
    assert_eq!(0x89d8, MOV::mov(AX, BX));
    assert_eq!(0x89d9, MOV::mov(CX, BX));
    assert_eq!(0x89da, MOV::mov(DX, BX));
    assert_eq!(0x89db, MOV::mov(BX, BX));
    assert_eq!(0x89dc, MOV::mov(SP, BX));
    assert_eq!(0x89dd, MOV::mov(BP, BX));
    assert_eq!(0x89de, MOV::mov(SI, BX));
    assert_eq!(0x89df, MOV::mov(DI, BX));
    assert_eq!(0x89e0, MOV::mov(AX, SP));
    assert_eq!(0x89e1, MOV::mov(CX, SP));
    assert_eq!(0x89e2, MOV::mov(DX, SP));
    assert_eq!(0x89e3, MOV::mov(BX, SP));
    assert_eq!(0x89e4, MOV::mov(SP, SP));
    assert_eq!(0x89e5, MOV::mov(BP, SP));
    assert_eq!(0x89e6, MOV::mov(SI, SP));
    assert_eq!(0x89e7, MOV::mov(DI, SP));
    assert_eq!(0x89e8, MOV::mov(AX, BP));
    assert_eq!(0x89e9, MOV::mov(CX, BP));
    assert_eq!(0x89ea, MOV::mov(DX, BP));
    assert_eq!(0x89eb, MOV::mov(BX, BP));
    assert_eq!(0x89ec, MOV::mov(SP, BP));
    assert_eq!(0x89ed, MOV::mov(BP, BP));
    assert_eq!(0x89ee, MOV::mov(SI, BP));
    assert_eq!(0x89ef, MOV::mov(DI, BP));
    assert_eq!(0x89f0, MOV::mov(AX, SI));
    assert_eq!(0x89f1, MOV::mov(CX, SI));
    assert_eq!(0x89f2, MOV::mov(DX, SI));
    assert_eq!(0x89f3, MOV::mov(BX, SI));
    assert_eq!(0x89f4, MOV::mov(SP, SI));
    assert_eq!(0x89f5, MOV::mov(BP, SI));
    assert_eq!(0x89f6, MOV::mov(SI, SI));
    assert_eq!(0x89f7, MOV::mov(DI, SI));
    assert_eq!(0x89f8, MOV::mov(AX, DI));
    assert_eq!(0x89f9, MOV::mov(CX, DI));
    assert_eq!(0x89fa, MOV::mov(DX, DI));
    assert_eq!(0x89fb, MOV::mov(BX, DI));
    assert_eq!(0x89fc, MOV::mov(SP, DI));
    assert_eq!(0x89fd, MOV::mov(BP, DI));
    assert_eq!(0x89fe, MOV::mov(SI, DI));
    assert_eq!(0x89ff, MOV::mov(DI, DI));
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_memory_to_gp() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_memory_to_segment() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_gp_to_segment() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_segment_to_gp() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_gp_to_control() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_control_to_gp() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_gp_to_debug() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_debug_to_gp() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_gp() {
    unimplemented!();
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
