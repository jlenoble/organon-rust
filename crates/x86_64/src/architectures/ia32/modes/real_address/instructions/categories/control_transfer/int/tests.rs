//! Unit tests for INT instruction in real-address mode

use super::super::super::super::{ ASM, operands::Imm8 };

#[test]
fn real_address_mode_int_instruction() {
    // BIOS real mode interrupt handler for video services
    assert_eq!(vec![0xcd, 0x10], ASM::int(Imm8(0x10)));
}
