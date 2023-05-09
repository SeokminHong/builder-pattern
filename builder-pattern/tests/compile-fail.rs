#[test]
fn compile_fail() {
    trybuild::TestCases::new().compile_fail("tests/compile-fail/*.rs");
}
