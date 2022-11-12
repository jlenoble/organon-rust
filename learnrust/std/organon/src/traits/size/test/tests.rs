use super::{ Size, test_initializers };

#[test]
fn size() {
    let (_, s12, s4, s12bis, _, _) = test_initializers();

    assert_eq!(s12, s12);
    assert_eq!(s4, s4);
    assert_eq!(s12bis, s12bis);
    assert_eq!(s12, s12bis);

    let s12 = Size(s12);
    let s4 = Size(s4);
    let s12bis = Size(s12bis);

    assert_eq!(s12, s12);
    assert_eq!(s4, s4);
    assert_eq!(s12bis, s12bis);
    assert_eq!(s12, s12bis);
}

#[test]
fn add() {
    let (_, s12, s4, s12bis, s8, _) = test_initializers();

    assert_eq!(s12 + s12, s12 + s12);
    assert_eq!(s12 + s4, s12 + s4);
    assert_eq!(s12 + s4, s4 + s12);
    assert_eq!(s4 + s8, s12);
    assert_eq!(s4 + s8, s12bis);
    assert_eq!(s12 + s4 + s12bis + s8, s12bis + s12bis + s12bis);

    let s12 = Size(s12);
    let s4 = Size(s4);
    let s12bis = Size(s12bis);
    let s8 = Size(s8);

    assert_eq!(s12 + s12, s12 + s12);
    assert_eq!(s12 + s4, s12 + s4);
    assert_eq!(s12 + s4, s4 + s12);
    assert_eq!(s4 + s8, s12);
    assert_eq!(s4 + s8, s12bis);
    assert_eq!(s12 + s4 + s12bis + s8, s12bis + s12bis + s12bis);
}

#[test]
fn sub() {
    let (smin, s12, s4, s12bis, s8, _) = test_initializers();

    assert_eq!(s12 - s12, s12 - s12);
    assert_eq!(s12 - s12, smin);
    assert_eq!(s12 - s4, s12 - s4);
    assert_eq!(s12 - s12bis, smin);
    assert_eq!(s12 - s4, s8);
    assert_eq!(s8 - s4, s4);
    assert_eq!(s12 - s4 - s8, s12bis - s4 - s4 - s4);

    let s12 = Size(s12);
    let s4 = Size(s4);
    let s12bis = Size(s12bis);
    let s8 = Size(s8);

    assert_eq!(s12 - s12, s12 - s12);
    assert_eq!(s12 - s12, Size(smin));
    assert_eq!(s12 - s4, s12 - s4);
    assert_eq!(s12 - s12bis, Size(smin));
    assert_eq!(s12 - s4, s8);
    assert_eq!(s8 - s4, s4);
    assert_eq!(s12 - s4 - s8, s12bis - s4 - s4 - s4);
}

#[test]
fn add_sub() {
    let (_, s12, s4, s12bis, s8, _) = test_initializers();

    assert_eq!(s12 + s4 - s8, s12bis - s4 + s4 - s4 - s8 + s12 - s4);

    let s12 = Size(s12);
    let s4 = Size(s4);
    let s12bis = Size(s12bis);
    let s8 = Size(s8);

    assert_eq!(s12 + s4 - s8, s12bis - s4 + s4 - s4 - s8 + s12 - s4);
}

#[test]
fn underflow_safeguard() {
    let (smin, s12, s4, _, _, _) = test_initializers();

    assert_eq!(s4.saturating_sub(s12), smin);

    let smin = Size(smin);
    let s12 = Size(s12);
    let s4 = Size(s4);

    assert_eq!(s4 - s12, smin);
}

#[test]
fn overflow_safeguard() {
    let (_, _, s4, _, _, smax) = test_initializers();

    assert_eq!(s4.saturating_add(smax), smax);

    let smax = Size(smax);
    let s4 = Size(s4);

    assert_eq!(s4 + smax, smax);
}