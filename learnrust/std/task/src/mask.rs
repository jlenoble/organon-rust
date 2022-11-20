#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Mask {
    Pending,
    Completed,
    Deleted,
    Waiting,
    Unknown,
}

impl Into<u8> for Mask {
    fn into(self) -> u8 {
        match self {
            Mask::Pending => b'-',
            Mask::Completed => b'+',
            Mask::Deleted => b'X',
            Mask::Waiting => b'W',
            Mask::Unknown => b'?',
        }
    }
}

impl Into<Mask> for u8 {
    fn into(self) -> Mask {
        match self {
            b'-' => Mask::Pending,
            b'+' => Mask::Completed,
            b'X' => Mask::Deleted,
            b'W' => Mask::Waiting,
            _ => Mask::Unknown,
        }
    }
}