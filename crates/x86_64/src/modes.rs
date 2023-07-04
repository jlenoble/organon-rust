//! Intel® 64 and IA-32 operating modes
//!
//! *ref.: Intel® 64 and IA-32 Architectures Software Developer’s Manual, Vol. 1, Section 3.1*

pub use crate::architectures::ia32::modes::protected;
pub use crate::architectures::ia32::modes::real_address;
pub use crate::architectures::ia32::modes::system_management;

pub use crate::architectures::intel64::modes::ia32e_compatibility;
pub use crate::architectures::intel64::modes::ia32e_64bit;
