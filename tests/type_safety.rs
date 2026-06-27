#[test]
fn public_type_safety_contract() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/compile_fail/*.rs");
    tests.pass("tests/compile_pass/*.rs");
}
