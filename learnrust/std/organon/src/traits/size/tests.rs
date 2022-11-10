use super::Size;

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