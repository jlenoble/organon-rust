//! IA-32 real-address mode instructions
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 3, Section 20.1.3*

/// Encapsulates ASM IA-32 real-address mode instructions and Nasm pseudo-instructions and directives
pub struct ASM {
    origin: u16,
}

impl ASM {
    /// Creates a new encapsulation of implemented IA-32 real-address mode instructions
    pub const fn new() -> ASM {
        ASM {
            origin: 0,
        }
    }

    /// Nasm directive org sets global offset for addresses
    pub fn org(&mut self, origin: u16) {
        self.origin = origin;
    }
}

pub mod categories;
pub mod encodings;
pub mod operands;
