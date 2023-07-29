//! Unit tests for MOV instruction in real-address mode

// The following tests follow *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Table 7.1*

use super::{ Mov, MOV };
use super::super::super::super::{
    operands::{ Imm8, Imm16 },
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
};

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
    assert_eq!(vec![0x88, 0xc0], MOV::mov(AL, AL));
    assert_eq!(vec![0x88, 0xc1], MOV::mov(CL, AL));
    assert_eq!(vec![0x88, 0xc2], MOV::mov(DL, AL));
    assert_eq!(vec![0x88, 0xc3], MOV::mov(BL, AL));
    assert_eq!(vec![0x88, 0xc4], MOV::mov(AH, AL));
    assert_eq!(vec![0x88, 0xc5], MOV::mov(CH, AL));
    assert_eq!(vec![0x88, 0xc6], MOV::mov(DH, AL));
    assert_eq!(vec![0x88, 0xc7], MOV::mov(BH, AL));

    assert_eq!(vec![0x88, 0xc8], MOV::mov(AL, CL));
    assert_eq!(vec![0x88, 0xc9], MOV::mov(CL, CL));
    assert_eq!(vec![0x88, 0xca], MOV::mov(DL, CL));
    assert_eq!(vec![0x88, 0xcb], MOV::mov(BL, CL));
    assert_eq!(vec![0x88, 0xcc], MOV::mov(AH, CL));
    assert_eq!(vec![0x88, 0xcd], MOV::mov(CH, CL));
    assert_eq!(vec![0x88, 0xce], MOV::mov(DH, CL));
    assert_eq!(vec![0x88, 0xcf], MOV::mov(BH, CL));

    assert_eq!(vec![0x88, 0xd0], MOV::mov(AL, DL));
    assert_eq!(vec![0x88, 0xd1], MOV::mov(CL, DL));
    assert_eq!(vec![0x88, 0xd2], MOV::mov(DL, DL));
    assert_eq!(vec![0x88, 0xd3], MOV::mov(BL, DL));
    assert_eq!(vec![0x88, 0xd4], MOV::mov(AH, DL));
    assert_eq!(vec![0x88, 0xd5], MOV::mov(CH, DL));
    assert_eq!(vec![0x88, 0xd6], MOV::mov(DH, DL));
    assert_eq!(vec![0x88, 0xd7], MOV::mov(BH, DL));

    assert_eq!(vec![0x88, 0xd8], MOV::mov(AL, BL));
    assert_eq!(vec![0x88, 0xd9], MOV::mov(CL, BL));
    assert_eq!(vec![0x88, 0xda], MOV::mov(DL, BL));
    assert_eq!(vec![0x88, 0xdb], MOV::mov(BL, BL));
    assert_eq!(vec![0x88, 0xdc], MOV::mov(AH, BL));
    assert_eq!(vec![0x88, 0xdd], MOV::mov(CH, BL));
    assert_eq!(vec![0x88, 0xde], MOV::mov(DH, BL));
    assert_eq!(vec![0x88, 0xdf], MOV::mov(BH, BL));

    assert_eq!(vec![0x88, 0xe0], MOV::mov(AL, AH));
    assert_eq!(vec![0x88, 0xe1], MOV::mov(CL, AH));
    assert_eq!(vec![0x88, 0xe2], MOV::mov(DL, AH));
    assert_eq!(vec![0x88, 0xe3], MOV::mov(BL, AH));
    assert_eq!(vec![0x88, 0xe4], MOV::mov(AH, AH));
    assert_eq!(vec![0x88, 0xe5], MOV::mov(CH, AH));
    assert_eq!(vec![0x88, 0xe6], MOV::mov(DH, AH));
    assert_eq!(vec![0x88, 0xe7], MOV::mov(BH, AH));

    assert_eq!(vec![0x88, 0xe8], MOV::mov(AL, CH));
    assert_eq!(vec![0x88, 0xe9], MOV::mov(CL, CH));
    assert_eq!(vec![0x88, 0xea], MOV::mov(DL, CH));
    assert_eq!(vec![0x88, 0xeb], MOV::mov(BL, CH));
    assert_eq!(vec![0x88, 0xec], MOV::mov(AH, CH));
    assert_eq!(vec![0x88, 0xed], MOV::mov(CH, CH));
    assert_eq!(vec![0x88, 0xee], MOV::mov(DH, CH));
    assert_eq!(vec![0x88, 0xef], MOV::mov(BH, CH));

    assert_eq!(vec![0x88, 0xf0], MOV::mov(AL, DH));
    assert_eq!(vec![0x88, 0xf1], MOV::mov(CL, DH));
    assert_eq!(vec![0x88, 0xf2], MOV::mov(DL, DH));
    assert_eq!(vec![0x88, 0xf3], MOV::mov(BL, DH));
    assert_eq!(vec![0x88, 0xf4], MOV::mov(AH, DH));
    assert_eq!(vec![0x88, 0xf5], MOV::mov(CH, DH));
    assert_eq!(vec![0x88, 0xf6], MOV::mov(DH, DH));
    assert_eq!(vec![0x88, 0xf7], MOV::mov(BH, DH));

    assert_eq!(vec![0x88, 0xf8], MOV::mov(AL, BH));
    assert_eq!(vec![0x88, 0xf9], MOV::mov(CL, BH));
    assert_eq!(vec![0x88, 0xfa], MOV::mov(DL, BH));
    assert_eq!(vec![0x88, 0xfb], MOV::mov(BL, BH));
    assert_eq!(vec![0x88, 0xfc], MOV::mov(AH, BH));
    assert_eq!(vec![0x88, 0xfd], MOV::mov(CH, BH));
    assert_eq!(vec![0x88, 0xfe], MOV::mov(DH, BH));
    assert_eq!(vec![0x88, 0xff], MOV::mov(BH, BH));

    // Move between 16-bit registers
    assert_eq!(vec![0x89, 0xc0], MOV::mov(AX, AX));
    assert_eq!(vec![0x89, 0xc1], MOV::mov(CX, AX));
    assert_eq!(vec![0x89, 0xc2], MOV::mov(DX, AX));
    assert_eq!(vec![0x89, 0xc3], MOV::mov(BX, AX));
    assert_eq!(vec![0x89, 0xc4], MOV::mov(SP, AX));
    assert_eq!(vec![0x89, 0xc5], MOV::mov(BP, AX));
    assert_eq!(vec![0x89, 0xc6], MOV::mov(SI, AX));
    assert_eq!(vec![0x89, 0xc7], MOV::mov(DI, AX));

    assert_eq!(vec![0x89, 0xc8], MOV::mov(AX, CX));
    assert_eq!(vec![0x89, 0xc9], MOV::mov(CX, CX));
    assert_eq!(vec![0x89, 0xca], MOV::mov(DX, CX));
    assert_eq!(vec![0x89, 0xcb], MOV::mov(BX, CX));
    assert_eq!(vec![0x89, 0xcc], MOV::mov(SP, CX));
    assert_eq!(vec![0x89, 0xcd], MOV::mov(BP, CX));
    assert_eq!(vec![0x89, 0xce], MOV::mov(SI, CX));
    assert_eq!(vec![0x89, 0xcf], MOV::mov(DI, CX));

    assert_eq!(vec![0x89, 0xd0], MOV::mov(AX, DX));
    assert_eq!(vec![0x89, 0xd1], MOV::mov(CX, DX));
    assert_eq!(vec![0x89, 0xd2], MOV::mov(DX, DX));
    assert_eq!(vec![0x89, 0xd3], MOV::mov(BX, DX));
    assert_eq!(vec![0x89, 0xd4], MOV::mov(SP, DX));
    assert_eq!(vec![0x89, 0xd5], MOV::mov(BP, DX));
    assert_eq!(vec![0x89, 0xd6], MOV::mov(SI, DX));
    assert_eq!(vec![0x89, 0xd7], MOV::mov(DI, DX));

    assert_eq!(vec![0x89, 0xd8], MOV::mov(AX, BX));
    assert_eq!(vec![0x89, 0xd9], MOV::mov(CX, BX));
    assert_eq!(vec![0x89, 0xda], MOV::mov(DX, BX));
    assert_eq!(vec![0x89, 0xdb], MOV::mov(BX, BX));
    assert_eq!(vec![0x89, 0xdc], MOV::mov(SP, BX));
    assert_eq!(vec![0x89, 0xdd], MOV::mov(BP, BX));
    assert_eq!(vec![0x89, 0xde], MOV::mov(SI, BX));
    assert_eq!(vec![0x89, 0xdf], MOV::mov(DI, BX));

    assert_eq!(vec![0x89, 0xe0], MOV::mov(AX, SP));
    assert_eq!(vec![0x89, 0xe1], MOV::mov(CX, SP));
    assert_eq!(vec![0x89, 0xe2], MOV::mov(DX, SP));
    assert_eq!(vec![0x89, 0xe3], MOV::mov(BX, SP));
    assert_eq!(vec![0x89, 0xe4], MOV::mov(SP, SP));
    assert_eq!(vec![0x89, 0xe5], MOV::mov(BP, SP));
    assert_eq!(vec![0x89, 0xe6], MOV::mov(SI, SP));
    assert_eq!(vec![0x89, 0xe7], MOV::mov(DI, SP));

    assert_eq!(vec![0x89, 0xe8], MOV::mov(AX, BP));
    assert_eq!(vec![0x89, 0xe9], MOV::mov(CX, BP));
    assert_eq!(vec![0x89, 0xea], MOV::mov(DX, BP));
    assert_eq!(vec![0x89, 0xeb], MOV::mov(BX, BP));
    assert_eq!(vec![0x89, 0xec], MOV::mov(SP, BP));
    assert_eq!(vec![0x89, 0xed], MOV::mov(BP, BP));
    assert_eq!(vec![0x89, 0xee], MOV::mov(SI, BP));
    assert_eq!(vec![0x89, 0xef], MOV::mov(DI, BP));

    assert_eq!(vec![0x89, 0xf0], MOV::mov(AX, SI));
    assert_eq!(vec![0x89, 0xf1], MOV::mov(CX, SI));
    assert_eq!(vec![0x89, 0xf2], MOV::mov(DX, SI));
    assert_eq!(vec![0x89, 0xf3], MOV::mov(BX, SI));
    assert_eq!(vec![0x89, 0xf4], MOV::mov(SP, SI));
    assert_eq!(vec![0x89, 0xf5], MOV::mov(BP, SI));
    assert_eq!(vec![0x89, 0xf6], MOV::mov(SI, SI));
    assert_eq!(vec![0x89, 0xf7], MOV::mov(DI, SI));

    assert_eq!(vec![0x89, 0xf8], MOV::mov(AX, DI));
    assert_eq!(vec![0x89, 0xf9], MOV::mov(CX, DI));
    assert_eq!(vec![0x89, 0xfa], MOV::mov(DX, DI));
    assert_eq!(vec![0x89, 0xfb], MOV::mov(BX, DI));
    assert_eq!(vec![0x89, 0xfc], MOV::mov(SP, DI));
    assert_eq!(vec![0x89, 0xfd], MOV::mov(BP, DI));
    assert_eq!(vec![0x89, 0xfe], MOV::mov(SI, DI));
    assert_eq!(vec![0x89, 0xff], MOV::mov(DI, DI));
}

#[test]
fn real_address_mode_mov_instructions_from_memory_to_gp() {
    // Move memory to 8-bit registers
    assert_eq!(vec![0xa0, 0x00, 0x00], MOV::mov(AL, [0]));
    assert_eq!(vec![0x8a, 0x0e, 0x00, 0x00], MOV::mov(CL, [0]));
    assert_eq!(vec![0x8a, 0x16, 0x00, 0x00], MOV::mov(DL, [0]));
    assert_eq!(vec![0x8a, 0x1e, 0x00, 0x00], MOV::mov(BL, [0]));
    assert_eq!(vec![0x8a, 0x26, 0x00, 0x00], MOV::mov(AH, [0]));
    assert_eq!(vec![0x8a, 0x2e, 0x00, 0x00], MOV::mov(CH, [0]));
    assert_eq!(vec![0x8a, 0x36, 0x00, 0x00], MOV::mov(DH, [0]));
    assert_eq!(vec![0x8a, 0x3e, 0x00, 0x00], MOV::mov(BH, [0]));

    assert_eq!(vec![0xa0, 0x64, 0x00], MOV::mov(AL, [100]));
    assert_eq!(vec![0x8a, 0x0e, 0x64, 0x00], MOV::mov(CL, [100]));
    assert_eq!(vec![0x8a, 0x16, 0x64, 0x00], MOV::mov(DL, [100]));
    assert_eq!(vec![0x8a, 0x1e, 0x64, 0x00], MOV::mov(BL, [100]));
    assert_eq!(vec![0x8a, 0x26, 0x64, 0x00], MOV::mov(AH, [100]));
    assert_eq!(vec![0x8a, 0x2e, 0x64, 0x00], MOV::mov(CH, [100]));
    assert_eq!(vec![0x8a, 0x36, 0x64, 0x00], MOV::mov(DH, [100]));
    assert_eq!(vec![0x8a, 0x3e, 0x64, 0x00], MOV::mov(BH, [100]));

    // Move memory to 16-bit registers
    assert_eq!(vec![0xa1, 0x00, 0x00], MOV::mov(AX, [0]));
    assert_eq!(vec![0x8b, 0x0e, 0x00, 0x00], MOV::mov(CX, [0]));
    assert_eq!(vec![0x8b, 0x16, 0x00, 0x00], MOV::mov(DX, [0]));
    assert_eq!(vec![0x8b, 0x1e, 0x00, 0x00], MOV::mov(BX, [0]));
    assert_eq!(vec![0x8b, 0x26, 0x00, 0x00], MOV::mov(SP, [0]));
    assert_eq!(vec![0x8b, 0x2e, 0x00, 0x00], MOV::mov(BP, [0]));
    assert_eq!(vec![0x8b, 0x36, 0x00, 0x00], MOV::mov(SI, [0]));
    assert_eq!(vec![0x8b, 0x3e, 0x00, 0x00], MOV::mov(DI, [0]));

    assert_eq!(vec![0xa1, 0x64, 0x00], MOV::mov(AX, [100]));
    assert_eq!(vec![0x8b, 0x0e, 0x64, 0x00], MOV::mov(CX, [100]));
    assert_eq!(vec![0x8b, 0x16, 0x64, 0x00], MOV::mov(DX, [100]));
    assert_eq!(vec![0x8b, 0x1e, 0x64, 0x00], MOV::mov(BX, [100]));
    assert_eq!(vec![0x8b, 0x26, 0x64, 0x00], MOV::mov(SP, [100]));
    assert_eq!(vec![0x8b, 0x2e, 0x64, 0x00], MOV::mov(BP, [100]));
    assert_eq!(vec![0x8b, 0x36, 0x64, 0x00], MOV::mov(SI, [100]));
    assert_eq!(vec![0x8b, 0x3e, 0x64, 0x00], MOV::mov(DI, [100]));

    assert_eq!(vec![0xa1, 0x10, 0x27], MOV::mov(AX, [10000]));
    assert_eq!(vec![0x8b, 0x0e, 0x10, 0x27], MOV::mov(CX, [10000]));
    assert_eq!(vec![0x8b, 0x16, 0x10, 0x27], MOV::mov(DX, [10000]));
    assert_eq!(vec![0x8b, 0x1e, 0x10, 0x27], MOV::mov(BX, [10000]));
    assert_eq!(vec![0x8b, 0x26, 0x10, 0x27], MOV::mov(SP, [10000]));
    assert_eq!(vec![0x8b, 0x2e, 0x10, 0x27], MOV::mov(BP, [10000]));
    assert_eq!(vec![0x8b, 0x36, 0x10, 0x27], MOV::mov(SI, [10000]));
    assert_eq!(vec![0x8b, 0x3e, 0x10, 0x27], MOV::mov(DI, [10000]));

    assert_eq!(vec![0x8b, 0x07], MOV::mov(AX, [BX]));
    assert_eq!(vec![0x8b, 0x0f], MOV::mov(CX, [BX]));
    assert_eq!(vec![0x8b, 0x17], MOV::mov(DX, [BX]));
    assert_eq!(vec![0x8b, 0x1f], MOV::mov(BX, [BX]));
    assert_eq!(vec![0x8b, 0x27], MOV::mov(SP, [BX]));
    assert_eq!(vec![0x8b, 0x2f], MOV::mov(BP, [BX]));
    assert_eq!(vec![0x8b, 0x37], MOV::mov(SI, [BX]));
    assert_eq!(vec![0x8b, 0x3f], MOV::mov(DI, [BX]));

    assert_eq!(vec![0x8b, 0x46, 0x00], MOV::mov(AX, [BP]));
    assert_eq!(vec![0x8b, 0x4e, 0x00], MOV::mov(CX, [BP]));
    assert_eq!(vec![0x8b, 0x56, 0x00], MOV::mov(DX, [BP]));
    assert_eq!(vec![0x8b, 0x5e, 0x00], MOV::mov(BX, [BP]));
    assert_eq!(vec![0x8b, 0x66, 0x00], MOV::mov(SP, [BP]));
    assert_eq!(vec![0x8b, 0x6e, 0x00], MOV::mov(BP, [BP]));
    assert_eq!(vec![0x8b, 0x76, 0x00], MOV::mov(SI, [BP]));
    assert_eq!(vec![0x8b, 0x7e, 0x00], MOV::mov(DI, [BP]));

    assert_eq!(vec![0x8b, 0x04], MOV::mov(AX, [SI]));
    assert_eq!(vec![0x8b, 0x0c], MOV::mov(CX, [SI]));
    assert_eq!(vec![0x8b, 0x14], MOV::mov(DX, [SI]));
    assert_eq!(vec![0x8b, 0x1c], MOV::mov(BX, [SI]));
    assert_eq!(vec![0x8b, 0x24], MOV::mov(SP, [SI]));
    assert_eq!(vec![0x8b, 0x2c], MOV::mov(BP, [SI]));
    assert_eq!(vec![0x8b, 0x34], MOV::mov(SI, [SI]));
    assert_eq!(vec![0x8b, 0x3c], MOV::mov(DI, [SI]));

    assert_eq!(vec![0x8b, 0x05], MOV::mov(AX, [DI]));
    assert_eq!(vec![0x8b, 0x0d], MOV::mov(CX, [DI]));
    assert_eq!(vec![0x8b, 0x15], MOV::mov(DX, [DI]));
    assert_eq!(vec![0x8b, 0x1d], MOV::mov(BX, [DI]));
    assert_eq!(vec![0x8b, 0x25], MOV::mov(SP, [DI]));
    assert_eq!(vec![0x8b, 0x2d], MOV::mov(BP, [DI]));
    assert_eq!(vec![0x8b, 0x35], MOV::mov(SI, [DI]));
    assert_eq!(vec![0x8b, 0x3d], MOV::mov(DI, [DI]));
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
    assert_eq!(vec![0xb0, 0x00], MOV::mov(AL, Imm8(0)));
    assert_eq!(vec![0xb1, 0x00], MOV::mov(CL, Imm8(0)));
    assert_eq!(vec![0xb2, 0x00], MOV::mov(DL, Imm8(0)));
    assert_eq!(vec![0xb3, 0x00], MOV::mov(BL, Imm8(0)));
    assert_eq!(vec![0xb4, 0x00], MOV::mov(AH, Imm8(0)));
    assert_eq!(vec![0xb5, 0x00], MOV::mov(CH, Imm8(0)));
    assert_eq!(vec![0xb6, 0x00], MOV::mov(DH, Imm8(0)));
    assert_eq!(vec![0xb7, 0x00], MOV::mov(BH, Imm8(0)));

    assert_eq!(vec![0xb0, 0x0c], MOV::mov(AL, Imm8(12)));
    assert_eq!(vec![0xb1, 0x0c], MOV::mov(CL, Imm8(12)));
    assert_eq!(vec![0xb2, 0x0c], MOV::mov(DL, Imm8(12)));
    assert_eq!(vec![0xb3, 0x0c], MOV::mov(BL, Imm8(12)));
    assert_eq!(vec![0xb4, 0x0c], MOV::mov(AH, Imm8(12)));
    assert_eq!(vec![0xb5, 0x0c], MOV::mov(CH, Imm8(12)));
    assert_eq!(vec![0xb6, 0x0c], MOV::mov(DH, Imm8(12)));
    assert_eq!(vec![0xb7, 0x0c], MOV::mov(BH, Imm8(12)));

    assert_eq!(vec![0xb0, 0x10], MOV::mov(AL, Imm8(16)));
    assert_eq!(vec![0xb1, 0x10], MOV::mov(CL, Imm8(16)));
    assert_eq!(vec![0xb2, 0x10], MOV::mov(DL, Imm8(16)));
    assert_eq!(vec![0xb3, 0x10], MOV::mov(BL, Imm8(16)));
    assert_eq!(vec![0xb4, 0x10], MOV::mov(AH, Imm8(16)));
    assert_eq!(vec![0xb5, 0x10], MOV::mov(CH, Imm8(16)));
    assert_eq!(vec![0xb6, 0x10], MOV::mov(DH, Imm8(16)));
    assert_eq!(vec![0xb7, 0x10], MOV::mov(BH, Imm8(16)));

    assert_eq!(vec![0xb0, 0x96], MOV::mov(AL, Imm8(150)));
    assert_eq!(vec![0xb1, 0x96], MOV::mov(CL, Imm8(150)));
    assert_eq!(vec![0xb2, 0x96], MOV::mov(DL, Imm8(150)));
    assert_eq!(vec![0xb3, 0x96], MOV::mov(BL, Imm8(150)));
    assert_eq!(vec![0xb4, 0x96], MOV::mov(AH, Imm8(150)));
    assert_eq!(vec![0xb5, 0x96], MOV::mov(CH, Imm8(150)));
    assert_eq!(vec![0xb6, 0x96], MOV::mov(DH, Imm8(150)));
    assert_eq!(vec![0xb7, 0x96], MOV::mov(BH, Imm8(150)));

    // Move values to 16-bit registers
    assert_eq!(vec![0xb8, 0x00, 0x00], MOV::mov(AX, Imm16(0)));
    assert_eq!(vec![0xb9, 0x00, 0x00], MOV::mov(CX, Imm16(0)));
    assert_eq!(vec![0xba, 0x00, 0x00], MOV::mov(DX, Imm16(0)));
    assert_eq!(vec![0xbb, 0x00, 0x00], MOV::mov(BX, Imm16(0)));
    assert_eq!(vec![0xbc, 0x00, 0x00], MOV::mov(SP, Imm16(0)));
    assert_eq!(vec![0xbd, 0x00, 0x00], MOV::mov(BP, Imm16(0)));
    assert_eq!(vec![0xbe, 0x00, 0x00], MOV::mov(SI, Imm16(0)));
    assert_eq!(vec![0xbf, 0x00, 0x00], MOV::mov(DI, Imm16(0)));

    assert_eq!(vec![0xb8, 0x0c, 0x00], MOV::mov(AX, Imm16(12)));
    assert_eq!(vec![0xb9, 0x0c, 0x00], MOV::mov(CX, Imm16(12)));
    assert_eq!(vec![0xba, 0x0c, 0x00], MOV::mov(DX, Imm16(12)));
    assert_eq!(vec![0xbb, 0x0c, 0x00], MOV::mov(BX, Imm16(12)));
    assert_eq!(vec![0xbc, 0x0c, 0x00], MOV::mov(SP, Imm16(12)));
    assert_eq!(vec![0xbd, 0x0c, 0x00], MOV::mov(BP, Imm16(12)));
    assert_eq!(vec![0xbe, 0x0c, 0x00], MOV::mov(SI, Imm16(12)));
    assert_eq!(vec![0xbf, 0x0c, 0x00], MOV::mov(DI, Imm16(12)));

    assert_eq!(vec![0xb8, 0x10, 0x00], MOV::mov(AX, Imm16(16)));
    assert_eq!(vec![0xb9, 0x10, 0x00], MOV::mov(CX, Imm16(16)));
    assert_eq!(vec![0xba, 0x10, 0x00], MOV::mov(DX, Imm16(16)));
    assert_eq!(vec![0xbb, 0x10, 0x00], MOV::mov(BX, Imm16(16)));
    assert_eq!(vec![0xbc, 0x10, 0x00], MOV::mov(SP, Imm16(16)));
    assert_eq!(vec![0xbd, 0x10, 0x00], MOV::mov(BP, Imm16(16)));
    assert_eq!(vec![0xbe, 0x10, 0x00], MOV::mov(SI, Imm16(16)));
    assert_eq!(vec![0xbf, 0x10, 0x00], MOV::mov(DI, Imm16(16)));

    assert_eq!(vec![0xb8, 0x96, 0x00], MOV::mov(AX, Imm16(150)));
    assert_eq!(vec![0xb9, 0x96, 0x00], MOV::mov(CX, Imm16(150)));
    assert_eq!(vec![0xba, 0x96, 0x00], MOV::mov(DX, Imm16(150)));
    assert_eq!(vec![0xbb, 0x96, 0x00], MOV::mov(BX, Imm16(150)));
    assert_eq!(vec![0xbc, 0x96, 0x00], MOV::mov(SP, Imm16(150)));
    assert_eq!(vec![0xbd, 0x96, 0x00], MOV::mov(BP, Imm16(150)));
    assert_eq!(vec![0xbe, 0x96, 0x00], MOV::mov(SI, Imm16(150)));
    assert_eq!(vec![0xbf, 0x96, 0x00], MOV::mov(DI, Imm16(150)));

    assert_eq!(vec![0xb8, 0xe8, 0x03], MOV::mov(AX, Imm16(1000)));
    assert_eq!(vec![0xb9, 0xe8, 0x03], MOV::mov(CX, Imm16(1000)));
    assert_eq!(vec![0xba, 0xe8, 0x03], MOV::mov(DX, Imm16(1000)));
    assert_eq!(vec![0xbb, 0xe8, 0x03], MOV::mov(BX, Imm16(1000)));
    assert_eq!(vec![0xbc, 0xe8, 0x03], MOV::mov(SP, Imm16(1000)));
    assert_eq!(vec![0xbd, 0xe8, 0x03], MOV::mov(BP, Imm16(1000)));
    assert_eq!(vec![0xbe, 0xe8, 0x03], MOV::mov(SI, Imm16(1000)));
    assert_eq!(vec![0xbf, 0xe8, 0x03], MOV::mov(DI, Imm16(1000)));

    assert_eq!(vec![0xb8, 0x00, 0x80], MOV::mov(AX, Imm16(32768)));
    assert_eq!(vec![0xb9, 0x00, 0x80], MOV::mov(CX, Imm16(32768)));
    assert_eq!(vec![0xba, 0x00, 0x80], MOV::mov(DX, Imm16(32768)));
    assert_eq!(vec![0xbb, 0x00, 0x80], MOV::mov(BX, Imm16(32768)));
    assert_eq!(vec![0xbc, 0x00, 0x80], MOV::mov(SP, Imm16(32768)));
    assert_eq!(vec![0xbd, 0x00, 0x80], MOV::mov(BP, Imm16(32768)));
    assert_eq!(vec![0xbe, 0x00, 0x80], MOV::mov(SI, Imm16(32768)));
    assert_eq!(vec![0xbf, 0x00, 0x80], MOV::mov(DI, Imm16(32768)));

    assert_eq!(vec![0xb8, 0x50, 0xc3], MOV::mov(AX, Imm16(50000)));
    assert_eq!(vec![0xb9, 0x50, 0xc3], MOV::mov(CX, Imm16(50000)));
    assert_eq!(vec![0xba, 0x50, 0xc3], MOV::mov(DX, Imm16(50000)));
    assert_eq!(vec![0xbb, 0x50, 0xc3], MOV::mov(BX, Imm16(50000)));
    assert_eq!(vec![0xbc, 0x50, 0xc3], MOV::mov(SP, Imm16(50000)));
    assert_eq!(vec![0xbd, 0x50, 0xc3], MOV::mov(BP, Imm16(50000)));
    assert_eq!(vec![0xbe, 0x50, 0xc3], MOV::mov(SI, Imm16(50000)));
    assert_eq!(vec![0xbf, 0x50, 0xc3], MOV::mov(DI, Imm16(50000)));
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
