//! IA-32 general-purpose registers
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 3.4.1*

#[derive(Clone, Copy)]
/// IA-32 general-purpose 32-bit registers
pub enum IA32Reg32 {
    /// IA-32 EAX general-purpose 32-bit register
    EAX,
    /// IA-32 ECX general-purpose 32-bit register
    ECX,
    /// IA-32 EDX general-purpose 32-bit register
    EDX,
    /// IA-32 EBX general-purpose 32-bit register
    EBX,
    /// IA-32 ESP general-purpose 32-bit register
    ESP,
    /// IA-32 EBP general-purpose 32-bit register
    EBP,
    /// IA-32 ESI general-purpose 32-bit register
    ESI,
    /// IA-32 EDI general-purpose 32-bit register
    EDI,
}

#[derive(Clone, Copy)]
/// IA-32 general-purpose 16-bit registers
pub enum IA32Reg16 {
    /// IA-32 AX general-purpose 16-bit register
    AX,
    /// IA-32 CX general-purpose 16-bit register
    CX,
    /// IA-32 DX general-purpose 16-bit register
    DX,
    /// IA-32 BX general-purpose 16-bit register
    BX,
    /// IA-32 SP general-purpose 16-bit register
    SP,
    /// IA-32 BP general-purpose 16-bit register
    BP,
    /// IA-32 SI general-purpose 16-bit register
    SI,
    /// IA-32 DI general-purpose 16-bit register
    DI,
}

#[derive(Clone, Copy)]
/// IA-32 general-purpose 8-bit registers
pub enum IA32Reg8 {
    /// IA-32 AL general-purpose low-byte register
    AL,
    /// IA-32 CL general-purpose low-byte register
    CL,
    /// IA-32 DL general-purpose low-byte register
    DL,
    /// IA-32 BL general-purpose low-byte register
    BL,
    /// IA-32 AH general-purpose high-byte register
    AH,
    /// IA-32 CH general-purpose high-byte register
    CH,
    /// IA-32 DH general-purpose high-byte register
    DH,
    /// IA-32 BH general-purpose high-byte register
    BH,
}
