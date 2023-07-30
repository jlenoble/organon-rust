//! Unit tests for JMP instruction in real-address mode

use super::{ Jmp, super::super::super::{ ASM, operands::{ Rel8, Rel16 } } };

#[test]
fn real_address_mode_jmp_instruction() {
    // infinite loops
    assert_eq!(vec![0xeb, 0xfe], ASM::jmp(Rel8(-2)));
    assert_eq!(vec![0xe9, 0xfd, 0xff], ASM::jmp(Rel16(-3)));
}
