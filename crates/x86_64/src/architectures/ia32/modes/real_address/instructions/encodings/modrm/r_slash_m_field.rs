//! r/m field in ModR/M addressing-mode encoding byte
//!
//! ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 2.1.3*

/// Helper trait for r/m field in ModR/M addressing-mode encoding byte
///
/// We want to inline correct encoding at compile time by overloading the method 'encode'
pub trait RSlashM<R, R32> {
    /// Encodes as a u8 the r/m field in ModR/M
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*
    fn encode(_reg: R) -> u8;
}

/// r/m field in ModR/M addressing-mode encoding byte
pub struct RSLASHM;

use super::super::super::{
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    operands::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, GPReg32 },
    operands::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7 },
    operands::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7 },
};

macro_rules! r_slash_m {
    ($reg:ident, $gp32:ident) => {
        impl RSlashM<$reg, $gp32> for RSLASHM {
            #[inline]
            fn encode(_reg: $reg) -> u8 {
                GPReg32::$gp32 as u8
            }
        }
    };
}

r_slash_m!(AL, EAX);
r_slash_m!(AX, EAX);
r_slash_m!(EAX, EAX);
r_slash_m!(MM0, EAX);
r_slash_m!(XMM0, EAX);

r_slash_m!(CL, ECX);
r_slash_m!(CX, ECX);
r_slash_m!(ECX, ECX);
r_slash_m!(MM1, ECX);
r_slash_m!(XMM1, ECX);

r_slash_m!(DL, EDX);
r_slash_m!(DX, EDX);
r_slash_m!(EDX, EDX);
r_slash_m!(MM2, EDX);
r_slash_m!(XMM2, EDX);

r_slash_m!(BL, EBX);
r_slash_m!(BX, EBX);
r_slash_m!(EBX, EBX);
r_slash_m!(MM3, EBX);
r_slash_m!(XMM3, EBX);

r_slash_m!(AH, ESP);
r_slash_m!(SP, ESP);
r_slash_m!(ESP, ESP);
r_slash_m!(MM4, ESP);
r_slash_m!(XMM4, ESP);

r_slash_m!(CH, EBP);
r_slash_m!(BP, EBP);
r_slash_m!(EBP, EBP);
r_slash_m!(MM5, EBP);
r_slash_m!(XMM5, EBP);

r_slash_m!(DH, ESI);
r_slash_m!(SI, ESI);
r_slash_m!(ESI, ESI);
r_slash_m!(MM6, ESI);
r_slash_m!(XMM6, ESI);

r_slash_m!(BH, EDI);
r_slash_m!(DI, EDI);
r_slash_m!(EDI, EDI);
r_slash_m!(MM7, EDI);
r_slash_m!(XMM7, EDI);
