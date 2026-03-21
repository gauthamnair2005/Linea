use std::process::Command;

#[test]
fn compile_missing_file_shows_red_bold_failure_prefix() {
    let exe = env!("CARGO_BIN_EXE_linea-compiler");
    let output = Command::new(exe)
        .arg("compile")
        .arg("definitely_missing_file.ln")
        .output()
        .expect("linea-compiler should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("\u{1b}[1m\u{1b}[31m✗ FAILURE:"));
}
