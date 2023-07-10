//! ModR/M addressing-mode encoding byte
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*

use super::{ Reg, MOD, Mod };
use super::{ REGOPCODE, RegOpcode };
use super::{ RSLASHM, RSlashM };

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
}

mod_ro_rm!(Reg, AL, AL);
mod_ro_rm!(Reg, AX, AX);
mod_ro_rm!(Reg, EAX, EAX);
mod_ro_rm!(Reg, MM0, MM0);
mod_ro_rm!(Reg, XMM0, XMM0);
mod_ro_rm!(Reg, CL, AL);
mod_ro_rm!(Reg, CX, AX);
mod_ro_rm!(Reg, ECX, EAX);
mod_ro_rm!(Reg, MM1, MM0);
mod_ro_rm!(Reg, XMM1, XMM0);
mod_ro_rm!(Reg, DL, AL);
mod_ro_rm!(Reg, DX, AX);
mod_ro_rm!(Reg, EDX, EAX);
mod_ro_rm!(Reg, MM2, MM0);
mod_ro_rm!(Reg, XMM2, XMM0);
mod_ro_rm!(Reg, BL, AL);
mod_ro_rm!(Reg, BX, AX);
mod_ro_rm!(Reg, EBX, EAX);
mod_ro_rm!(Reg, MM3, MM0);
mod_ro_rm!(Reg, XMM3, XMM0);
mod_ro_rm!(Reg, AH, AL);
mod_ro_rm!(Reg, SP, AX);
mod_ro_rm!(Reg, ESP, EAX);
mod_ro_rm!(Reg, MM4, MM0);
mod_ro_rm!(Reg, XMM4, XMM0);
mod_ro_rm!(Reg, CH, AL);
mod_ro_rm!(Reg, BP, AX);
mod_ro_rm!(Reg, EBP, EAX);
mod_ro_rm!(Reg, MM5, MM0);
mod_ro_rm!(Reg, XMM5, XMM0);
mod_ro_rm!(Reg, DH, AL);
mod_ro_rm!(Reg, SI, AX);
mod_ro_rm!(Reg, ESI, EAX);
mod_ro_rm!(Reg, MM6, MM0);
mod_ro_rm!(Reg, XMM6, XMM0);
mod_ro_rm!(Reg, BH, AL);
mod_ro_rm!(Reg, DI, AX);
mod_ro_rm!(Reg, EDI, EAX);
mod_ro_rm!(Reg, MM7, MM0);
mod_ro_rm!(Reg, XMM7, XMM0);

mod_ro_rm!(Reg, AL, CL);
mod_ro_rm!(Reg, AX, CX);
mod_ro_rm!(Reg, EAX, ECX);
mod_ro_rm!(Reg, MM0, MM1);
mod_ro_rm!(Reg, XMM0, XMM1);
mod_ro_rm!(Reg, CL, CL);
mod_ro_rm!(Reg, CX, CX);
mod_ro_rm!(Reg, ECX, ECX);
mod_ro_rm!(Reg, MM1, MM1);
mod_ro_rm!(Reg, XMM1, XMM1);
mod_ro_rm!(Reg, DL, CL);
mod_ro_rm!(Reg, DX, CX);
mod_ro_rm!(Reg, EDX, ECX);
mod_ro_rm!(Reg, MM2, MM1);
mod_ro_rm!(Reg, XMM2, XMM1);
mod_ro_rm!(Reg, BL, CL);
mod_ro_rm!(Reg, BX, CX);
mod_ro_rm!(Reg, EBX, ECX);
mod_ro_rm!(Reg, MM3, MM1);
mod_ro_rm!(Reg, XMM3, XMM1);
mod_ro_rm!(Reg, AH, CL);
mod_ro_rm!(Reg, SP, CX);
mod_ro_rm!(Reg, ESP, ECX);
mod_ro_rm!(Reg, MM4, MM1);
mod_ro_rm!(Reg, XMM4, XMM1);
mod_ro_rm!(Reg, CH, CL);
mod_ro_rm!(Reg, BP, CX);
mod_ro_rm!(Reg, EBP, ECX);
mod_ro_rm!(Reg, MM5, MM1);
mod_ro_rm!(Reg, XMM5, XMM1);
mod_ro_rm!(Reg, DH, CL);
mod_ro_rm!(Reg, SI, CX);
mod_ro_rm!(Reg, ESI, ECX);
mod_ro_rm!(Reg, MM6, MM1);
mod_ro_rm!(Reg, XMM6, XMM1);
mod_ro_rm!(Reg, BH, CL);
mod_ro_rm!(Reg, DI, CX);
mod_ro_rm!(Reg, EDI, ECX);
mod_ro_rm!(Reg, MM7, MM1);
mod_ro_rm!(Reg, XMM7, XMM1);

mod_ro_rm!(Reg, AL, DL);
mod_ro_rm!(Reg, AX, DX);
mod_ro_rm!(Reg, EAX, EDX);
mod_ro_rm!(Reg, MM0, MM2);
mod_ro_rm!(Reg, XMM0, XMM2);
mod_ro_rm!(Reg, CL, DL);
mod_ro_rm!(Reg, CX, DX);
mod_ro_rm!(Reg, ECX, EDX);
mod_ro_rm!(Reg, MM1, MM2);
mod_ro_rm!(Reg, XMM1, XMM2);
mod_ro_rm!(Reg, DL, DL);
mod_ro_rm!(Reg, DX, DX);
mod_ro_rm!(Reg, EDX, EDX);
mod_ro_rm!(Reg, MM2, MM2);
mod_ro_rm!(Reg, XMM2, XMM2);
mod_ro_rm!(Reg, BL, DL);
mod_ro_rm!(Reg, BX, DX);
mod_ro_rm!(Reg, EBX, EDX);
mod_ro_rm!(Reg, MM3, MM2);
mod_ro_rm!(Reg, XMM3, XMM2);
mod_ro_rm!(Reg, AH, DL);
mod_ro_rm!(Reg, SP, DX);
mod_ro_rm!(Reg, ESP, EDX);
mod_ro_rm!(Reg, MM4, MM2);
mod_ro_rm!(Reg, XMM4, XMM2);
mod_ro_rm!(Reg, CH, DL);
mod_ro_rm!(Reg, BP, DX);
mod_ro_rm!(Reg, EBP, EDX);
mod_ro_rm!(Reg, MM5, MM2);
mod_ro_rm!(Reg, XMM5, XMM2);
mod_ro_rm!(Reg, DH, DL);
mod_ro_rm!(Reg, SI, DX);
mod_ro_rm!(Reg, ESI, EDX);
mod_ro_rm!(Reg, MM6, MM2);
mod_ro_rm!(Reg, XMM6, XMM2);
mod_ro_rm!(Reg, BH, DL);
mod_ro_rm!(Reg, DI, DX);
mod_ro_rm!(Reg, EDI, EDX);
mod_ro_rm!(Reg, MM7, MM2);
mod_ro_rm!(Reg, XMM7, XMM2);

mod_ro_rm!(Reg, AL, BL);
mod_ro_rm!(Reg, AX, BX);
mod_ro_rm!(Reg, EAX, EBX);
mod_ro_rm!(Reg, MM0, MM3);
mod_ro_rm!(Reg, XMM0, XMM3);
mod_ro_rm!(Reg, CL, BL);
mod_ro_rm!(Reg, CX, BX);
mod_ro_rm!(Reg, ECX, EBX);
mod_ro_rm!(Reg, MM1, MM3);
mod_ro_rm!(Reg, XMM1, XMM3);
mod_ro_rm!(Reg, DL, BL);
mod_ro_rm!(Reg, DX, BX);
mod_ro_rm!(Reg, EDX, EBX);
mod_ro_rm!(Reg, MM2, MM3);
mod_ro_rm!(Reg, XMM2, XMM3);
mod_ro_rm!(Reg, BL, BL);
mod_ro_rm!(Reg, BX, BX);
mod_ro_rm!(Reg, EBX, EBX);
mod_ro_rm!(Reg, MM3, MM3);
mod_ro_rm!(Reg, XMM3, XMM3);
mod_ro_rm!(Reg, AH, BL);
mod_ro_rm!(Reg, SP, BX);
mod_ro_rm!(Reg, ESP, EBX);
mod_ro_rm!(Reg, MM4, MM3);
mod_ro_rm!(Reg, XMM4, XMM3);
mod_ro_rm!(Reg, CH, BL);
mod_ro_rm!(Reg, BP, BX);
mod_ro_rm!(Reg, EBP, EBX);
mod_ro_rm!(Reg, MM5, MM3);
mod_ro_rm!(Reg, XMM5, XMM3);
mod_ro_rm!(Reg, DH, BL);
mod_ro_rm!(Reg, SI, BX);
mod_ro_rm!(Reg, ESI, EBX);
mod_ro_rm!(Reg, MM6, MM3);
mod_ro_rm!(Reg, XMM6, XMM3);
mod_ro_rm!(Reg, BH, BL);
mod_ro_rm!(Reg, DI, BX);
mod_ro_rm!(Reg, EDI, EBX);
mod_ro_rm!(Reg, MM7, MM3);
mod_ro_rm!(Reg, XMM7, XMM3);

mod_ro_rm!(Reg, AL, AH);
mod_ro_rm!(Reg, AX, SP);
mod_ro_rm!(Reg, EAX, ESP);
mod_ro_rm!(Reg, MM0, MM4);
mod_ro_rm!(Reg, XMM0, XMM4);
mod_ro_rm!(Reg, CL, AH);
mod_ro_rm!(Reg, CX, SP);
mod_ro_rm!(Reg, ECX, ESP);
mod_ro_rm!(Reg, MM1, MM4);
mod_ro_rm!(Reg, XMM1, XMM4);
mod_ro_rm!(Reg, DL, AH);
mod_ro_rm!(Reg, DX, SP);
mod_ro_rm!(Reg, EDX, ESP);
mod_ro_rm!(Reg, MM2, MM4);
mod_ro_rm!(Reg, XMM2, XMM4);
mod_ro_rm!(Reg, BL, AH);
mod_ro_rm!(Reg, BX, SP);
mod_ro_rm!(Reg, EBX, ESP);
mod_ro_rm!(Reg, MM3, MM4);
mod_ro_rm!(Reg, XMM3, XMM4);
mod_ro_rm!(Reg, AH, AH);
mod_ro_rm!(Reg, SP, SP);
mod_ro_rm!(Reg, ESP, ESP);
mod_ro_rm!(Reg, MM4, MM4);
mod_ro_rm!(Reg, XMM4, XMM4);
mod_ro_rm!(Reg, CH, AH);
mod_ro_rm!(Reg, BP, SP);
mod_ro_rm!(Reg, EBP, ESP);
mod_ro_rm!(Reg, MM5, MM4);
mod_ro_rm!(Reg, XMM5, XMM4);
mod_ro_rm!(Reg, DH, AH);
mod_ro_rm!(Reg, SI, SP);
mod_ro_rm!(Reg, ESI, ESP);
mod_ro_rm!(Reg, MM6, MM4);
mod_ro_rm!(Reg, XMM6, XMM4);
mod_ro_rm!(Reg, BH, AH);
mod_ro_rm!(Reg, DI, SP);
mod_ro_rm!(Reg, EDI, ESP);
mod_ro_rm!(Reg, MM7, MM4);
mod_ro_rm!(Reg, XMM7, XMM4);

mod_ro_rm!(Reg, AL, CH);
mod_ro_rm!(Reg, AX, BP);
mod_ro_rm!(Reg, EAX, EBP);
mod_ro_rm!(Reg, MM0, MM5);
mod_ro_rm!(Reg, XMM0, XMM5);
mod_ro_rm!(Reg, CL, CH);
mod_ro_rm!(Reg, CX, BP);
mod_ro_rm!(Reg, ECX, EBP);
mod_ro_rm!(Reg, MM1, MM5);
mod_ro_rm!(Reg, XMM1, XMM5);
mod_ro_rm!(Reg, DL, CH);
mod_ro_rm!(Reg, DX, BP);
mod_ro_rm!(Reg, EDX, EBP);
mod_ro_rm!(Reg, MM2, MM5);
mod_ro_rm!(Reg, XMM2, XMM5);
mod_ro_rm!(Reg, BL, CH);
mod_ro_rm!(Reg, BX, BP);
mod_ro_rm!(Reg, EBX, EBP);
mod_ro_rm!(Reg, MM3, MM5);
mod_ro_rm!(Reg, XMM3, XMM5);
mod_ro_rm!(Reg, AH, CH);
mod_ro_rm!(Reg, SP, BP);
mod_ro_rm!(Reg, ESP, EBP);
mod_ro_rm!(Reg, MM4, MM5);
mod_ro_rm!(Reg, XMM4, XMM5);
mod_ro_rm!(Reg, CH, CH);
mod_ro_rm!(Reg, BP, BP);
mod_ro_rm!(Reg, EBP, EBP);
mod_ro_rm!(Reg, MM5, MM5);
mod_ro_rm!(Reg, XMM5, XMM5);
mod_ro_rm!(Reg, DH, CH);
mod_ro_rm!(Reg, SI, BP);
mod_ro_rm!(Reg, ESI, EBP);
mod_ro_rm!(Reg, MM6, MM5);
mod_ro_rm!(Reg, XMM6, XMM5);
mod_ro_rm!(Reg, BH, CH);
mod_ro_rm!(Reg, DI, BP);
mod_ro_rm!(Reg, EDI, EBP);
mod_ro_rm!(Reg, MM7, MM5);
mod_ro_rm!(Reg, XMM7, XMM5);

mod_ro_rm!(Reg, AL, DH);
mod_ro_rm!(Reg, AX, SI);
mod_ro_rm!(Reg, EAX, ESI);
mod_ro_rm!(Reg, MM0, MM6);
mod_ro_rm!(Reg, XMM0, XMM6);
mod_ro_rm!(Reg, CL, DH);
mod_ro_rm!(Reg, CX, SI);
mod_ro_rm!(Reg, ECX, ESI);
mod_ro_rm!(Reg, MM1, MM6);
mod_ro_rm!(Reg, XMM1, XMM6);
mod_ro_rm!(Reg, DL, DH);
mod_ro_rm!(Reg, DX, SI);
mod_ro_rm!(Reg, EDX, ESI);
mod_ro_rm!(Reg, MM2, MM6);
mod_ro_rm!(Reg, XMM2, XMM6);
mod_ro_rm!(Reg, BL, DH);
mod_ro_rm!(Reg, BX, SI);
mod_ro_rm!(Reg, EBX, ESI);
mod_ro_rm!(Reg, MM3, MM6);
mod_ro_rm!(Reg, XMM3, XMM6);
mod_ro_rm!(Reg, AH, DH);
mod_ro_rm!(Reg, SP, SI);
mod_ro_rm!(Reg, ESP, ESI);
mod_ro_rm!(Reg, MM4, MM6);
mod_ro_rm!(Reg, XMM4, XMM6);
mod_ro_rm!(Reg, CH, DH);
mod_ro_rm!(Reg, BP, SI);
mod_ro_rm!(Reg, EBP, ESI);
mod_ro_rm!(Reg, MM5, MM6);
mod_ro_rm!(Reg, XMM5, XMM6);
mod_ro_rm!(Reg, DH, DH);
mod_ro_rm!(Reg, SI, SI);
mod_ro_rm!(Reg, ESI, ESI);
mod_ro_rm!(Reg, MM6, MM6);
mod_ro_rm!(Reg, XMM6, XMM6);
mod_ro_rm!(Reg, BH, DH);
mod_ro_rm!(Reg, DI, SI);
mod_ro_rm!(Reg, EDI, ESI);
mod_ro_rm!(Reg, MM7, MM6);
mod_ro_rm!(Reg, XMM7, XMM6);

mod_ro_rm!(Reg, AL, BH);
mod_ro_rm!(Reg, AX, DI);
mod_ro_rm!(Reg, EAX, EDI);
mod_ro_rm!(Reg, MM0, MM7);
mod_ro_rm!(Reg, XMM0, XMM7);
mod_ro_rm!(Reg, CL, BH);
mod_ro_rm!(Reg, CX, DI);
mod_ro_rm!(Reg, ECX, EDI);
mod_ro_rm!(Reg, MM1, MM7);
mod_ro_rm!(Reg, XMM1, XMM7);
mod_ro_rm!(Reg, DL, BH);
mod_ro_rm!(Reg, DX, DI);
mod_ro_rm!(Reg, EDX, EDI);
mod_ro_rm!(Reg, MM2, MM7);
mod_ro_rm!(Reg, XMM2, XMM7);
mod_ro_rm!(Reg, BL, BH);
mod_ro_rm!(Reg, BX, DI);
mod_ro_rm!(Reg, EBX, EDI);
mod_ro_rm!(Reg, MM3, MM7);
mod_ro_rm!(Reg, XMM3, XMM7);
mod_ro_rm!(Reg, AH, BH);
mod_ro_rm!(Reg, SP, DI);
mod_ro_rm!(Reg, ESP, EDI);
mod_ro_rm!(Reg, MM4, MM7);
mod_ro_rm!(Reg, XMM4, XMM7);
mod_ro_rm!(Reg, CH, BH);
mod_ro_rm!(Reg, BP, DI);
mod_ro_rm!(Reg, EBP, EDI);
mod_ro_rm!(Reg, MM5, MM7);
mod_ro_rm!(Reg, XMM5, XMM7);
mod_ro_rm!(Reg, DH, BH);
mod_ro_rm!(Reg, SI, DI);
mod_ro_rm!(Reg, ESI, EDI);
mod_ro_rm!(Reg, MM6, MM7);
mod_ro_rm!(Reg, XMM6, XMM7);
mod_ro_rm!(Reg, BH, BH);
mod_ro_rm!(Reg, DI, DI);
mod_ro_rm!(Reg, EDI, EDI);
mod_ro_rm!(Reg, MM7, MM7);
mod_ro_rm!(Reg, XMM7, XMM7);
