use std::{
    fs,
    ffi::OsStr,
    io,
    path::Path,
};

#[test]
fn compile_fail() {
    prepare_stderr_files("tests/compile-fail").unwrap();

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}

fn prepare_stderr_files(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if entry.path().extension().and_then(OsStr::to_str) == Some("beta") {
            let renamed = entry.path().with_extension("");

            if renamed.exists() {
                fs::remove_file(&renamed)?;
            }

            rename_beta_stderr(entry.path(), renamed)?;
        }
    }

    Ok(())
}

#[rustversion::beta]
fn rename_beta_stderr(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    fs::copy(from, to)?;

    Ok(())
}

#[rustversion::not(beta)]
fn rename_beta_stderr(_: impl AsRef<Path>, _: impl AsRef<Path>) -> io::Result<()> {
    Ok(())
}
