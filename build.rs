use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    let status = Command::new("make")
        .current_dir("src/parser/")
        .status()
        .expect("failed to build DW dependency");
    assert!(status.success());
}
