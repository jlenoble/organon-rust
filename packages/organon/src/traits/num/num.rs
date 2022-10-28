use crate::{ Add, Sub };

pub trait IsNum: Copy + PartialEq + Add + Sub {}