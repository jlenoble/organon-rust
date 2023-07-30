//! ModR/M addressing-mode encoding byte
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*

use super::{ Reg, Disp0, Disp8, Disp16, MOD, Mod };
use super::{
    REGOPCODE,
    RegOpcode,
    Opcode0,
    Opcode1,
    Opcode2,
    Opcode3,
    Opcode4,
    Opcode5,
    Opcode6,
    Opcode7,
};
use super::{ BXpSI, BXpDI, BPpSI, BPpDI, RSLASHM, RSlashM };

use super::super::super::{
    operands::{ AL, CL, DL, BL },
    operands::{ AH, CH, DH, BH },
    operands::{ AX, CX, DX, BX, SP, BP, SI, DI },
    operands::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI },
    operands::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7 },
    operands::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7 },
};

/// ModR/M addressing-mode encoding byte trait
pub trait ModRM<M, RO, RM> {
    ///
    fn encode(md: M, reg_opcode: RO, r_slash_m: RM) -> u8;
}

///
pub struct MODRM;

macro_rules! mod_ro_rm {
    ($md:ident, $ro:ident, $rm:ident) => {
        impl ModRM<$md, $ro, $rm> for MODRM {
            #[inline]
            fn encode(_md: $md, _reg_opcode: $ro, _r_slash_m: $rm) -> u8 {
                MOD::encode($md) | REGOPCODE::encode($ro) | RSLASHM::encode($rm)
            }
        }
    };
    ($md:ident, $ro:ident, [$rm:ident]) => {
        impl ModRM<$md, $ro, [$rm; 1]> for MODRM {
            #[inline]
            fn encode(_md: $md, _reg_opcode: $ro, _r_slash_m: [$rm; 1]) -> u8 {
                MOD::encode($md) | REGOPCODE::encode($ro) | RSLASHM::encode([$rm])
            }
        }
    };
}

macro_rules! mod_rm {
    ($md:ident, $rm:ident) => {
        mod_ro_rm!($md, AL, $rm);
        mod_ro_rm!($md, AX, $rm);
        mod_ro_rm!($md, EAX, $rm);
        mod_ro_rm!($md, MM0, $rm);
        mod_ro_rm!($md, XMM0, $rm);
        mod_ro_rm!($md, Opcode0, $rm);
        mod_ro_rm!($md, CL, $rm);
        mod_ro_rm!($md, CX, $rm);
        mod_ro_rm!($md, ECX, $rm);
        mod_ro_rm!($md, MM1, $rm);
        mod_ro_rm!($md, XMM1, $rm);
        mod_ro_rm!($md, Opcode1, $rm);
        mod_ro_rm!($md, DL, $rm);
        mod_ro_rm!($md, DX, $rm);
        mod_ro_rm!($md, EDX, $rm);
        mod_ro_rm!($md, MM2, $rm);
        mod_ro_rm!($md, XMM2, $rm);
        mod_ro_rm!($md, Opcode2, $rm);
        mod_ro_rm!($md, BL, $rm);
        mod_ro_rm!($md, BX, $rm);
        mod_ro_rm!($md, EBX, $rm);
        mod_ro_rm!($md, MM3, $rm);
        mod_ro_rm!($md, XMM3, $rm);
        mod_ro_rm!($md, Opcode3, $rm);
        mod_ro_rm!($md, AH, $rm);
        mod_ro_rm!($md, SP, $rm);
        mod_ro_rm!($md, ESP, $rm);
        mod_ro_rm!($md, MM4, $rm);
        mod_ro_rm!($md, XMM4, $rm);
        mod_ro_rm!($md, Opcode4, $rm);
        mod_ro_rm!($md, CH, $rm);
        mod_ro_rm!($md, BP, $rm);
        mod_ro_rm!($md, EBP, $rm);
        mod_ro_rm!($md, MM5, $rm);
        mod_ro_rm!($md, XMM5, $rm);
        mod_ro_rm!($md, Opcode5, $rm);
        mod_ro_rm!($md, DH, $rm);
        mod_ro_rm!($md, SI, $rm);
        mod_ro_rm!($md, ESI, $rm);
        mod_ro_rm!($md, MM6, $rm);
        mod_ro_rm!($md, XMM6, $rm);
        mod_ro_rm!($md, Opcode6, $rm);
        mod_ro_rm!($md, BH, $rm);
        mod_ro_rm!($md, DI, $rm);
        mod_ro_rm!($md, EDI, $rm);
        mod_ro_rm!($md, MM7, $rm);
        mod_ro_rm!($md, XMM7, $rm);
        mod_ro_rm!($md, Opcode7, $rm);
    };
    ($md:ident, [$rm:ident]) => {
        mod_ro_rm!($md, AL, [$rm]);
        mod_ro_rm!($md, AX, [$rm]);
        mod_ro_rm!($md, EAX, [$rm]);
        mod_ro_rm!($md, MM0, [$rm]);
        mod_ro_rm!($md, XMM0, [$rm]);
        mod_ro_rm!($md, Opcode0, [$rm]);
        mod_ro_rm!($md, CL, [$rm]);
        mod_ro_rm!($md, CX, [$rm]);
        mod_ro_rm!($md, ECX, [$rm]);
        mod_ro_rm!($md, MM1, [$rm]);
        mod_ro_rm!($md, XMM1, [$rm]);
        mod_ro_rm!($md, Opcode1, [$rm]);
        mod_ro_rm!($md, DL, [$rm]);
        mod_ro_rm!($md, DX, [$rm]);
        mod_ro_rm!($md, EDX, [$rm]);
        mod_ro_rm!($md, MM2, [$rm]);
        mod_ro_rm!($md, XMM2, [$rm]);
        mod_ro_rm!($md, Opcode2, [$rm]);
        mod_ro_rm!($md, BL, [$rm]);
        mod_ro_rm!($md, BX, [$rm]);
        mod_ro_rm!($md, EBX, [$rm]);
        mod_ro_rm!($md, MM3, [$rm]);
        mod_ro_rm!($md, XMM3, [$rm]);
        mod_ro_rm!($md, Opcode3, [$rm]);
        mod_ro_rm!($md, AH, [$rm]);
        mod_ro_rm!($md, SP, [$rm]);
        mod_ro_rm!($md, ESP, [$rm]);
        mod_ro_rm!($md, MM4, [$rm]);
        mod_ro_rm!($md, XMM4, [$rm]);
        mod_ro_rm!($md, Opcode4, [$rm]);
        mod_ro_rm!($md, CH, [$rm]);
        mod_ro_rm!($md, BP, [$rm]);
        mod_ro_rm!($md, EBP, [$rm]);
        mod_ro_rm!($md, MM5, [$rm]);
        mod_ro_rm!($md, XMM5, [$rm]);
        mod_ro_rm!($md, Opcode5, [$rm]);
        mod_ro_rm!($md, DH, [$rm]);
        mod_ro_rm!($md, SI, [$rm]);
        mod_ro_rm!($md, ESI, [$rm]);
        mod_ro_rm!($md, MM6, [$rm]);
        mod_ro_rm!($md, XMM6, [$rm]);
        mod_ro_rm!($md, Opcode6, [$rm]);
        mod_ro_rm!($md, BH, [$rm]);
        mod_ro_rm!($md, DI, [$rm]);
        mod_ro_rm!($md, EDI, [$rm]);
        mod_ro_rm!($md, MM7, [$rm]);
        mod_ro_rm!($md, XMM7, [$rm]);
        mod_ro_rm!($md, Opcode7, [$rm]);
    };
}

mod_rm!(Disp0, [BXpSI]);
mod_rm!(Disp0, [BXpDI]);
mod_rm!(Disp0, [BPpSI]);
mod_rm!(Disp0, [BPpDI]);
mod_rm!(Disp0, [SI]);
mod_rm!(Disp0, [DI]);
mod_rm!(Disp0, Disp16);
mod_rm!(Disp0, [BX]);

mod_rm!(Disp8, [BXpSI]);
mod_rm!(Disp8, [BXpDI]);
mod_rm!(Disp8, [BPpSI]);
mod_rm!(Disp8, [BPpDI]);
mod_rm!(Disp8, [SI]);
mod_rm!(Disp8, [DI]);
mod_rm!(Disp8, [BP]);
mod_rm!(Disp8, [BX]);

mod_rm!(Disp16, [BXpSI]);
mod_rm!(Disp16, [BXpDI]);
mod_rm!(Disp16, [BPpSI]);
mod_rm!(Disp16, [BPpDI]);
mod_rm!(Disp16, [SI]);
mod_rm!(Disp16, [DI]);
mod_rm!(Disp16, [BP]);
mod_rm!(Disp16, [BX]);

macro_rules! mod_rm_gp8 {
    ($r:tt) => {
        mod_ro_rm!(Reg, AL, $r);
        mod_ro_rm!(Reg, CL, $r);
        mod_ro_rm!(Reg, DL, $r);
        mod_ro_rm!(Reg, BL, $r);
        mod_ro_rm!(Reg, AH, $r);
        mod_ro_rm!(Reg, CH, $r);
        mod_ro_rm!(Reg, DH, $r);
        mod_ro_rm!(Reg, BH, $r);
        mod_ro_rm!(Reg, Opcode0, $r);
        mod_ro_rm!(Reg, Opcode1, $r);
        mod_ro_rm!(Reg, Opcode2, $r);
        mod_ro_rm!(Reg, Opcode3, $r);
        mod_ro_rm!(Reg, Opcode4, $r);
        mod_ro_rm!(Reg, Opcode5, $r);
        mod_ro_rm!(Reg, Opcode6, $r);
        mod_ro_rm!(Reg, Opcode7, $r);
    };
}

mod_rm_gp8!(AL);
mod_rm_gp8!(CL);
mod_rm_gp8!(DL);
mod_rm_gp8!(BL);
mod_rm_gp8!(AH);
mod_rm_gp8!(CH);
mod_rm_gp8!(DH);
mod_rm_gp8!(BH);

macro_rules! mod_rm_gp16 {
    ($r:tt) => {
        mod_ro_rm!(Reg, AX, $r);
        mod_ro_rm!(Reg, CX, $r);
        mod_ro_rm!(Reg, DX, $r);
        mod_ro_rm!(Reg, BX, $r);
        mod_ro_rm!(Reg, SP, $r);
        mod_ro_rm!(Reg, BP, $r);
        mod_ro_rm!(Reg, SI, $r);
        mod_ro_rm!(Reg, DI, $r);
        mod_ro_rm!(Reg, Opcode0, $r);
        mod_ro_rm!(Reg, Opcode1, $r);
        mod_ro_rm!(Reg, Opcode2, $r);
        mod_ro_rm!(Reg, Opcode3, $r);
        mod_ro_rm!(Reg, Opcode4, $r);
        mod_ro_rm!(Reg, Opcode5, $r);
        mod_ro_rm!(Reg, Opcode6, $r);
        mod_ro_rm!(Reg, Opcode7, $r);
    };
}

mod_rm_gp16!(AX);
mod_rm_gp16!(CX);
mod_rm_gp16!(DX);
mod_rm_gp16!(BX);
mod_rm_gp16!(SP);
mod_rm_gp16!(BP);
mod_rm_gp16!(SI);
mod_rm_gp16!(DI);

macro_rules! mod_rm_gp32 {
    ($r:tt) => {
        mod_ro_rm!(Reg, EAX, $r);
        mod_ro_rm!(Reg, ECX, $r);
        mod_ro_rm!(Reg, EDX, $r);
        mod_ro_rm!(Reg, EBX, $r);
        mod_ro_rm!(Reg, ESP, $r);
        mod_ro_rm!(Reg, EBP, $r);
        mod_ro_rm!(Reg, ESI, $r);
        mod_ro_rm!(Reg, EDI, $r);
    };
}

mod_rm_gp32!(EAX);
mod_rm_gp32!(ECX);
mod_rm_gp32!(EDX);
mod_rm_gp32!(EBX);
mod_rm_gp32!(ESP);
mod_rm_gp32!(EBP);
mod_rm_gp32!(ESI);
mod_rm_gp32!(EDI);

macro_rules! mod_rm_mm {
    ($r:tt) => {
        mod_ro_rm!(Reg, MM0, $r);
        mod_ro_rm!(Reg, MM1, $r);
        mod_ro_rm!(Reg, MM2, $r);
        mod_ro_rm!(Reg, MM3, $r);
        mod_ro_rm!(Reg, MM4, $r);
        mod_ro_rm!(Reg, MM5, $r);
        mod_ro_rm!(Reg, MM6, $r);
        mod_ro_rm!(Reg, MM7, $r);
    };
}

mod_rm_mm!(MM0);
mod_rm_mm!(MM1);
mod_rm_mm!(MM2);
mod_rm_mm!(MM3);
mod_rm_mm!(MM4);
mod_rm_mm!(MM5);
mod_rm_mm!(MM6);
mod_rm_mm!(MM7);

macro_rules! mod_rm_xmm {
    ($r:tt) => {
        mod_ro_rm!(Reg, XMM0, $r);
        mod_ro_rm!(Reg, XMM1, $r);
        mod_ro_rm!(Reg, XMM2, $r);
        mod_ro_rm!(Reg, XMM3, $r);
        mod_ro_rm!(Reg, XMM4, $r);
        mod_ro_rm!(Reg, XMM5, $r);
        mod_ro_rm!(Reg, XMM6, $r);
        mod_ro_rm!(Reg, XMM7, $r);
    };
}

mod_rm_xmm!(XMM0);
mod_rm_xmm!(XMM1);
mod_rm_xmm!(XMM2);
mod_rm_xmm!(XMM3);
mod_rm_xmm!(XMM4);
mod_rm_xmm!(XMM5);
mod_rm_xmm!(XMM6);
mod_rm_xmm!(XMM7);
