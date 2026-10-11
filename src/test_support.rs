use std::{path::PathBuf, process::Command, sync::OnceLock};

// Resolve the actual Cargo dependency, including a developer's local patch override.
pub fn library_root() -> &'static PathBuf {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        let output = Command::new("cargo")
            .args(["metadata", "--locked", "--offline", "--format-version", "1"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("run cargo metadata for library fixtures");
        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let package = metadata["packages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|package| package["name"] == "libxgwx")
            .expect("libxgwx dependency in Cargo metadata");
        PathBuf::from(package["manifest_path"].as_str().unwrap())
            .parent()
            .unwrap()
            .to_owned()
    })
}
