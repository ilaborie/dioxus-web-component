#![allow(missing_docs)]
use trybuild::TestCases;

#[test]
fn should_success() {
    let tests = TestCases::new();
    tests.pass("tests/assets/success/*.rs");
}

#[test]
#[ignore = "Fail with nightly because of a different output"]
fn should_fail() {
    let tests = TestCases::new();
    tests.compile_fail("tests/assets/failures/*.rs");
}
