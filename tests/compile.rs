// Compiler messages change frequently enough that we can end up with
// an impossible build between error messages emitted on various channels.
// Since https://github.com/dtolnay/trybuild/pull/170 we always need to have a
// `stderr` file for each test so we can't simply ignore the output on different channels.
#[cfg(not(miri))]
#[rustversion::attr(beta, test)]
#[allow(dead_code)]
fn fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/**/*.rs");
}

#[cfg(not(miri))]
#[test]
fn pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile-pass/**/*.rs");
}
