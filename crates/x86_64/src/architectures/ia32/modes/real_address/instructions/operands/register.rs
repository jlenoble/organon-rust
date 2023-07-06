//! IA-32 real-address mode register operands

/// General purpose low byte register operands
pub mod gp_low_byte {
    /// General purpose low byte register operand AL
    pub struct AL;
    /// General purpose low byte register operand CL
    pub struct CL;
    /// General purpose low byte register operand DL
    pub struct DL;
    /// General purpose low byte register operand BL
    pub struct BL;
}

/// General purpose high byte register operands
pub mod gp_high_byte {
    /// General purpose low byte register operand AH
    pub struct AH;
    /// General purpose low byte register operand CH
    pub struct CH;
    /// General purpose low byte register operand DH
    pub struct DH;
    /// General purpose low byte register operand BH
    pub struct BH;
}

/// General purpose 16-bit register operands
pub mod gp_16bits {
    /// General purpose 16-bit register operand AX
    pub struct AX;
    /// General purpose 16-bit register operand CX
    pub struct CX;
    /// General purpose 16-bit register operand DX
    pub struct DX;
    /// General purpose 16-bit register operand BX
    pub struct BX;
    /// General purpose 16-bit register operand SP
    pub struct SP;
    /// General purpose 16-bit register operand BP
    pub struct BP;
    /// General purpose 16-bit register operand SI
    pub struct SI;
    /// General purpose 16-bit register operand DI
    pub struct DI;
}

/// Segment 16-bit register operands
pub mod segment {
    /// Segment 16-bit register operand CS
    pub struct CS;
    /// Segment 16-bit register operand DS
    pub struct DS;
    /// Segment 16-bit register operand SS
    pub struct SS;
    /// Segment 16-bit register operand ES
    pub struct ES;
    /// Segment 16-bit register operand FS
    pub struct FS;
    /// Segment 16-bit register operand GS
    pub struct GS;
}

pub use self::gp_low_byte::{ AL, CL, DL, BL };
pub use self::gp_high_byte::{ AH, CH, DH, BH };
pub use self::gp_16bits::{ AX, CX, DX, BX, SP, BP, SI, DI };
pub use self::segment::{ CS, DS, SS, ES, FS, GS };
