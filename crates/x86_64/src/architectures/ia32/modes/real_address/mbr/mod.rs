//! Master boot record (MBR)

const MBR_SIZE: usize = 512;

/// Master boot record (MBR)
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct MBR {
    buffer: [u8; MBR_SIZE],
    current_position: usize,
}

impl MBR {
    /// Constructor for master boot record
    pub fn new() -> MBR {
        let mut buf: [u8; MBR_SIZE] = [0; MBR_SIZE];
        buf[MBR_SIZE - 2] = 0x55;
        buf[MBR_SIZE - 1] = 0xaa;

        MBR {
            buffer: buf,
            current_position: 0,
        }
    }

    /// Push instruction to master boot record
    pub fn push(&mut self, instr: Vec<u8>) {
        self.buffer[self.current_position..self.current_position + instr.len()].copy_from_slice(
            instr.as_slice()
        );
        self.current_position += instr.len();
    }
}

#[cfg(test)]
mod tests;
