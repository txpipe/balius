use serde_json::Value;
use std::{path::PathBuf, process::Command};

pub fn get_project_info() -> (PathBuf, String) {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    let target_directory = PathBuf::from(metadata["target_directory"].as_str().unwrap());

    let package = &metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == metadata["resolve"]["root"])
        .expect("Failed to find root package");

    let package_name = package["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| {
            t["kind"]
                .as_array()
                .unwrap()
                .contains(&Value::String("cdylib".to_string()))
        })
        .map(|t| t["name"].as_str().unwrap().to_string())
        .unwrap_or_default();

    (target_directory, package_name)
}

pub fn execute() {
    println!("Building...");
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.arg("--target");
    cmd.arg("wasm32-unknown-unknown");

    let status = cmd.status().unwrap();

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    println!("Turning into component...");

    let (target_dir, package_name) = get_project_info();
    let wasm_file = target_dir.join(format!(
        "wasm32-unknown-unknown/debug/{}.wasm",
        package_name
    ));

    let mut cmd = Command::new("wasm-tools");
    cmd.arg("component");
    cmd.arg("new");
    cmd.arg(&wasm_file);
    cmd.arg("-o");
    cmd.arg(format!("{}-c.wasm", package_name));

    let status = cmd.status().unwrap();

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
