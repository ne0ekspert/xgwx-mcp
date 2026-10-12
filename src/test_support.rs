use std::{path::PathBuf, process::Command, sync::OnceLock};

// Resolve the compiled dependency for source audits, including local overrides.
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
        let root = PathBuf::from(package["manifest_path"].as_str().unwrap())
            .parent()
            .unwrap()
            .to_owned();
        root
    })
}

// Published crates omit fixtures; select test data independently of the dependency.
pub fn fixture_root() -> PathBuf {
    let root = std::env::var_os("LIBXGWX_TEST_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| library_root().clone());
    assert!(
        root.join("fixtures").is_dir(),
        "libxgwx fixtures unavailable; set LIBXGWX_TEST_ROOT to a libxgwx repository checkout"
    );
    root
}
