use core::ops::{ Add, Sub };

pub trait IsSize: Copy + PartialEq + Add + Sub {}

#[cfg(test)]
pub mod tests {
    use core::ops::{ Add, Sub };
    use super::IsSize;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Size(u16);

    impl IsSize for Size {}

    impl Add for Size {
        type Output = Size;

        fn add(self, s: Size) -> Size {
            Size(self.0 + s.0)
        }
    }

    impl Sub for Size {
        type Output = Size;

        fn sub(self, s: Size) -> Size {
            Size(self.0 - s.0)
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
}