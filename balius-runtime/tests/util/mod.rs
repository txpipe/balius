use std::{path::{Path, PathBuf}, process::Command};

use wit_component::ComponentEncoder;

pub(crate) fn build_module(src_dir: impl AsRef<Path>, module_name: &str, target: impl AsRef<Path>) {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir(src_dir.as_ref())
        .output()
        .unwrap();
    if !output.stderr.is_empty() {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
    }
    if !output.status.success() {
        panic!("command failed: {}", output.status);
    }

    let wasm_path =
        PathBuf::from("../target/wasm32-unknown-unknown/debug").join(format!("{module_name}.wasm"));
    let module = wat::Parser::new().parse_file(wasm_path).unwrap();
    let component = ComponentEncoder::default()
        .validate(true)
        .module(&module)
        .unwrap()
        .encode()
        .unwrap();

    std::fs::write(target.as_ref(), component).unwrap();
}