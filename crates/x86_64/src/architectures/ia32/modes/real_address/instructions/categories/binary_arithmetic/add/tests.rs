//! Unit tests for MOV instruction in real-address mode

// The following tests follow *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Table 7.1*

use super::{
    Add,
    super::super::super::{
        ASM,
        operands::{ Imm8, Imm16 },
        operands::{ AL, CL, DL, BL },
        operands::{ AH, CH, DH, BH },
        operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    },
};

#[test]
fn real_address_mode_add_immediate_to_gp_instruction() {
    let asm = ASM::new();

    // add value to 8-bit registers
    assert_eq!(vec![0x04, 0x00], asm.add(AL, Imm8(0)));
    assert_eq!(vec![0x80, 0xc1, 0x00], asm.add(CL, Imm8(0)));
    assert_eq!(vec![0x80, 0xc2, 0x00], asm.add(DL, Imm8(0)));
    assert_eq!(vec![0x80, 0xc3, 0x00], asm.add(BL, Imm8(0)));
    assert_eq!(vec![0x80, 0xc4, 0x00], asm.add(AH, Imm8(0)));
    assert_eq!(vec![0x80, 0xc5, 0x00], asm.add(CH, Imm8(0)));
    assert_eq!(vec![0x80, 0xc6, 0x00], asm.add(DH, Imm8(0)));
    assert_eq!(vec![0x80, 0xc7, 0x00], asm.add(BH, Imm8(0)));

    assert_eq!(vec![0x04, 0x0c], asm.add(AL, Imm8(12)));
    assert_eq!(vec![0x80, 0xc1, 0x0c], asm.add(CL, Imm8(12)));
    assert_eq!(vec![0x80, 0xc2, 0x0c], asm.add(DL, Imm8(12)));
    assert_eq!(vec![0x80, 0xc3, 0x0c], asm.add(BL, Imm8(12)));
    assert_eq!(vec![0x80, 0xc4, 0x0c], asm.add(AH, Imm8(12)));
    assert_eq!(vec![0x80, 0xc5, 0x0c], asm.add(CH, Imm8(12)));
    assert_eq!(vec![0x80, 0xc6, 0x0c], asm.add(DH, Imm8(12)));
    assert_eq!(vec![0x80, 0xc7, 0x0c], asm.add(BH, Imm8(12)));

    assert_eq!(vec![0x04, 0x10], asm.add(AL, Imm8(16)));
    assert_eq!(vec![0x80, 0xc1, 0x10], asm.add(CL, Imm8(16)));
    assert_eq!(vec![0x80, 0xc2, 0x10], asm.add(DL, Imm8(16)));
    assert_eq!(vec![0x80, 0xc3, 0x10], asm.add(BL, Imm8(16)));
    assert_eq!(vec![0x80, 0xc4, 0x10], asm.add(AH, Imm8(16)));
    assert_eq!(vec![0x80, 0xc5, 0x10], asm.add(CH, Imm8(16)));
    assert_eq!(vec![0x80, 0xc6, 0x10], asm.add(DH, Imm8(16)));
    assert_eq!(vec![0x80, 0xc7, 0x10], asm.add(BH, Imm8(16)));

    assert_eq!(vec![0x04, 0x96], asm.add(AL, Imm8(150)));
    assert_eq!(vec![0x80, 0xc1, 0x96], asm.add(CL, Imm8(150)));
    assert_eq!(vec![0x80, 0xc2, 0x96], asm.add(DL, Imm8(150)));
    assert_eq!(vec![0x80, 0xc3, 0x96], asm.add(BL, Imm8(150)));
    assert_eq!(vec![0x80, 0xc4, 0x96], asm.add(AH, Imm8(150)));
    assert_eq!(vec![0x80, 0xc5, 0x96], asm.add(CH, Imm8(150)));
    assert_eq!(vec![0x80, 0xc6, 0x96], asm.add(DH, Imm8(150)));
    assert_eq!(vec![0x80, 0xc7, 0x96], asm.add(BH, Imm8(150)));

    // Add value to 16-bit registers
    assert_eq!(vec![0x83, 0xc0, 0x00], asm.add(AX, Imm8(0)));
    assert_eq!(vec![0x83, 0xc1, 0x00], asm.add(CX, Imm8(0)));
    assert_eq!(vec![0x83, 0xc2, 0x00], asm.add(DX, Imm8(0)));
    assert_eq!(vec![0x83, 0xc3, 0x00], asm.add(BX, Imm8(0)));
    assert_eq!(vec![0x83, 0xc4, 0x00], asm.add(SP, Imm8(0)));
    assert_eq!(vec![0x83, 0xc5, 0x00], asm.add(BP, Imm8(0)));
    assert_eq!(vec![0x83, 0xc6, 0x00], asm.add(SI, Imm8(0)));
    assert_eq!(vec![0x83, 0xc7, 0x00], asm.add(DI, Imm8(0)));

    assert_eq!(vec![0x83, 0xc0, 0x0c], asm.add(AX, Imm8(12)));
    assert_eq!(vec![0x83, 0xc1, 0x0c], asm.add(CX, Imm8(12)));
    assert_eq!(vec![0x83, 0xc2, 0x0c], asm.add(DX, Imm8(12)));
    assert_eq!(vec![0x83, 0xc3, 0x0c], asm.add(BX, Imm8(12)));
    assert_eq!(vec![0x83, 0xc4, 0x0c], asm.add(SP, Imm8(12)));
    assert_eq!(vec![0x83, 0xc5, 0x0c], asm.add(BP, Imm8(12)));
    assert_eq!(vec![0x83, 0xc6, 0x0c], asm.add(SI, Imm8(12)));
    assert_eq!(vec![0x83, 0xc7, 0x0c], asm.add(DI, Imm8(12)));

    assert_eq!(vec![0x83, 0xc0, 0x10], asm.add(AX, Imm8(16)));
    assert_eq!(vec![0x83, 0xc1, 0x10], asm.add(CX, Imm8(16)));
    assert_eq!(vec![0x83, 0xc2, 0x10], asm.add(DX, Imm8(16)));
    assert_eq!(vec![0x83, 0xc3, 0x10], asm.add(BX, Imm8(16)));
    assert_eq!(vec![0x83, 0xc4, 0x10], asm.add(SP, Imm8(16)));
    assert_eq!(vec![0x83, 0xc5, 0x10], asm.add(BP, Imm8(16)));
    assert_eq!(vec![0x83, 0xc6, 0x10], asm.add(SI, Imm8(16)));
    assert_eq!(vec![0x83, 0xc7, 0x10], asm.add(DI, Imm8(16)));

    assert_eq!(vec![0x05, 0x96, 0x00], asm.add(AX, Imm16(150)));
    assert_eq!(vec![0x81, 0xc1, 0x96, 0x00], asm.add(CX, Imm16(150)));
    assert_eq!(vec![0x81, 0xc2, 0x96, 0x00], asm.add(DX, Imm16(150)));
    assert_eq!(vec![0x81, 0xc3, 0x96, 0x00], asm.add(BX, Imm16(150)));
    assert_eq!(vec![0x81, 0xc4, 0x96, 0x00], asm.add(SP, Imm16(150)));
    assert_eq!(vec![0x81, 0xc5, 0x96, 0x00], asm.add(BP, Imm16(150)));
    assert_eq!(vec![0x81, 0xc6, 0x96, 0x00], asm.add(SI, Imm16(150)));
    assert_eq!(vec![0x81, 0xc7, 0x96, 0x00], asm.add(DI, Imm16(150)));

    assert_eq!(vec![0x05, 0xe8, 0x03], asm.add(AX, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc1, 0xe8, 0x03], asm.add(CX, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc2, 0xe8, 0x03], asm.add(DX, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc3, 0xe8, 0x03], asm.add(BX, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc4, 0xe8, 0x03], asm.add(SP, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc5, 0xe8, 0x03], asm.add(BP, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc6, 0xe8, 0x03], asm.add(SI, Imm16(1000)));
    assert_eq!(vec![0x81, 0xc7, 0xe8, 0x03], asm.add(DI, Imm16(1000)));

    assert_eq!(vec![0x05, 0x00, 0x80], asm.add(AX, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc1, 0x00, 0x80], asm.add(CX, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc2, 0x00, 0x80], asm.add(DX, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc3, 0x00, 0x80], asm.add(BX, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc4, 0x00, 0x80], asm.add(SP, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc5, 0x00, 0x80], asm.add(BP, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc6, 0x00, 0x80], asm.add(SI, Imm16(32768)));
    assert_eq!(vec![0x81, 0xc7, 0x00, 0x80], asm.add(DI, Imm16(32768)));

    assert_eq!(vec![0x05, 0x50, 0xc3], asm.add(AX, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc1, 0x50, 0xc3], asm.add(CX, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc2, 0x50, 0xc3], asm.add(DX, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc3, 0x50, 0xc3], asm.add(BX, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc4, 0x50, 0xc3], asm.add(SP, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc5, 0x50, 0xc3], asm.add(BP, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc6, 0x50, 0xc3], asm.add(SI, Imm16(50000)));
    assert_eq!(vec![0x81, 0xc7, 0x50, 0xc3], asm.add(DI, Imm16(50000)));
}
