use crate::{ Add, Copy, PartialEq, Sub };

pub trait IsNum: Copy + PartialEq + Add + Sub {}