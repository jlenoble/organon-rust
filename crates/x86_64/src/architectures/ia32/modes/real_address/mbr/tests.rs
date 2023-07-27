//! Unit tests for Master Boot Record (MBR)

use super::{
    MBR,
    super::instructions::{ categories::control_transfer::jmp::{ JMP, Jmp }, operands::Rel16 },
};

// *ref. https://github.com/cfenollosa/os-tutorial/tree/master/01-bootsector-barebones*
#[test]
fn boot_sector_simple() {
    let mut mbr = MBR::new();

    let mut buf: [u8; 512] = [0; 512];
    buf[0] = 0xe9;
    buf[1] = 0xfd;
    buf[2] = 0xff;
    buf[510] = 0x55;
    buf[511] = 0xaa;

    mbr.push(JMP::jmp(Rel16(-3)));

    assert_eq!(
        MBR {
            buffer: buf,
            current_position: 3,
        },
        mbr
    );
}
