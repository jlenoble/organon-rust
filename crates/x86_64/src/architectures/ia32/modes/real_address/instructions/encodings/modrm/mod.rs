//! ModR/M addressing-mode encoding byte
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*

pub mod mod_field;
pub mod reg_opcode_field;
pub mod r_slash_m_field;

pub use mod_field::{ Disp0, Disp8, Disp16, Reg, MOD, Mod };
pub use reg_opcode_field::{
    Opcode0,
    Opcode1,
    Opcode2,
    Opcode3,
    Opcode4,
    Opcode5,
    Opcode6,
    Opcode7,
    REGOPCODE,
    RegOpcode,
};
pub use r_slash_m_field::{ BXpSI, BXpDI, BPpSI, BPpDI, RSLASHM, RSlashM };

#[cfg(test)]
mod tests;
