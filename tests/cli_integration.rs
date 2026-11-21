use std::fs;
use std::process::Command;

fn temp_source(name: &str, source: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("vend-{}-{}.vend", name, std::process::id()));
    fs::write(&path, source).expect("temporary source should be writable");
    path
}

#[test]
fn cli_emits_assembly_to_stdout() {
    let source = temp_source("stdout", "fn main() { mov RAX, 1 }");
    let output = Command::new(env!("CARGO_BIN_EXE_vend"))
        .arg(&source)
        .output()
        .expect("vend binary should run");
    fs::remove_file(source).expect("temporary source should be removable");

    assert!(output.status.success());
    let assembly = String::from_utf8_lossy(&output.stdout);
    assert!(assembly.contains(".globl main"));
    assert!(assembly.contains("mov RAX, 1"));
}

#[test]
fn cli_reports_source_diagnostics_and_fails() {
    let source = temp_source("error", "fn main() { mov RAX }");
    let output = Command::new(env!("CARGO_BIN_EXE_vend"))
        .arg(&source)
        .output()
        .expect("vend binary should run");
    fs::remove_file(source).expect("temporary source should be removable");

    assert!(!output.status.success());
    let diagnostics = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostics.contains("expects exactly two operands"));
}

#[test]
fn cli_reports_semantic_diagnostics() {
    let source = temp_source("semantic-error", "fn main() {\n    mov RAX, ECX\n}");
    let output = Command::new(env!("CARGO_BIN_EXE_vend"))
        .arg(&source)
        .output()
        .expect("vend binary should run");
    fs::remove_file(source).expect("temporary source should be removable");

    assert!(!output.status.success());
    let diagnostics = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostics.contains("matching register widths"));
}

#[test]
fn cli_rejects_break_outside_a_loop() {
    let source = temp_source("break-outside-loop", "fn main() { break }");
    let output = Command::new(env!("CARGO_BIN_EXE_vend"))
        .arg(&source)
        .output()
        .expect("vend binary should run");
    fs::remove_file(source).expect("temporary source should be removable");

    assert!(!output.status.success());
    let diagnostics = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostics.contains("'break' is only valid inside a 'while' loop"));
}
