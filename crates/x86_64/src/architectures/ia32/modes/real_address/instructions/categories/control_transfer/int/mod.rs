//! IA-32 real-address mode INT instruction
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 7.3.8.1*

use super::super::super::operands::Imm8;

/// IA-32 real-address mode INT instruction implementations
pub struct INT;

// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#INT line `CD ib`*
impl INT {
    /// INT n instruction for software-generated calls to interrupt handlers.
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 3.2#INT*
    #[inline]
    pub fn int(inter: Imm8) -> Vec<u8> {
        vec![0xcd, inter.0]
    }
}

#[cfg(test)]
mod tests;
