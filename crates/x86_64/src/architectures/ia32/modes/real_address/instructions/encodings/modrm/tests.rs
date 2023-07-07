use super::{ Disp0, Disp8, Disp16, Reg, MOD, Mod };

#[test]
fn test_mod_field() {
    assert_eq!(MOD::encode(Disp0), 0b00_000000);
    assert_eq!(MOD::encode(Disp8), 0b01_000000);
    assert_eq!(MOD::encode(Disp16), 0b10_000000);
    assert_eq!(MOD::encode(Reg), 0b11_000000);
}

#[ignore = "not implemented yet"]
#[test]
fn test_reg_opcode_field() {
    unimplemented!()
}

#[ignore = "not implemented yet"]
#[test]
fn test_r_slash_m_field() {
    unimplemented!()
}

#[ignore = "not implemented yet"]
#[test]
fn test_mod_r_slash_m() {
    unimplemented!()
}
