use crate::{ Add, Sub };

pub trait IsNum: Copy + PartialEq + Add + Sub {}

#[cfg(test)]
pub mod tests {
    use crate::{ Add, Sub };
    use super::IsNum;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Num(u16);

    impl IsNum for Num {}

    impl Add for Num {
        type Output = Num;

        fn add(self, s: Num) -> Num {
            Num(self.0 + s.0)
        }
    }

    impl Sub for Num {
        type Output = Num;

        fn sub(self, s: Num) -> Num {
            Num(self.0 - s.0)
        }
    }

    #[test]
    fn size() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(12);

        assert_eq!(s1, s1);
        assert_eq!(s2, s2);
        assert_eq!(s3, s3);
        assert_eq!(s1, s3);
    }

    #[test]
    fn add() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(12);
        let s4 = Num(8);

        assert_eq!(s1 + s1, s1 + s1);
        assert_eq!(s1 + s2, s1 + s2);
        assert_eq!(s1 + s2, s2 + s1);
        assert_eq!(s2 + s4, s1);
        assert_eq!(s2 + s4, s3);
        assert_eq!(s1 + s2 + s3 + s4, s3 + s3 + s3);
    }

    #[test]
    fn sub() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(12);
        let s4 = Num(8);

        assert_eq!(s1 - s1, s1 - s1);
        assert_eq!(s1 - s1, Num(0));
        assert_eq!(s1 - s2, s1 - s2);
        assert_eq!(s1 - s3, Num(0));
        assert_eq!(s1 - s2, s4);
        assert_eq!(s4 - s2, s2);
        assert_eq!(s1 - s2 - s4, s3 - s2 - s2 - s2);
    }

    #[test]
    fn add_sub() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(12);
        let s4 = Num(8);

        assert_eq!(s1 + s2 - s4, s3 - s2 + s2 - s2 - s4 + s1 - s2);
    }

    #[test]
    #[should_panic]
    fn no_negative_safeguard() {
        let s1 = Num(12);
        let s2 = Num(4);

        assert_eq!(s2 - s1, Num(0));
    }
}