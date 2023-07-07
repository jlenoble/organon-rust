use super::{ Disp0, Disp8, Disp16, Reg, MOD, Mod };
use super::{
    Opcode0,
    Opcode1,
    Opcode2,
    Opcode3,
    Opcode4,
    Opcode5,
    Opcode6,
    Opcode7,
    REGOPCODE,
    RegOpcode,
};

use super::super::super::{
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    operands::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI },
    operands::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7 },
    operands::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7 },
};

#[test]
fn test_mod_field() {
    assert_eq!(MOD::encode(Disp0), 0b00_000000);
    assert_eq!(MOD::encode(Disp8), 0b01_000000);
    assert_eq!(MOD::encode(Disp16), 0b10_000000);
    assert_eq!(MOD::encode(Reg), 0b11_000000);
}

#[test]
fn test_reg_opcode_field() {
    assert_eq!(REGOPCODE::encode(AL), 0b00_000_000);
    assert_eq!(REGOPCODE::encode(AX), 0b00_000_000);
    assert_eq!(REGOPCODE::encode(EAX), 0b00_000_000);
    assert_eq!(REGOPCODE::encode(MM0), 0b00_000_000);
    assert_eq!(REGOPCODE::encode(XMM0), 0b00_000_000);
    assert_eq!(REGOPCODE::encode(Opcode0), 0b00_000_000);

    assert_eq!(REGOPCODE::encode(CL), 0b00_001_000);
    assert_eq!(REGOPCODE::encode(CX), 0b00_001_000);
    assert_eq!(REGOPCODE::encode(ECX), 0b00_001_000);
    assert_eq!(REGOPCODE::encode(MM1), 0b00_001_000);
    assert_eq!(REGOPCODE::encode(XMM1), 0b00_001_000);
    assert_eq!(REGOPCODE::encode(Opcode1), 0b00_001_000);

    assert_eq!(REGOPCODE::encode(DL), 0b00_010_000);
    assert_eq!(REGOPCODE::encode(DX), 0b00_010_000);
    assert_eq!(REGOPCODE::encode(EDX), 0b00_010_000);
    assert_eq!(REGOPCODE::encode(MM2), 0b00_010_000);
    assert_eq!(REGOPCODE::encode(XMM2), 0b00_010_000);
    assert_eq!(REGOPCODE::encode(Opcode2), 0b00_010_000);

    assert_eq!(REGOPCODE::encode(BL), 0b00_011_000);
    assert_eq!(REGOPCODE::encode(BX), 0b00_011_000);
    assert_eq!(REGOPCODE::encode(EBX), 0b00_011_000);
    assert_eq!(REGOPCODE::encode(MM3), 0b00_011_000);
    assert_eq!(REGOPCODE::encode(XMM3), 0b00_011_000);
    assert_eq!(REGOPCODE::encode(Opcode3), 0b00_011_000);

    assert_eq!(REGOPCODE::encode(AH), 0b00_100_000);
    assert_eq!(REGOPCODE::encode(SP), 0b00_100_000);
    assert_eq!(REGOPCODE::encode(ESP), 0b00_100_000);
    assert_eq!(REGOPCODE::encode(MM4), 0b00_100_000);
    assert_eq!(REGOPCODE::encode(XMM4), 0b00_100_000);
    assert_eq!(REGOPCODE::encode(Opcode4), 0b00_100_000);

    assert_eq!(REGOPCODE::encode(CH), 0b00_101_000);
    assert_eq!(REGOPCODE::encode(BP), 0b00_101_000);
    assert_eq!(REGOPCODE::encode(EBP), 0b00_101_000);
    assert_eq!(REGOPCODE::encode(MM5), 0b00_101_000);
    assert_eq!(REGOPCODE::encode(XMM5), 0b00_101_000);
    assert_eq!(REGOPCODE::encode(Opcode5), 0b00_101_000);

    assert_eq!(REGOPCODE::encode(DH), 0b00_110_000);
    assert_eq!(REGOPCODE::encode(SI), 0b00_110_000);
    assert_eq!(REGOPCODE::encode(ESI), 0b00_110_000);
    assert_eq!(REGOPCODE::encode(MM6), 0b00_110_000);
    assert_eq!(REGOPCODE::encode(XMM6), 0b00_110_000);
    assert_eq!(REGOPCODE::encode(Opcode6), 0b00_110_000);

    assert_eq!(REGOPCODE::encode(BH), 0b00_111_000);
    assert_eq!(REGOPCODE::encode(DI), 0b00_111_000);
    assert_eq!(REGOPCODE::encode(EDI), 0b00_111_000);
    assert_eq!(REGOPCODE::encode(MM7), 0b00_111_000);
    assert_eq!(REGOPCODE::encode(XMM7), 0b00_111_000);
    assert_eq!(REGOPCODE::encode(Opcode7), 0b00_111_000);
}

#[ignore = "not implemented yet"]
#[test]
fn test_r_slash_m_field() {
    unimplemented!()
}

#[ignore = "not implemented yet"]
#[test]
fn test_mod_r_slash_m() {
    unimplemented!()
}
