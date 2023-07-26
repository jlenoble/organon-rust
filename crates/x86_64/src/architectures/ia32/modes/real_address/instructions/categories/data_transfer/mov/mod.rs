//! IA-32 real-address mode MOV and CMOVcc instructions
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.1.1*

use super::super::super::{
    operands::{ Imm8, Imm16 },
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    encodings::modrm::{ Reg, Disp0, Disp16, MODRM, ModRM },
};

/// Trait encompassing all IA-32 real-address mode MOV instruction variants
pub trait Mov<Dest, Src> {
    /// generic real-address mode MOV instruction
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV*
    fn mov(dest: Dest, src: Src) -> Vec<u8>;
}

/// IA-32 real-address mode MOV instruction implementations
pub struct MOV;

macro_rules! r8_imm8 {
    ($r:ty, $op_code:expr) => {
        impl Mov<$r, Imm8> for MOV {
            #[inline]
            fn mov(_: $r, imm: Imm8) -> Vec<u8> {
                vec![$op_code, imm.0]
            }
        }
    };
}

macro_rules! rr8_imm8 {
    ($op_code:literal) => {
        r8_imm8!(AL, $op_code + 0);
        r8_imm8!(CL, $op_code + 1);
        r8_imm8!(DL, $op_code + 2);
        r8_imm8!(BL, $op_code + 3);
        r8_imm8!(AH, $op_code + 4);
        r8_imm8!(CH, $op_code + 5);
        r8_imm8!(DH, $op_code + 6);
        r8_imm8!(BH, $op_code + 7);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `B0+rb ib`*
rr8_imm8!(0xb0);

macro_rules! r16_imm16 {
    ($r:ty, $op_code:expr) => {
        impl Mov<$r, Imm16> for MOV {
            #[inline]
            fn mov(_: $r, imm: Imm16) -> Vec<u8> {
                vec![$op_code, (imm.0 & 0xff) as u8, ((imm.0 & 0xff00) >> 8) as u8]
            }
        }
    };
}

macro_rules! rr16_imm16 {
    ($op_code:literal) => {
        r16_imm16!(AX, $op_code + 0);
        r16_imm16!(CX, $op_code + 1);
        r16_imm16!(DX, $op_code + 2);
        r16_imm16!(BX, $op_code + 3);
        r16_imm16!(SP, $op_code + 4);
        r16_imm16!(BP, $op_code + 5);
        r16_imm16!(SI, $op_code + 6);
        r16_imm16!(DI, $op_code + 7);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `B8+rw iw`*
rr16_imm16!(0xb8);

macro_rules! r_r {
    ($r1:tt, $r2:tt, $op_code:literal) => {
        impl Mov<$r1, $r2> for MOV {
            #[inline]
            fn mov(_r1: $r1, _r2: $r2)-> Vec<u8> {
                vec![$op_code, MODRM::encode(Reg, $r2, $r1)]
            }
        }
    };
}

macro_rules! rr8_r8 {
    ($r2:tt, $op_code:literal) => {
        r_r!(AL, $r2, $op_code);
        r_r!(CL, $r2, $op_code);
        r_r!(DL, $r2, $op_code);
        r_r!(BL, $r2, $op_code);
        r_r!(AH, $r2, $op_code);
        r_r!(CH, $r2, $op_code);
        r_r!(DH, $r2, $op_code);
        r_r!(BH, $r2, $op_code);
    };
}

macro_rules! rr8_rr8 {
    ($op_code:literal) => {
        rr8_r8!(AL, $op_code);
        rr8_r8!(CL, $op_code);
        rr8_r8!(DL, $op_code);
        rr8_r8!(BL, $op_code);
        rr8_r8!(AH, $op_code);
        rr8_r8!(CH, $op_code);
        rr8_r8!(DH, $op_code);
        rr8_r8!(BH, $op_code);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `88 /r`*
rr8_rr8!(0x88);

macro_rules! rr16_r16 {
    ($r2:tt, $op_code:literal) => {
        r_r!(AX, $r2, $op_code);
        r_r!(CX, $r2, $op_code);
        r_r!(DX, $r2, $op_code);
        r_r!(BX, $r2, $op_code);
        r_r!(SP, $r2, $op_code);
        r_r!(BP, $r2, $op_code);
        r_r!(SI, $r2, $op_code);
        r_r!(DI, $r2, $op_code);
    };
}

macro_rules! rr16_rr16 {
    ($op_code:literal) => {
        rr16_r16!(AX, $op_code);
        rr16_r16!(CX, $op_code);
        rr16_r16!(DX, $op_code);
        rr16_r16!(BX, $op_code);
        rr16_r16!(SP, $op_code);
        rr16_r16!(BP, $op_code);
        rr16_r16!(SI, $op_code);
        rr16_r16!(DI, $op_code);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `89 /r`*
rr16_rr16!(0x89);

macro_rules! r16_mem16_num {
    (AX, $op_code:literal) => {
        impl Mov<AX, [u16; 1]> for MOV {
            #[inline]
            fn mov(_: AX, mem: [u16; 1]) -> Vec<u8> {
                vec![$op_code, (mem[0] & 0xff) as u8, ((mem[0] & 0xff00) >> 8) as u8]
            }
        }
    };
    ($r:tt, $op_code:literal) => {
        impl Mov<$r, [u16; 1]> for MOV {
            #[inline]
            fn mov(_: $r, mem: [u16; 1]) -> Vec<u8> {
                vec![$op_code, MODRM::encode(Disp0, $r, Disp16), (mem[0] & 0xff) as u8, ((mem[0] & 0xff00) >> 8) as u8]
            }
        }
    };
}

macro_rules! rr16_mem16_num {
    ($ax_op_code:literal, $op_code:literal) => {
        r16_mem16_num!(AX, $ax_op_code);
        r16_mem16_num!(CX, $op_code);
        r16_mem16_num!(DX, $op_code);
        r16_mem16_num!(BX, $op_code);
        r16_mem16_num!(SP, $op_code);
        r16_mem16_num!(BP, $op_code);
        r16_mem16_num!(SI, $op_code);
        r16_mem16_num!(DI, $op_code);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `A1`*
// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `8B /r`*
rr16_mem16_num!(0xa1, 0x8b);

macro_rules! r16_mem16_reg {
    ($r1:ty, BP, $op_code:literal, $reg_code:expr) => {
        impl Mov<$r1, [BP; 1]> for MOV {
            #[inline]
            fn mov(_: $r1, _mem: [BP; 1]) -> Vec<u8> {
                vec![$op_code, $reg_code, 0x00]
            }
        }
    };
    ($r1:ty, $r2:ty, $op_code:literal, $reg_code:expr) => {
        impl Mov<$r1, [$r2; 1]> for MOV {
        #[inline]
            fn mov(_: $r1, _mem: [$r2; 1]) -> Vec<u8> {
                vec![$op_code, $reg_code]
            }
        }
    };
}

macro_rules! rr16_mem16_reg {
    ($r2:tt, $op_code:literal, $reg_code:literal) => {
        r16_mem16_reg!(AX, $r2, $op_code, $reg_code + 0);
        r16_mem16_reg!(CX, $r2, $op_code, $reg_code + 8);
        r16_mem16_reg!(DX, $r2, $op_code, $reg_code + 16);
        r16_mem16_reg!(BX, $r2, $op_code, $reg_code + 24);
        r16_mem16_reg!(SP, $r2, $op_code, $reg_code + 32);
        r16_mem16_reg!(BP, $r2, $op_code, $reg_code + 40);
        r16_mem16_reg!(SI, $r2, $op_code, $reg_code + 48);
        r16_mem16_reg!(DI, $r2, $op_code, $reg_code + 56);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `8B /r`*
rr16_mem16_reg!(BX, 0x8b, 0x07);
rr16_mem16_reg!(SI, 0x8b, 0x04);
rr16_mem16_reg!(DI, 0x8b, 0x05);

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 4.3#MOV line `8B /r`*
rr16_mem16_reg!(BP, 0x8b, 0x46);

#[cfg(test)]
mod tests;
