//! IA-32 general-purpose registers
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 3.4.1*

/// IA-32 EAX general-purpose 32-bit register
pub struct EAX;
/// IA-32 ECX general-purpose 32-bit register
pub struct ECX;
/// IA-32 EDX general-purpose 32-bit register
pub struct EDX;
/// IA-32 EBX general-purpose 32-bit register
pub struct EBX;
/// IA-32 ESP general-purpose 32-bit register
pub struct ESP;
/// IA-32 EBP general-purpose 32-bit register
pub struct EBP;
/// IA-32 ESI general-purpose 32-bit register
pub struct ESI;
/// IA-32 EDI general-purpose 32-bit register
pub struct EDI;

/// IA-32 AX general-purpose 16-bit register
pub struct AX;
/// IA-32 CX general-purpose 16-bit register
pub struct CX;
/// IA-32 DX general-purpose 16-bit register
pub struct DX;
/// IA-32 BX general-purpose 16-bit register
pub struct BX;
/// IA-32 SP general-purpose 16-bit register
pub struct SP;
/// IA-32 BP general-purpose 16-bit register
pub struct BP;
/// IA-32 SI general-purpose 16-bit register
pub struct SI;
/// IA-32 DI general-purpose 16-bit register
pub struct DI;

/// IA-32 AL general-purpose low-byte register
pub struct AL;
/// IA-32 CL general-purpose low-byte register
pub struct CL;
/// IA-32 DL general-purpose low-byte register
pub struct DL;
/// IA-32 BL general-purpose low-byte register
pub struct BL;

/// IA-32 AH general-purpose high-byte register
pub struct AH;
/// IA-32 CH general-purpose high-byte register
pub struct CH;
/// IA-32 DH general-purpose high-byte register
pub struct DH;
/// IA-32 BH general-purpose high-byte register
pub struct BH;
