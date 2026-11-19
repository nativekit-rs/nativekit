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

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if fname != "mod.rs" && fname != "lib.rs" && fname != "main.rs" {
                out.push(path);
            }
        }
    }
}

fn extract_pub_symbols(file_path: &Path) -> Vec<(String, &'static str)> {
    let Ok(content) = fs::read_to_string(file_path) else {
        return Vec::new();
    };
    let mut items = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        for (prefix, ns) in [
            ("pub fn ", "value"),
            ("pub struct ", "type"),
            ("pub enum ", "type"),
            ("pub type ", "type"),
            ("pub const ", "value"),
        ] {
            if let Some(rest) = trimmed.strip_prefix(prefix) {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    items.push((name, ns));
                }
            }
        }
    }
    items
}

fn emit_inline_dir_modules(dir: &Path, indent: usize, out: &mut String) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();

    let pad = "    ".repeat(indent);
    for path in paths {
        if path.is_dir() {
            let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !dir_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                continue;
            }
            let mut sub_files = Vec::new();
            collect_rs_files(&path, &mut sub_files);
            if sub_files.is_empty() {
                continue;
            }
            out.push_str(&format!(
                "{}#[allow(non_snake_case, dead_code, unused_imports)]\n{}pub mod {} {{\n",
                pad, pad, dir_name
            ));
            emit_inline_dir_modules(&path, indent + 1, out);
            out.push_str(&format!("{}}}\n{}pub use {}::*;\n", pad, pad, dir_name));
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let fname = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if fname == "mod.rs" || fname == "lib.rs" || fname == "main.rs" {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if !stem.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                continue;
            }
            let rel_path = path
                .strip_prefix(path.ancestors().find(|a| a.ends_with("src")).unwrap_or(dir))
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push_str(&format!(
                "{}#[allow(non_snake_case, dead_code, unused_imports)]\n{}pub mod {} {{\n{}    #[allow(unused_imports)]\n{}    use ::native_kit::*;\n{}    #[allow(unused_imports)]\n{}    use crate::__nk_auto::*;\n{}    include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/{}\"));\n{}}}\n{}pub use {}::*;\n",
                pad, pad, stem, pad, pad, pad, pad, pad, rel_path, pad, pad, stem
            ));
        }
    }
}

/// Automatically discovers all `.rs` files inside `src/{components,pages,views}/**`
/// and writes standard `pub mod` declarations into `src/lib.rs` so `rust-analyzer`
/// indexes every component and page with full IDE autocomplete (zero `mod.rs` files required).
pub fn sync_auto_modules_in_dir(root: &Path) {
    let src_dir = root.join("src");
    let lib_rs = src_dir.join("lib.rs");
    if !lib_rs.exists() {
        return;
    }
    let Ok(lib_content) = fs::read_to_string(&lib_rs) else {
        return;
    };
    if !lib_content.contains("export_app!") {
        return;
    }

    let mut files = Vec::new();
    let mut active_folders = Vec::new();
    for folder in ["components", "pages", "views"] {
        let dir = src_dir.join(folder);
        if dir.exists() && dir.is_dir() {
            if dir.join("mod.rs").exists() && lib_content.contains(&format!("pub mod {};", folder)) {
                continue;
            }
            let mut folder_files = Vec::new();
            collect_rs_files(&dir, &mut folder_files);
            if !folder_files.is_empty() {
                active_folders.push((folder, dir));
                files.extend(folder_files);
            }
        }
    }

    let mut seen: std::collections::HashMap<(String, &'static str), String> =
        std::collections::HashMap::new();
    let mut clash_lines = Vec::new();
    for file in &files {
        let rel_display = file
            .strip_prefix(root)
            .unwrap_or(file)
            .to_string_lossy()
            .replace('\\', "/");
        for (sym_name, ns) in extract_pub_symbols(file) {
            if let Some(prev_file) = seen.insert((sym_name.clone(), ns), rel_display.clone()) {
                if prev_file != rel_display {
                    clash_lines.push(format!(
                        "    compile_error!(\"Duplicate public `{}` found in both `{}` and `{}`. Please give each component/page a unique name.\");",
                        sym_name, prev_file, rel_display
                    ));
                }
            }
        }
    }

    let begin_marker = "// BEGIN_NK_AUTO_MODULES";
    let end_marker = "// END_NK_AUTO_MODULES";

    let mut block = String::new();
    block.push_str(begin_marker);
    block.push('\n');
    for (folder, dir) in &active_folders {
        block.push_str(&format!(
            "#[allow(non_snake_case, dead_code, unused_imports)]\npub mod {} {{\n",
            folder
        ));
        emit_inline_dir_modules(dir, 1, &mut block);
        block.push_str("}\n");
    }
    block.push_str("#[allow(non_snake_case, dead_code, unused_imports)]\npub mod __nk_auto {\n");
    for err in &clash_lines {
        block.push_str(err);
        block.push('\n');
    }
    if clash_lines.is_empty() {
        block.push_str("    pub use super::app::Store;\n");
        for (folder, _) in &active_folders {
            block.push_str(&format!("    pub use super::{}::*;\n", folder));
        }
    }
    block.push_str("}\n#[allow(unused_imports)]\npub use __nk_auto::*;\n");
    block.push_str(end_marker);

    let updated = if let (Some(s), Some(e)) = (
        lib_content.find(begin_marker),
        lib_content.find(end_marker),
    ) {
        if s < e {
            format!(
                "{}{}{}",
                &lib_content[..s],
                block,
                &lib_content[e + end_marker.len()..]
            )
        } else {
            lib_content.clone()
        }
    } else {
        format!("{}\n\n{}\n", block, lib_content.trim_start())
    };

    if updated != lib_content {
        let _ = fs::write(&lib_rs, updated);
    }
}

pub fn sync_auto_modules() {
    sync_auto_modules_in_dir(&project_root());
}
