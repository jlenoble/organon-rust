use super::{ num::IsNum, saturating::IsSaturating };

pub trait IsSize: IsNum + IsSaturating {}

#[cfg(test)]
pub mod tests {
    use crate::{ Add, Sub };
    use super::super::{
        num::IsNum,
        saturating::{ IsSaturating, IsSaturatingAdd, IsSaturatingSub },
    };
    use super::IsSize;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Size(u16);

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

    #[test]
    fn size() {
        let s1 = Size(12);
        let s2 = Size(4);
        let s3 = Size(12);

        assert_eq!(s1, s1);
        assert_eq!(s2, s2);
        assert_eq!(s3, s3);
        assert_eq!(s1, s3);
    }

    #[test]
    fn add() {
        let s1 = Size(12);
        let s2 = Size(4);
        let s3 = Size(12);
        let s4 = Size(8);

        assert_eq!(s1 + s1, s1 + s1);
        assert_eq!(s1 + s2, s1 + s2);
        assert_eq!(s1 + s2, s2 + s1);
        assert_eq!(s2 + s4, s1);
        assert_eq!(s2 + s4, s3);
        assert_eq!(s1 + s2 + s3 + s4, s3 + s3 + s3);
    }

    #[test]
    fn sub() {
        let s1 = Size(12);
        let s2 = Size(4);
        let s3 = Size(12);
        let s4 = Size(8);

        assert_eq!(s1 - s1, s1 - s1);
        assert_eq!(s1 - s1, Size(0));
        assert_eq!(s1 - s2, s1 - s2);
        assert_eq!(s1 - s3, Size(0));
        assert_eq!(s1 - s2, s4);
        assert_eq!(s4 - s2, s2);
        assert_eq!(s1 - s2 - s4, s3 - s2 - s2 - s2);
    }

    #[test]
    fn add_sub() {
        let s1 = Size(12);
        let s2 = Size(4);
        let s3 = Size(12);
        let s4 = Size(8);

        assert_eq!(s1 + s2 - s4, s3 - s2 + s2 - s2 - s4 + s1 - s2);
    }

    #[test]
    fn underflow_safeguard() {
        let s1 = Size(12);
        let s2 = Size(4);

        assert_eq!(s2 - s1, Size(0));
    }

    #[test]
    fn overflow_safeguard() {
        let s1 = Size(12);
        let s2 = Size(u16::MAX);

        assert_eq!(s2 + s1, Size(u16::MAX));
    }
}