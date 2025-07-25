use balius::utils::get_project_info;
use std::process::Command;

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
    let wasm_file = target_dir.join(format!("wasm32-unknown-unknown/debug/{package_name}.wasm",));

    let mut cmd = Command::new("wasm-tools");
    cmd.arg("component");
    cmd.arg("new");
    cmd.arg(&wasm_file);
    cmd.arg("-o");
    cmd.arg(format!("{package_name}-c.wasm"));

    let status = cmd.status().unwrap();

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
