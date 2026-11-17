use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde_json::Value;

#[allow(dead_code)]
pub fn get_component_override(platform: &str, component: &str, prop: &str) -> Option<String> {
    let path = if Path::new(&format!("demo_app/config/{}/component.json", platform)).exists() {
        format!("demo_app/config/{}/component.json", platform)
    } else {
        format!("config/{}/component.json", platform)
    };
    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(val) = serde_json::from_str::<Value>(&text) {
            if let Some(comp_obj) = val.get(component) {
                if let Some(prop_val) = comp_obj.get(prop) {
                    if let Some(s) = prop_val.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }
    }
    None
}

fn parse_color_hex(hex: &str) -> (f32, f32, f32, String) {
    let clean = hex.trim_start_matches('#');
    let full_hex = if clean.len() == 6 {
        format!("FF{}", clean)
    } else {
        clean.to_string()
    };

    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0) as f32 / 255.0;
        (r, g, b, full_hex)
    } else {
        (0.0, 0.0, 0.0, "FF000000".to_string())
    }
}

pub fn export_theme(ios_dir: &Path, android_dir: &Path) {
    export_ios_theme(ios_dir);
    export_android_theme(android_dir);
    export_linux_theme(&crate::project::project_path("platforms/Linux"));
    export_web_theme(&crate::project::project_path("platforms/Web"));
    process_app_icons_and_logo(ios_dir, android_dir);
    export_custom_icons(ios_dir, android_dir);
}

fn generate_image_asset(src_path: &Path, dst_path: &Path, width: u32, height: u32) {
    if dst_path.exists() {
        if let (Ok(src_meta), Ok(dst_meta)) = (src_path.metadata(), dst_path.metadata()) {
            if let (Ok(src_t), Ok(dst_t)) = (src_meta.modified(), dst_meta.modified()) {
                if dst_t >= src_t {
                    return;
                }
            }
        }
    }
    if let Some(parent) = dst_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let status = Command::new("sips")
        .arg("-z")
        .arg(height.to_string())
        .arg(width.to_string())
        .arg(src_path)
        .arg("--out")
        .arg(dst_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if status.is_err() || !status.unwrap().success() {
        let _ = fs::copy(src_path, dst_path);
    }
}

fn generate_ico_asset(png_src: &Path, ico_dst: &Path) {
    if let Ok(png_bytes) = fs::read(png_src) {
        if let Some(parent) = ico_dst.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let mut ico = Vec::with_capacity(6 + 16 + png_bytes.len());
        ico.extend_from_slice(&[0, 0, 1, 0, 1, 0]);
        let width = 0u8;
        let height = 0u8;
        let color_count = 0u8;
        let reserved = 0u8;
        let planes = 1u16;
        let bpp = 32u16;
        let size = png_bytes.len() as u32;
        let offset = 22u32;

        ico.push(width);
        ico.push(height);
        ico.push(color_count);
        ico.push(reserved);
        ico.extend_from_slice(&planes.to_le_bytes());
        ico.extend_from_slice(&bpp.to_le_bytes());
        ico.extend_from_slice(&size.to_le_bytes());
        ico.extend_from_slice(&offset.to_le_bytes());
        ico.extend_from_slice(&png_bytes);

        let _ = fs::write(ico_dst, ico);
    }
}

fn resolve_asset_file(raw_path: &str) -> Option<PathBuf> {
    let clean = raw_path.trim_start_matches("./").trim_start_matches("demo_app/");
    let candidates = [
        PathBuf::from(clean),
        Path::new("demo_app").join(clean),
        Path::new("demo_app/assets/logo").join(clean.trim_start_matches("assets/").trim_start_matches("logo/").trim_start_matches("logos/")),
        Path::new("assets/logo").join(clean.trim_start_matches("assets/").trim_start_matches("logo/").trim_start_matches("logos/")),
        Path::new("demo_app/assets").join(clean.trim_start_matches("assets/")),
        Path::new("assets").join(clean.trim_start_matches("assets/")),
        Path::new("demo_app/config").join(clean),
        Path::new("config").join(clean),
    ];
    for c in &candidates {
        if c.exists() {
            return Some(c.clone());
        }
    }
    None
}

fn get_platform_color_json_path(platform: &str) -> String {
    let candidates = [
        format!("demo_app/config/colors/{}.json", platform),
        format!("config/colors/{}.json", platform),
        format!("demo_app/config/{}/color.json", platform),
        format!("config/{}/color.json", platform),
        format!("demo_app/config/{}/material-theme.json", platform),
        format!("config/{}/material-theme.json", platform),
    ];
    for p in &candidates {
        if Path::new(p).exists() {
            return p.clone();
        }
    }
    format!("demo_app/config/colors/{}.json", platform)
}

fn get_main_colors_json_path() -> String {
    let candidates = [
        "demo_app/config/colors/main.json",
        "config/colors/main.json",
        "demo_app/config/colors.json",
        "config/colors.json",
    ];
    for p in &candidates {
        if Path::new(p).exists() {
            return p.to_string();
        }
    }
    "demo_app/config/colors/main.json".to_string()
}

fn process_app_icons_and_logo(ios_dir: &Path, android_dir: &Path) {
    let app_json_path = if Path::new("demo_app/config/app.json").exists() {
        "demo_app/config/app.json"
    } else if Path::new("app.json").exists() {
        "app.json"
    } else {
        ""
    };

    let app_name = if !app_json_path.is_empty() {
        let val = load_json(app_json_path);
        val.get("name").and_then(|v| v.as_str()).unwrap_or("NativeKit").to_string()
    } else {
        "NativeKit".to_string()
    };

    // --- Update Android Manifest Icon, App Name Label, & AppTheme ---
    let mut cur = android_dir;
    while cur.file_name().is_some_and(|name| name != "main") {
        if let Some(parent) = cur.parent() {
            cur = parent;
        } else {
            break;
        }
    }
    let manifest_path = cur.join("AndroidManifest.xml");
    let manifest_xml = format!(
r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

    <application
        android:allowBackup="true"
        android:usesCleartextTraffic="true"
        android:icon="@mipmap/ic_launcher"
        android:roundIcon="@mipmap/ic_launcher_round"
        android:label="{}"
        android:supportsRtl="true"
        android:theme="@style/AppTheme">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:label="{}"
            android:theme="@style/AppTheme">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>

</manifest>
"#, app_name, app_name);
    let _ = fs::write(&manifest_path, manifest_xml);

    let app_val = if !app_json_path.is_empty() {
        load_json(app_json_path)
    } else {
        Value::Null
    };

    let app_icon_val = app_val.get("app_icon").or_else(|| app_val.get("app-icon"));
    let favicon_val = app_val.get("favicon").and_then(|v| v.as_str());

    let resolve_platform_icon = |platform: &str| -> Option<PathBuf> {
        if let Some(val) = app_icon_val {
            if let Some(s) = val.as_str() {
                if let Some(p) = resolve_asset_file(s) {
                    return Some(p);
                }
            } else if let Some(obj) = val.as_object() {
                if let Some(s) = obj.get(platform).and_then(|v| v.as_str()) {
                    if let Some(p) = resolve_asset_file(s) {
                        return Some(p);
                    }
                }
            }
        }
        resolve_asset_file("assets/logo/icon.png")
            .or_else(|| resolve_asset_file("assets/logo/logo.png"))
            .or_else(|| resolve_asset_file("config/icon.png"))
            .or_else(|| resolve_asset_file("config/logo.png"))
    };

    let logo_src = favicon_val
        .and_then(resolve_asset_file)
        .or_else(|| resolve_asset_file("assets/logo/logo.png"))
        .or_else(|| resolve_asset_file("assets/logo/icon.png"))
        .or_else(|| resolve_asset_file("config/logo.png"))
        .or_else(|| resolve_asset_file("config/icon.png"));

    // --- iOS Assets ---
    let ios_icon_src = resolve_platform_icon("ios");
    let ios_logo_src = logo_src.clone().or_else(|| ios_icon_src.clone());
    if let Some(src) = ios_icon_src.as_ref() {
        let ios_app_icon_dir = ios_dir.join("Assets.xcassets/AppIcon.appiconset");
        let _ = fs::create_dir_all(&ios_app_icon_dir);
        let ios_icon_path = ios_app_icon_dir.join("icon_1024.png");
        generate_image_asset(src, &ios_icon_path, 1024, 1024);

        let ios_app_icon_json = r#"{
  "images" : [
    {
      "filename" : "icon_1024.png",
      "idiom" : "universal",
      "platform" : "ios",
      "size" : "1024x1024"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}"#;
        let _ = fs::write(ios_app_icon_dir.join("Contents.json"), ios_app_icon_json);


    } else {
        let ios_app_icon_dir = ios_dir.join("Assets.xcassets/AppIcon.appiconset");
        let _ = fs::create_dir_all(&ios_app_icon_dir);
        let ios_app_icon_json = r#"{
  "images" : [
    {
      "idiom" : "universal",
      "platform" : "ios",
      "size" : "1024x1024"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}"#;
        let _ = fs::write(ios_app_icon_dir.join("Contents.json"), ios_app_icon_json);
    }

    if let Some(src) = ios_logo_src.as_ref() {
        let ios_logo_dir = ios_dir.join("Assets.xcassets/Logo.imageset");
        let _ = fs::create_dir_all(&ios_logo_dir);
        let ios_logo_path = ios_logo_dir.join("logo.png");
        let _ = fs::copy(src, &ios_logo_path);

        let ios_logo_json = r#"{
  "images" : [
    {
      "filename" : "logo.png",
      "idiom" : "universal",
      "scale" : "1x"
    },
    {
      "filename" : "logo.png",
      "idiom" : "universal",
      "scale" : "2x"
    },
    {
      "filename" : "logo.png",
      "idiom" : "universal",
      "scale" : "3x"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}"#;
        let _ = fs::write(ios_logo_dir.join("Contents.json"), ios_logo_json);
    }

    // --- Android Assets ---
    let android_icon_src = resolve_platform_icon("android");
    let android_fg_src = logo_src.clone().or_else(|| android_icon_src.clone());
    let android_res = cur.join("res");

    if let Some(src) = android_icon_src.as_ref() {
        let densities = [
            ("mipmap-mdpi", 48),
            ("mipmap-hdpi", 72),
            ("mipmap-xhdpi", 96),
            ("mipmap-xxhdpi", 144),
            ("mipmap-xxxhdpi", 192),
        ];

        for (folder, size) in densities.iter() {
            let mipmap_dir = android_res.join(folder);
            let icon_file = mipmap_dir.join("ic_launcher.png");
            let round_icon_file = mipmap_dir.join("ic_launcher_round.png");

            generate_image_asset(src, &icon_file, *size, *size);
            generate_image_asset(src, &round_icon_file, *size, *size);
        }
    }

    if let Some(src) = android_fg_src.as_ref() {
        let drawable_dir = android_res.join("drawable");
        let _ = fs::create_dir_all(&drawable_dir);
        let _ = fs::copy(src, drawable_dir.join("logo.png"));
        let _ = fs::copy(src, drawable_dir.join("ic_launcher_foreground_img.png"));

        let inset_fg_xml = r#"<?xml version="1.0" encoding="utf-8"?>
<inset xmlns:android="http://schemas.android.com/apk/res/android"
    android:drawable="@drawable/ic_launcher_foreground_img"
    android:insetLeft="18%"
    android:insetRight="18%"
    android:insetTop="18%"
    android:insetBottom="18%" />
"#;
        let _ = fs::write(drawable_dir.join("ic_launcher_foreground.xml"), inset_fg_xml);

        let anydpi_dir = android_res.join("mipmap-anydpi-v26");
        let _ = fs::create_dir_all(&anydpi_dir);

        let adaptive_xml = r#"<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background"/>
    <foreground android:drawable="@drawable/ic_launcher_foreground"/>
</adaptive-icon>
"#;
        let _ = fs::write(anydpi_dir.join("ic_launcher.xml"), adaptive_xml);
        let _ = fs::write(anydpi_dir.join("ic_launcher_round.xml"), adaptive_xml);

        let values_dir = android_res.join("values");
        let _ = fs::create_dir_all(&values_dir);
        let bg_color_xml = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">#FFFFFF</color>
</resources>
"#;
        let _ = fs::write(values_dir.join("ic_launcher_background.xml"), bg_color_xml);
    } else {
        let anydpi_dir = android_res.join("mipmap-anydpi-v26");
        let _ = fs::create_dir_all(&anydpi_dir);
        let adaptive_xml = r#"<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background"/>
    <foreground android:drawable="@drawable/ic_launcher_foreground"/>
</adaptive-icon>
"#;
        let _ = fs::write(anydpi_dir.join("ic_launcher.xml"), adaptive_xml);
        let _ = fs::write(anydpi_dir.join("ic_launcher_round.xml"), adaptive_xml);

        let values_dir = android_res.join("values");
        let _ = fs::create_dir_all(&values_dir);
        let bg_color_xml = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">#FFFFFF</color>
</resources>
"#;
        let _ = fs::write(values_dir.join("ic_launcher_background.xml"), bg_color_xml);

        let drawable_dir = android_res.join("drawable");
        let _ = fs::create_dir_all(&drawable_dir);
        let fg_xml = r##"<?xml version="1.0" encoding="utf-8"?>
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp"
    android:height="108dp"
    android:viewportWidth="108"
    android:viewportHeight="108">
    <path
        android:fillColor="#FF5500"
        android:pathData="M30,30h48v48h-48z" />
</vector>
"##;
        let _ = fs::write(drawable_dir.join("ic_launcher_foreground.xml"), fg_xml);
    }

    // --- Windows Assets ---
    let win_icon_src = resolve_platform_icon("windows");
    let win_logo_src = logo_src.clone().or_else(|| win_icon_src.clone());
    let win_dist = crate::project::project_path("platforms/Windows");
    let _ = fs::create_dir_all(&win_dist);
    if let Some(src) = win_logo_src.as_ref() {
        let _ = fs::copy(src, win_dist.join("logo.png"));
    }
    if let Some(src) = win_icon_src.as_ref() {
        generate_ico_asset(src, &win_dist.join("app.ico"));
    }

    // --- Linux Assets ---
    let linux_icon_src = resolve_platform_icon("linux");
    let linux_dist = crate::project::project_path("platforms/Linux");
    let _ = fs::create_dir_all(&linux_dist);
    if let Some(src) = linux_icon_src.as_ref() {
        let _ = fs::copy(src, linux_dist.join("icon.png"));

        for size in [16, 32, 48, 64, 128, 256] {
            let hicolor_dir = linux_dist.join(format!("hicolor/{}x{}/apps", size, size));
            let _ = fs::create_dir_all(&hicolor_dir);
            generate_image_asset(src, &hicolor_dir.join("icon.png"), size, size);

            let icons_dir = linux_dist.join(format!("icons/hicolor/{}x{}/apps", size, size));
            let _ = fs::create_dir_all(&icons_dir);
            generate_image_asset(src, &icons_dir.join("icon.png"), size, size);
        }
    }

    let app_name_safe = app_name.replace(" ", "");
    let desktop_file = format!(r#"[Desktop Entry]
Version=1.0
Type=Application
Name={}
Comment={}
Exec=./{}
Icon=icon
Terminal=false
Categories=Utility;Development;
StartupWMClass=com.nativeui.{}
"#, app_name, app_name, app_name_safe, app_name_safe.to_lowercase());
    let _ = fs::write(linux_dist.join(format!("{}.desktop", app_name_safe)), &desktop_file);

    // --- Web Assets ---
    let web_src = logo_src.clone().or_else(|| resolve_platform_icon("web"));
    let web_dist = crate::project::project_path("platforms/Web");
    let _ = fs::create_dir_all(&web_dist);
    if let Some(src) = web_src.as_ref() {
        let _ = fs::copy(src, web_dist.join("favicon.png"));
        generate_ico_asset(src, &web_dist.join("favicon.ico"));
    }
}


fn svg_to_android_vector_xml(svg_content: &str) -> String {
    let mut paths = Vec::new();
    let mut rest = svg_content;
    while let Some(p_idx) = rest.find("<path") {
        let after_path = &rest[p_idx..];
        if let Some(end_idx) = after_path.find('>') {
            let tag = &after_path[..=end_idx];
            if let Some(d_idx) = tag.find("d=\"") {
                let after_d = &tag[d_idx + 3..];
                if let Some(q_idx) = after_d.find('"') {
                    let d_val = &after_d[..q_idx];
                    if !d_val.trim().is_empty() {
                        paths.push(d_val.trim().to_string());
                    }
                }
            }
            rest = &after_path[end_idx + 1..];
        } else {
            break;
        }
    }
    if paths.is_empty() {
        paths.push("M12,2L14.8,9.2L22,12L14.8,14.8L12,22L9.2,14.8L2,12L9.2,9.2L12,2Z".to_string());
    }
    let mut path_nodes = String::new();
    for p in paths {
        path_nodes.push_str(&format!(
            "    <path\n        android:fillColor=\"#FFFFFFFF\"\n        android:pathData=\"{}\" />\n",
            p
        ));
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<vector xmlns:android=\"http://schemas.android.com/apk/res/android\"\n    android:width=\"24dp\"\n    android:height=\"24dp\"\n    android:viewportWidth=\"24\"\n    android:viewportHeight=\"24\">\n{}</vector>\n",
        path_nodes
    )
}

fn is_svg_path_str(s: &str) -> bool {
    let trimmed = s.trim();
    trimmed.ends_with(".svg")
        || trimmed.starts_with("./")
        || trimmed.starts_with("config/")
        || trimmed.starts_with("icons/")
        || trimmed.contains('/')
}

fn resolve_svg_content(raw_path: &str) -> Option<(String, PathBuf)> {
    let clean = raw_path.trim_start_matches("./").trim_start_matches("demo_app/");
    let candidates = [
        PathBuf::from(clean),
        Path::new("demo_app/config").join(clean),
        Path::new("demo_app/config/icons").join(clean.trim_start_matches("icons/")),
        Path::new("demo_app").join(clean),
        Path::new("config").join(clean),
        Path::new("config/icons").join(clean.trim_start_matches("icons/")),
    ];
    for c in &candidates {
        if c.exists() {
            if let Ok(content) = fs::read_to_string(c) {
                return Some((content, c.clone()));
            }
        }
    }
    None
}

fn export_custom_icons(ios_dir: &Path, android_dir: &Path) {
    let icons_path = if Path::new("demo_app/config/icons.json").exists() {
        "demo_app/config/icons.json"
    } else if Path::new("config/icons.json").exists() {
        "config/icons.json"
    } else {
        return;
    };
    let icons_json = load_json(icons_path);
    let Some(obj) = icons_json.as_object() else { return; };

    let mut cur = android_dir;
    while let Some(parent) = cur.parent() {
        if cur.ends_with("main") {
            break;
        }
        cur = parent;
    }
    let android_drawable_dir = cur.join("res/drawable");
    let _ = fs::create_dir_all(&android_drawable_dir);

    let ios_assets_dir = ios_dir.join("Assets.xcassets");
    let _ = fs::create_dir_all(&ios_assets_dir);

    let linux_icons_dir = crate::project::project_path("platforms/Linux/icons");
    let _ = fs::create_dir_all(&linux_icons_dir);

    for (key, val) in obj {
        if key.starts_with('_') {
            continue;
        }

        // Global shorthand string or object with "svg":
        let global_svg = if let Some(s) = val.as_str() {
            if is_svg_path_str(s) { Some(s) } else { None }
        } else {
            val.get("svg").and_then(|v| v.as_str())
        };

        // Determine if iOS has custom SVG
        let ios_svg = global_svg.or_else(|| {
            val.get("ios").and_then(|v| v.as_str()).filter(|s| is_svg_path_str(s))
        });
        if let Some(path) = ios_svg {
            if let Some((svg_content, _)) = resolve_svg_content(path) {
                let imageset_dir = ios_assets_dir.join(format!("{}.imageset", key));
                let _ = fs::create_dir_all(&imageset_dir);
                let svg_filename = format!("{}.svg", key);
                let _ = fs::write(imageset_dir.join(&svg_filename), &svg_content);
                let contents_json = format!(
                    r#"{{
  "images" : [
    {{
      "filename" : "{}",
      "idiom" : "universal"
    }}
  ],
  "info" : {{
    "author" : "xcode",
    "version" : 1
  }},
  "properties" : {{
    "preserves-vector-representation" : true,
    "template-rendering-intent" : "template"
  }}
}}"#,
                    svg_filename
                );
                let _ = fs::write(imageset_dir.join("Contents.json"), contents_json);
            }
        }

        // Determine if Android has custom SVG
        let android_svg = global_svg.or_else(|| {
            val.get("android").and_then(|v| v.as_str()).filter(|s| is_svg_path_str(s))
        });
        if let Some(path) = android_svg {
            if let Some((svg_content, _)) = resolve_svg_content(path) {
                let vector_xml = svg_to_android_vector_xml(&svg_content);
                let _ = fs::write(android_drawable_dir.join(format!("ic_{}.xml", key)), vector_xml);
            }
        }

        // Determine if Linux has custom SVG
        let linux_svg = global_svg.or_else(|| {
            val.get("linux").and_then(|v| v.as_str()).filter(|s| is_svg_path_str(s))
        });
        if let Some(path) = linux_svg {
            if let Some((svg_content, _)) = resolve_svg_content(path) {
                let symbolic_filename = format!("{}-symbolic.svg", key);
                let regular_filename = format!("{}.svg", key);
                let _ = fs::write(linux_icons_dir.join(&symbolic_filename), &svg_content);
                let _ = fs::write(linux_icons_dir.join(&regular_filename), &svg_content);
            }
        }
    }
}

fn export_linux_theme(linux_dir: &Path) {
    if !linux_dir.exists() { return; }
    let color_json_path = get_platform_color_json_path("linux");
    let root = if Path::new(&color_json_path).exists() { load_json(&color_json_path) } else { Value::Null };

    let light_accent = root.get("light").and_then(|l| l.get("accent_color")).and_then(|v| v.as_str())
        .or_else(|| root.get("accent_color").and_then(|v| v.as_str()));
    let dark_accent = root.get("dark").and_then(|d| d.get("accent_color")).and_then(|v| v.as_str())
        .or(light_accent);

    // Base theme.css: ZERO accent color overrides (pure vanilla Libadwaita!)
    let mut base_css = String::from(
        "headerbar.flat { background-color: transparent; border-color: transparent; box-shadow: none; }\nlabel.title-1 { font-size: 20pt; }\nlabel.title-2 { font-size: 15pt; }\nlabel.title-3 { font-size: 15pt; }\nlabel.title-4 { font-size: 13pt; }\nlabel.heading { font-size: 11pt; }\nlabel.body { font-size: 11pt; }\nlabel.caption-heading { font-size: 9pt; }\nlabel.caption { font-size: 9pt; }\nlabel.weight-thin { font-weight: 100; }\nlabel.weight-ultralight { font-weight: 200; }\nlabel.weight-light { font-weight: 300; }\nlabel.weight-regular { font-weight: 400; }\nlabel.weight-medium { font-weight: 500; }\nlabel.weight-semibold { font-weight: 600; }\nlabel.weight-bold { font-weight: 700; }\nlabel.weight-heavy { font-weight: 800; }\nlabel.weight-black { font-weight: 900; }\n"
    );
    let colors_json = load_json(&get_main_colors_json_path());
    if let Some(obj) = colors_json.as_object() {
        for (key, val) in obj {
            if key.starts_with('_') { continue; }
            if let Some(lin_val) = val.get("linux") {
                let resolved = if let Some(s) = lin_val.as_str() {
                    if s.starts_with('#') || s.starts_with("rgb") || s.starts_with('@') { s.to_string() }
                    else { format!("@{}", s) }
                } else if let Some(m) = lin_val.as_object() {
                    m.get("light").and_then(|v| v.as_str()).unwrap_or("#000000").to_string()
                } else { continue; };
                base_css.push_str(&format!(".nk-fg-{key} {{ color: {resolved}; }}\n.nk-bg-{key} {{ background-color: {resolved}; }}\n"));
            }
        }
    }
    let _ = fs::write(linux_dir.join("theme.css"), &base_css);

    let build_brand = |hex_opt: Option<&str>| -> String {
        if let Some(h) = hex_opt {
            format!(":root {{ --accent-bg-color: {0}; --accent-color: {0}; }}\n@define-color accent_color {0};\n@define-color accent_bg_color {0};\nbutton.suggested-action {{ background-color: {0}; color: #ffffff; }}\n", h)
        } else {
            String::new()
        }
    };
    let _ = fs::write(linux_dir.join("brand-light.css"), build_brand(light_accent));
    let _ = fs::write(linux_dir.join("brand-dark.css"), build_brand(dark_accent));
}

fn export_web_theme(web_dir: &Path) {
    let _ = fs::create_dir_all(web_dir);
    let color_json_path = get_platform_color_json_path("web");
    let color_json = load_json(&color_json_path);
    let colors_palette = load_json(&get_main_colors_json_path());

    let mut light_vars = String::new();
    let mut dark_vars = String::new();
    let mut utility_classes = String::new();

    if let Some(obj) = color_json.as_object() {
        if let Some(light_obj) = obj.get("light").and_then(|v| v.as_object()) {
            for (k, v) in light_obj {
                if let Some(hex_val) = v.as_str() {
                    let kebab = k.replace('_', "-");
                    light_vars.push_str(&format!("  --rn-{}: {};\n", kebab, hex_val));
                }
            }
            if let Some(surf) = light_obj.get("on_surface").and_then(|v| v.as_str()) {
                light_vars.push_str(&format!("  --rn-text: {};\n", surf));
            }
            if let Some(out) = light_obj.get("outline_variant").and_then(|v| v.as_str()) {
                light_vars.push_str(&format!("  --rn-border: {};\n", out));
            }
        } else {
            for (key, val) in obj {
                if let Some(hex_val) = val.as_str() {
                    let var_name = if key.starts_with("--") {
                        key.to_string()
                    } else if key.starts_with("rn-") {
                        format!("--{}", key)
                    } else {
                        format!("--rn-{}", key.replace('_', "-"))
                    };
                    light_vars.push_str(&format!("  {}: {};\n", var_name, hex_val));
                }
            }
        }

        if let Some(dark_obj) = obj.get("dark").and_then(|v| v.as_object()) {
            for (k, v) in dark_obj {
                if let Some(hex_val) = v.as_str() {
                    let kebab = k.replace('_', "-");
                    dark_vars.push_str(&format!("    --rn-{}: {};\n", kebab, hex_val));
                }
            }
            if let Some(surf) = dark_obj.get("on_surface").and_then(|v| v.as_str()) {
                dark_vars.push_str(&format!("    --rn-text: {};\n", surf));
            }
            if let Some(out) = dark_obj.get("outline_variant").and_then(|v| v.as_str()) {
                dark_vars.push_str(&format!("    --rn-border: {};\n", out));
            }
        }
    }

    if let Some(obj) = colors_palette.as_object() {
        for (key, val) in obj {
            if key.starts_with('_') { continue; }
            if let Some(web_val) = val.get("web") {
                if let Some(s) = web_val.as_str() {
                    light_vars.push_str(&format!("  --nk-color-{}: {};\n", key, s));
                } else if let Some(m) = web_val.as_object() {
                    if let Some(l) = m.get("light").and_then(|v| v.as_str()) {
                        light_vars.push_str(&format!("  --nk-color-{}: {};\n", key, l));
                    }
                    if let Some(d) = m.get("dark").and_then(|v| v.as_str()) {
                        dark_vars.push_str(&format!("    --nk-color-{}: {};\n", key, d));
                    }
                }
                utility_classes.push_str(&format!(
                    ".nk-fg-{key} {{ color: var(--nk-color-{key}); }}\n.nk-bg-{key} {{ background-color: var(--nk-color-{key}); }}\n"
                ));
            }
        }
    }

    let mut css = String::new();
    css.push_str(":root {\n");
    css.push_str(&light_vars);
    css.push_str("}\n");
    if !dark_vars.is_empty() {
        css.push_str("@media (prefers-color-scheme: dark) {\n  :root:not([data-theme=\"light\"]) {\n");
        css.push_str(&dark_vars);
        css.push_str("  }\n}\n");
        css.push_str(":root[data-theme=\"dark\"] {\n");
        css.push_str(&dark_vars);
        css.push_str("}\n");
    }
    css.push_str(":root[data-accent=\"system\"] {\n  --rn-primary: AccentColor;\n  --nk-color-primary: AccentColor;\n}\n");
    css.push_str(r#"
.large-title { font-size: 34px; line-height: 41px; }
.title-1 { font-size: 28px; line-height: 34px; }
.title-2 { font-size: 22px; line-height: 28px; }
.title-3 { font-size: 20px; line-height: 25px; }
.headline { font-size: 17px; line-height: 22px; }
.body { font-size: 17px; line-height: 22px; }
.callout { font-size: 16px; line-height: 21px; }
.subhead { font-size: 15px; line-height: 20px; }
.footnote { font-size: 13px; line-height: 18px; }
.caption-1 { font-size: 12px; line-height: 16px; }
.caption-2 { font-size: 11px; line-height: 13px; }

.weight-thin { font-weight: 100; }
.weight-ultralight { font-weight: 200; }
.weight-light { font-weight: 300; }
.weight-regular { font-weight: 400; }
.weight-medium { font-weight: 500; }
.weight-semibold { font-weight: 600; }
.weight-bold { font-weight: 700; }
.weight-heavy { font-weight: 800; }
.weight-black { font-weight: 900; }
"#);
    css.push_str(&utility_classes);
    let _ = fs::write(web_dir.join("theme.css"), css);
}

fn export_ios_theme(ios_dir: &Path) {
    let color_json_path = get_platform_color_json_path("ios");
    let (light_json, dark_json) = if Path::new(&color_json_path).exists() {
        let root = load_json(&color_json_path);
        let light = root.get("light").cloned().unwrap_or(Value::Null);
        let dark = root.get("dark").cloned().unwrap_or(Value::Null);
        (light, dark)
    } else {
        (load_json("demo_app/config/ios/light.json"), load_json("demo_app/config/ios/dark.json"))
    };

    let assets_dir = ios_dir.join("Assets.xcassets");
    let _ = fs::create_dir_all(&assets_dir);

    let mut has_accent = false;
    if let Some(obj) = light_json.as_object() {
        for (key, val) in obj {
            if key == "AccentColor" {
                has_accent = true;
            }
            if let Some(light_hex) = val.as_str() {
                let dark_hex = dark_json.get(key).and_then(|v| v.as_str()).unwrap_or(light_hex);
                create_ios_colorset(&assets_dir, key, light_hex, dark_hex);
            }
        }
    }
    if !has_accent {
        create_ios_colorset(&assets_dir, "AccentColor", "#FF5500", "#FF8833");
    }
    let light_accent = light_json.get("AccentColor").and_then(|v| v.as_str()).unwrap_or("#FF5500");
    let dark_accent = dark_json.get("AccentColor").and_then(|v| v.as_str()).unwrap_or(light_accent);
    let (lr, lg, lb, _) = parse_color_hex(light_accent);
    let (dr, dg, db, _) = parse_color_hex(dark_accent);
    let _theme_swift = format!(
        "import SwiftUI\nimport UIKit\n\npublic struct AppTheme {{\n    public static let primary = Color(uiColor: UIColor {{ traits in\n        traits.userInterfaceStyle == .dark\n            ? UIColor(red: {dr:.3}, green: {dg:.3}, blue: {db:.3}, alpha: 1.0)\n            : UIColor(red: {lr:.3}, green: {lg:.3}, blue: {lb:.3}, alpha: 1.0)\n    }})\n    public static let background = Color(\"background\")\n}}\n"
    );
    

    // Also export Windows brand accent colors (Rest, Hover, Pressed) from config/colors/windows.json!
    let win_json = load_json(&get_platform_color_json_path("windows"));
    let wl = win_json.get("light");
    let wd = win_json.get("dark");
    let l_rest = wl.and_then(|o| o.get("accentColor")).and_then(|v| v.as_str()).unwrap_or("#0078D4");
    let l_hover = wl.and_then(|o| o.get("accentColorHover")).and_then(|v| v.as_str()).unwrap_or(l_rest);
    let l_press = wl.and_then(|o| o.get("accentColorPressed")).and_then(|v| v.as_str()).unwrap_or(l_hover);
    let d_rest = wd.and_then(|o| o.get("accentColor")).and_then(|v| v.as_str()).unwrap_or(l_rest);
    let d_hover = wd.and_then(|o| o.get("accentColorHover")).and_then(|v| v.as_str()).unwrap_or(d_rest);
    let d_press = wd.and_then(|o| o.get("accentColorPressed")).and_then(|v| v.as_str()).unwrap_or(d_hover);
    let win_theme_cs = format!(
        "namespace CargoNativeApp {{\n    public static class AppThemeConfig {{\n        public const string LightAccentHex = \"{0}\";\n        public const string LightAccentHoverHex = \"{1}\";\n        public const string LightAccentPressedHex = \"{2}\";\n        public const string DarkAccentHex = \"{3}\";\n        public const string DarkAccentHoverHex = \"{4}\";\n        public const string DarkAccentPressedHex = \"{5}\";\n    }}\n}}\n",
        l_rest, l_hover, l_press, d_rest, d_hover, d_press
    );
    let _ = fs::write(crate::project::project_path("platforms/Windows/ThemeConfig.cs"), win_theme_cs);
}

fn create_ios_colorset(assets_dir: &Path, key: &str, light_hex: &str, dark_hex: &str) {
    let colorset_dir = assets_dir.join(format!("{}.colorset", key));
    let _ = fs::create_dir_all(&colorset_dir);

    let (lr, lg, lb, _) = parse_color_hex(light_hex);
    let (dr, dg, db, _) = parse_color_hex(dark_hex);

    let contents_json = format!(
r#"{{
  "colors" : [
    {{
      "color" : {{
        "color-space" : "srgb",
        "components" : {{
          "alpha" : "1.000",
          "blue" : "{lb:.3}",
          "green" : "{lg:.3}",
          "red" : "{lr:.3}"
        }}
      }},
      "idiom" : "universal"
    }},
    {{
      "appearances" : [
        {{
          "appearance" : "luminosity",
          "value" : "dark"
        }}
      ],
      "color" : {{
        "color-space" : "srgb",
        "components" : {{
          "alpha" : "1.000",
          "blue" : "{db:.3}",
          "green" : "{dg:.3}",
          "red" : "{dr:.3}"
        }}
      }},
      "idiom" : "universal"
    }}
  ],
  "info" : {{
    "author" : "xcode",
    "version" : 1
  }}
}}"#);

    let _ = fs::write(colorset_dir.join("Contents.json"), contents_json);
}

fn format_color_hex(hex: &str) -> String {
    let clean = hex.trim_start_matches('#');
    let full = if clean.len() == 6 {
        format!("FF{}", clean)
    } else {
        clean.to_string()
    };
    format!("Color(0x{})", full.to_uppercase())
}

fn build_scheme_code(scheme_obj: Option<&serde_json::Map<String, Value>>, is_dark: bool) -> String {
    let valid_keys = [
        "primary", "onPrimary", "primaryContainer", "onPrimaryContainer", "inversePrimary",
        "secondary", "onSecondary", "secondaryContainer", "onSecondaryContainer",
        "tertiary", "onTertiary", "tertiaryContainer", "onTertiaryContainer",
        "background", "onBackground", "surface", "onSurface", "surfaceVariant", "onSurfaceVariant",
        "inverseSurface", "inverseOnSurface", "error", "onError", "errorContainer", "onErrorContainer",
        "outline", "outlineVariant", "scrim"
    ];

    let fn_name = if is_dark { "darkColorScheme" } else { "lightColorScheme" };

    let mut lines = Vec::new();
    lines.push(format!("{} (", fn_name));

    if let Some(obj) = scheme_obj {
        for key in valid_keys.iter() {
            if let Some(hex_val) = obj.get(*key).and_then(|v| v.as_str()) {
                lines.push(format!("    {} = {},", key, format_color_hex(hex_val)));
            }
        }
    }

    lines.push(")".to_string());
    lines.join("\n")
}

fn export_android_theme(android_dir: &Path) {
    let theme_json_path = get_platform_color_json_path("android");
    let theme_json = load_json(&theme_json_path);
    let schemes = theme_json.get("schemes").unwrap_or(&theme_json);

    let light_obj = schemes.get("light").and_then(|v| v.as_object());
    let dark_obj = schemes.get("dark").and_then(|v| v.as_object());
    let light_med_obj = schemes.get("light-medium-contrast").and_then(|v| v.as_object());
    let light_high_obj = schemes.get("light-high-contrast").and_then(|v| v.as_object());
    let dark_med_obj = schemes.get("dark-medium-contrast").and_then(|v| v.as_object());
    let dark_high_obj = schemes.get("dark-high-contrast").and_then(|v| v.as_object());

    let light_code = build_scheme_code(light_obj, false);
    let dark_code = build_scheme_code(dark_obj, true);
    let light_med_code = build_scheme_code(light_med_obj, false);
    let light_high_code = build_scheme_code(light_high_obj, false);
    let dark_med_code = build_scheme_code(dark_med_obj, true);
    let dark_high_code = build_scheme_code(dark_high_obj, true);

    let icons_json = load_json(&crate::project::project_path_str("config/icons.json"));
    let icon_style = icons_json
        .get("_defaults")
        .and_then(|d| d.get("android"))
        .and_then(|a| a.get("style"))
        .and_then(|s| s.as_str())
        .map(|s| s.to_string())
        .or_else(|| get_component_override("android", "icon", "style"))
        .unwrap_or_else(|| "rounded".to_string())
        .to_lowercase();

    let mut tab_icon_cases = String::new();
    if let Some(obj) = icons_json.as_object() {
        for (key, val) in obj {
            if key.starts_with('_') {
                continue;
            }
            let and_val = val.as_str().or_else(|| val.get("android").and_then(|v| v.as_str()));
            if let Some(sym) = and_val {
                if !is_svg_path_str(sym) {
                    let mut pascal = String::new();
                    let mut cap = true;
                    for c in sym.chars() {
                        if c == '_' || c == '-' {
                            cap = true;
                        } else if cap {
                            pascal.extend(c.to_uppercase());
                            cap = false;
                        } else {
                            pascal.push(c);
                        }
                    }
                    let extra_aliases = match key.as_str() {
                        "home" => r#", "house""#,
                        "orders" => r#", "order", "bag", "cart""#,
                        "cart" => r#", "bag""#,
                        "settings" => r#", "gear""#,
                        "user" => r#", "person", "profile""#,
                        "qrcode" => r#", "qr_code", "scan""#,
                        "coffee" => r#", "mug""#,
                        _ => "",
                    };
                    tab_icon_cases.push_str(&format!(
                        "    \"{key}\"{extra_aliases} -> when (AppIconStyle) {{\n        \"rounded\" -> Icons.Rounded.{pascal}\n        \"outlined\" -> Icons.Outlined.{pascal}\n        \"sharp\" -> Icons.Sharp.{pascal}\n        \"twotone\", \"two-tone\" -> Icons.TwoTone.{pascal}\n        else -> Icons.Filled.{pascal}\n    }}\n"
                    ));
                }
            }
        }
    }

    let theme_kt = format!(
r#"package com.nativeui.cargonativeapp


import android.app.UiModeManager
import android.content.Context
import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import android.app.Activity
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.rounded.*
import androidx.compose.material.icons.outlined.*
import androidx.compose.material.icons.sharp.*
import androidx.compose.material.icons.twotone.*
import androidx.compose.ui.graphics.vector.ImageVector

private val lightScheme = {}

private val darkScheme = {}

private val mediumContrastLightColorScheme = {}

private val highContrastLightColorScheme = {}

private val mediumContrastDarkColorScheme = {}

private val highContrastDarkColorScheme = {}

@Composable
fun isHighContrastEnabled(): Boolean {{
    val context = LocalContext.current
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {{
        val uiModeManager = context.getSystemService(Context.UI_MODE_SERVICE) as? UiModeManager
        (uiModeManager?.contrast ?: 0f) >= 0.66f
    }} else {{
        false
    }}
}}

@Composable
fun isMediumContrastEnabled(): Boolean {{
    val context = LocalContext.current
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {{
        val uiModeManager = context.getSystemService(Context.UI_MODE_SERVICE) as? UiModeManager
        val c = uiModeManager?.contrast ?: 0f
        c >= 0.33f && c < 0.66f
    }} else {{
        false
    }}
}}

@Composable
fun AppTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = false,
    content: @Composable () -> Unit
) {{
    val isHighContrast = isHighContrastEnabled()
    val isMediumContrast = isMediumContrastEnabled()

    val colorScheme = when {{
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {{
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }}
        darkTheme && isHighContrast -> highContrastDarkColorScheme
        darkTheme && isMediumContrast -> mediumContrastDarkColorScheme
        darkTheme -> darkScheme
        isHighContrast -> highContrastLightColorScheme
        isMediumContrast -> mediumContrastLightColorScheme
        else -> lightScheme
    }}

    val view = LocalView.current
    if (!view.isInEditMode) {{
        SideEffect {{
            val window = (view.context as Activity).window
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
            WindowCompat.getInsetsController(window, view).isAppearanceLightNavigationBars = !darkTheme
        }}
    }}

    MaterialTheme(
        colorScheme = colorScheme,
        content = content
    )
}}

const val AppIconStyle = "{}"

val AppBackIcon: ImageVector = when (AppIconStyle) {{
    "rounded" -> Icons.Rounded.ArrowBack
    "outlined" -> Icons.Outlined.ArrowBack
    "sharp" -> Icons.Sharp.ArrowBack
    "twotone", "two-tone" -> Icons.TwoTone.ArrowBack
    else -> Icons.Filled.ArrowBack
}}

val AppCloseIcon: ImageVector = when (AppIconStyle) {{
    "rounded" -> Icons.Rounded.Close
    "outlined" -> Icons.Outlined.Close
    "sharp" -> Icons.Sharp.Close
    "twotone", "two-tone" -> Icons.TwoTone.Close
    else -> Icons.Filled.Close
}}

fun appTabIcon(iconName: String): ImageVector = when (iconName.lowercase()) {{
{tab_icon_cases}    else -> when (AppIconStyle) {{
        "rounded" -> Icons.Rounded.Star
        "outlined" -> Icons.Outlined.Star
        "sharp" -> Icons.Sharp.Star
        "twotone", "two-tone" -> Icons.TwoTone.Star
        else -> Icons.Filled.Star
    }}
}}
"#,
        light_code,
        dark_code,
        light_med_code,
        light_high_code,
        dark_med_code,
        dark_high_code,
        icon_style
    );

    let _ = fs::create_dir_all(android_dir);
    let _ = fs::write(android_dir.join("Theme.kt"), theme_kt);
}

fn load_json(path: &str) -> Value {
    if let Ok(text) = fs::read_to_string(path) {
        serde_json::from_str(&text).unwrap_or(Value::Null)
    } else {
        Value::Null
    }
}

fn hsl_hex(h: f32, s: f32, l: f32) -> String {
    let (h, s, l) = (((h % 360.0) + 360.0) % 360.0, (s / 100.0).clamp(0.0, 1.0), (l / 100.0).clamp(0.0, 1.0));
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = match h as u32 { 0..=59 => (c, x, 0.0), 60..=119 => (x, c, 0.0), 120..=179 => (0.0, c, x), 180..=239 => (0.0, x, c), 240..=299 => (x, 0.0, c), _ => (c, 0.0, x) };
    format!("#{:02X}{:02X}{:02X}", ((r1 + m) * 255.0).round() as u8, ((g1 + m) * 255.0).round() as u8, ((b1 + m) * 255.0).round() as u8)
}

fn make_scheme(h: f32, s: f32, dark: bool, contrast: f32) -> serde_json::Value {
    let (hs, ht) = ((h + 18.0) % 360.0, (h + 65.0) % 360.0);
    let p_l = if !dark { (38.0 - contrast * 18.0).max(14.0) } else { (76.0 + contrast * 18.0).min(98.0) };
    let on_p = if !dark { "#FFFFFF".to_string() } else { hsl_hex(h, s, (18.0 - contrast * 14.0).max(0.0)) };
    let pc_l = if !dark { 88.0 - contrast * 45.0 } else { 30.0 + contrast * 45.0 };
    let on_pc = if !dark { if contrast > 0.3 { "#FFFFFF".to_string() } else { hsl_hex(h, s, 14.0) } } else { if contrast > 0.3 { "#000000".to_string() } else { hsl_hex(h, s, 90.0) } };
    let bg = if !dark { hsl_hex(h, 25.0, 98.0) } else { hsl_hex(h, 22.0, 8.0) };
    let on_bg = if !dark { hsl_hex(h, 20.0, 10.0) } else { hsl_hex(h, 18.0, 92.0) };
    let surf_v = if !dark { hsl_hex(h, 22.0, 91.0) } else { hsl_hex(h, 20.0, 16.0) };
    let on_sv = if !dark { hsl_hex(h, 15.0, (30.0 - contrast * 20.0).max(0.0)) } else { hsl_hex(h, 15.0, (78.0 + contrast * 20.0).min(100.0)) };
    serde_json::json!({
        "primary": hsl_hex(h, s, p_l), "surfaceTint": hsl_hex(h, s, p_l), "onPrimary": on_p,
        "primaryContainer": hsl_hex(h, s, pc_l), "onPrimaryContainer": on_pc,
        "secondary": hsl_hex(hs, s * 0.65, p_l), "onSecondary": on_p,
        "secondaryContainer": hsl_hex(hs, s * 0.65, pc_l), "onSecondaryContainer": on_pc,
        "tertiary": hsl_hex(ht, s * 0.7, p_l), "onTertiary": on_p,
        "tertiaryContainer": hsl_hex(ht, s * 0.7, pc_l), "onTertiaryContainer": on_pc,
        "error": if !dark { "#BA1A1A" } else { "#FFB4AB" }, "onError": if !dark { "#FFFFFF" } else { "#690005" },
        "errorContainer": if !dark { "#FFDAD6" } else { "#93000A" }, "onErrorContainer": if !dark { "#410002" } else { "#FFDAD6" },
        "background": bg, "onBackground": on_bg, "surface": bg, "onSurface": on_bg,
        "surfaceVariant": surf_v, "onSurfaceVariant": on_sv,
        "outline": hsl_hex(h, 12.0, if !dark { 48.0 } else { 58.0 }),
        "outlineVariant": hsl_hex(h, 14.0, if !dark { 80.0 } else { 28.0 }),
        "shadow": "#000000", "scrim": "#000000",
        "inverseSurface": if !dark { hsl_hex(h, 15.0, 20.0) } else { hsl_hex(h, 15.0, 90.0) },
        "inverseOnSurface": if !dark { hsl_hex(h, 20.0, 95.0) } else { hsl_hex(h, 20.0, 16.0) },
        "inversePrimary": hsl_hex(h, s, if !dark { 76.0 } else { 38.0 })
    })
}

pub fn randomize_theme_palette() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos() as usize;
    static STEP: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let presets: &[(&str, f32, f32)] = &[
        ("Electric Indigo", 248.0, 84.0), ("Emerald Mint", 158.0, 78.0), ("Oceanic Cyan", 196.0, 88.0),
        ("Cyber Magenta", 318.0, 82.0), ("Sunset Coral", 14.0, 88.0), ("Royal Amber", 38.0, 92.0),
        ("Crimson Rose", 346.0, 84.0), ("Amethyst Violet", 276.0, 80.0), ("Cobalt Blue", 216.0, 90.0), ("Forest Jade", 140.0, 74.0),
    ];
    let (name, h, s) = presets[(nanos + STEP.fetch_add(1, std::sync::atomic::Ordering::SeqCst) * 3) % presets.len()];
    let (hs, ht) = ((h + 18.0) % 360.0, (h + 65.0) % 360.0);
    let seed = hsl_hex(h, s, 50.0);
    let android = serde_json::json!({
        "description": format!("TYPE: CUSTOM\nNativeKit Randomizer ({})", name), "seed": seed,
        "coreColors": { "primary": seed, "tertiary": hsl_hex(ht, 70.0, 55.0), "error": "#DF3F2B", "neutral": hsl_hex(h, 10.0, 60.0), "neutralVariant": hsl_hex(h, 16.0, 60.0) },
        "schemes": {
            "light": make_scheme(h, s, false, 0.0), "light-medium-contrast": make_scheme(h, s, false, 0.45), "light-high-contrast": make_scheme(h, s, false, 0.85),
            "dark": make_scheme(h, s, true, 0.0), "dark-medium-contrast": make_scheme(h, s, true, 0.45), "dark-high-contrast": make_scheme(h, s, true, 0.85)
        }
    });
    let ios = serde_json::json!({ "light": { "AccentColor": hsl_hex(h, s, 46.0), "brandColor": hsl_hex(h, 30.0, 12.0) }, "dark": { "AccentColor": hsl_hex(h, s, 64.0), "brandColor": hsl_hex(h, 30.0, 88.0) } });
    let web = serde_json::json!({
        "light": { "primary": hsl_hex(h, s, 46.0), "on_primary": "#FFFFFF", "secondary": hsl_hex(hs, s * 0.7, 46.0), "tertiary": hsl_hex(ht, s * 0.75, 44.0), "background": hsl_hex(h, 24.0, 96.0), "on_background": hsl_hex(h, 20.0, 10.0), "surface": "#FFFFFF", "on_surface": hsl_hex(h, 20.0, 10.0), "surface_container": hsl_hex(h, 22.0, 93.0), "surface_variant": hsl_hex(h, 20.0, 90.0), "on_surface_variant": hsl_hex(h, 12.0, 38.0), "outline": hsl_hex(h, 10.0, 56.0), "outline_variant": hsl_hex(h, 14.0, 84.0), "error": "#FF3B30" },
        "dark": { "primary": hsl_hex(h, s, 64.0), "on_primary": "#FFFFFF", "secondary": hsl_hex(hs, s * 0.7, 64.0), "tertiary": hsl_hex(ht, s * 0.75, 62.0), "background": hsl_hex(h, 24.0, 7.0), "on_background": "#FFFFFF", "surface": hsl_hex(h, 22.0, 12.0), "on_surface": "#FFFFFF", "surface_container": hsl_hex(h, 20.0, 17.0), "surface_variant": hsl_hex(h, 18.0, 22.0), "on_surface_variant": hsl_hex(h, 12.0, 72.0), "outline": hsl_hex(h, 10.0, 56.0), "outline_variant": hsl_hex(h, 14.0, 26.0), "error": "#FF453A" }
    });
    let win = serde_json::json!({
        "light": {
            "accentColor": hsl_hex(h, s, 44.0),
            "accentColorHover": hsl_hex(h, s, 38.0),
            "accentColorPressed": hsl_hex(h, s, 32.0)
        },
        "dark": {
            "accentColor": hsl_hex(h, s, 64.0),
            "accentColorHover": hsl_hex(h, s, 70.0),
            "accentColorPressed": hsl_hex(h, s, 56.0)
        }
    });
    let lin = serde_json::json!({ "light": { "accent_color": hsl_hex(h, s, 46.0) }, "dark": { "accent_color": hsl_hex(h, s, 64.0) } });
    let dir = if Path::new("demo_app/config/colors").exists() { Path::new("demo_app/config/colors") } else { Path::new("config/colors") };
    let _ = fs::write(dir.join("android.json"), serde_json::to_string_pretty(&android).unwrap());
    let _ = fs::write(dir.join("ios.json"), serde_json::to_string_pretty(&ios).unwrap());
    let _ = fs::write(dir.join("web.json"), serde_json::to_string_pretty(&web).unwrap());
    let _ = fs::write(dir.join("windows.json"), serde_json::to_string_pretty(&win).unwrap());
    let _ = fs::write(dir.join("linux.json"), serde_json::to_string_pretty(&lin).unwrap());
    format!("{} ({})", name, seed)
}
