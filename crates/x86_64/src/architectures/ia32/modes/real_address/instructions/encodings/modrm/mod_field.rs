//! mod field in ModR/M addressing-mode encoding byte
//!
//! ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Section 2.1.3*

/// Mod part in ModR/M addressing-mode encoding byte indicating to expect no additional displacement or register operand
pub struct Disp0;
/// Mod part in ModR/M addressing-mode encoding byte indicating to expect an additional 1-byte displacement operand
pub struct Disp8;
/// Mod part in ModR/M addressing-mode encoding byte indicating to expect an additional 2-byte displacement operand
pub struct Disp16;
/// Mod part in ModR/M addressing-mode encoding byte indicating to expect a register operand
pub struct Reg;

/// Helper trait for mod field in ModR/M addressing-mode encoding byte
///
/// We want to inline correct encoding at compile time by overloading the method 'encode'
pub trait Mod<M> {
    /// Encodes as a u8 the mod field in ModR/M using 1 of the 4 ZSTs Disp0, Disp8, Disp16 and Reg
    ///
    /// *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 2, Table 2.1*
    fn encode(_mod: M) -> u8;
}

/// mod field in ModR/M addressing-mode encoding byte
pub struct MOD;

macro_rules! mode {
    ($md:ident) => {
        impl Mod<$md> for MOD {
            #[inline]
            fn encode(_mod: $md) -> u8 {
                DispMod::$md as u8
            }
        }
    };
}

mode!(Disp0);
mode!(Disp8);
mode!(Disp16);
mode!(Reg);

#[repr(u8)]
/// Mod part (0b1100_0000) of ModR/M addressing-mode encoding byte
pub enum DispMod {
    /// Value of Mod part in ModR/M addressing-mode encoding byte indicating to expect no additional displacement or register operand
    Disp0 = 0x00,
    /// Value of Mod part in ModR/M addressing-mode encoding byte indicating to expect an additional 1-byte displacement operand
    Disp8 = 0x40,
    /// Value of Mod part in ModR/M addressing-mode encoding byte indicating to expect an additional 2-byte displacement operand
    Disp16 = 0x80,
    /// Value of Mod part in ModR/M addressing-mode encoding byte indicating to expect a register operand
    Reg = 0xc0,
}
