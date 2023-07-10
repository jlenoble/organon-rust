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
use super::{ BXpSI, BXpDI, BPpSI, BPpDI, RSLASHM, RSlashM };
use super::{ MODRM, ModRM };

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

#[test]
fn test_r_slash_m_field() {
    assert_eq!(RSLASHM::encode([BXpSI]), 0b00000_000);
    assert_eq!(RSLASHM::encode([BXpDI]), 0b00000_001);
    assert_eq!(RSLASHM::encode([BPpSI]), 0b00000_010);
    assert_eq!(RSLASHM::encode([BPpDI]), 0b00000_011);
    assert_eq!(RSLASHM::encode([SI]), 0b00000_100);
    assert_eq!(RSLASHM::encode([DI]), 0b00000_101);
    assert_eq!(RSLASHM::encode(Disp16), 0b00000_110);
    assert_eq!(RSLASHM::encode([BP]), 0b00000_110);
    assert_eq!(RSLASHM::encode([BX]), 0b00000_111);

    assert_eq!(RSLASHM::encode(AL), 0b00000_000);
    assert_eq!(RSLASHM::encode(AX), 0b00000_000);
    assert_eq!(RSLASHM::encode(EAX), 0b00000_000);
    assert_eq!(RSLASHM::encode(MM0), 0b00000_000);
    assert_eq!(RSLASHM::encode(XMM0), 0b00000_000);

    assert_eq!(RSLASHM::encode(CL), 0b00000_001);
    assert_eq!(RSLASHM::encode(CX), 0b00000_001);
    assert_eq!(RSLASHM::encode(ECX), 0b00000_001);
    assert_eq!(RSLASHM::encode(MM1), 0b00000_001);
    assert_eq!(RSLASHM::encode(XMM1), 0b00000_001);

    assert_eq!(RSLASHM::encode(DL), 0b00000_010);
    assert_eq!(RSLASHM::encode(DX), 0b00000_010);
    assert_eq!(RSLASHM::encode(EDX), 0b00000_010);
    assert_eq!(RSLASHM::encode(MM2), 0b00000_010);
    assert_eq!(RSLASHM::encode(XMM2), 0b00000_010);

    assert_eq!(RSLASHM::encode(BL), 0b00000_011);
    assert_eq!(RSLASHM::encode(BX), 0b00000_011);
    assert_eq!(RSLASHM::encode(EBX), 0b00000_011);
    assert_eq!(RSLASHM::encode(MM3), 0b00000_011);
    assert_eq!(RSLASHM::encode(XMM3), 0b00000_011);

    assert_eq!(RSLASHM::encode(AH), 0b00000_100);
    assert_eq!(RSLASHM::encode(SP), 0b00000_100);
    assert_eq!(RSLASHM::encode(ESP), 0b00000_100);
    assert_eq!(RSLASHM::encode(MM4), 0b00000_100);
    assert_eq!(RSLASHM::encode(XMM4), 0b00000_100);

    assert_eq!(RSLASHM::encode(CH), 0b00000_101);
    assert_eq!(RSLASHM::encode(BP), 0b00000_101);
    assert_eq!(RSLASHM::encode(EBP), 0b00000_101);
    assert_eq!(RSLASHM::encode(MM5), 0b00000_101);
    assert_eq!(RSLASHM::encode(XMM5), 0b00000_101);

    assert_eq!(RSLASHM::encode(DH), 0b00000_110);
    assert_eq!(RSLASHM::encode(SI), 0b00000_110);
    assert_eq!(RSLASHM::encode(ESI), 0b00000_110);
    assert_eq!(RSLASHM::encode(MM6), 0b00000_110);
    assert_eq!(RSLASHM::encode(XMM6), 0b00000_110);

    assert_eq!(RSLASHM::encode(BH), 0b00000_111);
    assert_eq!(RSLASHM::encode(DI), 0b00000_111);
    assert_eq!(RSLASHM::encode(EDI), 0b00000_111);
    assert_eq!(RSLASHM::encode(MM7), 0b00000_111);
    assert_eq!(RSLASHM::encode(XMM7), 0b00000_111);
}

#[test]
fn test_mod_r_slash_m() {
    assert_eq!(MODRM::encode(Reg, AL, AL), 0b11_000_000);
    assert_eq!(MODRM::encode(Reg, AX, AX), 0b11_000_000);
    assert_eq!(MODRM::encode(Reg, EAX, EAX), 0b11_000_000);
    assert_eq!(MODRM::encode(Reg, MM0, MM0), 0b11_000_000);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM0), 0b11_000_000);
    assert_eq!(MODRM::encode(Reg, CL, AL), 0b11_001_000);
    assert_eq!(MODRM::encode(Reg, CX, AX), 0b11_001_000);
    assert_eq!(MODRM::encode(Reg, ECX, EAX), 0b11_001_000);
    assert_eq!(MODRM::encode(Reg, MM1, MM0), 0b11_001_000);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM0), 0b11_001_000);
    assert_eq!(MODRM::encode(Reg, DL, AL), 0b11_010_000);
    assert_eq!(MODRM::encode(Reg, DX, AX), 0b11_010_000);
    assert_eq!(MODRM::encode(Reg, EDX, EAX), 0b11_010_000);
    assert_eq!(MODRM::encode(Reg, MM2, MM0), 0b11_010_000);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM0), 0b11_010_000);
    assert_eq!(MODRM::encode(Reg, BL, AL), 0b11_011_000);
    assert_eq!(MODRM::encode(Reg, BX, AX), 0b11_011_000);
    assert_eq!(MODRM::encode(Reg, EBX, EAX), 0b11_011_000);
    assert_eq!(MODRM::encode(Reg, MM3, MM0), 0b11_011_000);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM0), 0b11_011_000);
    assert_eq!(MODRM::encode(Reg, AH, AL), 0b11_100_000);
    assert_eq!(MODRM::encode(Reg, SP, AX), 0b11_100_000);
    assert_eq!(MODRM::encode(Reg, ESP, EAX), 0b11_100_000);
    assert_eq!(MODRM::encode(Reg, MM4, MM0), 0b11_100_000);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM0), 0b11_100_000);
    assert_eq!(MODRM::encode(Reg, CH, AL), 0b11_101_000);
    assert_eq!(MODRM::encode(Reg, BP, AX), 0b11_101_000);
    assert_eq!(MODRM::encode(Reg, EBP, EAX), 0b11_101_000);
    assert_eq!(MODRM::encode(Reg, MM5, MM0), 0b11_101_000);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM0), 0b11_101_000);
    assert_eq!(MODRM::encode(Reg, DH, AL), 0b11_110_000);
    assert_eq!(MODRM::encode(Reg, SI, AX), 0b11_110_000);
    assert_eq!(MODRM::encode(Reg, ESI, EAX), 0b11_110_000);
    assert_eq!(MODRM::encode(Reg, MM6, MM0), 0b11_110_000);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM0), 0b11_110_000);
    assert_eq!(MODRM::encode(Reg, BH, AL), 0b11_111_000);
    assert_eq!(MODRM::encode(Reg, DI, AX), 0b11_111_000);
    assert_eq!(MODRM::encode(Reg, EDI, EAX), 0b11_111_000);
    assert_eq!(MODRM::encode(Reg, MM7, MM0), 0b11_111_000);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM0), 0b11_111_000);

    assert_eq!(MODRM::encode(Reg, AL, CL), 0b11_000_001);
    assert_eq!(MODRM::encode(Reg, AX, CX), 0b11_000_001);
    assert_eq!(MODRM::encode(Reg, EAX, ECX), 0b11_000_001);
    assert_eq!(MODRM::encode(Reg, MM0, MM1), 0b11_000_001);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM1), 0b11_000_001);
    assert_eq!(MODRM::encode(Reg, CL, CL), 0b11_001_001);
    assert_eq!(MODRM::encode(Reg, CX, CX), 0b11_001_001);
    assert_eq!(MODRM::encode(Reg, ECX, ECX), 0b11_001_001);
    assert_eq!(MODRM::encode(Reg, MM1, MM1), 0b11_001_001);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM1), 0b11_001_001);
    assert_eq!(MODRM::encode(Reg, DL, CL), 0b11_010_001);
    assert_eq!(MODRM::encode(Reg, DX, CX), 0b11_010_001);
    assert_eq!(MODRM::encode(Reg, EDX, ECX), 0b11_010_001);
    assert_eq!(MODRM::encode(Reg, MM2, MM1), 0b11_010_001);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM1), 0b11_010_001);
    assert_eq!(MODRM::encode(Reg, BL, CL), 0b11_011_001);
    assert_eq!(MODRM::encode(Reg, BX, CX), 0b11_011_001);
    assert_eq!(MODRM::encode(Reg, EBX, ECX), 0b11_011_001);
    assert_eq!(MODRM::encode(Reg, MM3, MM1), 0b11_011_001);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM1), 0b11_011_001);
    assert_eq!(MODRM::encode(Reg, AH, CL), 0b11_100_001);
    assert_eq!(MODRM::encode(Reg, SP, CX), 0b11_100_001);
    assert_eq!(MODRM::encode(Reg, ESP, ECX), 0b11_100_001);
    assert_eq!(MODRM::encode(Reg, MM4, MM1), 0b11_100_001);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM1), 0b11_100_001);
    assert_eq!(MODRM::encode(Reg, CH, CL), 0b11_101_001);
    assert_eq!(MODRM::encode(Reg, BP, CX), 0b11_101_001);
    assert_eq!(MODRM::encode(Reg, EBP, ECX), 0b11_101_001);
    assert_eq!(MODRM::encode(Reg, MM5, MM1), 0b11_101_001);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM1), 0b11_101_001);
    assert_eq!(MODRM::encode(Reg, DH, CL), 0b11_110_001);
    assert_eq!(MODRM::encode(Reg, SI, CX), 0b11_110_001);
    assert_eq!(MODRM::encode(Reg, ESI, ECX), 0b11_110_001);
    assert_eq!(MODRM::encode(Reg, MM6, MM1), 0b11_110_001);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM1), 0b11_110_001);
    assert_eq!(MODRM::encode(Reg, BH, CL), 0b11_111_001);
    assert_eq!(MODRM::encode(Reg, DI, CX), 0b11_111_001);
    assert_eq!(MODRM::encode(Reg, EDI, ECX), 0b11_111_001);
    assert_eq!(MODRM::encode(Reg, MM7, MM1), 0b11_111_001);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM1), 0b11_111_001);

    assert_eq!(MODRM::encode(Reg, AL, DL), 0b11_000_010);
    assert_eq!(MODRM::encode(Reg, AX, DX), 0b11_000_010);
    assert_eq!(MODRM::encode(Reg, EAX, EDX), 0b11_000_010);
    assert_eq!(MODRM::encode(Reg, MM0, MM2), 0b11_000_010);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM2), 0b11_000_010);
    assert_eq!(MODRM::encode(Reg, CL, DL), 0b11_001_010);
    assert_eq!(MODRM::encode(Reg, CX, DX), 0b11_001_010);
    assert_eq!(MODRM::encode(Reg, ECX, EDX), 0b11_001_010);
    assert_eq!(MODRM::encode(Reg, MM1, MM2), 0b11_001_010);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM2), 0b11_001_010);
    assert_eq!(MODRM::encode(Reg, DL, DL), 0b11_010_010);
    assert_eq!(MODRM::encode(Reg, DX, DX), 0b11_010_010);
    assert_eq!(MODRM::encode(Reg, EDX, EDX), 0b11_010_010);
    assert_eq!(MODRM::encode(Reg, MM2, MM2), 0b11_010_010);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM2), 0b11_010_010);
    assert_eq!(MODRM::encode(Reg, BL, DL), 0b11_011_010);
    assert_eq!(MODRM::encode(Reg, BX, DX), 0b11_011_010);
    assert_eq!(MODRM::encode(Reg, EBX, EDX), 0b11_011_010);
    assert_eq!(MODRM::encode(Reg, MM3, MM2), 0b11_011_010);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM2), 0b11_011_010);
    assert_eq!(MODRM::encode(Reg, AH, DL), 0b11_100_010);
    assert_eq!(MODRM::encode(Reg, SP, DX), 0b11_100_010);
    assert_eq!(MODRM::encode(Reg, ESP, EDX), 0b11_100_010);
    assert_eq!(MODRM::encode(Reg, MM4, MM2), 0b11_100_010);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM2), 0b11_100_010);
    assert_eq!(MODRM::encode(Reg, CH, DL), 0b11_101_010);
    assert_eq!(MODRM::encode(Reg, BP, DX), 0b11_101_010);
    assert_eq!(MODRM::encode(Reg, EBP, EDX), 0b11_101_010);
    assert_eq!(MODRM::encode(Reg, MM5, MM2), 0b11_101_010);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM2), 0b11_101_010);
    assert_eq!(MODRM::encode(Reg, DH, DL), 0b11_110_010);
    assert_eq!(MODRM::encode(Reg, SI, DX), 0b11_110_010);
    assert_eq!(MODRM::encode(Reg, ESI, EDX), 0b11_110_010);
    assert_eq!(MODRM::encode(Reg, MM6, MM2), 0b11_110_010);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM2), 0b11_110_010);
    assert_eq!(MODRM::encode(Reg, BH, DL), 0b11_111_010);
    assert_eq!(MODRM::encode(Reg, DI, DX), 0b11_111_010);
    assert_eq!(MODRM::encode(Reg, EDI, EDX), 0b11_111_010);
    assert_eq!(MODRM::encode(Reg, MM7, MM2), 0b11_111_010);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM2), 0b11_111_010);

    assert_eq!(MODRM::encode(Reg, AL, BL), 0b11_000_011);
    assert_eq!(MODRM::encode(Reg, AX, BX), 0b11_000_011);
    assert_eq!(MODRM::encode(Reg, EAX, EBX), 0b11_000_011);
    assert_eq!(MODRM::encode(Reg, MM0, MM3), 0b11_000_011);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM3), 0b11_000_011);
    assert_eq!(MODRM::encode(Reg, CL, BL), 0b11_001_011);
    assert_eq!(MODRM::encode(Reg, CX, BX), 0b11_001_011);
    assert_eq!(MODRM::encode(Reg, ECX, EBX), 0b11_001_011);
    assert_eq!(MODRM::encode(Reg, MM1, MM3), 0b11_001_011);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM3), 0b11_001_011);
    assert_eq!(MODRM::encode(Reg, DL, BL), 0b11_010_011);
    assert_eq!(MODRM::encode(Reg, DX, BX), 0b11_010_011);
    assert_eq!(MODRM::encode(Reg, EDX, EBX), 0b11_010_011);
    assert_eq!(MODRM::encode(Reg, MM2, MM3), 0b11_010_011);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM3), 0b11_010_011);
    assert_eq!(MODRM::encode(Reg, BL, BL), 0b11_011_011);
    assert_eq!(MODRM::encode(Reg, BX, BX), 0b11_011_011);
    assert_eq!(MODRM::encode(Reg, EBX, EBX), 0b11_011_011);
    assert_eq!(MODRM::encode(Reg, MM3, MM3), 0b11_011_011);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM3), 0b11_011_011);
    assert_eq!(MODRM::encode(Reg, AH, BL), 0b11_100_011);
    assert_eq!(MODRM::encode(Reg, SP, BX), 0b11_100_011);
    assert_eq!(MODRM::encode(Reg, ESP, EBX), 0b11_100_011);
    assert_eq!(MODRM::encode(Reg, MM4, MM3), 0b11_100_011);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM3), 0b11_100_011);
    assert_eq!(MODRM::encode(Reg, CH, BL), 0b11_101_011);
    assert_eq!(MODRM::encode(Reg, BP, BX), 0b11_101_011);
    assert_eq!(MODRM::encode(Reg, EBP, EBX), 0b11_101_011);
    assert_eq!(MODRM::encode(Reg, MM5, MM3), 0b11_101_011);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM3), 0b11_101_011);
    assert_eq!(MODRM::encode(Reg, DH, BL), 0b11_110_011);
    assert_eq!(MODRM::encode(Reg, SI, BX), 0b11_110_011);
    assert_eq!(MODRM::encode(Reg, ESI, EBX), 0b11_110_011);
    assert_eq!(MODRM::encode(Reg, MM6, MM3), 0b11_110_011);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM3), 0b11_110_011);
    assert_eq!(MODRM::encode(Reg, BH, BL), 0b11_111_011);
    assert_eq!(MODRM::encode(Reg, DI, BX), 0b11_111_011);
    assert_eq!(MODRM::encode(Reg, EDI, EBX), 0b11_111_011);
    assert_eq!(MODRM::encode(Reg, MM7, MM3), 0b11_111_011);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM3), 0b11_111_011);

    assert_eq!(MODRM::encode(Reg, AL, AH), 0b11_000_100);
    assert_eq!(MODRM::encode(Reg, AX, SP), 0b11_000_100);
    assert_eq!(MODRM::encode(Reg, EAX, ESP), 0b11_000_100);
    assert_eq!(MODRM::encode(Reg, MM0, MM4), 0b11_000_100);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM4), 0b11_000_100);
    assert_eq!(MODRM::encode(Reg, CL, AH), 0b11_001_100);
    assert_eq!(MODRM::encode(Reg, CX, SP), 0b11_001_100);
    assert_eq!(MODRM::encode(Reg, ECX, ESP), 0b11_001_100);
    assert_eq!(MODRM::encode(Reg, MM1, MM4), 0b11_001_100);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM4), 0b11_001_100);
    assert_eq!(MODRM::encode(Reg, DL, AH), 0b11_010_100);
    assert_eq!(MODRM::encode(Reg, DX, SP), 0b11_010_100);
    assert_eq!(MODRM::encode(Reg, EDX, ESP), 0b11_010_100);
    assert_eq!(MODRM::encode(Reg, MM2, MM4), 0b11_010_100);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM4), 0b11_010_100);
    assert_eq!(MODRM::encode(Reg, BL, AH), 0b11_011_100);
    assert_eq!(MODRM::encode(Reg, BX, SP), 0b11_011_100);
    assert_eq!(MODRM::encode(Reg, EBX, ESP), 0b11_011_100);
    assert_eq!(MODRM::encode(Reg, MM3, MM4), 0b11_011_100);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM4), 0b11_011_100);
    assert_eq!(MODRM::encode(Reg, AH, AH), 0b11_100_100);
    assert_eq!(MODRM::encode(Reg, SP, SP), 0b11_100_100);
    assert_eq!(MODRM::encode(Reg, ESP, ESP), 0b11_100_100);
    assert_eq!(MODRM::encode(Reg, MM4, MM4), 0b11_100_100);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM4), 0b11_100_100);
    assert_eq!(MODRM::encode(Reg, CH, AH), 0b11_101_100);
    assert_eq!(MODRM::encode(Reg, BP, SP), 0b11_101_100);
    assert_eq!(MODRM::encode(Reg, EBP, ESP), 0b11_101_100);
    assert_eq!(MODRM::encode(Reg, MM5, MM4), 0b11_101_100);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM4), 0b11_101_100);
    assert_eq!(MODRM::encode(Reg, DH, AH), 0b11_110_100);
    assert_eq!(MODRM::encode(Reg, SI, SP), 0b11_110_100);
    assert_eq!(MODRM::encode(Reg, ESI, ESP), 0b11_110_100);
    assert_eq!(MODRM::encode(Reg, MM6, MM4), 0b11_110_100);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM4), 0b11_110_100);
    assert_eq!(MODRM::encode(Reg, BH, AH), 0b11_111_100);
    assert_eq!(MODRM::encode(Reg, DI, SP), 0b11_111_100);
    assert_eq!(MODRM::encode(Reg, EDI, ESP), 0b11_111_100);
    assert_eq!(MODRM::encode(Reg, MM7, MM4), 0b11_111_100);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM4), 0b11_111_100);

    assert_eq!(MODRM::encode(Reg, AL, CH), 0b11_000_101);
    assert_eq!(MODRM::encode(Reg, AX, BP), 0b11_000_101);
    assert_eq!(MODRM::encode(Reg, EAX, EBP), 0b11_000_101);
    assert_eq!(MODRM::encode(Reg, MM0, MM5), 0b11_000_101);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM5), 0b11_000_101);
    assert_eq!(MODRM::encode(Reg, CL, CH), 0b11_001_101);
    assert_eq!(MODRM::encode(Reg, CX, BP), 0b11_001_101);
    assert_eq!(MODRM::encode(Reg, ECX, EBP), 0b11_001_101);
    assert_eq!(MODRM::encode(Reg, MM1, MM5), 0b11_001_101);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM5), 0b11_001_101);
    assert_eq!(MODRM::encode(Reg, DL, CH), 0b11_010_101);
    assert_eq!(MODRM::encode(Reg, DX, BP), 0b11_010_101);
    assert_eq!(MODRM::encode(Reg, EDX, EBP), 0b11_010_101);
    assert_eq!(MODRM::encode(Reg, MM2, MM5), 0b11_010_101);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM5), 0b11_010_101);
    assert_eq!(MODRM::encode(Reg, BL, CH), 0b11_011_101);
    assert_eq!(MODRM::encode(Reg, BX, BP), 0b11_011_101);
    assert_eq!(MODRM::encode(Reg, EBX, EBP), 0b11_011_101);
    assert_eq!(MODRM::encode(Reg, MM3, MM5), 0b11_011_101);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM5), 0b11_011_101);
    assert_eq!(MODRM::encode(Reg, AH, CH), 0b11_100_101);
    assert_eq!(MODRM::encode(Reg, SP, BP), 0b11_100_101);
    assert_eq!(MODRM::encode(Reg, ESP, EBP), 0b11_100_101);
    assert_eq!(MODRM::encode(Reg, MM4, MM5), 0b11_100_101);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM5), 0b11_100_101);
    assert_eq!(MODRM::encode(Reg, CH, CH), 0b11_101_101);
    assert_eq!(MODRM::encode(Reg, BP, BP), 0b11_101_101);
    assert_eq!(MODRM::encode(Reg, EBP, EBP), 0b11_101_101);
    assert_eq!(MODRM::encode(Reg, MM5, MM5), 0b11_101_101);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM5), 0b11_101_101);
    assert_eq!(MODRM::encode(Reg, DH, CH), 0b11_110_101);
    assert_eq!(MODRM::encode(Reg, SI, BP), 0b11_110_101);
    assert_eq!(MODRM::encode(Reg, ESI, EBP), 0b11_110_101);
    assert_eq!(MODRM::encode(Reg, MM6, MM5), 0b11_110_101);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM5), 0b11_110_101);
    assert_eq!(MODRM::encode(Reg, BH, CH), 0b11_111_101);
    assert_eq!(MODRM::encode(Reg, DI, BP), 0b11_111_101);
    assert_eq!(MODRM::encode(Reg, EDI, EBP), 0b11_111_101);
    assert_eq!(MODRM::encode(Reg, MM7, MM5), 0b11_111_101);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM5), 0b11_111_101);

    assert_eq!(MODRM::encode(Reg, AL, DH), 0b11_000_110);
    assert_eq!(MODRM::encode(Reg, AX, SI), 0b11_000_110);
    assert_eq!(MODRM::encode(Reg, EAX, ESI), 0b11_000_110);
    assert_eq!(MODRM::encode(Reg, MM0, MM6), 0b11_000_110);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM6), 0b11_000_110);
    assert_eq!(MODRM::encode(Reg, CL, DH), 0b11_001_110);
    assert_eq!(MODRM::encode(Reg, CX, SI), 0b11_001_110);
    assert_eq!(MODRM::encode(Reg, ECX, ESI), 0b11_001_110);
    assert_eq!(MODRM::encode(Reg, MM1, MM6), 0b11_001_110);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM6), 0b11_001_110);
    assert_eq!(MODRM::encode(Reg, DL, DH), 0b11_010_110);
    assert_eq!(MODRM::encode(Reg, DX, SI), 0b11_010_110);
    assert_eq!(MODRM::encode(Reg, EDX, ESI), 0b11_010_110);
    assert_eq!(MODRM::encode(Reg, MM2, MM6), 0b11_010_110);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM6), 0b11_010_110);
    assert_eq!(MODRM::encode(Reg, BL, DH), 0b11_011_110);
    assert_eq!(MODRM::encode(Reg, BX, SI), 0b11_011_110);
    assert_eq!(MODRM::encode(Reg, EBX, ESI), 0b11_011_110);
    assert_eq!(MODRM::encode(Reg, MM3, MM6), 0b11_011_110);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM6), 0b11_011_110);
    assert_eq!(MODRM::encode(Reg, AH, DH), 0b11_100_110);
    assert_eq!(MODRM::encode(Reg, SP, SI), 0b11_100_110);
    assert_eq!(MODRM::encode(Reg, ESP, ESI), 0b11_100_110);
    assert_eq!(MODRM::encode(Reg, MM4, MM6), 0b11_100_110);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM6), 0b11_100_110);
    assert_eq!(MODRM::encode(Reg, CH, DH), 0b11_101_110);
    assert_eq!(MODRM::encode(Reg, BP, SI), 0b11_101_110);
    assert_eq!(MODRM::encode(Reg, EBP, ESI), 0b11_101_110);
    assert_eq!(MODRM::encode(Reg, MM5, MM6), 0b11_101_110);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM6), 0b11_101_110);
    assert_eq!(MODRM::encode(Reg, DH, DH), 0b11_110_110);
    assert_eq!(MODRM::encode(Reg, SI, SI), 0b11_110_110);
    assert_eq!(MODRM::encode(Reg, ESI, ESI), 0b11_110_110);
    assert_eq!(MODRM::encode(Reg, MM6, MM6), 0b11_110_110);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM6), 0b11_110_110);
    assert_eq!(MODRM::encode(Reg, BH, DH), 0b11_111_110);
    assert_eq!(MODRM::encode(Reg, DI, SI), 0b11_111_110);
    assert_eq!(MODRM::encode(Reg, EDI, ESI), 0b11_111_110);
    assert_eq!(MODRM::encode(Reg, MM7, MM6), 0b11_111_110);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM6), 0b11_111_110);

    assert_eq!(MODRM::encode(Reg, AL, BH), 0b11_000_111);
    assert_eq!(MODRM::encode(Reg, AX, DI), 0b11_000_111);
    assert_eq!(MODRM::encode(Reg, EAX, EDI), 0b11_000_111);
    assert_eq!(MODRM::encode(Reg, MM0, MM7), 0b11_000_111);
    assert_eq!(MODRM::encode(Reg, XMM0, XMM7), 0b11_000_111);
    assert_eq!(MODRM::encode(Reg, CL, BH), 0b11_001_111);
    assert_eq!(MODRM::encode(Reg, CX, DI), 0b11_001_111);
    assert_eq!(MODRM::encode(Reg, ECX, EDI), 0b11_001_111);
    assert_eq!(MODRM::encode(Reg, MM1, MM7), 0b11_001_111);
    assert_eq!(MODRM::encode(Reg, XMM1, XMM7), 0b11_001_111);
    assert_eq!(MODRM::encode(Reg, DL, BH), 0b11_010_111);
    assert_eq!(MODRM::encode(Reg, DX, DI), 0b11_010_111);
    assert_eq!(MODRM::encode(Reg, EDX, EDI), 0b11_010_111);
    assert_eq!(MODRM::encode(Reg, MM2, MM7), 0b11_010_111);
    assert_eq!(MODRM::encode(Reg, XMM2, XMM7), 0b11_010_111);
    assert_eq!(MODRM::encode(Reg, BL, BH), 0b11_011_111);
    assert_eq!(MODRM::encode(Reg, BX, DI), 0b11_011_111);
    assert_eq!(MODRM::encode(Reg, EBX, EDI), 0b11_011_111);
    assert_eq!(MODRM::encode(Reg, MM3, MM7), 0b11_011_111);
    assert_eq!(MODRM::encode(Reg, XMM3, XMM7), 0b11_011_111);
    assert_eq!(MODRM::encode(Reg, AH, BH), 0b11_100_111);
    assert_eq!(MODRM::encode(Reg, SP, DI), 0b11_100_111);
    assert_eq!(MODRM::encode(Reg, ESP, EDI), 0b11_100_111);
    assert_eq!(MODRM::encode(Reg, MM4, MM7), 0b11_100_111);
    assert_eq!(MODRM::encode(Reg, XMM4, XMM7), 0b11_100_111);
    assert_eq!(MODRM::encode(Reg, CH, BH), 0b11_101_111);
    assert_eq!(MODRM::encode(Reg, BP, DI), 0b11_101_111);
    assert_eq!(MODRM::encode(Reg, EBP, EDI), 0b11_101_111);
    assert_eq!(MODRM::encode(Reg, MM5, MM7), 0b11_101_111);
    assert_eq!(MODRM::encode(Reg, XMM5, XMM7), 0b11_101_111);
    assert_eq!(MODRM::encode(Reg, DH, BH), 0b11_110_111);
    assert_eq!(MODRM::encode(Reg, SI, DI), 0b11_110_111);
    assert_eq!(MODRM::encode(Reg, ESI, EDI), 0b11_110_111);
    assert_eq!(MODRM::encode(Reg, MM6, MM7), 0b11_110_111);
    assert_eq!(MODRM::encode(Reg, XMM6, XMM7), 0b11_110_111);
    assert_eq!(MODRM::encode(Reg, BH, BH), 0b11_111_111);
    assert_eq!(MODRM::encode(Reg, DI, DI), 0b11_111_111);
    assert_eq!(MODRM::encode(Reg, EDI, EDI), 0b11_111_111);
    assert_eq!(MODRM::encode(Reg, MM7, MM7), 0b11_111_111);
    assert_eq!(MODRM::encode(Reg, XMM7, XMM7), 0b11_111_111);
}
