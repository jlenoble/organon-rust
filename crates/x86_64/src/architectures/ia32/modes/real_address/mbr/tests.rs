//! Unit tests for Master Boot Record (MBR)

use super::{
    MBR,
    super::instructions::{
        categories::{
            control_transfer::{ int::INT, jmp::{ JMP, Jmp } },
            data_transfer::mov::{ MOV, Mov },
        },
        operands::{ Imm8, Rel16, AH, AL },
    },
};

// *ref. https://github.com/cfenollosa/os-tutorial/tree/master/01-bootsector-barebones*
#[test]
fn bootsector_barebones() {
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

// *ref. https://github.com/cfenollosa/os-tutorial/tree/master/02-bootsector-print*
#[test]
fn bootsector_print() {
    let mut mbr = MBR::new();

    let mut buf: [u8; 512] = [0; 512];
    buf[510] = 0x55;
    buf[511] = 0xaa;

    // mov ah, 0x0e ; tty mode
    buf[0] = 0xb4;
    buf[1] = 0x0e;
    // mov al, 'H'
    buf[2] = 0xb0;
    buf[3] = b'H';
    // int 0x10
    buf[4] = 0xcd;
    buf[5] = 0x10;
    // mov al, 'e'
    buf[6] = 0xb0;
    buf[7] = b'e';
    // int 0x10
    buf[8] = 0xcd;
    buf[9] = 0x10;
    // mov al, 'l'
    buf[10] = 0xb0;
    buf[11] = b'l';
    // int 0x10
    buf[12] = 0xcd;
    buf[13] = 0x10;
    // int 0x10 ; 'l' is still on al, remember?
    buf[14] = 0xcd;
    buf[15] = 0x10;
    // mov al, 'o'
    buf[16] = 0xb0;
    buf[17] = b'o';
    // int 0x10
    buf[18] = 0xcd;
    buf[19] = 0x10;
    // jmp $ ; jump to current address = infinite loop
    buf[20] = 0xe9;
    buf[21] = 0xfd;
    buf[22] = 0xff;

    mbr.push(MOV::mov(AH, Imm8(0x0e)));
    mbr.push(MOV::mov(AL, Imm8(b'H')));
    mbr.push(INT::int(Imm8(0x10)));
    mbr.push(MOV::mov(AL, Imm8(b'e')));
    mbr.push(INT::int(Imm8(0x10)));
    mbr.push(MOV::mov(AL, Imm8(b'l')));
    mbr.push(INT::int(Imm8(0x10)));
    mbr.push(INT::int(Imm8(0x10)));
    mbr.push(MOV::mov(AL, Imm8(b'o')));
    mbr.push(INT::int(Imm8(0x10)));
    mbr.push(JMP::jmp(Rel16(-3)));

    assert_eq!(
        MBR {
            buffer: buf,
            current_position: 23,
        },
        mbr
    );
}
