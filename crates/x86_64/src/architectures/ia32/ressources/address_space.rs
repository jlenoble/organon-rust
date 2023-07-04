//! IA-32 address space
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 3.2*

use ux::u36;

/// IA-32 linear address
///
/// Since IA-32 processor can address a linear address space of
/// up to 4 GBytes (2^32 bytes), struct `LinearAddress` is just a wrapper
/// around `u32`.
pub struct LinearAddress(u32);

/// IA-32 physical address
///
/// Since IA-32 processor can address a physical address space of
/// up to 64 GBytes (2^36 bytes), we use crate `ux` and make struct
/// `PhysicalAddress` a wrapper around `ux::u36`
pub struct PhysicalAddress(u36);
