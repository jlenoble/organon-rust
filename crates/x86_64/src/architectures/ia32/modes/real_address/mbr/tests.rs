//! Unit tests for Master Boot Record (MBR)

use super::{
    MBR,
    super::instructions::{
        ASM,
        categories::{
            binary_arithmetic::add::Add,
            control_transfer::jmp::Jmp,
            data_transfer::mov::Mov,
        },
        operands::{ Imm8, Imm16, Rel8, Rel16, AH, AL, BX },
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

    let asm = ASM::new();

    mbr.push(asm.jmp(Rel16(-3)));

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

    let asm = ASM::new();

    mbr.push(asm.mov(AH, Imm8(0x0e)));
    mbr.push(asm.mov(AL, Imm8(b'H')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'e')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'l')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'o')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.jmp(Rel16(-3)));

    assert_eq!(
        MBR {
            buffer: buf,
            current_position: 23,
        },
        mbr
    );
}

// *ref. https://github.com/cfenollosa/os-tutorial/tree/master/03-bootsector-memory*
#[test]
fn bootsector_memory() {
    let mut mbr = MBR::new();

    let mut buf: [u8; 512] = [0; 512];
    buf[510] = 0x55;
    buf[511] = 0xaa;

    // mov ah, 0x0e
    buf[0] = 0xb4;
    buf[1] = 0x0e;
    // mov al, "1"
    buf[2] = 0xb0;
    buf[3] = b'1';
    // int 0x10
    buf[4] = 0xcd;
    buf[5] = 0x10;
    // mov al, the_secret; Fails because it tries to print the memory address (i.e. pointer), not the actual content
    buf[6] = 0xb0;
    buf[7] = 0x2d; // 45
    // int 0x10
    buf[8] = 0xcd;
    buf[9] = 0x10;

    // mov al, "2"
    buf[10] = 0xb0;
    buf[11] = b'2';
    // int 0x10
    buf[12] = 0xcd;
    buf[13] = 0x10;
    // mov al, [the_secret]; Fails because BIOS places our bootsector binary at address 0x7c00
    buf[14] = 0xa0;
    buf[15] = 0x2d; // 45
    buf[16] = 0x00;
    // int 0x10
    buf[17] = 0xcd;
    buf[18] = 0x10;

    // mov al, "3"
    buf[19] = 0xb0;
    buf[20] = b'3';
    // int 0x10
    buf[21] = 0xcd;
    buf[22] = 0x10;
    // mov bx, the_secret; different register 'bx' because 'mov al, [ax]' is illegal.
    buf[23] = 0xbb;
    buf[24] = 0x2d; // 45
    buf[25] = 0x00;
    // add bx, 0x7c00; Add the BIOS starting offset 0x7c00 to the memory address of the X
    buf[26] = 0x81;
    buf[27] = 0xc3;
    buf[28] = 0x00;
    buf[29] = 0x7c;
    // mov al, [bx]
    buf[30] = 0x8a;
    buf[31] = 0x07;
    // int 0x10
    buf[32] = 0xcd;
    buf[33] = 0x10;

    // mov al, "4"
    buf[34] = 0xb0;
    buf[35] = b'4';
    // int 0x10
    buf[36] = 0xcd;
    buf[37] = 0x10;
    // mov al, [0x7c2d]; Use address in binary
    buf[38] = 0xa0;
    buf[39] = 0x2d;
    buf[40] = 0x7c;
    // int 0x10
    buf[41] = 0xcd;
    buf[42] = 0x10;
    // jmp $ ; infinite loop
    buf[43] = 0xeb;
    buf[44] = 0xfe;
    // the_secret: db "X"
    buf[45] = b'X';

    let asm = ASM::new();

    mbr.push(asm.mov(AH, Imm8(0x0e)));
    mbr.push(asm.mov(AL, Imm8(b'1')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(0x2d)));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'2')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, [0x2d]));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'3')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(BX, Imm16(0x2d)));
    mbr.push(asm.add(BX, Imm16(0x7c00)));
    mbr.push(asm.mov(AL, [BX]));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, Imm8(b'4')));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.mov(AL, [0x7c2d]));
    mbr.push(asm.int(Imm8(0x10)));
    mbr.push(asm.jmp(Rel8(-2)));
    mbr.push(asm.db(b'X'));

    assert_eq!(
        MBR {
            buffer: buf,
            current_position: 46,
        },
        mbr
    );
}
