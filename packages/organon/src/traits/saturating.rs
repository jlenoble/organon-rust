use crate::{ Add, Sub };

pub trait IsSaturating<Rhs = Self, Output = Self>: IsSaturatingAdd<Rhs, Output> +
    IsSaturatingSub<Rhs, Output> {}

pub trait IsSaturatingAdd<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Copy + PartialEq {
    fn saturating_add(self, v: Rhs) -> Output;
}

pub trait IsSaturatingSub<Rhs = Self, Output = Self>: Sub<Rhs, Output = Output> + Copy + PartialEq {
    fn saturating_sub(self, v: Rhs) -> Output;
}

#[cfg(test)]
pub mod tests {
    use crate::{ Add, Sub };
    use super::{ IsSaturating, IsSaturatingAdd, IsSaturatingSub };

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Num(u16);

    impl IsSaturating for Num {}

    impl IsSaturatingAdd for Num {
        fn saturating_add(self, v: Num) -> Num {
            Num(self.0.saturating_add(v.0))
        }
    }

    impl IsSaturatingSub for Num {
        fn saturating_sub(self, v: Num) -> Num {
            Num(self.0.saturating_sub(v.0))
        }
    }

    impl Add for Num {
        type Output = Num;

        fn add(self, v: Num) -> Num {
            self.saturating_add(v)
        }
    }

    impl Sub for Num {
        type Output = Num;

        fn sub(self, v: Num) -> Num {
            self.saturating_sub(v)
        }
    }

    #[test]
    fn broken_distributivity() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(8);

        assert_eq!(s1.saturating_add(s2.saturating_sub(s3)), s1 + (s2 - s3));
        assert_ne!(s1.saturating_add(s2.saturating_sub(s3)), s1 + s2 - s3);
    }

    #[test]
    fn reaching_bounds() {
        let s1 = Num(12);
        let s2 = Num(4);
        let s3 = Num(u16::MAX);

        assert_eq!(s2 - s1, Num(0));
        assert_eq!(s2 + s3, Num(u16::MAX));
    }
}