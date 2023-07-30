//! Unit tests for INT instruction in real-address mode

use super::super::super::super::{ ASM, operands::Imm8 };

#[test]
fn real_address_mode_int_instruction() {
    let asm = ASM::new();

    // BIOS real mode interrupt handler for video services
    assert_eq!(vec![0xcd, 0x10], asm.int(Imm8(0x10)));
}
