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
    let asm = ASM::new();

    // Move between 8-bit registers
    assert_eq!(vec![0x88, 0xc0], asm.mov(AL, AL));
    assert_eq!(vec![0x88, 0xc1], asm.mov(CL, AL));
    assert_eq!(vec![0x88, 0xc2], asm.mov(DL, AL));
    assert_eq!(vec![0x88, 0xc3], asm.mov(BL, AL));
    assert_eq!(vec![0x88, 0xc4], asm.mov(AH, AL));
    assert_eq!(vec![0x88, 0xc5], asm.mov(CH, AL));
    assert_eq!(vec![0x88, 0xc6], asm.mov(DH, AL));
    assert_eq!(vec![0x88, 0xc7], asm.mov(BH, AL));

    assert_eq!(vec![0x88, 0xc8], asm.mov(AL, CL));
    assert_eq!(vec![0x88, 0xc9], asm.mov(CL, CL));
    assert_eq!(vec![0x88, 0xca], asm.mov(DL, CL));
    assert_eq!(vec![0x88, 0xcb], asm.mov(BL, CL));
    assert_eq!(vec![0x88, 0xcc], asm.mov(AH, CL));
    assert_eq!(vec![0x88, 0xcd], asm.mov(CH, CL));
    assert_eq!(vec![0x88, 0xce], asm.mov(DH, CL));
    assert_eq!(vec![0x88, 0xcf], asm.mov(BH, CL));

    assert_eq!(vec![0x88, 0xd0], asm.mov(AL, DL));
    assert_eq!(vec![0x88, 0xd1], asm.mov(CL, DL));
    assert_eq!(vec![0x88, 0xd2], asm.mov(DL, DL));
    assert_eq!(vec![0x88, 0xd3], asm.mov(BL, DL));
    assert_eq!(vec![0x88, 0xd4], asm.mov(AH, DL));
    assert_eq!(vec![0x88, 0xd5], asm.mov(CH, DL));
    assert_eq!(vec![0x88, 0xd6], asm.mov(DH, DL));
    assert_eq!(vec![0x88, 0xd7], asm.mov(BH, DL));

    assert_eq!(vec![0x88, 0xd8], asm.mov(AL, BL));
    assert_eq!(vec![0x88, 0xd9], asm.mov(CL, BL));
    assert_eq!(vec![0x88, 0xda], asm.mov(DL, BL));
    assert_eq!(vec![0x88, 0xdb], asm.mov(BL, BL));
    assert_eq!(vec![0x88, 0xdc], asm.mov(AH, BL));
    assert_eq!(vec![0x88, 0xdd], asm.mov(CH, BL));
    assert_eq!(vec![0x88, 0xde], asm.mov(DH, BL));
    assert_eq!(vec![0x88, 0xdf], asm.mov(BH, BL));

    assert_eq!(vec![0x88, 0xe0], asm.mov(AL, AH));
    assert_eq!(vec![0x88, 0xe1], asm.mov(CL, AH));
    assert_eq!(vec![0x88, 0xe2], asm.mov(DL, AH));
    assert_eq!(vec![0x88, 0xe3], asm.mov(BL, AH));
    assert_eq!(vec![0x88, 0xe4], asm.mov(AH, AH));
    assert_eq!(vec![0x88, 0xe5], asm.mov(CH, AH));
    assert_eq!(vec![0x88, 0xe6], asm.mov(DH, AH));
    assert_eq!(vec![0x88, 0xe7], asm.mov(BH, AH));

    assert_eq!(vec![0x88, 0xe8], asm.mov(AL, CH));
    assert_eq!(vec![0x88, 0xe9], asm.mov(CL, CH));
    assert_eq!(vec![0x88, 0xea], asm.mov(DL, CH));
    assert_eq!(vec![0x88, 0xeb], asm.mov(BL, CH));
    assert_eq!(vec![0x88, 0xec], asm.mov(AH, CH));
    assert_eq!(vec![0x88, 0xed], asm.mov(CH, CH));
    assert_eq!(vec![0x88, 0xee], asm.mov(DH, CH));
    assert_eq!(vec![0x88, 0xef], asm.mov(BH, CH));

    assert_eq!(vec![0x88, 0xf0], asm.mov(AL, DH));
    assert_eq!(vec![0x88, 0xf1], asm.mov(CL, DH));
    assert_eq!(vec![0x88, 0xf2], asm.mov(DL, DH));
    assert_eq!(vec![0x88, 0xf3], asm.mov(BL, DH));
    assert_eq!(vec![0x88, 0xf4], asm.mov(AH, DH));
    assert_eq!(vec![0x88, 0xf5], asm.mov(CH, DH));
    assert_eq!(vec![0x88, 0xf6], asm.mov(DH, DH));
    assert_eq!(vec![0x88, 0xf7], asm.mov(BH, DH));

    assert_eq!(vec![0x88, 0xf8], asm.mov(AL, BH));
    assert_eq!(vec![0x88, 0xf9], asm.mov(CL, BH));
    assert_eq!(vec![0x88, 0xfa], asm.mov(DL, BH));
    assert_eq!(vec![0x88, 0xfb], asm.mov(BL, BH));
    assert_eq!(vec![0x88, 0xfc], asm.mov(AH, BH));
    assert_eq!(vec![0x88, 0xfd], asm.mov(CH, BH));
    assert_eq!(vec![0x88, 0xfe], asm.mov(DH, BH));
    assert_eq!(vec![0x88, 0xff], asm.mov(BH, BH));

    // Move between 16-bit registers
    assert_eq!(vec![0x89, 0xc0], asm.mov(AX, AX));
    assert_eq!(vec![0x89, 0xc1], asm.mov(CX, AX));
    assert_eq!(vec![0x89, 0xc2], asm.mov(DX, AX));
    assert_eq!(vec![0x89, 0xc3], asm.mov(BX, AX));
    assert_eq!(vec![0x89, 0xc4], asm.mov(SP, AX));
    assert_eq!(vec![0x89, 0xc5], asm.mov(BP, AX));
    assert_eq!(vec![0x89, 0xc6], asm.mov(SI, AX));
    assert_eq!(vec![0x89, 0xc7], asm.mov(DI, AX));

    assert_eq!(vec![0x89, 0xc8], asm.mov(AX, CX));
    assert_eq!(vec![0x89, 0xc9], asm.mov(CX, CX));
    assert_eq!(vec![0x89, 0xca], asm.mov(DX, CX));
    assert_eq!(vec![0x89, 0xcb], asm.mov(BX, CX));
    assert_eq!(vec![0x89, 0xcc], asm.mov(SP, CX));
    assert_eq!(vec![0x89, 0xcd], asm.mov(BP, CX));
    assert_eq!(vec![0x89, 0xce], asm.mov(SI, CX));
    assert_eq!(vec![0x89, 0xcf], asm.mov(DI, CX));

    assert_eq!(vec![0x89, 0xd0], asm.mov(AX, DX));
    assert_eq!(vec![0x89, 0xd1], asm.mov(CX, DX));
    assert_eq!(vec![0x89, 0xd2], asm.mov(DX, DX));
    assert_eq!(vec![0x89, 0xd3], asm.mov(BX, DX));
    assert_eq!(vec![0x89, 0xd4], asm.mov(SP, DX));
    assert_eq!(vec![0x89, 0xd5], asm.mov(BP, DX));
    assert_eq!(vec![0x89, 0xd6], asm.mov(SI, DX));
    assert_eq!(vec![0x89, 0xd7], asm.mov(DI, DX));

    assert_eq!(vec![0x89, 0xd8], asm.mov(AX, BX));
    assert_eq!(vec![0x89, 0xd9], asm.mov(CX, BX));
    assert_eq!(vec![0x89, 0xda], asm.mov(DX, BX));
    assert_eq!(vec![0x89, 0xdb], asm.mov(BX, BX));
    assert_eq!(vec![0x89, 0xdc], asm.mov(SP, BX));
    assert_eq!(vec![0x89, 0xdd], asm.mov(BP, BX));
    assert_eq!(vec![0x89, 0xde], asm.mov(SI, BX));
    assert_eq!(vec![0x89, 0xdf], asm.mov(DI, BX));

    assert_eq!(vec![0x89, 0xe0], asm.mov(AX, SP));
    assert_eq!(vec![0x89, 0xe1], asm.mov(CX, SP));
    assert_eq!(vec![0x89, 0xe2], asm.mov(DX, SP));
    assert_eq!(vec![0x89, 0xe3], asm.mov(BX, SP));
    assert_eq!(vec![0x89, 0xe4], asm.mov(SP, SP));
    assert_eq!(vec![0x89, 0xe5], asm.mov(BP, SP));
    assert_eq!(vec![0x89, 0xe6], asm.mov(SI, SP));
    assert_eq!(vec![0x89, 0xe7], asm.mov(DI, SP));

    assert_eq!(vec![0x89, 0xe8], asm.mov(AX, BP));
    assert_eq!(vec![0x89, 0xe9], asm.mov(CX, BP));
    assert_eq!(vec![0x89, 0xea], asm.mov(DX, BP));
    assert_eq!(vec![0x89, 0xeb], asm.mov(BX, BP));
    assert_eq!(vec![0x89, 0xec], asm.mov(SP, BP));
    assert_eq!(vec![0x89, 0xed], asm.mov(BP, BP));
    assert_eq!(vec![0x89, 0xee], asm.mov(SI, BP));
    assert_eq!(vec![0x89, 0xef], asm.mov(DI, BP));

    assert_eq!(vec![0x89, 0xf0], asm.mov(AX, SI));
    assert_eq!(vec![0x89, 0xf1], asm.mov(CX, SI));
    assert_eq!(vec![0x89, 0xf2], asm.mov(DX, SI));
    assert_eq!(vec![0x89, 0xf3], asm.mov(BX, SI));
    assert_eq!(vec![0x89, 0xf4], asm.mov(SP, SI));
    assert_eq!(vec![0x89, 0xf5], asm.mov(BP, SI));
    assert_eq!(vec![0x89, 0xf6], asm.mov(SI, SI));
    assert_eq!(vec![0x89, 0xf7], asm.mov(DI, SI));

    assert_eq!(vec![0x89, 0xf8], asm.mov(AX, DI));
    assert_eq!(vec![0x89, 0xf9], asm.mov(CX, DI));
    assert_eq!(vec![0x89, 0xfa], asm.mov(DX, DI));
    assert_eq!(vec![0x89, 0xfb], asm.mov(BX, DI));
    assert_eq!(vec![0x89, 0xfc], asm.mov(SP, DI));
    assert_eq!(vec![0x89, 0xfd], asm.mov(BP, DI));
    assert_eq!(vec![0x89, 0xfe], asm.mov(SI, DI));
    assert_eq!(vec![0x89, 0xff], asm.mov(DI, DI));
}

#[test]
fn real_address_mode_mov_instructions_from_memory_to_gp() {
    let asm = ASM::new();

    // Move memory to 8-bit registers
    assert_eq!(vec![0xa0, 0x00, 0x00], asm.mov(AL, [0]));
    assert_eq!(vec![0x8a, 0x0e, 0x00, 0x00], asm.mov(CL, [0]));
    assert_eq!(vec![0x8a, 0x16, 0x00, 0x00], asm.mov(DL, [0]));
    assert_eq!(vec![0x8a, 0x1e, 0x00, 0x00], asm.mov(BL, [0]));
    assert_eq!(vec![0x8a, 0x26, 0x00, 0x00], asm.mov(AH, [0]));
    assert_eq!(vec![0x8a, 0x2e, 0x00, 0x00], asm.mov(CH, [0]));
    assert_eq!(vec![0x8a, 0x36, 0x00, 0x00], asm.mov(DH, [0]));
    assert_eq!(vec![0x8a, 0x3e, 0x00, 0x00], asm.mov(BH, [0]));

    assert_eq!(vec![0xa0, 0x64, 0x00], asm.mov(AL, [100]));
    assert_eq!(vec![0x8a, 0x0e, 0x64, 0x00], asm.mov(CL, [100]));
    assert_eq!(vec![0x8a, 0x16, 0x64, 0x00], asm.mov(DL, [100]));
    assert_eq!(vec![0x8a, 0x1e, 0x64, 0x00], asm.mov(BL, [100]));
    assert_eq!(vec![0x8a, 0x26, 0x64, 0x00], asm.mov(AH, [100]));
    assert_eq!(vec![0x8a, 0x2e, 0x64, 0x00], asm.mov(CH, [100]));
    assert_eq!(vec![0x8a, 0x36, 0x64, 0x00], asm.mov(DH, [100]));
    assert_eq!(vec![0x8a, 0x3e, 0x64, 0x00], asm.mov(BH, [100]));

    assert_eq!(vec![0xa0, 0x10, 0x27], asm.mov(AL, [10000]));
    assert_eq!(vec![0x8a, 0x0e, 0x10, 0x27], asm.mov(CL, [10000]));
    assert_eq!(vec![0x8a, 0x16, 0x10, 0x27], asm.mov(DL, [10000]));
    assert_eq!(vec![0x8a, 0x1e, 0x10, 0x27], asm.mov(BL, [10000]));
    assert_eq!(vec![0x8a, 0x26, 0x10, 0x27], asm.mov(AH, [10000]));
    assert_eq!(vec![0x8a, 0x2e, 0x10, 0x27], asm.mov(CH, [10000]));
    assert_eq!(vec![0x8a, 0x36, 0x10, 0x27], asm.mov(DH, [10000]));
    assert_eq!(vec![0x8a, 0x3e, 0x10, 0x27], asm.mov(BH, [10000]));

    assert_eq!(vec![0x8a, 0x07], asm.mov(AL, [BX]));
    assert_eq!(vec![0x8a, 0x0f], asm.mov(CL, [BX]));
    assert_eq!(vec![0x8a, 0x17], asm.mov(DL, [BX]));
    assert_eq!(vec![0x8a, 0x1f], asm.mov(BL, [BX]));
    assert_eq!(vec![0x8a, 0x27], asm.mov(AH, [BX]));
    assert_eq!(vec![0x8a, 0x2f], asm.mov(CH, [BX]));
    assert_eq!(vec![0x8a, 0x37], asm.mov(DH, [BX]));
    assert_eq!(vec![0x8a, 0x3f], asm.mov(BH, [BX]));

    assert_eq!(vec![0x8a, 0x46, 0x00], asm.mov(AL, [BP]));
    assert_eq!(vec![0x8a, 0x4e, 0x00], asm.mov(CL, [BP]));
    assert_eq!(vec![0x8a, 0x56, 0x00], asm.mov(DL, [BP]));
    assert_eq!(vec![0x8a, 0x5e, 0x00], asm.mov(BL, [BP]));
    assert_eq!(vec![0x8a, 0x66, 0x00], asm.mov(AH, [BP]));
    assert_eq!(vec![0x8a, 0x6e, 0x00], asm.mov(CH, [BP]));
    assert_eq!(vec![0x8a, 0x76, 0x00], asm.mov(DH, [BP]));
    assert_eq!(vec![0x8a, 0x7e, 0x00], asm.mov(BH, [BP]));

    assert_eq!(vec![0x8a, 0x04], asm.mov(AL, [SI]));
    assert_eq!(vec![0x8a, 0x0c], asm.mov(CL, [SI]));
    assert_eq!(vec![0x8a, 0x14], asm.mov(DL, [SI]));
    assert_eq!(vec![0x8a, 0x1c], asm.mov(BL, [SI]));
    assert_eq!(vec![0x8a, 0x24], asm.mov(AH, [SI]));
    assert_eq!(vec![0x8a, 0x2c], asm.mov(CH, [SI]));
    assert_eq!(vec![0x8a, 0x34], asm.mov(DH, [SI]));
    assert_eq!(vec![0x8a, 0x3c], asm.mov(BH, [SI]));

    assert_eq!(vec![0x8a, 0x05], asm.mov(AL, [DI]));
    assert_eq!(vec![0x8a, 0x0d], asm.mov(CL, [DI]));
    assert_eq!(vec![0x8a, 0x15], asm.mov(DL, [DI]));
    assert_eq!(vec![0x8a, 0x1d], asm.mov(BL, [DI]));
    assert_eq!(vec![0x8a, 0x25], asm.mov(AH, [DI]));
    assert_eq!(vec![0x8a, 0x2d], asm.mov(CH, [DI]));
    assert_eq!(vec![0x8a, 0x35], asm.mov(DH, [DI]));
    assert_eq!(vec![0x8a, 0x3d], asm.mov(BH, [DI]));

    // Move memory to 16-bit registers
    assert_eq!(vec![0xa1, 0x00, 0x00], asm.mov(AX, [0]));
    assert_eq!(vec![0x8b, 0x0e, 0x00, 0x00], asm.mov(CX, [0]));
    assert_eq!(vec![0x8b, 0x16, 0x00, 0x00], asm.mov(DX, [0]));
    assert_eq!(vec![0x8b, 0x1e, 0x00, 0x00], asm.mov(BX, [0]));
    assert_eq!(vec![0x8b, 0x26, 0x00, 0x00], asm.mov(SP, [0]));
    assert_eq!(vec![0x8b, 0x2e, 0x00, 0x00], asm.mov(BP, [0]));
    assert_eq!(vec![0x8b, 0x36, 0x00, 0x00], asm.mov(SI, [0]));
    assert_eq!(vec![0x8b, 0x3e, 0x00, 0x00], asm.mov(DI, [0]));

    assert_eq!(vec![0xa1, 0x64, 0x00], asm.mov(AX, [100]));
    assert_eq!(vec![0x8b, 0x0e, 0x64, 0x00], asm.mov(CX, [100]));
    assert_eq!(vec![0x8b, 0x16, 0x64, 0x00], asm.mov(DX, [100]));
    assert_eq!(vec![0x8b, 0x1e, 0x64, 0x00], asm.mov(BX, [100]));
    assert_eq!(vec![0x8b, 0x26, 0x64, 0x00], asm.mov(SP, [100]));
    assert_eq!(vec![0x8b, 0x2e, 0x64, 0x00], asm.mov(BP, [100]));
    assert_eq!(vec![0x8b, 0x36, 0x64, 0x00], asm.mov(SI, [100]));
    assert_eq!(vec![0x8b, 0x3e, 0x64, 0x00], asm.mov(DI, [100]));

    assert_eq!(vec![0xa1, 0x10, 0x27], asm.mov(AX, [10000]));
    assert_eq!(vec![0x8b, 0x0e, 0x10, 0x27], asm.mov(CX, [10000]));
    assert_eq!(vec![0x8b, 0x16, 0x10, 0x27], asm.mov(DX, [10000]));
    assert_eq!(vec![0x8b, 0x1e, 0x10, 0x27], asm.mov(BX, [10000]));
    assert_eq!(vec![0x8b, 0x26, 0x10, 0x27], asm.mov(SP, [10000]));
    assert_eq!(vec![0x8b, 0x2e, 0x10, 0x27], asm.mov(BP, [10000]));
    assert_eq!(vec![0x8b, 0x36, 0x10, 0x27], asm.mov(SI, [10000]));
    assert_eq!(vec![0x8b, 0x3e, 0x10, 0x27], asm.mov(DI, [10000]));

    assert_eq!(vec![0x8b, 0x07], asm.mov(AX, [BX]));
    assert_eq!(vec![0x8b, 0x0f], asm.mov(CX, [BX]));
    assert_eq!(vec![0x8b, 0x17], asm.mov(DX, [BX]));
    assert_eq!(vec![0x8b, 0x1f], asm.mov(BX, [BX]));
    assert_eq!(vec![0x8b, 0x27], asm.mov(SP, [BX]));
    assert_eq!(vec![0x8b, 0x2f], asm.mov(BP, [BX]));
    assert_eq!(vec![0x8b, 0x37], asm.mov(SI, [BX]));
    assert_eq!(vec![0x8b, 0x3f], asm.mov(DI, [BX]));

    assert_eq!(vec![0x8b, 0x46, 0x00], asm.mov(AX, [BP]));
    assert_eq!(vec![0x8b, 0x4e, 0x00], asm.mov(CX, [BP]));
    assert_eq!(vec![0x8b, 0x56, 0x00], asm.mov(DX, [BP]));
    assert_eq!(vec![0x8b, 0x5e, 0x00], asm.mov(BX, [BP]));
    assert_eq!(vec![0x8b, 0x66, 0x00], asm.mov(SP, [BP]));
    assert_eq!(vec![0x8b, 0x6e, 0x00], asm.mov(BP, [BP]));
    assert_eq!(vec![0x8b, 0x76, 0x00], asm.mov(SI, [BP]));
    assert_eq!(vec![0x8b, 0x7e, 0x00], asm.mov(DI, [BP]));

    assert_eq!(vec![0x8b, 0x04], asm.mov(AX, [SI]));
    assert_eq!(vec![0x8b, 0x0c], asm.mov(CX, [SI]));
    assert_eq!(vec![0x8b, 0x14], asm.mov(DX, [SI]));
    assert_eq!(vec![0x8b, 0x1c], asm.mov(BX, [SI]));
    assert_eq!(vec![0x8b, 0x24], asm.mov(SP, [SI]));
    assert_eq!(vec![0x8b, 0x2c], asm.mov(BP, [SI]));
    assert_eq!(vec![0x8b, 0x34], asm.mov(SI, [SI]));
    assert_eq!(vec![0x8b, 0x3c], asm.mov(DI, [SI]));

    assert_eq!(vec![0x8b, 0x05], asm.mov(AX, [DI]));
    assert_eq!(vec![0x8b, 0x0d], asm.mov(CX, [DI]));
    assert_eq!(vec![0x8b, 0x15], asm.mov(DX, [DI]));
    assert_eq!(vec![0x8b, 0x1d], asm.mov(BX, [DI]));
    assert_eq!(vec![0x8b, 0x25], asm.mov(SP, [DI]));
    assert_eq!(vec![0x8b, 0x2d], asm.mov(BP, [DI]));
    assert_eq!(vec![0x8b, 0x35], asm.mov(SI, [DI]));
    assert_eq!(vec![0x8b, 0x3d], asm.mov(DI, [DI]));
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
    let asm = ASM::new();

    // Move values to 8-bit registers
    assert_eq!(vec![0xb0, 0x00], asm.mov(AL, Imm8(0)));
    assert_eq!(vec![0xb1, 0x00], asm.mov(CL, Imm8(0)));
    assert_eq!(vec![0xb2, 0x00], asm.mov(DL, Imm8(0)));
    assert_eq!(vec![0xb3, 0x00], asm.mov(BL, Imm8(0)));
    assert_eq!(vec![0xb4, 0x00], asm.mov(AH, Imm8(0)));
    assert_eq!(vec![0xb5, 0x00], asm.mov(CH, Imm8(0)));
    assert_eq!(vec![0xb6, 0x00], asm.mov(DH, Imm8(0)));
    assert_eq!(vec![0xb7, 0x00], asm.mov(BH, Imm8(0)));

    assert_eq!(vec![0xb0, 0x0c], asm.mov(AL, Imm8(12)));
    assert_eq!(vec![0xb1, 0x0c], asm.mov(CL, Imm8(12)));
    assert_eq!(vec![0xb2, 0x0c], asm.mov(DL, Imm8(12)));
    assert_eq!(vec![0xb3, 0x0c], asm.mov(BL, Imm8(12)));
    assert_eq!(vec![0xb4, 0x0c], asm.mov(AH, Imm8(12)));
    assert_eq!(vec![0xb5, 0x0c], asm.mov(CH, Imm8(12)));
    assert_eq!(vec![0xb6, 0x0c], asm.mov(DH, Imm8(12)));
    assert_eq!(vec![0xb7, 0x0c], asm.mov(BH, Imm8(12)));

    assert_eq!(vec![0xb0, 0x10], asm.mov(AL, Imm8(16)));
    assert_eq!(vec![0xb1, 0x10], asm.mov(CL, Imm8(16)));
    assert_eq!(vec![0xb2, 0x10], asm.mov(DL, Imm8(16)));
    assert_eq!(vec![0xb3, 0x10], asm.mov(BL, Imm8(16)));
    assert_eq!(vec![0xb4, 0x10], asm.mov(AH, Imm8(16)));
    assert_eq!(vec![0xb5, 0x10], asm.mov(CH, Imm8(16)));
    assert_eq!(vec![0xb6, 0x10], asm.mov(DH, Imm8(16)));
    assert_eq!(vec![0xb7, 0x10], asm.mov(BH, Imm8(16)));

    assert_eq!(vec![0xb0, 0x96], asm.mov(AL, Imm8(150)));
    assert_eq!(vec![0xb1, 0x96], asm.mov(CL, Imm8(150)));
    assert_eq!(vec![0xb2, 0x96], asm.mov(DL, Imm8(150)));
    assert_eq!(vec![0xb3, 0x96], asm.mov(BL, Imm8(150)));
    assert_eq!(vec![0xb4, 0x96], asm.mov(AH, Imm8(150)));
    assert_eq!(vec![0xb5, 0x96], asm.mov(CH, Imm8(150)));
    assert_eq!(vec![0xb6, 0x96], asm.mov(DH, Imm8(150)));
    assert_eq!(vec![0xb7, 0x96], asm.mov(BH, Imm8(150)));

    // Move values to 16-bit registers
    assert_eq!(vec![0xb8, 0x00, 0x00], asm.mov(AX, Imm16(0)));
    assert_eq!(vec![0xb9, 0x00, 0x00], asm.mov(CX, Imm16(0)));
    assert_eq!(vec![0xba, 0x00, 0x00], asm.mov(DX, Imm16(0)));
    assert_eq!(vec![0xbb, 0x00, 0x00], asm.mov(BX, Imm16(0)));
    assert_eq!(vec![0xbc, 0x00, 0x00], asm.mov(SP, Imm16(0)));
    assert_eq!(vec![0xbd, 0x00, 0x00], asm.mov(BP, Imm16(0)));
    assert_eq!(vec![0xbe, 0x00, 0x00], asm.mov(SI, Imm16(0)));
    assert_eq!(vec![0xbf, 0x00, 0x00], asm.mov(DI, Imm16(0)));

    assert_eq!(vec![0xb8, 0x0c, 0x00], asm.mov(AX, Imm16(12)));
    assert_eq!(vec![0xb9, 0x0c, 0x00], asm.mov(CX, Imm16(12)));
    assert_eq!(vec![0xba, 0x0c, 0x00], asm.mov(DX, Imm16(12)));
    assert_eq!(vec![0xbb, 0x0c, 0x00], asm.mov(BX, Imm16(12)));
    assert_eq!(vec![0xbc, 0x0c, 0x00], asm.mov(SP, Imm16(12)));
    assert_eq!(vec![0xbd, 0x0c, 0x00], asm.mov(BP, Imm16(12)));
    assert_eq!(vec![0xbe, 0x0c, 0x00], asm.mov(SI, Imm16(12)));
    assert_eq!(vec![0xbf, 0x0c, 0x00], asm.mov(DI, Imm16(12)));

    assert_eq!(vec![0xb8, 0x10, 0x00], asm.mov(AX, Imm16(16)));
    assert_eq!(vec![0xb9, 0x10, 0x00], asm.mov(CX, Imm16(16)));
    assert_eq!(vec![0xba, 0x10, 0x00], asm.mov(DX, Imm16(16)));
    assert_eq!(vec![0xbb, 0x10, 0x00], asm.mov(BX, Imm16(16)));
    assert_eq!(vec![0xbc, 0x10, 0x00], asm.mov(SP, Imm16(16)));
    assert_eq!(vec![0xbd, 0x10, 0x00], asm.mov(BP, Imm16(16)));
    assert_eq!(vec![0xbe, 0x10, 0x00], asm.mov(SI, Imm16(16)));
    assert_eq!(vec![0xbf, 0x10, 0x00], asm.mov(DI, Imm16(16)));

    assert_eq!(vec![0xb8, 0x96, 0x00], asm.mov(AX, Imm16(150)));
    assert_eq!(vec![0xb9, 0x96, 0x00], asm.mov(CX, Imm16(150)));
    assert_eq!(vec![0xba, 0x96, 0x00], asm.mov(DX, Imm16(150)));
    assert_eq!(vec![0xbb, 0x96, 0x00], asm.mov(BX, Imm16(150)));
    assert_eq!(vec![0xbc, 0x96, 0x00], asm.mov(SP, Imm16(150)));
    assert_eq!(vec![0xbd, 0x96, 0x00], asm.mov(BP, Imm16(150)));
    assert_eq!(vec![0xbe, 0x96, 0x00], asm.mov(SI, Imm16(150)));
    assert_eq!(vec![0xbf, 0x96, 0x00], asm.mov(DI, Imm16(150)));

    assert_eq!(vec![0xb8, 0xe8, 0x03], asm.mov(AX, Imm16(1000)));
    assert_eq!(vec![0xb9, 0xe8, 0x03], asm.mov(CX, Imm16(1000)));
    assert_eq!(vec![0xba, 0xe8, 0x03], asm.mov(DX, Imm16(1000)));
    assert_eq!(vec![0xbb, 0xe8, 0x03], asm.mov(BX, Imm16(1000)));
    assert_eq!(vec![0xbc, 0xe8, 0x03], asm.mov(SP, Imm16(1000)));
    assert_eq!(vec![0xbd, 0xe8, 0x03], asm.mov(BP, Imm16(1000)));
    assert_eq!(vec![0xbe, 0xe8, 0x03], asm.mov(SI, Imm16(1000)));
    assert_eq!(vec![0xbf, 0xe8, 0x03], asm.mov(DI, Imm16(1000)));

    assert_eq!(vec![0xb8, 0x00, 0x80], asm.mov(AX, Imm16(32768)));
    assert_eq!(vec![0xb9, 0x00, 0x80], asm.mov(CX, Imm16(32768)));
    assert_eq!(vec![0xba, 0x00, 0x80], asm.mov(DX, Imm16(32768)));
    assert_eq!(vec![0xbb, 0x00, 0x80], asm.mov(BX, Imm16(32768)));
    assert_eq!(vec![0xbc, 0x00, 0x80], asm.mov(SP, Imm16(32768)));
    assert_eq!(vec![0xbd, 0x00, 0x80], asm.mov(BP, Imm16(32768)));
    assert_eq!(vec![0xbe, 0x00, 0x80], asm.mov(SI, Imm16(32768)));
    assert_eq!(vec![0xbf, 0x00, 0x80], asm.mov(DI, Imm16(32768)));

    assert_eq!(vec![0xb8, 0x50, 0xc3], asm.mov(AX, Imm16(50000)));
    assert_eq!(vec![0xb9, 0x50, 0xc3], asm.mov(CX, Imm16(50000)));
    assert_eq!(vec![0xba, 0x50, 0xc3], asm.mov(DX, Imm16(50000)));
    assert_eq!(vec![0xbb, 0x50, 0xc3], asm.mov(BX, Imm16(50000)));
    assert_eq!(vec![0xbc, 0x50, 0xc3], asm.mov(SP, Imm16(50000)));
    assert_eq!(vec![0xbd, 0x50, 0xc3], asm.mov(BP, Imm16(50000)));
    assert_eq!(vec![0xbe, 0x50, 0xc3], asm.mov(SI, Imm16(50000)));
    assert_eq!(vec![0xbf, 0x50, 0xc3], asm.mov(DI, Imm16(50000)));
}

#[ignore = "unimplemented yet"]
#[test]
fn real_address_mode_mov_instructions_from_immediate_to_memory() {
    unimplemented!();
}
