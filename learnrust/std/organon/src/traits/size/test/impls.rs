use crate::{
    Add,
    Sub,
    traits::{
        num::IsNum,
        saturating::{ IsSaturating, IsSaturatingAdd, IsSaturatingSub },
        size::IsSize,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Size(pub usize);

impl IsSize for Size {}
impl IsNum for Size {}
impl IsSaturating for Size {}

impl IsSaturatingAdd for Size {
    fn saturating_add(self, v: Size) -> Size {
        Size(self.0.saturating_add(v.0))
    }
}

impl IsSaturatingSub for Size {
    fn saturating_sub(self, v: Size) -> Size {
        Size(self.0.saturating_sub(v.0))
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, s: Size) -> Size {
        self.saturating_add(s)
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, s: Size) -> Size {
        self.saturating_sub(s)
    }
}