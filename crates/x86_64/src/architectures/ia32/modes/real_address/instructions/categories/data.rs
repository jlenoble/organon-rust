//! Data declaration pseudo-instructions
//!
//! *ref. https://www.nasm.us/doc/nasmdoc3.html#section-3.2.1*

/// Structure encapsulating Nasm data declaration pseudo-instructions
///
/// *ref. https://www.nasm.us/doc/nasmdoc3.html#section-3.2.1*
pub struct DATA;

impl DATA {
    /// Nasm data declaration pseudo-instruction db
    ///
    /// *ref. https://www.nasm.us/doc/nasmdoc3.html#section-3.2.1*
    #[inline]
    pub fn db(arg: u8) -> Vec<u8> {
        vec![arg]
    }

    /// Nasm data declaration pseudo-instruction dw
    ///
    /// *ref. https://www.nasm.us/doc/nasmdoc3.html#section-3.2.1*
    #[inline]
    pub fn dw(arg: u16) -> Vec<u8> {
        vec![(arg & 0xff) as u8, ((arg & 0xff00) >> 8) as u8]
    }
}
