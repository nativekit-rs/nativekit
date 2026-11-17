use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default = "default_tier")]
    pub tier: String,
    #[serde(default = "default_order")]
    pub order: usize,
    #[serde(skip)]
    pub source_dir: Option<PathBuf>,
}

fn default_tier() -> String {
    "free".to_string()
}

fn default_order() -> usize {
    99
}

fn locate_templates_root() -> Option<PathBuf> {
    let manifest_templates = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../templates");
    let candidates = [
        PathBuf::from("templates"),
        PathBuf::from("../templates"),
        manifest_templates,
    ];
    for c in candidates {
        if c.exists() && c.is_dir() {
            return Some(c);
        }
    }
    None
}

fn locate_reference_demo_app() -> Option<PathBuf> {
    let manifest_demo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../demo_app");
    let candidates = [
        PathBuf::from("demo_app"),
        PathBuf::from("../demo_app"),
        manifest_demo,
    ];
    for c in candidates {
        if c.exists() && c.is_dir() {
            return Some(c);
        }
    }
    None
}

pub fn discover_templates() -> Vec<TemplateMeta> {
    let mut list = Vec::new();
    if let Some(root) = locate_templates_root() {
        if let Ok(entries) = fs::read_dir(&root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let meta_file = path.join("template.json");
                    if let Ok(txt) = fs::read_to_string(&meta_file) {
                        if let Ok(mut meta) = serde_json::from_str::<TemplateMeta>(&txt) {
                            meta.source_dir = Some(path);
                            list.push(meta);
                        }
                    }
                }
            }
        }
    }

    if list.is_empty() {
        list = vec![
            TemplateMeta {
                id: "basic".to_string(),
                name: "Basic Starter".to_string(),
                description: "Minimal single-screen app with reactive state and clean SwiftUI-style syntax".to_string(),
                tier: "free".to_string(),
                order: 1,
                source_dir: None,
            },
            TemplateMeta {
                id: "routing".to_string(),
                name: "Tabs & Navigation".to_string(),
                description: "Multi-page architecture with bottom tabs, drill-in routes, and modal sheets".to_string(),
                tier: "free".to_string(),
                order: 2,
                source_dir: None,
            },
            TemplateMeta {
                id: "fullstack".to_string(),
                name: "Fullstack + OTA Server".to_string(),
                description: "Next.js-style #[server] functions for Over-The-Air UI components and backend routes".to_string(),
                tier: "free".to_string(),
                order: 3,
                source_dir: None,
            },
        ];
    }

    list.sort_by_key(|t| (t.order, t.id.clone()));
    list
}

pub fn print_templates_list() {
    let templates = discover_templates();
    println!("\n📦 \x1B[1;36mAvailable NativeKit Starter Templates\x1B[0m");
    println!("   \x1B[2m(Add or customize templates in the `templates/` directory)\x1B[0m\n");
    for (idx, t) in templates.iter().enumerate() {
        let badge = if t.tier.eq_ignore_ascii_case("pro") {
            "\x1B[35;1m[PRO]\x1B[0m"
        } else {
            "\x1B[32m[FREE]\x1B[0m"
        };
        println!(
            "  \x1B[1m{}.\x1B[0m \x1B[1;37m{:<14}\x1B[0m {} — {} \x1B[2m({})\x1B[0m",
            idx + 1,
            t.id,
            badge,
            t.name,
            t.description
        );
    }
    println!("\n💡 Usage: \x1B[1mnk new <app-name> --template <id>\x1B[0m\n");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "build" || name == ".gradle" || name == "target" || name == ".DS_Store" {
            continue;
        }
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn scaffold_new_project(app_arg: Option<String>, template_arg: Option<String>) {
    let raw_name = match app_arg {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => {
            print!("✨ Project name (e.g. my-app): ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            let trimmed = input.trim();
            if trimmed.is_empty() {
                "my-nativekit-app".to_string()
            } else {
                trimmed.to_string()
            }
        }
    };

    let target_dir = PathBuf::from(&raw_name);
    if target_dir.exists() {
        eprintln!("❌ Directory `{}` already exists.", raw_name);
        return;
    }

    let templates = discover_templates();
    let chosen = if let Some(req_id) = template_arg {
        templates
            .iter()
            .find(|t| t.id.eq_ignore_ascii_case(&req_id))
            .cloned()
            .unwrap_or_else(|| {
                eprintln!("⚠️ Template `{}` not found, defaulting to `{}`.", req_id, templates[0].id);
                templates[0].clone()
            })
    } else {
        println!("\n🎨 \x1B[1mSelect a NativeKit Starter Template:\x1B[0m");
        for (i, t) in templates.iter().enumerate() {
            let badge = if t.tier.eq_ignore_ascii_case("pro") {
                "\x1B[35;1m[PRO]\x1B[0m"
            } else {
                "\x1B[32m[FREE]\x1B[0m"
            };
            println!(
                "  \x1B[1;36m[{}]\x1B[0m \x1B[1m{}\x1B[0m {} — {}",
                i + 1,
                t.name,
                badge,
                t.description
            );
        }
        print!("\n› Enter choice [1-{}] (default: 1): ", templates.len());
        let _ = io::stdout().flush();
        let mut choice = String::new();
        let _ = io::stdin().read_line(&mut choice);
        let idx = choice
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .unwrap_or(0);
        templates.get(idx).cloned().unwrap_or_else(|| templates[0].clone())
    };

    let dir_stem = target_dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| raw_name.clone());
    let crate_name = dir_stem
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c.to_ascii_lowercase() } else { '_' })
        .collect::<String>();
    let crate_ident = crate_name.replace('-', "_");

    println!(
        "\n🚀 Scaffolding \x1B[1;36m{}\x1B[0m using template \x1B[1m{}\x1B[0m...",
        raw_name, chosen.name
    );

    let _ = fs::create_dir_all(target_dir.join("src"));
    let _ = fs::create_dir_all(target_dir.join("config/colors"));
    let _ = fs::create_dir_all(target_dir.join("assets/logo"));

    // Copy template `src/`
    if let Some(ref src_dir) = chosen.source_dir {
        let tmpl_src = src_dir.join("src");
        if tmpl_src.exists() {
            let _ = copy_dir_recursive(&tmpl_src, &target_dir.join("src"));
        }
    } else {
        // Fallback embedded basic starter (self-contained for crates.io packaging)
        let basic_app_rs = r#"#![allow(non_snake_case)]
use native_kit::*;

#[derive(Clone, Default)]
pub struct Store {
    pub count: i32,
}

pub type AppState = Store;

pub fn App() -> impl IntoNode {
    Router!({
        Route!("/", HomeScreen()),
    })
}

pub fn HomeScreen() -> impl IntoNode {
    let state = use_state::<Store>();
    let count = state.count;

    Return! {
        Page.title("Welcome").large_title(true).top_style(.soft) {
            Section {
                VStack.spacing(12) {
                    Text("NativeKit Starter").font(.title3).emphasized()
                    Text("Edit src/app.rs and save to hot-reload across all 5 native platforms.")
                        .font(.subhead)
                        .color(.text_secondary)
                }
            }

            Section {
                HStack.align(.space_between) {
                    VStack.spacing(4) {
                        Text("Counter").font(.headline)
                        Text("Current value: {}", count).font(.caption1).color(.text_secondary)
                    }
                    HStack.spacing(8) {
                        Button("-").variant(.secondary).action(|s: &mut Store| s.count -= 1)
                        Button("+").variant(.primary).action(|s: &mut Store| s.count += 1)
                    }
                }
            }
        }
    }
}
"#;
        let basic_lib_rs = r#"pub mod app;

pub struct NativeApp;

impl native_kit::App for NativeApp {
    type State = app::Store;

    fn init() -> Self::State {
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();
        app::Store::default()
    }

    fn view(state: &Self::State) -> impl native_kit::IntoNode {
        let _scope = native_kit::set_scoped_state(state);
        app::App()
    }
}

native_kit::export_app!(NativeApp);
"#;
        let basic_main_rs = "fn main() {\n    native_kit::engine::dev_server::start::<__CRATE_IDENT__::NativeApp>();\n    std::thread::park();\n}\n";
        let _ = fs::write(target_dir.join("src/app.rs"), basic_app_rs);
        let _ = fs::write(target_dir.join("src/lib.rs"), basic_lib_rs);
        let _ = fs::write(target_dir.join("src/main.rs"), basic_main_rs);
    }

    // Replace `__CRATE_IDENT__` in generated `src/main.rs`
    let main_rs_path = target_dir.join("src/main.rs");
    if let Ok(main_content) = fs::read_to_string(&main_rs_path) {
        let _ = fs::write(
            &main_rs_path,
            main_content.replace("__CRATE_IDENT__", &crate_ident),
        );
    }

    // Resolve path to framework crate (or published version)
    let framework_abs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../framework")
        .canonicalize()
        .ok();
    let nk_dep_line = if let Some(abs) = framework_abs {
        format!(
            "native-kit = {{ path = \"{}\" }}",
            abs.to_string_lossy().replace('\\', "/")
        )
    } else {
        "native-kit = \"0.1.0\"".to_string()
    };

    let cargo_toml = format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
base64 = "0.22.1"
{nk_dep_line}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = {{ version = "0.12", default-features = false, features = ["json", "rustls-tls-webpki-roots"] }}
tokio = {{ version = "1", features = ["rt-multi-thread"] }}

[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = {{ version = "0.12", default-features = false, features = ["json"] }}
tokio = {{ version = "1", features = [] }}
wasm-bindgen = "=0.2.93"
console_error_panic_hook = "0.1"

[lib]
crate-type = ["rlib", "staticlib", "cdylib"]
name = "{crate_ident}"

[features]
client = ["native-kit/client"]
"#
    );
    let _ = fs::write(target_dir.join("Cargo.toml"), cargo_toml);

    // Write `config/app.json` with cloud sync fields pre-wired for `your-org.nativekit.rs/projectid/config`
    let app_json = serde_json::json!({
        "name": raw_name,
        "cloud": {
            "org": "",
            "project_id": "",
            "endpoint": ""
        },
        "server": {
            "prod": "https://api.nativekit.rs"
        },
        "app_icon": {
            "ios": "./assets/logo/icon.png",
            "android": "./assets/logo/icon.png",
            "linux": "./assets/logo/icon.png",
            "windows": "./assets/logo/icon.png"
        },
        "favicon": "./assets/logo/logo.png",
        "platforms": ["ios", "android", "linux", "windows", "web"]
    });
    let _ = fs::write(
        target_dir.join("config/app.json"),
        serde_json::to_string_pretty(&app_json).unwrap(),
    );

    // Copy default config & platform shells from reference `demo_app` if present
    if let Some(ref_demo) = locate_reference_demo_app() {
        for cfg_file in ["icons.json", "typography.json"] {
            let src_cfg = ref_demo.join("config").join(cfg_file);
            if src_cfg.exists() {
                let _ = fs::copy(&src_cfg, target_dir.join("config").join(cfg_file));
            }
        }
        if ref_demo.join("config/colors").exists() {
            let _ = copy_dir_recursive(
                &ref_demo.join("config/colors"),
                &target_dir.join("config/colors"),
            );
        }
        if ref_demo.join("assets/logo").exists() {
            let _ = copy_dir_recursive(
                &ref_demo.join("assets/logo"),
                &target_dir.join("assets/logo"),
            );
        }
        if ref_demo.join("platforms").exists() {
            let _ = copy_dir_recursive(
                &ref_demo.join("platforms"),
                &target_dir.join("platforms"),
            );
        }
    }

    println!("\n✅ \x1B[1;32mCreated `{}`!\x1B[0m", raw_name);
    println!("\nNext steps:");
    println!("  \x1B[1;36mcd {}\x1B[0m", raw_name);
    println!("  \x1B[1;36mnk dev\x1B[0m        \x1B[2m# Start live hot-reloading server\x1B[0m");
    println!("  \x1B[1;36mnk link <org> <project-id>\x1B[0m \x1B[2m# Connect to NativeKit Cloud Theme Studio\x1B[0m\n");
}
