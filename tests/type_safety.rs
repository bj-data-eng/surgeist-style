#[test]
fn public_type_safety_contract() {
    let tests = trybuild::TestCases::new();
    // Includes font value privacy cases such as invalid_font_*_literal.rs.
    tests.compile_fail("tests/compile_fail/*.rs");
    tests.pass("tests/compile_pass/*.rs");
}
