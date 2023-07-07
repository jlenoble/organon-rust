//! IA-32 real-address mode register operands

/// General purpose 8-bit register operands
pub mod gp_reg_8 {
    /// General purpose low byte register operand AL
    pub struct AL;
    /// General purpose low byte register operand CL
    pub struct CL;
    /// General purpose low byte register operand DL
    pub struct DL;
    /// General purpose low byte register operand BL
    pub struct BL;
    /// General purpose high byte register operand AH
    pub struct AH;
    /// General purpose high byte register operand CH
    pub struct CH;
    /// General purpose high byte register operand DH
    pub struct DH;
    /// General purpose high byte register operand BH
    pub struct BH;

    #[repr(u8)]
    /// General purpose 8-bit register numbers
    pub enum GPReg8 {
        /// General purpose low byte AL register number
        AL,
        /// General purpose low byte CL register number
        CL,
        /// General purpose low byte DL register number
        DL,
        /// General purpose low byte BL register number
        BL,
        /// General purpose high byte AH register number
        AH,
        /// General purpose high byte CH register number
        CH,
        /// General purpose high byte DH register number
        DH,
        /// General purpose high byte BH register number
        BH,
    }
}

/// General purpose 16-bit register operands
pub mod gp_reg_16 {
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

    #[repr(u8)]
    /// General purpose 16-bit register numbers
    pub enum GPReg16 {
        /// General purpose 16-bit AX register number
        AX,
        /// General purpose 16-bit CX register number
        CX,
        /// General purpose 16-bit DX register number
        DX,
        /// General purpose 16-bit BX register number
        BX,
        /// General purpose 16-bit SP register number
        SP,
        /// General purpose 16-bit BP register number
        BP,
        /// General purpose 16-bit SI register number
        SI,
        /// General purpose 16-bit DI register number
        DI,
    }
}

/// General purpose 32-bit register operands
pub mod gp_reg_32 {
    /// General purpose 32-bit register operand EAX
    pub struct EAX;
    /// General purpose 32-bit register operand ECX
    pub struct ECX;
    /// General purpose 32-bit register operand EDX
    pub struct EDX;
    /// General purpose 32-bit register operand EBX
    pub struct EBX;
    /// General purpose 32-bit register operand ESP
    pub struct ESP;
    /// General purpose 32-bit register operand EBP
    pub struct EBP;
    /// General purpose 32-bit register operand ESI
    pub struct ESI;
    /// General purpose 32-bit register operand EDI
    pub struct EDI;

    #[repr(u8)]
    /// General purpose 32-bit register numbers
    pub enum GPReg32 {
        /// General purpose 32-bit EAX register number
        EAX,
        /// General purpose 32-bit ECX register number
        ECX,
        /// General purpose 32-bit EDX register number
        EDX,
        /// General purpose 32-bit EBX register number
        EBX,
        /// General purpose 32-bit ESP register number
        ESP,
        /// General purpose 32-bit EBP register number
        EBP,
        /// General purpose 32-bit ESI register number
        ESI,
        /// General purpose 32-bit EDI register number
        EDI,
    }
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

/// MMX 64-bit register operands
pub mod mmx {
    /// General purpose 64-bit register operand MM0
    pub struct MM0;
    /// General purpose 64-bit register operand MM1
    pub struct MM1;
    /// General purpose 64-bit register operand MM2
    pub struct MM2;
    /// General purpose 64-bit register operand MM3
    pub struct MM3;
    /// General purpose 64-bit register operand MM4
    pub struct MM4;
    /// General purpose 64-bit register operand MM5
    pub struct MM5;
    /// General purpose 64-bit register operand MM6
    pub struct MM6;
    /// General purpose 64-bit register operand MM7
    pub struct MM7;

    #[repr(u8)]
    /// General purpose 64-bit MMX register numbers
    pub enum MMReg {
        /// General purpose 64-bit MM0 register number
        MM0,
        /// General purpose 64-bit MM1 register number
        MM1,
        /// General purpose 64-bit MM2 register number
        MM2,
        /// General purpose 64-bit MM3 register number
        MM3,
        /// General purpose 64-bit MM4 register number
        MM4,
        /// General purpose 64-bit MM5 register number
        MM5,
        /// General purpose 64-bit MM6 register number
        MM6,
        /// General purpose 64-bit MM7 register number
        MM7,
    }
}

/// SSE 128-bit register operands
pub mod sse {
    /// General purpose 128-bit register operand XMM0
    pub struct XMM0;
    /// General purpose 128-bit register operand XMM1
    pub struct XMM1;
    /// General purpose 128-bit register operand XMM2
    pub struct XMM2;
    /// General purpose 128-bit register operand XMM3
    pub struct XMM3;
    /// General purpose 128-bit register operand XMM4
    pub struct XMM4;
    /// General purpose 128-bit register operand XMM5
    pub struct XMM5;
    /// General purpose 128-bit register operand XMM6
    pub struct XMM6;
    /// General purpose 128-bit register operand XMM7
    pub struct XMM7;

    #[repr(u8)]
    /// General purpose 128-bit XMM register numbers
    pub enum XMMReg {
        /// General purpose 128-bit XMM0 register number
        XMM0,
        /// General purpose 128-bit XMM1 register number
        XMM1,
        /// General purpose 128-bit XMM2 register number
        XMM2,
        /// General purpose 128-bit XMM3 register number
        XMM3,
        /// General purpose 128-bit XMM4 register number
        XMM4,
        /// General purpose 128-bit XMM5 register number
        XMM5,
        /// General purpose 128-bit XMM6 register number
        XMM6,
        /// General purpose 128-bit XMM7 register number
        XMM7,
    }
}

pub use self::gp_reg_8::{ AL, CL, DL, BL, AH, CH, DH, BH, GPReg8 };
pub use self::gp_reg_16::{ AX, CX, DX, BX, SP, BP, SI, DI, GPReg16 };
pub use self::gp_reg_32::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, GPReg32 };

pub use self::segment::{ CS, DS, SS, ES, FS, GS };

pub use self::mmx::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7, MMReg };
pub use self::sse::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7, XMMReg };
