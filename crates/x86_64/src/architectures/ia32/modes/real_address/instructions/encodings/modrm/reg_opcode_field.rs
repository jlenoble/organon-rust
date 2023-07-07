//! reg/opcode field in ModR/M addressing-mode encoding byte
//!
//! ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 2.1.3*

/// 0th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode0;
/// 1st opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode1;
/// 2nd opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode2;
/// 3rd opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode3;
/// 4th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode4;
/// 5th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode5;
/// 6th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode6;
/// 7th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
pub struct Opcode7;

use super::super::super::{
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    operands::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI },
    operands::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7 },
    operands::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7 },
};

/// Helper trait for reg/opcode field in ModR/M addressing-mode encoding byte
///
/// We want to inline correct encoding at compile time by overloading the method 'encode'
pub trait RegOpcode<R, O> {
    /// Encodes as a u8 the reg/opcode field in ModR/M using 1 of the 8 ZSTs Opcode0 to Opcode7 or any register ZST such as EAX or MM0
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*
    fn encode(_reg: R) -> u8;
}

/// reg/opcode field in ModR/M addressing-mode encoding byte
pub struct REGOPCODE;

macro_rules! reg_opcode {
    ($reg:ident, $opcode:ident) => {
            impl RegOpcode<$reg, $opcode> for REGOPCODE {
                #[inline]
                fn encode(_reg: $reg) -> u8 {
                    Opcode::$opcode as u8
                }
            }
    };
}

reg_opcode!(AL, Opcode0);
reg_opcode!(AX, Opcode0);
reg_opcode!(EAX, Opcode0);
reg_opcode!(MM0, Opcode0);
reg_opcode!(XMM0, Opcode0);
reg_opcode!(Opcode0, Opcode0);

reg_opcode!(CL, Opcode1);
reg_opcode!(CX, Opcode1);
reg_opcode!(ECX, Opcode1);
reg_opcode!(MM1, Opcode1);
reg_opcode!(XMM1, Opcode1);
reg_opcode!(Opcode1, Opcode1);

reg_opcode!(DL, Opcode2);
reg_opcode!(DX, Opcode2);
reg_opcode!(EDX, Opcode2);
reg_opcode!(MM2, Opcode2);
reg_opcode!(XMM2, Opcode2);
reg_opcode!(Opcode2, Opcode2);

reg_opcode!(BL, Opcode3);
reg_opcode!(BX, Opcode3);
reg_opcode!(EBX, Opcode3);
reg_opcode!(MM3, Opcode3);
reg_opcode!(XMM3, Opcode3);
reg_opcode!(Opcode3, Opcode3);

reg_opcode!(AH, Opcode4);
reg_opcode!(SP, Opcode4);
reg_opcode!(ESP, Opcode4);
reg_opcode!(MM4, Opcode4);
reg_opcode!(XMM4, Opcode4);
reg_opcode!(Opcode4, Opcode4);

reg_opcode!(CH, Opcode5);
reg_opcode!(BP, Opcode5);
reg_opcode!(EBP, Opcode5);
reg_opcode!(MM5, Opcode5);
reg_opcode!(XMM5, Opcode5);
reg_opcode!(Opcode5, Opcode5);

reg_opcode!(DH, Opcode6);
reg_opcode!(SI, Opcode6);
reg_opcode!(ESI, Opcode6);
reg_opcode!(MM6, Opcode6);
reg_opcode!(XMM6, Opcode6);
reg_opcode!(Opcode6, Opcode6);

reg_opcode!(BH, Opcode7);
reg_opcode!(DI, Opcode7);
reg_opcode!(EDI, Opcode7);
reg_opcode!(MM7, Opcode7);
reg_opcode!(XMM7, Opcode7);
reg_opcode!(Opcode7, Opcode7);

#[repr(u8)]
/// reg/opcode part (0b00_111_000) of ModR/M addressing-mode encoding byte
pub enum Opcode {
    /// 0th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode0 = 0x00,
    /// 1st opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode1 = 0x08,
    /// 2nd opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode2 = 0x10,
    /// 3rd opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode3 = 0x18,
    /// 4th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode4 = 0x20,
    /// 5th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode5 = 0x28,
    /// 6th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode6 = 0x30,
    /// 7th opcode of reg/opcode field in ModR/M addressing-mode encoding byte
    Opcode7 = 0x38,
}
