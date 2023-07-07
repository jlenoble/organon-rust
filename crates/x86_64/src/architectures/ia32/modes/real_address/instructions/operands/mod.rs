//! IA-32 real-address mode operands

pub mod immediate;
pub mod register;

pub use immediate::{ Imm8, Imm16 };

pub use register::{ AL, CL, DL, BL };
pub use register::{ AH, CH, DH, BH };

pub use register::{ AX, CX, DX, BX, SP, BP, SI, DI };
pub use register::{ EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI };
pub use register::{ MM0, MM1, MM2, MM3, MM4, MM5, MM6, MM7 };
pub use register::{ XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7 };
pub use register::{ GPReg8, GPReg16, GPReg32, MMReg, XMMReg };

pub use register::{ CS, DS, SS, ES, FS, GS };
