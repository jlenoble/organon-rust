//! IA-32 real-address mode ADD instruction
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.2.1*

use super::super::super::{
    ASM,
    operands::{ Imm8, Imm16 },
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    encodings::modrm::{ Reg, MODRM, ModRM, Opcode0 },
};

/// Trait encompassing all IA-32 real-address mode ADD instruction variants
pub trait Add<Dest, Src> {
    /// generic real-address mode ADD instruction
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD*
    fn add(dest: Dest, src: Src) -> Vec<u8>;
}

macro_rules! r_imm {
    (AL, $op_code:expr) => {
        impl Add<AL, Imm8> for ASM {
            #[inline]
            fn add(_: AL, imm: Imm8) -> Vec<u8> {
                vec![$op_code, imm.0]
            }
        }
    };
    (AX, $op_code:expr) => {
        impl Add<AX, Imm16> for ASM {
            #[inline]
            fn add(_: AX, imm: Imm16) -> Vec<u8> {
                vec![$op_code, (imm.0 & 0xff) as u8, ((imm.0 & 0xff00) >> 8) as u8]
            }
        }
    };
    ($r:ty, $op_code:expr, Imm8) => {
        impl Add<$r, Imm8> for ASM {
            #[inline]
            fn add(reg: $r, imm: Imm8) -> Vec<u8> {
                vec![$op_code, MODRM::encode(Reg, Opcode0, reg), imm.0]
            }
        }
    };
    ($r:ty, $op_code:expr, Imm16) => {
        impl Add<$r, Imm16> for ASM {
            #[inline]
            fn add(reg: $r, imm: Imm16) -> Vec<u8> {
                vec![$op_code, MODRM::encode(Reg, Opcode0, reg), (imm.0 & 0xff) as u8, ((imm.0 & 0xff00) >> 8) as u8]
            }
        }
    };
}

macro_rules! rr8_imm8 {
    ($al_op_code:literal, $op_code:literal) => {
        r_imm!(AL, $al_op_code);
        r_imm!(CL, $op_code, Imm8);
        r_imm!(DL, $op_code, Imm8);
        r_imm!(BL, $op_code, Imm8);
        r_imm!(AH, $op_code, Imm8);
        r_imm!(CH, $op_code, Imm8);
        r_imm!(DH, $op_code, Imm8);
        r_imm!(BH, $op_code, Imm8);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD line `04 ib`*
// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD line `80 /0 ib`*
rr8_imm8!(0x04, 0x80);

macro_rules! rr16_imm16 {
    ($ax_op_code:literal, $op_code:literal) => {
        r_imm!(AX, $ax_op_code);
        r_imm!(CX, $op_code, Imm16);
        r_imm!(DX, $op_code, Imm16);
        r_imm!(BX, $op_code, Imm16);
        r_imm!(SP, $op_code, Imm16);
        r_imm!(BP, $op_code, Imm16);
        r_imm!(SI, $op_code, Imm16);
        r_imm!(DI, $op_code, Imm16);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD line `05 iw`*
// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD line `81 /0 iw`*
rr16_imm16!(0x05, 0x81);

macro_rules! rr16_imm8 {
    ($op_code:literal) => {
        r_imm!(AX, $op_code, Imm8);
        r_imm!(CX, $op_code, Imm8);
        r_imm!(DX, $op_code, Imm8);
        r_imm!(BX, $op_code, Imm8);
        r_imm!(SP, $op_code, Imm8);
        r_imm!(BP, $op_code, Imm8);
        r_imm!(SI, $op_code, Imm8);
        r_imm!(DI, $op_code, Imm8);
    };
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#ADD line `83 /0 ib`*
rr16_imm8!(0x83);

#[cfg(test)]
mod tests;
