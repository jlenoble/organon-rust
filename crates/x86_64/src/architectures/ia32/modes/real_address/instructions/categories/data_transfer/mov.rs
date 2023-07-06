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
    // Move between 8-bit registers
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

    // Move between 16-bit registers
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

#[test]
fn real_address_mode_mov_instructions_from_immediate_to_gp() {
    // Move values to 8-bit registers
    assert_eq!(0xb000, MOV::mov(AL, Imm8(0)));
    assert_eq!(0xb100, MOV::mov(CL, Imm8(0)));
    assert_eq!(0xb200, MOV::mov(DL, Imm8(0)));
    assert_eq!(0xb300, MOV::mov(BL, Imm8(0)));
    assert_eq!(0xb400, MOV::mov(AH, Imm8(0)));
    assert_eq!(0xb500, MOV::mov(CH, Imm8(0)));
    assert_eq!(0xb600, MOV::mov(DH, Imm8(0)));
    assert_eq!(0xb700, MOV::mov(BH, Imm8(0)));

    assert_eq!(0xb00c, MOV::mov(AL, Imm8(12)));
    assert_eq!(0xb10c, MOV::mov(CL, Imm8(12)));
    assert_eq!(0xb20c, MOV::mov(DL, Imm8(12)));
    assert_eq!(0xb30c, MOV::mov(BL, Imm8(12)));
    assert_eq!(0xb40c, MOV::mov(AH, Imm8(12)));
    assert_eq!(0xb50c, MOV::mov(CH, Imm8(12)));
    assert_eq!(0xb60c, MOV::mov(DH, Imm8(12)));
    assert_eq!(0xb70c, MOV::mov(BH, Imm8(12)));

    assert_eq!(0xb010, MOV::mov(AL, Imm8(16)));
    assert_eq!(0xb110, MOV::mov(CL, Imm8(16)));
    assert_eq!(0xb210, MOV::mov(DL, Imm8(16)));
    assert_eq!(0xb310, MOV::mov(BL, Imm8(16)));
    assert_eq!(0xb410, MOV::mov(AH, Imm8(16)));
    assert_eq!(0xb510, MOV::mov(CH, Imm8(16)));
    assert_eq!(0xb610, MOV::mov(DH, Imm8(16)));
    assert_eq!(0xb710, MOV::mov(BH, Imm8(16)));

    assert_eq!(0xb096, MOV::mov(AL, Imm8(150)));
    assert_eq!(0xb196, MOV::mov(CL, Imm8(150)));
    assert_eq!(0xb296, MOV::mov(DL, Imm8(150)));
    assert_eq!(0xb396, MOV::mov(BL, Imm8(150)));
    assert_eq!(0xb496, MOV::mov(AH, Imm8(150)));
    assert_eq!(0xb596, MOV::mov(CH, Imm8(150)));
    assert_eq!(0xb696, MOV::mov(DH, Imm8(150)));
    assert_eq!(0xb796, MOV::mov(BH, Imm8(150)));

    // Move values to 16-bit registers
    assert_eq!(0xb80000, MOV::mov(AX, Imm16(0)));
    assert_eq!(0xb90000, MOV::mov(CX, Imm16(0)));
    assert_eq!(0xba0000, MOV::mov(DX, Imm16(0)));
    assert_eq!(0xbb0000, MOV::mov(BX, Imm16(0)));
    assert_eq!(0xbc0000, MOV::mov(SP, Imm16(0)));
    assert_eq!(0xbd0000, MOV::mov(BP, Imm16(0)));
    assert_eq!(0xbe0000, MOV::mov(SI, Imm16(0)));
    assert_eq!(0xbf0000, MOV::mov(DI, Imm16(0)));

    assert_eq!(0xb80c00, MOV::mov(AX, Imm16(12)));
    assert_eq!(0xb90c00, MOV::mov(CX, Imm16(12)));
    assert_eq!(0xba0c00, MOV::mov(DX, Imm16(12)));
    assert_eq!(0xbb0c00, MOV::mov(BX, Imm16(12)));
    assert_eq!(0xbc0c00, MOV::mov(SP, Imm16(12)));
    assert_eq!(0xbd0c00, MOV::mov(BP, Imm16(12)));
    assert_eq!(0xbe0c00, MOV::mov(SI, Imm16(12)));
    assert_eq!(0xbf0c00, MOV::mov(DI, Imm16(12)));

    assert_eq!(0xb81000, MOV::mov(AX, Imm16(16)));
    assert_eq!(0xb91000, MOV::mov(CX, Imm16(16)));
    assert_eq!(0xba1000, MOV::mov(DX, Imm16(16)));
    assert_eq!(0xbb1000, MOV::mov(BX, Imm16(16)));
    assert_eq!(0xbc1000, MOV::mov(SP, Imm16(16)));
    assert_eq!(0xbd1000, MOV::mov(BP, Imm16(16)));
    assert_eq!(0xbe1000, MOV::mov(SI, Imm16(16)));
    assert_eq!(0xbf1000, MOV::mov(DI, Imm16(16)));

    assert_eq!(0xb89600, MOV::mov(AX, Imm16(150)));
    assert_eq!(0xb99600, MOV::mov(CX, Imm16(150)));
    assert_eq!(0xba9600, MOV::mov(DX, Imm16(150)));
    assert_eq!(0xbb9600, MOV::mov(BX, Imm16(150)));
    assert_eq!(0xbc9600, MOV::mov(SP, Imm16(150)));
    assert_eq!(0xbd9600, MOV::mov(BP, Imm16(150)));
    assert_eq!(0xbe9600, MOV::mov(SI, Imm16(150)));
    assert_eq!(0xbf9600, MOV::mov(DI, Imm16(150)));

    assert_eq!(0xb8e803, MOV::mov(AX, Imm16(1000)));
    assert_eq!(0xb9e803, MOV::mov(CX, Imm16(1000)));
    assert_eq!(0xbae803, MOV::mov(DX, Imm16(1000)));
    assert_eq!(0xbbe803, MOV::mov(BX, Imm16(1000)));
    assert_eq!(0xbce803, MOV::mov(SP, Imm16(1000)));
    assert_eq!(0xbde803, MOV::mov(BP, Imm16(1000)));
    assert_eq!(0xbee803, MOV::mov(SI, Imm16(1000)));
    assert_eq!(0xbfe803, MOV::mov(DI, Imm16(1000)));

    assert_eq!(0xb80080, MOV::mov(AX, Imm16(32768)));
    assert_eq!(0xb90080, MOV::mov(CX, Imm16(32768)));
    assert_eq!(0xba0080, MOV::mov(DX, Imm16(32768)));
    assert_eq!(0xbb0080, MOV::mov(BX, Imm16(32768)));
    assert_eq!(0xbc0080, MOV::mov(SP, Imm16(32768)));
    assert_eq!(0xbd0080, MOV::mov(BP, Imm16(32768)));
    assert_eq!(0xbe0080, MOV::mov(SI, Imm16(32768)));
    assert_eq!(0xbf0080, MOV::mov(DI, Imm16(32768)));

    assert_eq!(0xb850c3, MOV::mov(AX, Imm16(50000)));
    assert_eq!(0xb950c3, MOV::mov(CX, Imm16(50000)));
    assert_eq!(0xba50c3, MOV::mov(DX, Imm16(50000)));
    assert_eq!(0xbb50c3, MOV::mov(BX, Imm16(50000)));
    assert_eq!(0xbc50c3, MOV::mov(SP, Imm16(50000)));
    assert_eq!(0xbd50c3, MOV::mov(BP, Imm16(50000)));
    assert_eq!(0xbe50c3, MOV::mov(SI, Imm16(50000)));
    assert_eq!(0xbf50c3, MOV::mov(DI, Imm16(50000)));
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
