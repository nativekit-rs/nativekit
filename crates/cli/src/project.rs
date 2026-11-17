use std::fs;
use std::path::{Path, PathBuf};

/// Returns the root directory of the active NativeKit application.
/// - In a standalone project (`./config/app.json` exists), returns `"."`.
/// - In the NativeKit workspace (`demo_app/config/app.json` exists), returns `"demo_app"`.
pub fn project_root() -> PathBuf {
    if let Ok(env_root) = std::env::var("NK_PROJECT_ROOT") {
        let p = PathBuf::from(env_root);
        if p.exists() {
            return p;
        }
    }
    if Path::new("config/app.json").exists() {
        PathBuf::from(".")
    } else if Path::new("demo_app/config/app.json").exists() {
        PathBuf::from("demo_app")
    } else {
        PathBuf::from(".")
    }
}

/// Joins a relative path onto the detected project root.
pub fn project_path(rel: &str) -> PathBuf {
    let root = project_root();
    if root == Path::new(".") {
        PathBuf::from(rel)
    } else {
        root.join(rel)
    }
}

/// Returns the string representation of a path inside the active project.
pub fn project_path_str(rel: &str) -> String {
    project_path(rel).to_string_lossy().to_string()
}

/// Reads the Cargo package/library name from the active project's `Cargo.toml`.
/// Defaults to `"demo_app"` when in the workspace or `"app"` as a fallback.
pub fn project_crate_name() -> String {
    let cargo_toml = project_path("Cargo.toml");
    if let Ok(content) = fs::read_to_string(&cargo_toml) {
        let mut in_package = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_package = trimmed == "[package]";
                continue;
            }
            if in_package && trimmed.starts_with("name") {
                if let Some((_, rhs)) = trimmed.split_once('=') {
                    let name = rhs.trim().trim_matches('"').trim_matches('\'');
                    if !name.is_empty() {
                        return name.to_string();
                    }
                }
            }
        }
    }
    if project_root() == Path::new("demo_app") {
        "demo_app".to_string()
    } else {
        "app".to_string()
    }
}

/// Returns the normalized Rust library symbol name (`my-app` -> `my_app`).
pub fn project_lib_name() -> String {
    project_crate_name().replace('-', "_")
}
