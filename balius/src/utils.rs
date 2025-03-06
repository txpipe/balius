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