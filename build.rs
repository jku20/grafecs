use std::process::Command;

fn main() {
    let status = Command::new("make")
        .current_dir("src/parser/")
        .status()
        .expect("failed to build DW dependency");
    assert!(status.success());
}
