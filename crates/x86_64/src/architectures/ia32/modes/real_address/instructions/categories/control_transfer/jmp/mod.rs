//! IA-32 real-address mode JMP instruction
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.8.1*

use super::super::super::operands::{ Rel8, Rel16 };

/// Trait for IA-32 real-address mode JMP instruction variants
pub trait Jmp<Dest> {
    /// generic real-address mode JMP instruction
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#JMP*
    fn jmp(dest: Dest) -> Vec<u8>;
}

/// IA-32 real-address mode JMP instruction implementations
pub struct JMP;

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#JMP line `EB cb`*
impl Jmp<Rel8> for JMP {
    #[inline]
    fn jmp(dest: Rel8) -> Vec<u8> {
        vec![0xeb, dest.0 as u8]
    }
}

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#JMP line `E9 cw`*
impl Jmp<Rel16> for JMP {
    #[inline]
    fn jmp(dest: Rel16) -> Vec<u8> {
        vec![0xe9, (dest.0 & 0xff) as u8, (((dest.0 as u16) & 0xff00) >> 8) as u8]
    }
}

#[cfg(test)]
mod tests;
