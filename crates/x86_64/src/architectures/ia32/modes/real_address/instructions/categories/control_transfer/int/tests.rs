//! Unit tests for INT instruction in real-address mode

use super::INT;
use super::super::super::super::operands::Imm8;

#[test]
fn real_address_mode_int_instruction() {
    // BIOS real mode interrupt handler for video services
    assert_eq!(vec![0xcd, 0x10], INT::int(Imm8(0x10)));
}
