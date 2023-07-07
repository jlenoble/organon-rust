//! ModR/M addressing-mode encoding byte
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*

pub mod mod_field;

pub use mod_field::{ Disp0, Disp8, Disp16, Reg, MOD, Mod };

#[cfg(test)]
mod tests;
