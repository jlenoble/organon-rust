//! Unit tests for MOV instruction in real-address mode

// The following tests follow *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Table 7.1*

use super::{
    Mov,
    super::super::super::{
        ASM,
        operands::{ Imm8, Imm16 },
        operands::{ AL, CL, DL, BL },
        operands::{ AH, CH, DH, BH },
        operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    },
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
    assert_eq!(vec![0x88, 0xc0], ASM::mov(AL, AL));
    assert_eq!(vec![0x88, 0xc1], ASM::mov(CL, AL));
    assert_eq!(vec![0x88, 0xc2], ASM::mov(DL, AL));
    assert_eq!(vec![0x88, 0xc3], ASM::mov(BL, AL));
    assert_eq!(vec![0x88, 0xc4], ASM::mov(AH, AL));
    assert_eq!(vec![0x88, 0xc5], ASM::mov(CH, AL));
    assert_eq!(vec![0x88, 0xc6], ASM::mov(DH, AL));
    assert_eq!(vec![0x88, 0xc7], ASM::mov(BH, AL));

    assert_eq!(vec![0x88, 0xc8], ASM::mov(AL, CL));
    assert_eq!(vec![0x88, 0xc9], ASM::mov(CL, CL));
    assert_eq!(vec![0x88, 0xca], ASM::mov(DL, CL));
    assert_eq!(vec![0x88, 0xcb], ASM::mov(BL, CL));
    assert_eq!(vec![0x88, 0xcc], ASM::mov(AH, CL));
    assert_eq!(vec![0x88, 0xcd], ASM::mov(CH, CL));
    assert_eq!(vec![0x88, 0xce], ASM::mov(DH, CL));
    assert_eq!(vec![0x88, 0xcf], ASM::mov(BH, CL));

    assert_eq!(vec![0x88, 0xd0], ASM::mov(AL, DL));
    assert_eq!(vec![0x88, 0xd1], ASM::mov(CL, DL));
    assert_eq!(vec![0x88, 0xd2], ASM::mov(DL, DL));
    assert_eq!(vec![0x88, 0xd3], ASM::mov(BL, DL));
    assert_eq!(vec![0x88, 0xd4], ASM::mov(AH, DL));
    assert_eq!(vec![0x88, 0xd5], ASM::mov(CH, DL));
    assert_eq!(vec![0x88, 0xd6], ASM::mov(DH, DL));
    assert_eq!(vec![0x88, 0xd7], ASM::mov(BH, DL));

    assert_eq!(vec![0x88, 0xd8], ASM::mov(AL, BL));
    assert_eq!(vec![0x88, 0xd9], ASM::mov(CL, BL));
    assert_eq!(vec![0x88, 0xda], ASM::mov(DL, BL));
    assert_eq!(vec![0x88, 0xdb], ASM::mov(BL, BL));
    assert_eq!(vec![0x88, 0xdc], ASM::mov(AH, BL));
    assert_eq!(vec![0x88, 0xdd], ASM::mov(CH, BL));
    assert_eq!(vec![0x88, 0xde], ASM::mov(DH, BL));
    assert_eq!(vec![0x88, 0xdf], ASM::mov(BH, BL));

    assert_eq!(vec![0x88, 0xe0], ASM::mov(AL, AH));
    assert_eq!(vec![0x88, 0xe1], ASM::mov(CL, AH));
    assert_eq!(vec![0x88, 0xe2], ASM::mov(DL, AH));
    assert_eq!(vec![0x88, 0xe3], ASM::mov(BL, AH));
    assert_eq!(vec![0x88, 0xe4], ASM::mov(AH, AH));
    assert_eq!(vec![0x88, 0xe5], ASM::mov(CH, AH));
    assert_eq!(vec![0x88, 0xe6], ASM::mov(DH, AH));
    assert_eq!(vec![0x88, 0xe7], ASM::mov(BH, AH));

    assert_eq!(vec![0x88, 0xe8], ASM::mov(AL, CH));
    assert_eq!(vec![0x88, 0xe9], ASM::mov(CL, CH));
    assert_eq!(vec![0x88, 0xea], ASM::mov(DL, CH));
    assert_eq!(vec![0x88, 0xeb], ASM::mov(BL, CH));
    assert_eq!(vec![0x88, 0xec], ASM::mov(AH, CH));
    assert_eq!(vec![0x88, 0xed], ASM::mov(CH, CH));
    assert_eq!(vec![0x88, 0xee], ASM::mov(DH, CH));
    assert_eq!(vec![0x88, 0xef], ASM::mov(BH, CH));

    assert_eq!(vec![0x88, 0xf0], ASM::mov(AL, DH));
    assert_eq!(vec![0x88, 0xf1], ASM::mov(CL, DH));
    assert_eq!(vec![0x88, 0xf2], ASM::mov(DL, DH));
    assert_eq!(vec![0x88, 0xf3], ASM::mov(BL, DH));
    assert_eq!(vec![0x88, 0xf4], ASM::mov(AH, DH));
    assert_eq!(vec![0x88, 0xf5], ASM::mov(CH, DH));
    assert_eq!(vec![0x88, 0xf6], ASM::mov(DH, DH));
    assert_eq!(vec![0x88, 0xf7], ASM::mov(BH, DH));

    assert_eq!(vec![0x88, 0xf8], ASM::mov(AL, BH));
    assert_eq!(vec![0x88, 0xf9], ASM::mov(CL, BH));
    assert_eq!(vec![0x88, 0xfa], ASM::mov(DL, BH));
    assert_eq!(vec![0x88, 0xfb], ASM::mov(BL, BH));
    assert_eq!(vec![0x88, 0xfc], ASM::mov(AH, BH));
    assert_eq!(vec![0x88, 0xfd], ASM::mov(CH, BH));
    assert_eq!(vec![0x88, 0xfe], ASM::mov(DH, BH));
    assert_eq!(vec![0x88, 0xff], ASM::mov(BH, BH));

    // Move between 16-bit registers
    assert_eq!(vec![0x89, 0xc0], ASM::mov(AX, AX));
    assert_eq!(vec![0x89, 0xc1], ASM::mov(CX, AX));
    assert_eq!(vec![0x89, 0xc2], ASM::mov(DX, AX));
    assert_eq!(vec![0x89, 0xc3], ASM::mov(BX, AX));
    assert_eq!(vec![0x89, 0xc4], ASM::mov(SP, AX));
    assert_eq!(vec![0x89, 0xc5], ASM::mov(BP, AX));
    assert_eq!(vec![0x89, 0xc6], ASM::mov(SI, AX));
    assert_eq!(vec![0x89, 0xc7], ASM::mov(DI, AX));

    assert_eq!(vec![0x89, 0xc8], ASM::mov(AX, CX));
    assert_eq!(vec![0x89, 0xc9], ASM::mov(CX, CX));
    assert_eq!(vec![0x89, 0xca], ASM::mov(DX, CX));
    assert_eq!(vec![0x89, 0xcb], ASM::mov(BX, CX));
    assert_eq!(vec![0x89, 0xcc], ASM::mov(SP, CX));
    assert_eq!(vec![0x89, 0xcd], ASM::mov(BP, CX));
    assert_eq!(vec![0x89, 0xce], ASM::mov(SI, CX));
    assert_eq!(vec![0x89, 0xcf], ASM::mov(DI, CX));

    assert_eq!(vec![0x89, 0xd0], ASM::mov(AX, DX));
    assert_eq!(vec![0x89, 0xd1], ASM::mov(CX, DX));
    assert_eq!(vec![0x89, 0xd2], ASM::mov(DX, DX));
    assert_eq!(vec![0x89, 0xd3], ASM::mov(BX, DX));
    assert_eq!(vec![0x89, 0xd4], ASM::mov(SP, DX));
    assert_eq!(vec![0x89, 0xd5], ASM::mov(BP, DX));
    assert_eq!(vec![0x89, 0xd6], ASM::mov(SI, DX));
    assert_eq!(vec![0x89, 0xd7], ASM::mov(DI, DX));

    assert_eq!(vec![0x89, 0xd8], ASM::mov(AX, BX));
    assert_eq!(vec![0x89, 0xd9], ASM::mov(CX, BX));
    assert_eq!(vec![0x89, 0xda], ASM::mov(DX, BX));
    assert_eq!(vec![0x89, 0xdb], ASM::mov(BX, BX));
    assert_eq!(vec![0x89, 0xdc], ASM::mov(SP, BX));
    assert_eq!(vec![0x89, 0xdd], ASM::mov(BP, BX));
    assert_eq!(vec![0x89, 0xde], ASM::mov(SI, BX));
    assert_eq!(vec![0x89, 0xdf], ASM::mov(DI, BX));

    assert_eq!(vec![0x89, 0xe0], ASM::mov(AX, SP));
    assert_eq!(vec![0x89, 0xe1], ASM::mov(CX, SP));
    assert_eq!(vec![0x89, 0xe2], ASM::mov(DX, SP));
    assert_eq!(vec![0x89, 0xe3], ASM::mov(BX, SP));
    assert_eq!(vec![0x89, 0xe4], ASM::mov(SP, SP));
    assert_eq!(vec![0x89, 0xe5], ASM::mov(BP, SP));
    assert_eq!(vec![0x89, 0xe6], ASM::mov(SI, SP));
    assert_eq!(vec![0x89, 0xe7], ASM::mov(DI, SP));

    assert_eq!(vec![0x89, 0xe8], ASM::mov(AX, BP));
    assert_eq!(vec![0x89, 0xe9], ASM::mov(CX, BP));
    assert_eq!(vec![0x89, 0xea], ASM::mov(DX, BP));
    assert_eq!(vec![0x89, 0xeb], ASM::mov(BX, BP));
    assert_eq!(vec![0x89, 0xec], ASM::mov(SP, BP));
    assert_eq!(vec![0x89, 0xed], ASM::mov(BP, BP));
    assert_eq!(vec![0x89, 0xee], ASM::mov(SI, BP));
    assert_eq!(vec![0x89, 0xef], ASM::mov(DI, BP));

    assert_eq!(vec![0x89, 0xf0], ASM::mov(AX, SI));
    assert_eq!(vec![0x89, 0xf1], ASM::mov(CX, SI));
    assert_eq!(vec![0x89, 0xf2], ASM::mov(DX, SI));
    assert_eq!(vec![0x89, 0xf3], ASM::mov(BX, SI));
    assert_eq!(vec![0x89, 0xf4], ASM::mov(SP, SI));
    assert_eq!(vec![0x89, 0xf5], ASM::mov(BP, SI));
    assert_eq!(vec![0x89, 0xf6], ASM::mov(SI, SI));
    assert_eq!(vec![0x89, 0xf7], ASM::mov(DI, SI));

    assert_eq!(vec![0x89, 0xf8], ASM::mov(AX, DI));
    assert_eq!(vec![0x89, 0xf9], ASM::mov(CX, DI));
    assert_eq!(vec![0x89, 0xfa], ASM::mov(DX, DI));
    assert_eq!(vec![0x89, 0xfb], ASM::mov(BX, DI));
    assert_eq!(vec![0x89, 0xfc], ASM::mov(SP, DI));
    assert_eq!(vec![0x89, 0xfd], ASM::mov(BP, DI));
    assert_eq!(vec![0x89, 0xfe], ASM::mov(SI, DI));
    assert_eq!(vec![0x89, 0xff], ASM::mov(DI, DI));
}

#[test]
fn real_address_mode_mov_instructions_from_memory_to_gp() {
    // Move memory to 8-bit registers
    assert_eq!(vec![0xa0, 0x00, 0x00], ASM::mov(AL, [0]));
    assert_eq!(vec![0x8a, 0x0e, 0x00, 0x00], ASM::mov(CL, [0]));
    assert_eq!(vec![0x8a, 0x16, 0x00, 0x00], ASM::mov(DL, [0]));
    assert_eq!(vec![0x8a, 0x1e, 0x00, 0x00], ASM::mov(BL, [0]));
    assert_eq!(vec![0x8a, 0x26, 0x00, 0x00], ASM::mov(AH, [0]));
    assert_eq!(vec![0x8a, 0x2e, 0x00, 0x00], ASM::mov(CH, [0]));
    assert_eq!(vec![0x8a, 0x36, 0x00, 0x00], ASM::mov(DH, [0]));
    assert_eq!(vec![0x8a, 0x3e, 0x00, 0x00], ASM::mov(BH, [0]));

    assert_eq!(vec![0xa0, 0x64, 0x00], ASM::mov(AL, [100]));
    assert_eq!(vec![0x8a, 0x0e, 0x64, 0x00], ASM::mov(CL, [100]));
    assert_eq!(vec![0x8a, 0x16, 0x64, 0x00], ASM::mov(DL, [100]));
    assert_eq!(vec![0x8a, 0x1e, 0x64, 0x00], ASM::mov(BL, [100]));
    assert_eq!(vec![0x8a, 0x26, 0x64, 0x00], ASM::mov(AH, [100]));
    assert_eq!(vec![0x8a, 0x2e, 0x64, 0x00], ASM::mov(CH, [100]));
    assert_eq!(vec![0x8a, 0x36, 0x64, 0x00], ASM::mov(DH, [100]));
    assert_eq!(vec![0x8a, 0x3e, 0x64, 0x00], ASM::mov(BH, [100]));

    assert_eq!(vec![0xa0, 0x10, 0x27], ASM::mov(AL, [10000]));
    assert_eq!(vec![0x8a, 0x0e, 0x10, 0x27], ASM::mov(CL, [10000]));
    assert_eq!(vec![0x8a, 0x16, 0x10, 0x27], ASM::mov(DL, [10000]));
    assert_eq!(vec![0x8a, 0x1e, 0x10, 0x27], ASM::mov(BL, [10000]));
    assert_eq!(vec![0x8a, 0x26, 0x10, 0x27], ASM::mov(AH, [10000]));
    assert_eq!(vec![0x8a, 0x2e, 0x10, 0x27], ASM::mov(CH, [10000]));
    assert_eq!(vec![0x8a, 0x36, 0x10, 0x27], ASM::mov(DH, [10000]));
    assert_eq!(vec![0x8a, 0x3e, 0x10, 0x27], ASM::mov(BH, [10000]));

    assert_eq!(vec![0x8a, 0x07], ASM::mov(AL, [BX]));
    assert_eq!(vec![0x8a, 0x0f], ASM::mov(CL, [BX]));
    assert_eq!(vec![0x8a, 0x17], ASM::mov(DL, [BX]));
    assert_eq!(vec![0x8a, 0x1f], ASM::mov(BL, [BX]));
    assert_eq!(vec![0x8a, 0x27], ASM::mov(AH, [BX]));
    assert_eq!(vec![0x8a, 0x2f], ASM::mov(CH, [BX]));
    assert_eq!(vec![0x8a, 0x37], ASM::mov(DH, [BX]));
    assert_eq!(vec![0x8a, 0x3f], ASM::mov(BH, [BX]));

    assert_eq!(vec![0x8a, 0x46, 0x00], ASM::mov(AL, [BP]));
    assert_eq!(vec![0x8a, 0x4e, 0x00], ASM::mov(CL, [BP]));
    assert_eq!(vec![0x8a, 0x56, 0x00], ASM::mov(DL, [BP]));
    assert_eq!(vec![0x8a, 0x5e, 0x00], ASM::mov(BL, [BP]));
    assert_eq!(vec![0x8a, 0x66, 0x00], ASM::mov(AH, [BP]));
    assert_eq!(vec![0x8a, 0x6e, 0x00], ASM::mov(CH, [BP]));
    assert_eq!(vec![0x8a, 0x76, 0x00], ASM::mov(DH, [BP]));
    assert_eq!(vec![0x8a, 0x7e, 0x00], ASM::mov(BH, [BP]));

    assert_eq!(vec![0x8a, 0x04], ASM::mov(AL, [SI]));
    assert_eq!(vec![0x8a, 0x0c], ASM::mov(CL, [SI]));
    assert_eq!(vec![0x8a, 0x14], ASM::mov(DL, [SI]));
    assert_eq!(vec![0x8a, 0x1c], ASM::mov(BL, [SI]));
    assert_eq!(vec![0x8a, 0x24], ASM::mov(AH, [SI]));
    assert_eq!(vec![0x8a, 0x2c], ASM::mov(CH, [SI]));
    assert_eq!(vec![0x8a, 0x34], ASM::mov(DH, [SI]));
    assert_eq!(vec![0x8a, 0x3c], ASM::mov(BH, [SI]));

    assert_eq!(vec![0x8a, 0x05], ASM::mov(AL, [DI]));
    assert_eq!(vec![0x8a, 0x0d], ASM::mov(CL, [DI]));
    assert_eq!(vec![0x8a, 0x15], ASM::mov(DL, [DI]));
    assert_eq!(vec![0x8a, 0x1d], ASM::mov(BL, [DI]));
    assert_eq!(vec![0x8a, 0x25], ASM::mov(AH, [DI]));
    assert_eq!(vec![0x8a, 0x2d], ASM::mov(CH, [DI]));
    assert_eq!(vec![0x8a, 0x35], ASM::mov(DH, [DI]));
    assert_eq!(vec![0x8a, 0x3d], ASM::mov(BH, [DI]));

    // Move memory to 16-bit registers
    assert_eq!(vec![0xa1, 0x00, 0x00], ASM::mov(AX, [0]));
    assert_eq!(vec![0x8b, 0x0e, 0x00, 0x00], ASM::mov(CX, [0]));
    assert_eq!(vec![0x8b, 0x16, 0x00, 0x00], ASM::mov(DX, [0]));
    assert_eq!(vec![0x8b, 0x1e, 0x00, 0x00], ASM::mov(BX, [0]));
    assert_eq!(vec![0x8b, 0x26, 0x00, 0x00], ASM::mov(SP, [0]));
    assert_eq!(vec![0x8b, 0x2e, 0x00, 0x00], ASM::mov(BP, [0]));
    assert_eq!(vec![0x8b, 0x36, 0x00, 0x00], ASM::mov(SI, [0]));
    assert_eq!(vec![0x8b, 0x3e, 0x00, 0x00], ASM::mov(DI, [0]));

    assert_eq!(vec![0xa1, 0x64, 0x00], ASM::mov(AX, [100]));
    assert_eq!(vec![0x8b, 0x0e, 0x64, 0x00], ASM::mov(CX, [100]));
    assert_eq!(vec![0x8b, 0x16, 0x64, 0x00], ASM::mov(DX, [100]));
    assert_eq!(vec![0x8b, 0x1e, 0x64, 0x00], ASM::mov(BX, [100]));
    assert_eq!(vec![0x8b, 0x26, 0x64, 0x00], ASM::mov(SP, [100]));
    assert_eq!(vec![0x8b, 0x2e, 0x64, 0x00], ASM::mov(BP, [100]));
    assert_eq!(vec![0x8b, 0x36, 0x64, 0x00], ASM::mov(SI, [100]));
    assert_eq!(vec![0x8b, 0x3e, 0x64, 0x00], ASM::mov(DI, [100]));

    assert_eq!(vec![0xa1, 0x10, 0x27], ASM::mov(AX, [10000]));
    assert_eq!(vec![0x8b, 0x0e, 0x10, 0x27], ASM::mov(CX, [10000]));
    assert_eq!(vec![0x8b, 0x16, 0x10, 0x27], ASM::mov(DX, [10000]));
    assert_eq!(vec![0x8b, 0x1e, 0x10, 0x27], ASM::mov(BX, [10000]));
    assert_eq!(vec![0x8b, 0x26, 0x10, 0x27], ASM::mov(SP, [10000]));
    assert_eq!(vec![0x8b, 0x2e, 0x10, 0x27], ASM::mov(BP, [10000]));
    assert_eq!(vec![0x8b, 0x36, 0x10, 0x27], ASM::mov(SI, [10000]));
    assert_eq!(vec![0x8b, 0x3e, 0x10, 0x27], ASM::mov(DI, [10000]));

    assert_eq!(vec![0x8b, 0x07], ASM::mov(AX, [BX]));
    assert_eq!(vec![0x8b, 0x0f], ASM::mov(CX, [BX]));
    assert_eq!(vec![0x8b, 0x17], ASM::mov(DX, [BX]));
    assert_eq!(vec![0x8b, 0x1f], ASM::mov(BX, [BX]));
    assert_eq!(vec![0x8b, 0x27], ASM::mov(SP, [BX]));
    assert_eq!(vec![0x8b, 0x2f], ASM::mov(BP, [BX]));
    assert_eq!(vec![0x8b, 0x37], ASM::mov(SI, [BX]));
    assert_eq!(vec![0x8b, 0x3f], ASM::mov(DI, [BX]));

    assert_eq!(vec![0x8b, 0x46, 0x00], ASM::mov(AX, [BP]));
    assert_eq!(vec![0x8b, 0x4e, 0x00], ASM::mov(CX, [BP]));
    assert_eq!(vec![0x8b, 0x56, 0x00], ASM::mov(DX, [BP]));
    assert_eq!(vec![0x8b, 0x5e, 0x00], ASM::mov(BX, [BP]));
    assert_eq!(vec![0x8b, 0x66, 0x00], ASM::mov(SP, [BP]));
    assert_eq!(vec![0x8b, 0x6e, 0x00], ASM::mov(BP, [BP]));
    assert_eq!(vec![0x8b, 0x76, 0x00], ASM::mov(SI, [BP]));
    assert_eq!(vec![0x8b, 0x7e, 0x00], ASM::mov(DI, [BP]));

    assert_eq!(vec![0x8b, 0x04], ASM::mov(AX, [SI]));
    assert_eq!(vec![0x8b, 0x0c], ASM::mov(CX, [SI]));
    assert_eq!(vec![0x8b, 0x14], ASM::mov(DX, [SI]));
    assert_eq!(vec![0x8b, 0x1c], ASM::mov(BX, [SI]));
    assert_eq!(vec![0x8b, 0x24], ASM::mov(SP, [SI]));
    assert_eq!(vec![0x8b, 0x2c], ASM::mov(BP, [SI]));
    assert_eq!(vec![0x8b, 0x34], ASM::mov(SI, [SI]));
    assert_eq!(vec![0x8b, 0x3c], ASM::mov(DI, [SI]));

    assert_eq!(vec![0x8b, 0x05], ASM::mov(AX, [DI]));
    assert_eq!(vec![0x8b, 0x0d], ASM::mov(CX, [DI]));
    assert_eq!(vec![0x8b, 0x15], ASM::mov(DX, [DI]));
    assert_eq!(vec![0x8b, 0x1d], ASM::mov(BX, [DI]));
    assert_eq!(vec![0x8b, 0x25], ASM::mov(SP, [DI]));
    assert_eq!(vec![0x8b, 0x2d], ASM::mov(BP, [DI]));
    assert_eq!(vec![0x8b, 0x35], ASM::mov(SI, [DI]));
    assert_eq!(vec![0x8b, 0x3d], ASM::mov(DI, [DI]));
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
    assert_eq!(vec![0xb0, 0x00], ASM::mov(AL, Imm8(0)));
    assert_eq!(vec![0xb1, 0x00], ASM::mov(CL, Imm8(0)));
    assert_eq!(vec![0xb2, 0x00], ASM::mov(DL, Imm8(0)));
    assert_eq!(vec![0xb3, 0x00], ASM::mov(BL, Imm8(0)));
    assert_eq!(vec![0xb4, 0x00], ASM::mov(AH, Imm8(0)));
    assert_eq!(vec![0xb5, 0x00], ASM::mov(CH, Imm8(0)));
    assert_eq!(vec![0xb6, 0x00], ASM::mov(DH, Imm8(0)));
    assert_eq!(vec![0xb7, 0x00], ASM::mov(BH, Imm8(0)));

    assert_eq!(vec![0xb0, 0x0c], ASM::mov(AL, Imm8(12)));
    assert_eq!(vec![0xb1, 0x0c], ASM::mov(CL, Imm8(12)));
    assert_eq!(vec![0xb2, 0x0c], ASM::mov(DL, Imm8(12)));
    assert_eq!(vec![0xb3, 0x0c], ASM::mov(BL, Imm8(12)));
    assert_eq!(vec![0xb4, 0x0c], ASM::mov(AH, Imm8(12)));
    assert_eq!(vec![0xb5, 0x0c], ASM::mov(CH, Imm8(12)));
    assert_eq!(vec![0xb6, 0x0c], ASM::mov(DH, Imm8(12)));
    assert_eq!(vec![0xb7, 0x0c], ASM::mov(BH, Imm8(12)));

    assert_eq!(vec![0xb0, 0x10], ASM::mov(AL, Imm8(16)));
    assert_eq!(vec![0xb1, 0x10], ASM::mov(CL, Imm8(16)));
    assert_eq!(vec![0xb2, 0x10], ASM::mov(DL, Imm8(16)));
    assert_eq!(vec![0xb3, 0x10], ASM::mov(BL, Imm8(16)));
    assert_eq!(vec![0xb4, 0x10], ASM::mov(AH, Imm8(16)));
    assert_eq!(vec![0xb5, 0x10], ASM::mov(CH, Imm8(16)));
    assert_eq!(vec![0xb6, 0x10], ASM::mov(DH, Imm8(16)));
    assert_eq!(vec![0xb7, 0x10], ASM::mov(BH, Imm8(16)));

    assert_eq!(vec![0xb0, 0x96], ASM::mov(AL, Imm8(150)));
    assert_eq!(vec![0xb1, 0x96], ASM::mov(CL, Imm8(150)));
    assert_eq!(vec![0xb2, 0x96], ASM::mov(DL, Imm8(150)));
    assert_eq!(vec![0xb3, 0x96], ASM::mov(BL, Imm8(150)));
    assert_eq!(vec![0xb4, 0x96], ASM::mov(AH, Imm8(150)));
    assert_eq!(vec![0xb5, 0x96], ASM::mov(CH, Imm8(150)));
    assert_eq!(vec![0xb6, 0x96], ASM::mov(DH, Imm8(150)));
    assert_eq!(vec![0xb7, 0x96], ASM::mov(BH, Imm8(150)));

    // Move values to 16-bit registers
    assert_eq!(vec![0xb8, 0x00, 0x00], ASM::mov(AX, Imm16(0)));
    assert_eq!(vec![0xb9, 0x00, 0x00], ASM::mov(CX, Imm16(0)));
    assert_eq!(vec![0xba, 0x00, 0x00], ASM::mov(DX, Imm16(0)));
    assert_eq!(vec![0xbb, 0x00, 0x00], ASM::mov(BX, Imm16(0)));
    assert_eq!(vec![0xbc, 0x00, 0x00], ASM::mov(SP, Imm16(0)));
    assert_eq!(vec![0xbd, 0x00, 0x00], ASM::mov(BP, Imm16(0)));
    assert_eq!(vec![0xbe, 0x00, 0x00], ASM::mov(SI, Imm16(0)));
    assert_eq!(vec![0xbf, 0x00, 0x00], ASM::mov(DI, Imm16(0)));

    assert_eq!(vec![0xb8, 0x0c, 0x00], ASM::mov(AX, Imm16(12)));
    assert_eq!(vec![0xb9, 0x0c, 0x00], ASM::mov(CX, Imm16(12)));
    assert_eq!(vec![0xba, 0x0c, 0x00], ASM::mov(DX, Imm16(12)));
    assert_eq!(vec![0xbb, 0x0c, 0x00], ASM::mov(BX, Imm16(12)));
    assert_eq!(vec![0xbc, 0x0c, 0x00], ASM::mov(SP, Imm16(12)));
    assert_eq!(vec![0xbd, 0x0c, 0x00], ASM::mov(BP, Imm16(12)));
    assert_eq!(vec![0xbe, 0x0c, 0x00], ASM::mov(SI, Imm16(12)));
    assert_eq!(vec![0xbf, 0x0c, 0x00], ASM::mov(DI, Imm16(12)));

    assert_eq!(vec![0xb8, 0x10, 0x00], ASM::mov(AX, Imm16(16)));
    assert_eq!(vec![0xb9, 0x10, 0x00], ASM::mov(CX, Imm16(16)));
    assert_eq!(vec![0xba, 0x10, 0x00], ASM::mov(DX, Imm16(16)));
    assert_eq!(vec![0xbb, 0x10, 0x00], ASM::mov(BX, Imm16(16)));
    assert_eq!(vec![0xbc, 0x10, 0x00], ASM::mov(SP, Imm16(16)));
    assert_eq!(vec![0xbd, 0x10, 0x00], ASM::mov(BP, Imm16(16)));
    assert_eq!(vec![0xbe, 0x10, 0x00], ASM::mov(SI, Imm16(16)));
    assert_eq!(vec![0xbf, 0x10, 0x00], ASM::mov(DI, Imm16(16)));

    assert_eq!(vec![0xb8, 0x96, 0x00], ASM::mov(AX, Imm16(150)));
    assert_eq!(vec![0xb9, 0x96, 0x00], ASM::mov(CX, Imm16(150)));
    assert_eq!(vec![0xba, 0x96, 0x00], ASM::mov(DX, Imm16(150)));
    assert_eq!(vec![0xbb, 0x96, 0x00], ASM::mov(BX, Imm16(150)));
    assert_eq!(vec![0xbc, 0x96, 0x00], ASM::mov(SP, Imm16(150)));
    assert_eq!(vec![0xbd, 0x96, 0x00], ASM::mov(BP, Imm16(150)));
    assert_eq!(vec![0xbe, 0x96, 0x00], ASM::mov(SI, Imm16(150)));
    assert_eq!(vec![0xbf, 0x96, 0x00], ASM::mov(DI, Imm16(150)));

    assert_eq!(vec![0xb8, 0xe8, 0x03], ASM::mov(AX, Imm16(1000)));
    assert_eq!(vec![0xb9, 0xe8, 0x03], ASM::mov(CX, Imm16(1000)));
    assert_eq!(vec![0xba, 0xe8, 0x03], ASM::mov(DX, Imm16(1000)));
    assert_eq!(vec![0xbb, 0xe8, 0x03], ASM::mov(BX, Imm16(1000)));
    assert_eq!(vec![0xbc, 0xe8, 0x03], ASM::mov(SP, Imm16(1000)));
    assert_eq!(vec![0xbd, 0xe8, 0x03], ASM::mov(BP, Imm16(1000)));
    assert_eq!(vec![0xbe, 0xe8, 0x03], ASM::mov(SI, Imm16(1000)));
    assert_eq!(vec![0xbf, 0xe8, 0x03], ASM::mov(DI, Imm16(1000)));

    assert_eq!(vec![0xb8, 0x00, 0x80], ASM::mov(AX, Imm16(32768)));
    assert_eq!(vec![0xb9, 0x00, 0x80], ASM::mov(CX, Imm16(32768)));
    assert_eq!(vec![0xba, 0x00, 0x80], ASM::mov(DX, Imm16(32768)));
    assert_eq!(vec![0xbb, 0x00, 0x80], ASM::mov(BX, Imm16(32768)));
    assert_eq!(vec![0xbc, 0x00, 0x80], ASM::mov(SP, Imm16(32768)));
    assert_eq!(vec![0xbd, 0x00, 0x80], ASM::mov(BP, Imm16(32768)));
    assert_eq!(vec![0xbe, 0x00, 0x80], ASM::mov(SI, Imm16(32768)));
    assert_eq!(vec![0xbf, 0x00, 0x80], ASM::mov(DI, Imm16(32768)));

    assert_eq!(vec![0xb8, 0x50, 0xc3], ASM::mov(AX, Imm16(50000)));
    assert_eq!(vec![0xb9, 0x50, 0xc3], ASM::mov(CX, Imm16(50000)));
    assert_eq!(vec![0xba, 0x50, 0xc3], ASM::mov(DX, Imm16(50000)));
    assert_eq!(vec![0xbb, 0x50, 0xc3], ASM::mov(BX, Imm16(50000)));
    assert_eq!(vec![0xbc, 0x50, 0xc3], ASM::mov(SP, Imm16(50000)));
    assert_eq!(vec![0xbd, 0x50, 0xc3], ASM::mov(BP, Imm16(50000)));
    assert_eq!(vec![0xbe, 0x50, 0xc3], ASM::mov(SI, Imm16(50000)));
    assert_eq!(vec![0xbf, 0x50, 0xc3], ASM::mov(DI, Imm16(50000)));
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
