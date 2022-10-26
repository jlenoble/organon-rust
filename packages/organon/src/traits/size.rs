pub trait IsSize: PartialEq {}

#[cfg(test)]
pub mod tests {
    use super::IsSize;

    #[derive(Debug, PartialEq)]
    struct Size(u16);

    impl IsSize for Size {}

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
}