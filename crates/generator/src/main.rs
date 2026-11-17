mod build_types;
mod build_rust_api;

use build_types::PrimitiveDef;
use build_rust_api::generate_rust_struct;
use std::fs;
use std::path::Path;

fn extract_fn_body(code: &str) -> String {
    if let (Some(s), Some(e)) = (code.find('{'), code.rfind('}')) {
        if s < e {
            return code[s + 1..e].trim().to_string();
        }
    }
    code.trim().to_string()
}

fn wrap_case(tag: &str, platform: &str, file_content: &str) -> String {
    let body = extract_fn_body(file_content);
    let indented = body.replace("\n", "\n    ");
    match platform {
        "ios" => format!("case \"{}\":\n    {}\n", tag, indented),
        "android" => format!("\"{}\" -> {{\n    {}\n}}\n", tag, indented),
        "linux" => format!("\"{}\" => {{\n    {}\n}}\n", tag, indented),
        "web" => format!("case \"{}\": {{\n    {}\n}}\n", tag, indented),
        "windows" => format!("case \"{}\": {{\n    {}\n}}\n", tag, indented),
        _ => body,
    }
}

fn main() {
    let blueprints_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("blueprints");
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/src/engine/primitives");
    let _ = fs::create_dir_all(&output_dir);

    let mut mod_rs_content = String::from(
        "pub mod color;\npub use color::*;\n\npub mod icon;\npub use icon::*;\n\npub mod text;\npub use text::*;\n\n"
    );

    if let Ok(entries) = fs::read_dir(&blueprints_dir) {
        let mut dirs: Vec<_> = entries.flatten().collect();
        dirs.sort_by_key(|e| e.file_name());
        for entry in dirs {
            let dir_path = entry.path();
            if !dir_path.is_dir() { continue; }
            let yml_path = dir_path.join("component.yml");
            if !yml_path.exists() { continue; }

            let name = dir_path.file_name().unwrap().to_string_lossy().to_string();
            let yml_content = fs::read_to_string(&yml_path).unwrap();
            let def: PrimitiveDef = serde_yaml::from_str(&yml_content)
                .unwrap_or_else(|e| panic!("Failed to parse {}: {}", yml_path.display(), e));

            if !matches!(def.name.as_str(), "Text" | "Icon" | "Color" | "VStack" | "HStack" | "Button") {
                let mut rust_code = generate_rust_struct(&def.name, &def.rust);
                let tag = def.name.to_lowercase();
                let upper = def.name.to_uppercase();
                

                if let Ok(kt) = fs::read_to_string(dir_path.join("android.kt")) {
                    rust_code.push_str(&format!("\npub const {}_ANDROID_SNIPPET: &str = r{}\n{}{};\n", upper, "#\"", wrap_case(&tag, "android", &kt), "\"#"));
                }
                if let Ok(swift) = fs::read_to_string(dir_path.join("ios.swift")) {
                    rust_code.push_str(&format!("\npub const {}_IOS_SNIPPET: &str = r{}\n{}{};\n", upper, "#\"", wrap_case(&tag, "ios", &swift), "\"#"));
                }
                if let Ok(lin) = fs::read_to_string(dir_path.join("linux.rs")) {
                    rust_code.push_str(&format!("\npub const {}_GTK_SNIPPET: &str = r{}\n{}{};\n", upper, "#\"", wrap_case(&tag, "linux", &lin), "\"#"));
                }
                if let Ok(web) = fs::read_to_string(dir_path.join("web.js")) {
                    rust_code.push_str(&format!("\npub const {}_WEB_SNIPPET: &str = r{}\n{}{};\n", upper, "#\"", wrap_case(&tag, "web", &web), "\"#"));
                }
                if let Ok(win) = fs::read_to_string(dir_path.join("windows.cs")) {
                    rust_code.push_str(&format!("\npub const {}_WINDOWS_SNIPPET: &str = r{}\n{}{};\n", upper, "#\"", wrap_case(&tag, "windows", &win), "\"#"));
                }

                fs::write(output_dir.join(format!("{}.rs", name)), rust_code).unwrap();
            }

            mod_rs_content.push_str(&format!("pub mod {};\npub use {}::*;\n\n", name, name));
        }
    }

    fs::write(output_dir.join("mod.rs"), mod_rs_content).unwrap();
    println!("Successfully generated all primitives from Storybook blueprint folders!");
}
