//! IA-32 real-address mode operands

pub mod immediate;
pub mod register;

pub use immediate::{ Imm8, Imm16 };
pub use register::{ AL, CL, DL, BL };
pub use register::{ AH, CH, DH, BH };
pub use register::{ AX, CX, DX, BX, SP, BP, SI, DI };
pub use register::{ CS, DS, SS, ES, FS, GS };
