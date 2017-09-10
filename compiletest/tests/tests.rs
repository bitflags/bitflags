extern crate compiletest_rs as compiletest;

use std::fs;
use std::path::PathBuf;
use compiletest::common::Mode;

fn run_mode(mode: Mode) {
    let config = compiletest::Config {
        mode: mode,
        src_base: PathBuf::from(format!("tests/{}", mode)),
        target_rustcflags: fs::read_dir("../target/debug/deps").unwrap().filter_map(|entry| {
            let path = entry.unwrap().path();
            path.file_name().map(|file_name| file_name.to_string_lossy()).and_then(|file_name| {
                if file_name.starts_with("libbitflags-") && file_name.ends_with(".rlib") {
                    Some(format!("--extern bitflags={}", path.to_string_lossy()))
                } else {
                    None
                }
            })
        }).next(),
        ..Default::default()
    };

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode(Mode::CompileFail);
}
