use crate::project::project_path;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Resolves the cloud config URL from `config/app.json`:
/// Supports either:
/// - `"cloud": { "org": "acme", "project_id": "my-app" }` -> `https://acme.nativekit.rs/my-app/config`
/// - `"cloud": { "endpoint": "https://acme.nativekit.rs/my-app/config" }`
pub fn resolve_cloud_endpoint() -> Option<String> {
    let app_json_path = project_path("config/app.json");
    let content = fs::read_to_string(app_json_path).ok()?;
    let val: Value = serde_json::from_str(&content).ok()?;
    let cloud = val.get("cloud")?;

    if let Some(ep) = cloud.get("endpoint").and_then(|v| v.as_str()) {
        if !ep.trim().is_empty() {
            return Some(ep.trim().to_string());
        }
    }

    let org = cloud.get("org").and_then(|v| v.as_str()).unwrap_or("").trim();
    let project_id = cloud
        .get("project_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();

    if !org.is_empty() && !project_id.is_empty() {
        return Some(format!("https://{}.nativekit.rs/{}/config", org, project_id));
    }

    None
}

fn credentials_file_path() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join(".nativekit/credentials.json"))
}

pub fn save_cloud_token(token: &str) {
    if let Some(path) = credentials_file_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let body = serde_json::json!({ "token": token.trim() });
        if fs::write(&path, serde_json::to_string_pretty(&body).unwrap()).is_ok() {
            println!("🔐 Saved NativeKit Cloud token to {}", path.display());
        }
    }
}

pub fn load_cloud_token() -> Option<String> {
    if let Ok(t) = std::env::var("NK_TOKEN") {
        if !t.trim().is_empty() {
            return Some(t.trim().to_string());
        }
    }
    let path = credentials_file_path()?;
    let txt = fs::read_to_string(path).ok()?;
    let val: Value = serde_json::from_str(&txt).ok()?;
    val.get("token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Links the active project's `config/app.json` to `<org>.nativekit.rs/<project_id>/config`.
pub fn link_cloud_project(org: &str, project_id: &str) {
    let app_json_path = project_path("config/app.json");
    let mut root: Value = if let Ok(txt) = fs::read_to_string(&app_json_path) {
        serde_json::from_str(&txt).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let endpoint = format!("https://{}.nativekit.rs/{}/config", org.trim(), project_id.trim());
    if let Some(obj) = root.as_object_mut() {
        obj.insert(
            "cloud".to_string(),
            serde_json::json!({
                "org": org.trim(),
                "project_id": project_id.trim(),
                "endpoint": endpoint,
            }),
        );
    }

    if let Some(parent) = app_json_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&app_json_path, serde_json::to_string_pretty(&root).unwrap());
    println!(
        "☁️  Linked project to \x1B[1;36m{}\x1B[0m in `{}`",
        endpoint,
        app_json_path.display()
    );
}

/// Pulls the latest colors, icons, and typography mappings from `<org>.nativekit.rs/<project_id>/config`.
/// Returns `true` if remote styles were fetched and applied.
pub fn sync_cloud_config(verbose: bool) -> bool {
    let Some(endpoint) = resolve_cloud_endpoint() else {
        if verbose {
            println!("ℹ️  No cloud project linked in `config/app.json`.");
            println!("   Run \x1B[1mnk link <your-org> <project-id>\x1B[0m to connect to nativekit.rs.");
        }
        return false;
    };

    if verbose {
        println!("☁️  Fetching latest theme & icon mappings from \x1B[36m{}\x1B[0m...", endpoint);
    }

    let mut cmd = Command::new("curl");
    cmd.args(["-fsSL", "--max-time", "3"]);
    if let Some(token) = load_cloud_token() {
        cmd.args(["-H", &format!("Authorization: Bearer {}", token)]);
    }
    cmd.arg(&endpoint);

    let Ok(out) = cmd.output() else {
        return false;
    };
    if !out.status.success() {
        if verbose {
            eprintln!("⚠️  Could not reach `{}` (using local `config/` files).", endpoint);
        }
        return false;
    }

    let Ok(payload) = serde_json::from_slice::<Value>(&out.stdout) else {
        if verbose {
            eprintln!("⚠️  Invalid JSON returned from `{}`.", endpoint);
        }
        return false;
    };

    let mut updated = false;
    let config_dir = project_path("config");
    let _ = fs::create_dir_all(config_dir.join("colors"));

    if let Some(colors) = payload.get("colors") {
        let dest = if config_dir.join("colors/main.json").exists() {
            config_dir.join("colors/main.json")
        } else {
            config_dir.join("colors.json")
        };
        let _ = fs::write(&dest, serde_json::to_string_pretty(colors).unwrap());
        updated = true;
    }

    if let Some(platform_colors) = payload.get("platform_colors").and_then(|v| v.as_object()) {
        for (plat, val) in platform_colors {
            let dest = config_dir.join("colors").join(format!("{}.json", plat));
            let _ = fs::write(&dest, serde_json::to_string_pretty(val).unwrap());
            updated = true;
        }
    }

    if let Some(icons) = payload.get("icons") {
        let _ = fs::write(
            config_dir.join("icons.json"),
            serde_json::to_string_pretty(icons).unwrap(),
        );
        updated = true;
    }

    if let Some(typography) = payload.get("typography") {
        let _ = fs::write(
            config_dir.join("typography.json"),
            serde_json::to_string_pretty(typography).unwrap(),
        );
        updated = true;
    }

    if updated && verbose {
        println!("✅ Synced latest theme & icon config from NativeKit Cloud!");
    }
    updated
}
