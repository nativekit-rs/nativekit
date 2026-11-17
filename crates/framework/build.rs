use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;

const DEFAULT_TYPOGRAPHY_JSON: &str = r#"{
  "large_title": {
    "ios": { "style": "largeTitle", "weight": "regular", "emphasized": "bold" },
    "android": { "style": "displaySmall", "weight": "regular", "emphasized": "bold" },
    "linux": { "style": "title-1", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "LargeTitle", "weight": "regular", "emphasized": "bold" },
    "web": { "style": "large-title", "weight": "regular", "emphasized": "bold" }
  },
  "title1": {
    "ios": { "style": "title", "weight": "regular", "emphasized": "bold" },
    "android": { "style": "headlineLarge", "weight": "regular", "emphasized": "bold" },
    "linux": { "style": "title-1", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "Title1", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "title-1", "weight": "regular", "emphasized": "bold" }
  },
  "title2": {
    "ios": { "style": "title2", "weight": "regular", "emphasized": "bold" },
    "android": { "style": "headlineMedium", "weight": "regular", "emphasized": "bold" },
    "linux": { "style": "title-2", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "Title2", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "title-2", "weight": "regular", "emphasized": "bold" }
  },
  "title3": {
    "ios": { "style": "title3", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "headlineSmall", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "title-3", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "Title3", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "title-3", "weight": "regular", "emphasized": "semibold" }
  },
  "headline": {
    "ios": { "style": "headline", "weight": "semibold", "emphasized": "bold" },
    "android": { "style": "titleMedium", "weight": "semibold", "emphasized": "bold" },
    "linux": { "style": "heading", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "Headline", "weight": "semibold", "emphasized": "bold" },
    "web": { "style": "headline", "weight": "semibold", "emphasized": "bold" }
  },
  "body": {
    "ios": { "style": "body", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "bodyLarge", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "body", "weight": "regular", "emphasized": "semibold" },
    "windows": { "style": "Body", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "body", "weight": "regular", "emphasized": "semibold" }
  },
  "callout": {
    "ios": { "style": "callout", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "bodyMedium", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "body", "weight": "regular", "emphasized": "semibold" },
    "windows": { "style": "Callout", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "callout", "weight": "regular", "emphasized": "semibold" }
  },
  "subhead": {
    "ios": { "style": "subheadline", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "titleSmall", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "title-4", "weight": "regular", "emphasized": "semibold" },
    "windows": { "style": "Subhead", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "subhead", "weight": "regular", "emphasized": "semibold" }
  },
  "footnote": {
    "ios": { "style": "footnote", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "bodySmall", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "caption", "weight": "regular", "emphasized": "semibold" },
    "windows": { "style": "Footnote", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "footnote", "weight": "regular", "emphasized": "semibold" }
  },
  "caption1": {
    "ios": { "style": "caption", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "labelMedium", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "caption-heading", "weight": "bold", "emphasized": "heavy" },
    "windows": { "style": "Caption1", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "caption-1", "weight": "regular", "emphasized": "semibold" }
  },
  "caption2": {
    "ios": { "style": "caption2", "weight": "regular", "emphasized": "semibold" },
    "android": { "style": "labelSmall", "weight": "regular", "emphasized": "semibold" },
    "linux": { "style": "caption", "weight": "regular", "emphasized": "semibold" },
    "windows": { "style": "Caption2", "weight": "regular", "emphasized": "semibold" },
    "web": { "style": "caption-2", "weight": "regular", "emphasized": "semibold" }
  }
}"#;

const DEFAULT_COLORS_JSON: &str = r##"{
  "primary": { "ios": "accentColor", "android": "primary", "linux": "accent_color", "windows": "SystemAccentColor", "web": "var(--rn-primary)" },
  "secondary": { "ios": "secondary", "android": "secondary", "linux": "purple_3", "windows": "TextFillColorSecondaryBrush", "web": "var(--rn-secondary)" },
  "text_primary": { "ios": "label", "android": "onSurface", "linux": "window_fg_color", "windows": "TextFillColorPrimaryBrush", "web": "var(--rn-on-surface)" },
  "text_secondary": { "ios": "secondaryLabel", "android": "onSurfaceVariant", "linux": { "light": "#5E5C64", "dark": "#C0BFBC" }, "windows": "TextFillColorSecondaryBrush", "web": "var(--rn-on-surface-variant)" },
  "surface": { "ios": "systemBackground", "android": "surface", "linux": "card_bg_color", "windows": "CardBackgroundFillColorDefaultBrush", "web": "var(--rn-surface)" },
  "surface_secondary": { "ios": "secondarySystemBackground", "android": "surfaceVariant", "linux": "view_bg_color", "windows": "LayerFillColorDefaultBrush", "web": "var(--rn-surface-container)" },
  "blue": { "ios": "blue", "android": { "light": "#007AFF", "dark": "#0A84FF" }, "linux": "blue_3", "windows": "#0078D4", "web": { "light": "#007AFF", "dark": "#0A84FF" } },
  "red": { "ios": "red", "android": { "light": "#FF3B30", "dark": "#FF453A" }, "linux": "red_3", "windows": "#D13438", "web": { "light": "#FF3B30", "dark": "#FF453A" } },
  "green": { "ios": "green", "android": { "light": "#34C759", "dark": "#30D158" }, "linux": "green_3", "windows": "#107C10", "web": { "light": "#34C759", "dark": "#30D158" } },
  "orange": { "ios": "orange", "android": { "light": "#FF9500", "dark": "#FF9F0A" }, "linux": "orange_3", "windows": "#FF8C00", "web": { "light": "#FF9500", "dark": "#FF9F0A" } },
  "yellow": { "ios": "yellow", "android": { "light": "#FFCC00", "dark": "#FFD60A" }, "linux": "yellow_3", "windows": "#FFB900", "web": { "light": "#FFCC00", "dark": "#FFD60A" } },
  "purple": { "ios": "purple", "android": { "light": "#AF52DE", "dark": "#BF5AF2" }, "linux": "purple_3", "windows": "#881798", "web": { "light": "#AF52DE", "dark": "#BF5AF2" } },
  "pink": { "ios": "pink", "android": { "light": "#FF2D55", "dark": "#FF375F" }, "linux": "red_2", "windows": "#E3008C", "web": { "light": "#FF2D55", "dark": "#FF375F" } },
  "teal": { "ios": "teal", "android": { "light": "#30B0C7", "dark": "#40C8E0" }, "linux": "blue_2", "windows": "#00B7C3", "web": { "light": "#30B0C7", "dark": "#40C8E0" } },
  "indigo": { "ios": "indigo", "android": { "light": "#5856D6", "dark": "#5E5CE6" }, "linux": "purple_4", "windows": "#4F46E5", "web": { "light": "#5856D6", "dark": "#5E5CE6" } },
  "gray": { "ios": "gray", "android": "outline", "linux": "dark_1", "windows": "#8A8886", "web": { "light": "#8E8E93", "dark": "#98989D" } },
  "white": { "ios": "white", "android": "#FFFFFF", "linux": "light_1", "windows": "#FFFFFF", "web": "#FFFFFF" },
  "black": { "ios": "black", "android": "#000000", "linux": "dark_5", "windows": "#000000", "web": "#000000" }
}"##;

const DEFAULT_ICONS_JSON: &str = r##"{
  "_defaults": {
    "ios": { "variant": "none", "rendering": "monochrome" },
    "android": { "style": "rounded" },
    "linux": { "style": "symbolic" },
    "windows": { "style": "regular" },
    "web": { "style": "outlined" }
  },
  "arrow_right": { "ios": "arrow.right", "android": "ArrowForward", "linux": "go-next-symbolic", "windows": "\uE72A", "web": "arrow_right" },
  "arrow_left": { "ios": "arrow.left", "android": "ArrowBack", "linux": "go-previous-symbolic", "windows": "\uE72B", "web": "arrow_left" },
  "sparkles": { "ios": "sparkles", "android": "Star", "linux": "starred-symbolic", "windows": "\uE734", "web": "sparkles" },
  "star": { "ios": "star", "android": "Star", "linux": "starred-symbolic", "windows": "\uE734", "web": "star" },
  "heart": { "ios": "heart", "android": "Favorite", "linux": "emblem-favorite-symbolic", "windows": "\uEB51", "web": "heart" },
  "check": { "ios": "checkmark", "android": "Check", "linux": "object-select-symbolic", "windows": "\uE73E", "web": "check" }
}"##;

fn is_valid_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_lowercase() {
        return false;
    }
    let mut prev_underscore = false;
    for c in chars {
        if c == '_' {
            if prev_underscore {
                return false;
            }
            prev_underscore = true;
        } else if c.is_ascii_lowercase() || c.is_ascii_digit() {
            prev_underscore = false;
        } else {
            return false;
        }
    }
    !prev_underscore
}

fn map_ios_weight(w: &str) -> &'static str {
    match w.to_lowercase().as_str() {
        "ultralight" | "ultra_light" => ".ultraLight",
        "thin" => ".thin",
        "light" => ".light",
        "medium" => ".medium",
        "semibold" | "semi_bold" => ".semibold",
        "bold" => ".bold",
        "heavy" | "extrabold" | "extra_bold" => ".heavy",
        "black" => ".black",
        _ => ".regular",
    }
}

fn map_android_weight(w: &str) -> &'static str {
    match w.to_lowercase().as_str() {
        "ultralight" | "ultra_light" => "androidx.compose.ui.text.font.FontWeight.ExtraLight",
        "thin" => "androidx.compose.ui.text.font.FontWeight.Thin",
        "light" => "androidx.compose.ui.text.font.FontWeight.Light",
        "medium" => "androidx.compose.ui.text.font.FontWeight.Medium",
        "semibold" | "semi_bold" => "androidx.compose.ui.text.font.FontWeight.SemiBold",
        "bold" => "androidx.compose.ui.text.font.FontWeight.Bold",
        "heavy" | "extrabold" | "extra_bold" => "androidx.compose.ui.text.font.FontWeight.ExtraBold",
        "black" => "androidx.compose.ui.text.font.FontWeight.Black",
        _ => "androidx.compose.ui.text.font.FontWeight.Normal",
    }
}

fn map_css_weight_class(w: &str) -> &'static str {
    match w.to_lowercase().as_str() {
        "ultralight" | "ultra_light" => "weight-ultralight",
        "thin" => "weight-thin",
        "light" => "weight-light",
        "medium" => "weight-medium",
        "semibold" | "semi_bold" => "weight-semibold",
        "bold" => "weight-bold",
        "heavy" | "extrabold" | "extra_bold" => "weight-heavy",
        "black" => "weight-black",
        _ => "weight-regular",
    }
}

fn map_windows_weight(w: &str) -> &'static str {
    match w.to_lowercase().as_str() {
        "ultralight" | "ultra_light" => "Microsoft.UI.Text.FontWeights.ExtraLight",
        "thin" => "Microsoft.UI.Text.FontWeights.Thin",
        "light" => "Microsoft.UI.Text.FontWeights.Light",
        "medium" => "Microsoft.UI.Text.FontWeights.Medium",
        "semibold" | "semi_bold" => "Microsoft.UI.Text.FontWeights.SemiBold",
        "bold" => "Microsoft.UI.Text.FontWeights.Bold",
        "heavy" | "extrabold" | "extra_bold" => "Microsoft.UI.Text.FontWeights.ExtraBold",
        "black" => "Microsoft.UI.Text.FontWeights.Black",
        _ => "Microsoft.UI.Text.FontWeights.Normal",
    }
}

fn map_windows_font_size(style: &str) -> u32 {
    match style {
        "LargeTitle" | "large_title" => 34,
        "Title1" | "title1" => 28,
        "Title2" | "title2" => 22,
        "Title3" | "title3" => 20,
        "Headline" | "headline" => 17,
        "Body" | "body" => 17,
        "Callout" | "callout" => 16,
        "Subhead" | "subhead" => 15,
        "Footnote" | "footnote" => 13,
        "Caption1" | "caption1" => 12,
        "Caption2" | "caption2" => 11,
        _ => 15,
    }
}

fn extract_platform_fields(entry: &Value, platform: &str, default_style: &str) -> (String, String, String) {
    if let Some(p) = entry.get(platform) {
        if let Some(s) = p.as_str() {
            return (s.to_string(), "regular".to_string(), "semibold".to_string());
        }
        let style = p.get("style").and_then(|v| v.as_str()).unwrap_or(default_style).to_string();
        let weight = p.get("weight").and_then(|v| v.as_str()).unwrap_or("regular").to_string();
        let emphasized = p.get("emphasized").and_then(|v| v.as_str()).unwrap_or("semibold").to_string();
        (style, weight, emphasized)
    } else {
        (default_style.to_string(), "regular".to_string(), "semibold".to_string())
    }
}

fn read_config_file(manifest_dir: &PathBuf, rel_name: &str, default_content: &str) -> String {
    let mut names = vec![rel_name.to_string()];
    if rel_name == "colors.json" {
        names.insert(0, "colors/main.json".to_string());
    }
    for name in &names {
        let candidates = [
            manifest_dir.join(format!("../../demo_app/config/{}", name)),
            manifest_dir.join(format!("../../config/{}", name)),
            manifest_dir.join(format!("../demo_app/config/{}", name)),
            PathBuf::from(format!("demo_app/config/{}", name)),
            PathBuf::from(format!("config/{}", name)),
        ];
        for path in &candidates {
            if path.exists() {
                println!("cargo:rerun-if-changed={}", path.display());
                if let Ok(c) = fs::read_to_string(path) {
                    return c;
                }
            }
        }
    }
    default_content.to_string()
}

fn is_svg_path(s: &str) -> bool {
    let trimmed = s.trim();
    trimmed.ends_with(".svg")
        || trimmed.starts_with("./")
        || trimmed.starts_with("config/")
        || trimmed.starts_with("icons/")
        || trimmed.contains('/')
}

fn read_svg_file(manifest_dir: &PathBuf, svg_rel: &str) -> Option<String> {
    let clean = svg_rel.trim_start_matches("./").trim_start_matches("demo_app/");
    let candidates = [
        manifest_dir.join(format!("../../demo_app/config/{}", clean)),
        manifest_dir.join(format!("../../demo_app/{}", clean)),
        manifest_dir.join(format!("../../config/{}", clean)),
        manifest_dir.join(format!("../../{}", clean)),
        PathBuf::from(format!("demo_app/config/{}", clean)),
        PathBuf::from(format!("demo_app/config/icons/{}", clean.trim_start_matches("icons/"))),
        PathBuf::from(format!("demo_app/{}", clean)),
        PathBuf::from(format!("config/{}", clean)),
        PathBuf::from(format!("config/icons/{}", clean.trim_start_matches("icons/"))),
        PathBuf::from(clean),
    ];
    for path in &candidates {
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
            if let Ok(c) = fs::read_to_string(path) {
                return Some(c);
            }
        }
    }
    None
}

fn format_kotlin_hex(hex: &str) -> String {
    let clean = hex.trim_start_matches('#');
    let full = if clean.len() == 6 {
        format!("FF{}", clean)
    } else if clean.len() == 8 {
        clean.to_string()
    } else {
        "FF000000".to_string()
    };
    format!("androidx.compose.ui.graphics.Color(0x{})", full.to_uppercase())
}

fn write_if_changed(path: &PathBuf, content: &str) {
    let existing = fs::read_to_string(path).unwrap_or_default();
    if existing != content {
        fs::write(path, content).unwrap_or_else(|e| panic!("Failed to write {}: {}", path.display(), e));
    }
}

fn builtin_web_svg(name: &str) -> &'static str {
    match name {
        "arrow_right" => r#"<path d="M5 12h14M13 6l6 6-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "arrow_left" => r#"<path d="M19 12H5M11 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "chevron_right" => r#"<path d="M9 6l6 6-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "chevron_left" => r#"<path d="M15 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "home" => r#"<path d="M3 10.5L12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1v-9.5z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" fill="none"/>"#,
        "orders" | "cart" => r#"<path d="M6 6h15l-1.5 9h-12z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" fill="none"/><circle cx="9" cy="19" r="1.5" fill="currentColor"/><circle cx="17" cy="19" r="1.5" fill="currentColor"/><path d="M3 3h2l1 3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/>"#,
        "settings" => r#"<circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9c.26.604.852.997 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" stroke="currentColor" stroke-width="1.6" fill="none"/>"#,
        "user" => r#"<circle cx="12" cy="8" r="4" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M4 20c1.8-3.6 5-5.5 8-5.5s6.2 1.9 8 5.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/>"#,
        "search" => r#"<circle cx="11" cy="11" r="7" stroke="currentColor" stroke-width="2" fill="none"/><path d="M20 20l-4.2-4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>"#,
        "plus" => r#"<path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>"#,
        "check" => r#"<path d="M5 13l4 4L19 7" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "close" => r#"<path d="M6 6l12 12M18 6L6 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>"#,
        "trash" => r#"<path d="M4 7h16M10 11v6M14 11v6M6 7l1 12a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-12M9 7V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "edit" => r#"<path d="M12 20h9M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "star" => r#"<path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" fill="currentColor"/>"#,
        "heart" => r#"<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" fill="currentColor"/>"#,
        "bell" => r#"<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "info" => r#"<circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M12 16v-4M12 8h.01" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>"#,
        "warning" => r#"<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0zM12 9v4M12 17h.01" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "mail" => r#"<rect x="3" y="5" width="18" height="14" rx="2" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M3 7l9 6 9-6" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "lock" => r#"<rect x="5" y="11" width="14" height="10" rx="2" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M8 11V7a4 4 0 0 1 8 0v4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/>"#,
        "share" => r#"<path d="M4 12v7a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7M16 6l-4-4-4 4M12 2v13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "refresh" => r#"<path d="M21 12a9 9 0 1 1-2.64-6.36L21 8M21 3v5h-5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"#,
        "sparkles" => r#"<path d="M12 2L14.6 9.4L22 12L14.6 14.6L12 22L9.4 14.6L2 12L9.4 9.4L12 2Z" fill="currentColor"/>"#,
        "bolt" => r#"<path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" fill="currentColor"/>"#,
        _ => r#"<circle cx="12" cy="12" r="8" fill="currentColor"/>"#,
    }
}

fn main() {
    if let Ok(out) = std::process::Command::new("defaults").args(["read", "-g", "AppleAccentColor"]).output() {
        if out.status.success() {
            let hex = match String::from_utf8_lossy(&out.stdout).trim() {
                "-1" => "#8E8E93", "0" => "#FF3B30", "1" => "#FF9500", "2" => "#FFCC00",
                "3" => "#34C759", "5" => "#AF52DE", "6" => "#FF2D55", _ => "#007AFF",
            };
            println!("cargo:rustc-env=NK_HOST_OS_ACCENT={}", hex);
        }
    }

    println!("cargo:rustc-check-cfg=cfg(wasm_bindgen_unstable_test_coverage)");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let primitives_dir = manifest_dir.join("src/engine/primitives");

    // 0. Parse app.json for server URLs
    let app_json_content = read_config_file(&manifest_dir, "app.json", "{}");
    let app_json_single_line = app_json_content.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" ");
    println!("cargo:rustc-env=NK_APP_JSON_CONTENT={}", app_json_single_line);

    // When compiled from a packaged crates.io tarball, `../generator/blueprints` is not present
    // and Cargo forbids modifying files under `src/`. Use the pre-generated files in `src/`.
    if !manifest_dir.join("../generator/blueprints").exists() {
        return;
    }

    // 1. Parse typography.json
    let typo_content = read_config_file(&manifest_dir, "typography.json", DEFAULT_TYPOGRAPHY_JSON);
    let typo_parsed: Value = serde_json::from_str(&typo_content)
        .expect("Failed to parse config/typography.json: invalid JSON syntax");
    let typo_obj = typo_parsed
        .as_object()
        .expect("config/typography.json must be a JSON object");

    // 2. Parse colors.json
    let colors_content = read_config_file(&manifest_dir, "colors.json", DEFAULT_COLORS_JSON);
    let colors_parsed: Value = serde_json::from_str(&colors_content)
        .expect("Failed to parse config/colors.json: invalid JSON syntax");
    let colors_obj = colors_parsed
        .as_object()
        .expect("config/colors.json must be a JSON object");

    // 3. Parse icons.json
    let icons_content = read_config_file(&manifest_dir, "icons.json", DEFAULT_ICONS_JSON);
    let icons_parsed: Value = serde_json::from_str(&icons_content)
        .expect("Failed to parse config/icons.json: invalid JSON syntax");
    let icons_obj = icons_parsed
        .as_object()
        .expect("config/icons.json must be a JSON object");

    let valid_fonts = typo_obj.keys().cloned().collect::<Vec<_>>().join(", ");
    let valid_colors = colors_obj
        .keys()
        .filter(|k| !k.starts_with('_'))
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    let valid_icons = icons_obj
        .keys()
        .filter(|k| !k.starts_with('_'))
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");

    // =====================================================
    // Build Color Helpers across all 5 platforms
    // =====================================================
    let mut ios_color_cases = String::new();
    let mut android_color_cases = String::new();
    let mut gtk_color_cases = String::new();
    let mut windows_color_cases = String::new();

    for (key, val) in colors_obj {
        if key.starts_with('_') {
            continue;
        }
        if !is_valid_snake_case(key) {
            panic!(
                "Invalid color key \"{}\" in config/colors.json: keys must be snake_case.",
                key
            );
        }

        // iOS color resolution
        if let Some(ios_val) = val.get("ios") {
            if let Some(s) = ios_val.as_str() {
                let expr = match s {
                    "accentColor" => "AppTheme.primary".to_string(),
                    "blue" | "red" | "green" | "orange" | "yellow" | "purple" | "pink"
                    | "teal" | "indigo" | "mint" | "cyan" | "brown" | "gray"
                    | "primary" | "secondary" | "white" | "black" | "clear" => {
                        format!("Color.{}", s)
                    }
                    "label" | "secondaryLabel" | "tertiaryLabel" | "quaternaryLabel"
                    | "systemBackground" | "secondarySystemBackground" | "tertiarySystemBackground"
                    | "systemGroupedBackground" | "secondarySystemGroupedBackground"
                    | "tertiarySystemGroupedBackground" | "systemFill" | "secondarySystemFill"
                    | "tertiarySystemFill" | "quaternarySystemFill" | "separator"
                    | "opaqueSeparator" | "link" | "placeholderText" => {
                        format!("Color(uiColor: .{})", s)
                    }
                    hex if hex.starts_with('#') => {
                        format!("Color(uiColor: parseHexUIColor(\"{}\"))", hex)
                    }
                    other => format!("Color(\"{}\")", other),
                };
                ios_color_cases.push_str(&format!("    case \"{}\": return {}\n", key, expr));
            } else if let Some(obj) = ios_val.as_object() {
                let light = obj.get("light").and_then(|v| v.as_str()).unwrap_or("#000000");
                let dark = obj.get("dark").and_then(|v| v.as_str()).unwrap_or(light);
                ios_color_cases.push_str(&format!(
                    "    case \"{}\": return Color(uiColor: UIColor {{ traits in traits.userInterfaceStyle == .dark ? parseHexUIColor(\"{}\") : parseHexUIColor(\"{}\") }})\n",
                    key, dark, light
                ));
            }
        }

        // Android color resolution
        if let Some(and_val) = val.get("android") {
            if let Some(s) = and_val.as_str() {
                let expr = if s.starts_with('#') {
                    format_kotlin_hex(s)
                } else {
                    match s {
                        "primary" | "onPrimary" | "primaryContainer" | "onPrimaryContainer"
                        | "inversePrimary" | "secondary" | "onSecondary" | "secondaryContainer"
                        | "onSecondaryContainer" | "tertiary" | "onTertiary"
                        | "tertiaryContainer" | "onTertiaryContainer" | "background"
                        | "onBackground" | "surface" | "onSurface" | "surfaceVariant"
                        | "onSurfaceVariant" | "surfaceTint" | "inverseSurface"
                        | "inverseOnSurface" | "error" | "onError" | "errorContainer"
                        | "onErrorContainer" | "outline" | "outlineVariant" | "scrim" => {
                            format!("MaterialTheme.colorScheme.{}", s)
                        }
                        "surfaceBright" | "surfaceDim" | "surfaceContainer"
                        | "surfaceContainerHigh" | "surfaceContainerHighest"
                        | "surfaceContainerLow" | "surfaceContainerLowest" => {
                            "MaterialTheme.colorScheme.surfaceVariant".to_string()
                        }
                        _ => "MaterialTheme.colorScheme.primary".to_string(),
                    }
                };
                android_color_cases.push_str(&format!("        \"{}\" -> {}\n", key, expr));
            } else if let Some(obj) = and_val.as_object() {
                let light = obj.get("light").and_then(|v| v.as_str()).unwrap_or("#000000");
                let dark = obj.get("dark").and_then(|v| v.as_str()).unwrap_or(light);
                android_color_cases.push_str(&format!(
                    "        \"{}\" -> if (isSystemInDarkTheme()) {} else {}\n",
                    key,
                    format_kotlin_hex(dark),
                    format_kotlin_hex(light)
                ));
            }
        }

        // GTK color resolution (for CSS gradients and classes)
        if let Some(lin_val) = val.get("linux") {
            if let Some(s) = lin_val.as_str() {
                let css_expr = if s.starts_with('#') || s.starts_with("rgb") || s.starts_with('@') {
                    s.to_string()
                } else {
                    format!("@{}", s)
                };
                gtk_color_cases.push_str(&format!("        \"{}\" => \"{}\".to_string(),\n", key, css_expr));
            } else if let Some(obj) = lin_val.as_object() {
                let light = obj.get("light").and_then(|v| v.as_str()).unwrap_or("#000000");
                gtk_color_cases.push_str(&format!("        \"{}\" => \"{}\".to_string(),\n", key, light));
            }
        }

        // Windows color resolution
        if let Some(win_val) = val.get("windows") {
            if let Some(s) = win_val.as_str() {
                windows_color_cases.push_str(&format!("                case \"{}\": return \"{}\";\n", key, s));
            } else if let Some(obj) = win_val.as_object() {
                let light = obj.get("light").and_then(|v| v.as_str()).unwrap_or("#000000");
                windows_color_cases.push_str(&format!("                case \"{}\": return \"{}\";\n", key, light));
            }
        }
    }

    let color_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json
// ==========================================
use serde_json::{{json, Value}};

#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {{
    pub color: String,
    pub position: Option<f32>,
    pub opacity: f32,
}}

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {{
    pub gradient_type: String,
    pub direction: String,
    pub angle: Option<f32>,
    pub stops: Vec<GradientStop>,
    pub opacity: f32,
}}

fn normalize_unit(val: f32) -> f32 {{
    if val > 1.0 {{
        (val / 100.0).clamp(0.0, 1.0)
    }} else {{
        val.clamp(0.0, 1.0)
    }}
}}

impl Gradient {{
    pub fn new() -> Self {{
        Self {{
            gradient_type: "linear".to_string(),
            direction: "to_right".to_string(),
            angle: None,
            stops: Vec::new(),
            opacity: 1.0,
        }}
    }}

    pub fn linear() -> Self {{
        Self::new()
    }}

    pub fn radial() -> Self {{
        Self {{
            gradient_type: "radial".to_string(),
            ..Self::new()
        }}
    }}

    pub fn angular() -> Self {{
        Self {{
            gradient_type: "angular".to_string(),
            ..Self::new()
        }}
    }}

    pub fn conic() -> Self {{
        Self::angular()
    }}

    /// Valid options: linear, radial, angular
    pub fn kind(mut self, kind: impl Into<String>) -> Self {{
        self.gradient_type = kind.into();
        self
    }}

    /// Valid options: to_right, to_left, to_bottom, to_top, to_bottom_right, to_bottom_left, to_top_right, to_top_left
    pub fn direction(mut self, direction: impl Into<String>) -> Self {{
        self.direction = direction.into();
        self
    }}

    pub fn angle(mut self, degrees: f32) -> Self {{
        self.angle = Some(degrees);
        self
    }}

    /// Valid options: {valid_colors}
    pub fn stop(mut self, color: impl Into<String>, position: f32) -> Self {{
        self.stops.push(GradientStop {{
            color: color.into(),
            position: Some(normalize_unit(position)),
            opacity: 1.0,
        }});
        self
    }}

    /// Valid options: {valid_colors}
    pub fn stop_with_opacity(mut self, color: impl Into<String>, position: f32, opacity: f32) -> Self {{
        self.stops.push(GradientStop {{
            color: color.into(),
            position: Some(normalize_unit(position)),
            opacity: normalize_unit(opacity),
        }});
        self
    }}

    /// Valid options: {valid_colors}
    pub fn add_stop(mut self, color: impl Into<String>) -> Self {{
        self.stops.push(GradientStop {{
            color: color.into(),
            position: None,
            opacity: 1.0,
        }});
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        self.opacity = normalize_unit(opacity);
        self
    }}

    pub fn to_json_value(&self) -> Value {{
        let count = self.stops.len();
        let mut stops_json = Vec::with_capacity(count);
        for (i, s) in self.stops.iter().enumerate() {{
            let pos = s.position.unwrap_or_else(|| {{
                if count <= 1 {{ 0.0 }} else {{ (i as f32) / ((count - 1) as f32) }}
            }});
            stops_json.push(json!({{
                "color": s.color,
                "position": pos,
                "opacity": s.opacity,
            }}));
        }}
        let angle_deg = self.angle.unwrap_or_else(|| match self.direction.as_str() {{
            "to_right" => 90.0,
            "to_left" => 270.0,
            "to_bottom" => 180.0,
            "to_top" => 0.0,
            "to_bottom_right" => 135.0,
            "to_bottom_left" => 225.0,
            "to_top_right" => 45.0,
            "to_top_left" => 315.0,
            _ => 90.0,
        }});
        json!({{
            "type": self.gradient_type,
            "direction": self.direction,
            "angle": angle_deg,
            "opacity": self.opacity,
            "stops": stops_json,
        }})
    }}
}}


pub trait IntoBackgroundImage {{
    fn into_bg_image_url(self) -> String;
}}
impl IntoBackgroundImage for &str {{
    fn into_bg_image_url(self) -> String {{ self.to_string() }}
}}
impl IntoBackgroundImage for String {{
    fn into_bg_image_url(self) -> String {{ self }}
}}
impl IntoBackgroundImage for crate::engine::primitives::Image {{
    fn into_bg_image_url(self) -> String {{
        self.props.into_iter().find_map(|(k, v)| {{
            if k == "src" {{
                if let crate::engine::ast::PropValue::String(s) = v {{ Some(s) }} else {{ None }}
            }} else {{ None }}
        }}).unwrap_or_default()
    }}
}}

pub enum BackgroundValue {{
    Solid(String),
    Gradient(Gradient),
}}

pub trait IntoBackground {{
    fn into_background(self) -> BackgroundValue;
}}

impl IntoBackground for &str {{
    fn into_background(self) -> BackgroundValue {{
        BackgroundValue::Solid(self.to_string())
    }}
}}

impl IntoBackground for String {{
    fn into_background(self) -> BackgroundValue {{
        BackgroundValue::Solid(self)
    }}
}}

impl IntoBackground for Gradient {{
    fn into_background(self) -> BackgroundValue {{
        BackgroundValue::Gradient(self)
    }}
}}

#[macro_export]
macro_rules! Gradient {{
    () => {{
        $crate::engine::primitives::Gradient::new()
    }};
    ([$($color:expr),+ $(,)?]) => {{
        let mut g = $crate::engine::primitives::Gradient::new();
        $(
            g = g.add_stop($color);
        )+
        g
    }};
    ($c1:expr, $c2:expr $(, $rest:expr)* $(,)?) => {{
        let mut g = $crate::engine::primitives::Gradient::new()
            .add_stop($c1)
            .add_stop($c2);
        $(
            g = g.add_stop($rest);
        )*
        g
    }};
}}

pub const COLOR_IOS_HELPERS: &str = r##"
func parseHexUIColor(_ hex: String) -> UIColor {{
    let clean = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
    var int: UInt64 = 0
    Scanner(string: clean).scanHexInt64(&int)
    let a, r, g, b: UInt64
    switch clean.count {{
    case 3:
        (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
    case 6:
        (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
    case 8:
        (r, g, b, a) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
    default:
        (a, r, g, b) = (255, 0, 0, 0)
    }}
    return UIColor(red: CGFloat(r) / 255.0, green: CGFloat(g) / 255.0, blue: CGFloat(b) / 255.0, alpha: CGFloat(a) / 255.0)
}}

func resolveAppColor(_ name: String) -> Color? {{
    if name.isEmpty {{ return nil }}
    if runtimeAccentMode == "app" {{
        if name == "primary" && !runtimePrimaryHex.isEmpty {{
            return Color(uiColor: parseHexUIColor(runtimePrimaryHex))
        }}
        if name == "secondary" && !runtimeSecondaryHex.isEmpty {{
            return Color(uiColor: parseHexUIColor(runtimeSecondaryHex))
        }}
    }}
    switch name {{
{ios_color_cases}    default:
        if name.hasPrefix("#") || name.hasPrefix("0x") {{
            return Color(uiColor: parseHexUIColor(name))
        }}
        return nil
    }}
}}

@ViewBuilder
func renderGradientView(_ grad: [String: Any]) -> some View {{
    let gType = grad["type"] as? String ?? "linear"
    let direction = grad["direction"] as? String ?? "to_right"
    let gradOpacity = (grad["opacity"] as? NSNumber)?.doubleValue ?? 1.0
    let rawStops = grad["stops"] as? [[String: Any]] ?? []
    let swiftStops: [Gradient.Stop] = rawStops.compactMap {{ s in
        let colName = s["color"] as? String ?? ""
        let pos = (s["position"] as? NSNumber)?.doubleValue ?? 0.0
        let op = (s["opacity"] as? NSNumber)?.doubleValue ?? 1.0
        guard let baseCol = resolveAppColor(colName) else {{ return nil }}
        return Gradient.Stop(color: baseCol.opacity(op), location: CGFloat(pos))
    }}
    let (startPt, endPt): (UnitPoint, UnitPoint) = {{
        switch direction {{
        case "to_left": return (.trailing, .leading)
        case "to_bottom": return (.top, .bottom)
        case "to_top": return (.bottom, .top)
        case "to_bottom_right": return (.topLeading, .bottomTrailing)
        case "to_bottom_left": return (.topTrailing, .bottomLeading)
        case "to_top_right": return (.bottomLeading, .topTrailing)
        case "to_top_left": return (.bottomTrailing, .topLeading)
        default: return (.leading, .trailing)
        }}
    }}()
    if gType == "radial" {{
        RadialGradient(gradient: Gradient(stops: swiftStops), center: .center, startRadius: 0, endRadius: 240)
            .opacity(gradOpacity)
    }} else if gType == "angular" || gType == "conic" {{
        AngularGradient(gradient: Gradient(stops: swiftStops), center: .center)
            .opacity(gradOpacity)
    }} else {{
        LinearGradient(gradient: Gradient(stops: swiftStops), startPoint: startPt, endPoint: endPt)
            .opacity(gradOpacity)
    }}
}}

struct UnclipListCell: UIViewRepresentable {{
    func makeUIView(context: Context) -> UIView {{
        let v = UIView()
        DispatchQueue.main.async {{
            var cur: UIView? = v
            while let p = cur?.superview {{
                p.clipsToBounds = false
                if String(describing: type(of: p)).contains("Cell") {{ break }}
                cur = p
            }}
        }}
        return v
    }}
    func updateUIView(_ uiView: UIView, context: Context) {{}}
}}

struct NativeFlowLayout: Layout {{
    var spacing: CGFloat = 0
    var alignX: String = "left"
    var alignY: String = "center"

    func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {{
        let maxWidth = proposal.width ?? .infinity
        var rowWidth: CGFloat = 0
        var rowHeight: CGFloat = 0
        var totalHeight: CGFloat = 0
        var maxRowWidth: CGFloat = 0
        for subview in subviews {{
            let sz = subview.sizeThatFits(.unspecified)
            if rowWidth > 0 && rowWidth + spacing + sz.width > maxWidth {{
                maxRowWidth = max(maxRowWidth, rowWidth)
                totalHeight += rowHeight + spacing
                rowWidth = sz.width
                rowHeight = sz.height
            }} else {{
                rowWidth = rowWidth > 0 ? rowWidth + spacing + sz.width : sz.width
                rowHeight = max(rowHeight, sz.height)
            }}
        }}
        maxRowWidth = max(maxRowWidth, rowWidth)
        totalHeight += rowHeight
        return CGSize(width: maxWidth.isFinite && alignX == "space_between" ? maxWidth : maxRowWidth, height: totalHeight)
    }}

    func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {{
        var rows: [[(LayoutSubviews.Element, CGSize)]] = []
        var currentRow: [(LayoutSubviews.Element, CGSize)] = []
        var rowWidth: CGFloat = 0
        for subview in subviews {{
            let sz = subview.sizeThatFits(.unspecified)
            if !currentRow.isEmpty && rowWidth + spacing + sz.width > bounds.width {{
                rows.append(currentRow)
                currentRow = [(subview, sz)]
                rowWidth = sz.width
            }} else {{
                rowWidth = currentRow.isEmpty ? sz.width : rowWidth + spacing + sz.width
                currentRow.append((subview, sz))
            }}
        }}
        if !currentRow.isEmpty {{ rows.append(currentRow) }}

        var y = bounds.minY
        for row in rows {{
            let rHeight = row.map {{ $0.1.height }}.max() ?? 0
            let itemsWidth = row.reduce(0) {{ $0 + $1.1.width }}
            let gapsWidth = CGFloat(max(0, row.count - 1)) * spacing
            let usedWidth = itemsWidth + gapsWidth
            let extraSpace = max(0, bounds.width - usedWidth)
            var x = bounds.minX
            var dynamicGap = spacing
            if alignX == "center" {{
                x += extraSpace / 2
            }} else if alignX == "right" {{
                x += extraSpace
            }} else if alignX == "space_between" && row.count > 1 {{
                dynamicGap = extraSpace / CGFloat(row.count - 1) + spacing
            }}
            for (subview, sz) in row {{
                let itemY: CGFloat = alignY == "top" ? y : (alignY == "bottom" ? y + (rHeight - sz.height) : y + (rHeight - sz.height) / 2)
                subview.place(at: CGPoint(x: x, y: itemY), proposal: ProposedViewSize(sz))
                x += sz.width + dynamicGap
            }}
            y += rHeight + spacing
        }}
    }}
}}

extension View {{
    @ViewBuilder
    func applyListRowBackground(node: DynamicUINode) -> some View {{
        let bgName = node.props["background"] as? String ?? ""
        let bgGrad = node.props["background_gradient"] as? [String: Any]
        if let grad = bgGrad {{
            self.listRowBackground(renderGradientView(grad))
        }} else if !bgName.isEmpty, let col = resolveAppColor(bgName) {{
            self.listRowBackground(col)
        }} else {{
            self
        }}
    }}

    @ViewBuilder
    func applyContainerStyle(node: DynamicUINode) -> some View {{
        let pad = CGFloat(node.props["padding"] as? Int ?? 0)
        let radius = CGFloat(node.props["corner_radius"] as? Int ?? 0)
        let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
        let bgName = node.props["background"] as? String ?? ""
        let bgImg = node.props["background_image"] as? String ?? ""
        let bgGrad = node.props["background_gradient"] as? [String: Any]

        self
            .padding(pad)
            .background {{
                if !bgImg.isEmpty {{
                    if (bgImg.hasPrefix("http://") || bgImg.hasPrefix("https://")), let url = URL(string: bgImg) {{
                        AsyncImage(url: url) {{ phase in
                            if case .success(let image) = phase {{ image.resizable().scaledToFill() }}
                            else {{ Color.secondary.opacity(0.12) }}
                        }}
                    }} else {{
                        let clean = ((bgImg as NSString).lastPathComponent as NSString).deletingPathExtension
                        if UIImage(named: clean) != nil {{
                            Image(clean).resizable().scaledToFill()
                        }} else if let devUrl = URL(string: "http://127.0.0.1:5824/" + bgImg.replacingOccurrences(of: "./", with: "").trimmingCharacters(in: CharacterSet(charactersIn: "/"))) {{
                            AsyncImage(url: devUrl) {{ phase in
                                if case .success(let image) = phase {{ image.resizable().scaledToFill() }}
                                else {{ Color.secondary.opacity(0.12) }}
                            }}
                        }}
                    }}
                }} else if let grad = bgGrad {{
                    renderGradientView(grad)
                }} else if !bgName.isEmpty, let col = resolveAppColor(bgName) {{
                    col
                }}
            }}
            .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
            .opacity(opacity)
    }}
}}
"##;

pub const COLOR_ANDROID_HELPERS: &str = r##"
@Composable
fun resolveAppColor(name: String): androidx.compose.ui.graphics.Color? {{
    if (name.isEmpty()) return null
    return when (name) {{
{android_color_cases}        else -> {{
            if (name.startsWith("#")) {{
                try {{
                    val clean = name.removePrefix("#")
                    val normalized = when (clean.length) {{
                        3 -> "#${{clean[0]}}${{clean[0]}}${{clean[1]}}${{clean[1]}}${{clean[2]}}${{clean[2]}}"
                        8 -> "#${{clean.substring(6, 8)}}${{clean.substring(0, 6)}}"
                        else -> name
                    }}
                    androidx.compose.ui.graphics.Color(android.graphics.Color.parseColor(normalized))
                }} catch (e: Exception) {{ null }}
            }} else null
        }}
    }}
}}

@Composable
fun resolveAppGradientBrush(grad: org.json.JSONObject): androidx.compose.ui.graphics.Brush? {{
    val gType = grad.optString("type", "linear")
    val direction = grad.optString("direction", "to_right")
    val gradOpacity = grad.optDouble("opacity", 1.0).toFloat()
    val stopsArr = grad.optJSONArray("stops") ?: return null
    val colorStops = mutableListOf<Pair<Float, androidx.compose.ui.graphics.Color>>()
    for (i in 0 until stopsArr.length()) {{
        val s = stopsArr.optJSONObject(i) ?: continue
        val colName = s.optString("color", "")
        val pos = s.optDouble("position", 0.0).toFloat()
        val op = (s.optDouble("opacity", 1.0).toFloat() * gradOpacity).coerceIn(0f, 1f)
        val baseCol = resolveAppColor(colName) ?: continue
        colorStops.add(Pair(pos, baseCol.copy(alpha = baseCol.alpha * op)))
    }}
    if (colorStops.size < 2) return null
    val stopsArray = colorStops.toTypedArray()
    return when (gType) {{
        "radial" -> androidx.compose.ui.graphics.Brush.radialGradient(colorStops = stopsArray)
        "angular", "conic" -> androidx.compose.ui.graphics.Brush.sweepGradient(colorStops = stopsArray)
        else -> {{
            val (start, end) = when (direction) {{
                "to_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 0f), androidx.compose.ui.geometry.Offset(0f, 0f))
                "to_bottom" -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(0f, 1000f))
                "to_top" -> Pair(androidx.compose.ui.geometry.Offset(0f, 1000f), androidx.compose.ui.geometry.Offset(0f, 0f))
                "to_bottom_right" -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(1000f, 1000f))
                "to_bottom_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 0f), androidx.compose.ui.geometry.Offset(0f, 1000f))
                "to_top_right" -> Pair(androidx.compose.ui.geometry.Offset(0f, 1000f), androidx.compose.ui.geometry.Offset(1000f, 0f))
                "to_top_left" -> Pair(androidx.compose.ui.geometry.Offset(1000f, 1000f), androidx.compose.ui.geometry.Offset(0f, 0f))
                else -> Pair(androidx.compose.ui.geometry.Offset(0f, 0f), androidx.compose.ui.geometry.Offset(1000f, 0f))
            }}
            androidx.compose.ui.graphics.Brush.linearGradient(colorStops = stopsArray, start = start, end = end)
        }}
    }}
}}

@Composable
fun Modifier.applyContainerStyle(node: DynamicUINode): Modifier {{
    var m = this
    val radius = node.getInt("corner_radius", 0)
    val pad = node.getInt("padding", 0)
    val opacity = node.getInt("opacity", 100)
    val bgName = node.getString("background", "")
    val bgGrad = node.props["background_gradient"] as? org.json.JSONObject

    if (radius > 0) {{
        m = m.clip(androidx.compose.foundation.shape.RoundedCornerShape(radius.dp))
    }}
    if (bgGrad != null) {{
        val brush = resolveAppGradientBrush(bgGrad)
        if (brush != null) {{
            m = m.background(brush = brush)
        }}
    }} else if (bgName.isNotEmpty()) {{
        val col = resolveAppColor(bgName)
        if (col != null) {{
            m = m.background(color = col)
        }}
    }}
    if (pad > 0) {{
        m = m.padding(pad.dp)
    }}
    if (opacity < 100) {{
        m = m.graphicsLayer(alpha = opacity.toFloat() / 100f)
    }}
    return m
}}
"##;

pub const COLOR_GTK_HELPERS: &str = r##"
pub fn resolve_gtk_color_css(name: &str) -> String {{
    match name {{
{gtk_color_cases}        other => {{
            if other.starts_with('#') || other.starts_with('@') || other.starts_with("rgb") || other.starts_with("hsl") {{
                other.to_string()
            }} else {{
                format!("@{{}}", other)
            }}
        }}
    }}
}}

pub fn apply_gtk_fg_color(widget: &impl gtk4::prelude::IsA<gtk4::Widget>, color: &str) {{
    if color.is_empty() {{
        return;
    }}
    if color.starts_with('#') || color.starts_with("rgb") || color.starts_with("hsl") {{
        widget.add_css_class("nk-dyn-fg");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".nk-dyn-fg {{{{ color: {{}}; }}}}", color));
        #[allow(deprecated)]
        widget.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }} else {{
        widget.add_css_class(&format!("nk-fg-{{}}", color));
    }}
}}

pub fn apply_gtk_container_style(bx: &impl gtk4::prelude::IsA<gtk4::Widget>, node: &DynamicUINode) {{
    let pad = node.get_int("padding", 0);
    let radius = node.get_int("corner_radius", 0);
    let opacity = node.get_int("opacity", 100);
    let bg = node.get_string("background", "");
    let bg_grad = node.props.get("background_gradient");

    if opacity < 100 {{
        bx.set_opacity((opacity as f64) / 100.0);
    }}

    let mut rules = String::new();
    if !bg.is_empty() && bg_grad.is_none() {{
        if bg.starts_with('#') || bg.starts_with("rgb") || bg.starts_with("hsl") {{
            rules.push_str(&format!("background-color: {{}}; ", bg));
        }} else {{
            bx.add_css_class(&format!("nk-bg-{{}}", bg));
        }}
    }}
    if pad > 0 {{
        rules.push_str(&format!("padding: {{}}px; ", pad));
    }}
    if radius > 0 {{
        rules.push_str(&format!("border-radius: {{}}px; ", radius));
    }}
    if let Some(grad) = bg_grad {{
        let g_type = grad.get("type").and_then(|v| v.as_str()).unwrap_or("linear");
        let angle = grad.get("angle").and_then(|v| v.as_f64()).unwrap_or(90.0);
        if let Some(stops) = grad.get("stops").and_then(|v| v.as_array()) {{
            let mut stop_strs = Vec::new();
            for s in stops {{
                let col_name = s.get("color").and_then(|v| v.as_str()).unwrap_or("primary");
                let pos = s.get("position").and_then(|v| v.as_f64()).unwrap_or(0.0) * 100.0;
                let op = s.get("opacity").and_then(|v| v.as_f64()).unwrap_or(1.0);
                let base_css = resolve_gtk_color_css(col_name);
                let col_css = if op < 0.999 {{
                    format!("alpha({{}}, {{:.2}})", base_css, op)
                }} else {{
                    base_css
                }};
                stop_strs.push(format!("{{}} {{:.0}}%", col_css, pos));
            }}
            if stop_strs.len() >= 2 {{
                let joined = stop_strs.join(", ");
                if g_type == "radial" {{
                    rules.push_str(&format!("background-image: radial-gradient(circle, {{}}); ", joined));
                }} else if g_type == "angular" || g_type == "conic" {{
                    rules.push_str(&format!("background-image: conic-gradient({{}}); ", joined));
                }} else {{
                    rules.push_str(&format!("background-image: linear-gradient({{:.0}}deg, {{}}); ", angle, joined));
                }}
            }}
        }}
    }}

    if !rules.is_empty() {{
        bx.add_css_class("nk-dyn-box");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".nk-dyn-box {{{{ {{}} }}}}", rules));
        #[allow(deprecated)]
        bx.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }}
}}
"##;

pub const COLOR_WEB_HELPERS: &str = r##"
function isInlineWebColor(name) {{
    return Boolean(name && (name.startsWith("#") || name.startsWith("rgb") || name.startsWith("hsl") || name.startsWith("var(")));
}}

function resolveWebColor(name) {{
    if (!name) return "";
    if (isInlineWebColor(name)) return name;
    return `var(--nk-color-${{name}})`;
}}

function applyWebFgColor(el, color) {{
    if (!color) return;
    if (isInlineWebColor(color)) {{
        el.style.color = color;
    }} else {{
        el.classList.add(`nk-fg-${{color}}`);
    }}
}}

function applyWebBgColor(el, color) {{
    if (!color) return;
    if (isInlineWebColor(color)) {{
        el.style.backgroundColor = color;
    }} else {{
        el.classList.add(`nk-bg-${{color}}`);
    }}
}}

function applyWebContainerStyle(el, node) {{
    if (!node || !node.props) return;
    if (node.props.spacing !== undefined) el.style.gap = node.props.spacing + "px";
    if (node.props.padding !== undefined) el.style.padding = node.props.padding + "px";
    if (node.props.corner_radius !== undefined) el.style.borderRadius = node.props.corner_radius + "px";
    if (node.props.opacity !== undefined) el.style.opacity = (node.props.opacity / 100.0).toString();
    if (node.props.background_image) {{
        el.style.backgroundImage = `url("${{node.props.background_image}}")`;
        el.style.backgroundSize = "cover";
        el.style.backgroundPosition = "center";
        el.style.backgroundRepeat = "no-repeat";
    }} else if (node.props.background_gradient) {{
        const grad = node.props.background_gradient;
        const gType = grad.type || "linear";
        const angle = (grad.angle !== undefined) ? grad.angle : 90;
        const stops = (grad.stops || []).map(s => {{
            const base = resolveWebColor(s.color);
            const pos = Math.round(((s.position !== undefined) ? s.position : 0) * 100);
            const op = (s.opacity !== undefined) ? s.opacity : 1.0;
            const colExpr = (op < 0.999) ? `color-mix(in srgb, ${{base}} ${{Math.round(op * 100)}}%, transparent)` : base;
            return `${{colExpr}} ${{pos}}%`;
        }});
        if (stops.length >= 2) {{
            if (gType === "radial") {{
                el.style.backgroundImage = `radial-gradient(circle, ${{stops.join(", ")}})`;
            }} else if (gType === "angular" || gType === "conic") {{
                el.style.backgroundImage = `conic-gradient(${{stops.join(", ")}})`;
            }} else {{
                el.style.backgroundImage = `linear-gradient(${{angle}}deg, ${{stops.join(", ")}})`;
            }}
        }}
    }} else if (node.props.background) {{
        applyWebBgColor(el, node.props.background);
    }}
}}
"##;

pub const COLOR_WINDOWS_HELPERS: &str = r##"
        private string MapColorToken(string name) {{
            switch (name) {{
{windows_color_cases}                default: return name;
            }}
        }}

        private Windows.UI.Color ParseHexWinColor(string hex, double opacity = 1.0) {{
            string clean = hex.TrimStart('#');
            if (clean.Length == 3) {{
                clean = $"{{clean[0]}}{{clean[0]}}{{clean[1]}}{{clean[1]}}{{clean[2]}}{{clean[2]}}";
            }}
            if (clean.Length == 6) {{
                byte r = Convert.ToByte(clean.Substring(0, 2), 16);
                byte g = Convert.ToByte(clean.Substring(2, 2), 16);
                byte b = Convert.ToByte(clean.Substring(4, 2), 16);
                byte a = (byte)Math.Clamp((int)Math.Round(opacity * 255.0), 0, 255);
                return Windows.UI.Color.FromArgb(a, r, g, b);
            }} else if (clean.Length == 8) {{
                byte r = Convert.ToByte(clean.Substring(0, 2), 16);
                byte g = Convert.ToByte(clean.Substring(2, 2), 16);
                byte b = Convert.ToByte(clean.Substring(4, 2), 16);
                byte hexA = Convert.ToByte(clean.Substring(6, 2), 16);
                byte a = (byte)Math.Clamp((int)Math.Round((hexA / 255.0) * opacity * 255.0), 0, 255);
                return Windows.UI.Color.FromArgb(a, r, g, b);
            }}
            return Windows.UI.Color.FromArgb(255, 0, 120, 212);
        }}

        private Microsoft.UI.Xaml.Media.Brush ResolveAppBrush(string name, double opacity = 1.0) {{
            if (string.IsNullOrEmpty(name)) return null;
            if (name == "primary" && !string.IsNullOrEmpty(runtimePrimaryHex)) {{
                return new Microsoft.UI.Xaml.Media.SolidColorBrush(ParseHexWinColor(runtimePrimaryHex, opacity));
            }}
            string token = MapColorToken(name);
            if (token.StartsWith("#")) {{
                return new Microsoft.UI.Xaml.Media.SolidColorBrush(ParseHexWinColor(token, opacity));
            }}
            try {{
                if (Application.Current.Resources.ContainsKey(token)) {{
                    var res = Application.Current.Resources[token];
                    if (res is Microsoft.UI.Xaml.Media.Brush b) return b;
                    if (res is Windows.UI.Color c) {{
                        return new Microsoft.UI.Xaml.Media.SolidColorBrush(c) {{ Opacity = opacity }};
                    }}
                }}
            }} catch {{ }}
            return null;
        }}

        private void ApplyWindowsContainerStyle(StackPanel sp, DynamicUINode node) {{
            if (node.Props == null) return;
            if (node.Props.ContainsKey("padding") && double.TryParse(node.Props["padding"].ToString(), out double pad)) {{
                sp.Padding = new Thickness(pad);
            }}
            if (node.Props.ContainsKey("corner_radius") && double.TryParse(node.Props["corner_radius"].ToString(), out double rad)) {{
                sp.CornerRadius = new CornerRadius(rad);
            }}
            if (node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double op)) {{
                sp.Opacity = op / 100.0;
            }}
            if (node.Props.ContainsKey("background_gradient") && node.Props["background_gradient"] is Dictionary<string, object> gradDict) {{
                var lgb = new Microsoft.UI.Xaml.Media.LinearGradientBrush();
                string dir = gradDict.ContainsKey("direction") ? gradDict["direction"].ToString() : "to_right";
                switch (dir) {{
                    case "to_left": lgb.StartPoint = new Windows.Foundation.Point(1, 0.5); lgb.EndPoint = new Windows.Foundation.Point(0, 0.5); break;
                    case "to_bottom": lgb.StartPoint = new Windows.Foundation.Point(0.5, 0); lgb.EndPoint = new Windows.Foundation.Point(0.5, 1); break;
                    case "to_top": lgb.StartPoint = new Windows.Foundation.Point(0.5, 1); lgb.EndPoint = new Windows.Foundation.Point(0.5, 0); break;
                    case "to_bottom_right": lgb.StartPoint = new Windows.Foundation.Point(0, 0); lgb.EndPoint = new Windows.Foundation.Point(1, 1); break;
                    default: lgb.StartPoint = new Windows.Foundation.Point(0, 0.5); lgb.EndPoint = new Windows.Foundation.Point(1, 0.5); break;
                }}
                if (gradDict.ContainsKey("stops") && gradDict["stops"] is System.Text.Json.JsonElement stopsEl && stopsEl.ValueKind == System.Text.Json.JsonValueKind.Array) {{
                    foreach (var s in stopsEl.EnumerateArray()) {{
                        string colName = s.TryGetProperty("color", out var cp) ? cp.GetString() : "blue";
                        double pos = s.TryGetProperty("position", out var pp) ? pp.GetDouble() : 0.0;
                        double sOp = s.TryGetProperty("opacity", out var opP) ? opP.GetDouble() : 1.0;
                        string tok = (colName == "primary" && !string.IsNullOrEmpty(runtimePrimaryHex)) ? runtimePrimaryHex : MapColorToken(colName);
                        var winCol = tok.StartsWith("#") ? ParseHexWinColor(tok, sOp) : Windows.UI.Color.FromArgb(255, 0, 120, 212);
                        lgb.GradientStops.Add(new Microsoft.UI.Xaml.Media.GradientStop {{ Color = winCol, Offset = pos }});
                    }}
                }}
                sp.Background = lgb;
            }} else if (node.Props.ContainsKey("background")) {{
                var brush = ResolveAppBrush(node.Props["background"].ToString());
                if (brush != null) sp.Background = brush;
            }}
        }}
"##;
"###
    );
    write_if_changed(&primitives_dir.join("color.rs"), &color_rs);

    // =====================================================
    // Build Text Primitive (text.rs)
    // =====================================================
    let mut ios_cases = String::new();
    let mut android_cases = String::new();
    let mut gtk_cases = String::new();
    let mut web_cases = String::new();
    let mut windows_cases = String::new();

    for (key, val) in typo_obj {
        if !is_valid_snake_case(key) {
            panic!(
                "Invalid typography style key \"{}\" in config/typography.json: keys must be snake_case.",
                key
            );
        }

        let (ios_style, ios_w, ios_emp) = extract_platform_fields(val, "ios", "body");
        ios_cases.push_str(&format!(
            "        case \"{}\": return Font.{}.weight(emphasized ? {} : {})\n",
            key,
            ios_style,
            map_ios_weight(&ios_emp),
            map_ios_weight(&ios_w)
        ));

        let (and_style, and_w, and_emp) = extract_platform_fields(val, "android", "bodyLarge");
        android_cases.push_str(&format!(
            "        \"{}\" -> MaterialTheme.typography.{}.copy(fontWeight = if (emphasized) {} else {})\n",
            key,
            and_style,
            map_android_weight(&and_emp),
            map_android_weight(&and_w)
        ));

        let (lin_style, lin_w, lin_emp) = extract_platform_fields(val, "linux", "body");
        gtk_cases.push_str(&format!(
            "                \"{}\" => (\"{}\", if emphasized {{ \"{}\" }} else {{ \"{}\" }}),\n",
            key,
            lin_style,
            map_css_weight_class(&lin_emp),
            map_css_weight_class(&lin_w)
        ));

        let (web_style, web_w, web_emp) = extract_platform_fields(val, "web", "body");
        web_cases.push_str(&format!(
            "        case \"{}\": el.classList.add(\"{}\", emphasized ? \"{}\" : \"{}\"); break;\n",
            key,
            web_style,
            map_css_weight_class(&web_emp),
            map_css_weight_class(&web_w)
        ));

        let (win_style, win_w, win_emp) = extract_platform_fields(val, "windows", "Body");
        let win_size = map_windows_font_size(&win_style);
        windows_cases.push_str(&format!(
            "                        case \"{}\":\n                            tb.FontSize = {};\n                            tb.FontWeight = emphasized ? {} : {};\n                            break;\n",
            key,
            win_size,
            map_windows_weight(&win_emp),
            map_windows_weight(&win_w)
        ));
    }

    let text_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/typography.json & config/colors.json
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;

pub struct Text {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}

impl Text {{
    pub fn on_delete<H, Args, S>(mut self, handler: H) -> Self
    where
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static,
    {{
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_delete".to_string(), PropValue::String(id)));
        self
    }}
    pub fn new() -> Self {{
        Self {{
            props: vec![],
            children: vec![],
        }}
    }}

    /// Valid options: {valid_fonts}
    pub fn font(mut self, style: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "style");
        self.props.push(("style".to_string(), PropValue::String(style.into())));
        self
    }}

    /// Applies the platform's emphasized weight for this text style.
    pub fn emphasized(mut self) -> Self {{
        self.props.retain(|(k, _)| k != "emphasized");
        self.props.push(("emphasized".to_string(), PropValue::Bool(true)));
        self
    }}

    /// Valid options: {valid_colors} (or any inline hex color like "#FF5733")
    pub fn color(mut self, color: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "color");
        self.props.push(("color".to_string(), PropValue::String(color.into())));
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        let pct = if opacity > 1.0 {{ opacity.clamp(0.0, 100.0) }} else {{ opacity.clamp(0.0, 1.0) * 100.0 }};
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }}
}}

impl IntoNode for Text {{
    fn into_node(self) -> Node {{
        Node::Element {{
            name: "text".to_string(),
            props: self.props,
            children: self.children,
        }}
    }}
}}

#[macro_export]
macro_rules! Text {{
    ($($arg:tt)*) => {{{{
        $crate::engine::primitives::Text {{
            props: vec![("label".to_string(), $crate::engine::ast::PropValue::String(format!($($arg)*).into()))],
            children: vec![],
        }}
    }}}}
}}

pub const TEXT_IOS_SNIPPET: &str = r#"case "text":
    let label = node.props["label"] as? String ?? ""
    let style = node.props["style"] as? String ?? "body"
    let emphasized = node.props["emphasized"] as? Bool ?? false
    let colorName = node.props["color"] as? String ?? ""
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let font: Font = {{
        switch style {{
{ios_cases}        default: return Font.body.weight(emphasized ? .semibold : .regular)
        }}
    }}()
    if let resolvedColor = resolveAppColor(colorName) {{
        Text(label)
            .font(font)
            .foregroundStyle(resolvedColor)
            .opacity(opacity)
    }} else {{
        Text(label)
            .font(font)
            .opacity(opacity)
    }}
"#;

pub const TEXT_ANDROID_SNIPPET: &str = r#"
"text" -> {{
    val label = node.getString("label") ?: ""
    val styleName = node.getString("style").ifEmpty {{ "body" }}
    val emphasized = node.getBoolean("emphasized", false)
    val colorName = node.getString("color", "")
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    val textStyle = when (styleName) {{
{android_cases}        else -> MaterialTheme.typography.bodyLarge.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
    }}
    val resolvedColor = resolveAppColor(colorName) ?: androidx.compose.ui.graphics.Color.Unspecified
    Text(
        text = label,
        style = textStyle,
        color = resolvedColor,
        modifier = if (opacity < 1f) Modifier.graphicsLayer(alpha = opacity) else Modifier
    )
}}
"#;

pub const TEXT_GTK_SNIPPET: &str = r#"
        "text" => {{
            let label_str = node.get_string("label", "");
            let style = node.get_string("style", "body");
            let emphasized = node.get_bool("emphasized", false);
            let color = node.get_string("color", "");
            let opacity = node.get_int("opacity", 100);
            let lbl = gtk4::Label::new(Some(&label_str));
            lbl.set_wrap(true);
            lbl.set_halign(gtk4::Align::Start);
            let (css_class, weight_class) = match style.as_str() {{
{gtk_cases}                _ => ("body", if emphasized {{ "weight-semibold" }} else {{ "weight-regular" }}),
            }};
            lbl.add_css_class(css_class);
            lbl.add_css_class(weight_class);
            apply_gtk_fg_color(&lbl, &color);
            if opacity < 100 {{
                lbl.set_opacity((opacity as f64) / 100.0);
            }}
            lbl.upcast::<gtk4::Widget>()
        }}
"#;

pub const TEXT_WEB_SNIPPET: &str = r#"
        case "text": {{
            const label = (node.props && node.props.label) || "";
            const style = (node.props && node.props.style) || "body";
            const emphasized = (node.props && node.props.emphasized) || false;
            const color = (node.props && node.props.color) || "";
            const opacity = (node.props && node.props.opacity !== undefined) ? (node.props.opacity / 100.0) : 1.0;
            const el = document.createElement("span");
            switch (style) {{
{web_cases}                default:
                    el.classList.add("body", emphasized ? "weight-semibold" : "weight-regular");
                    break;
            }}
            applyWebFgColor(el, color);
            if (opacity < 1.0) el.style.opacity = opacity.toString();
            el.textContent = label;
            return el;
        }}
"#;

pub const TEXT_WINDOWS_SNIPPET: &str = r#"
                case "text":
                    var tb = new TextBlock {{ TextWrapping = TextWrapping.Wrap }};
                    if (node.Props != null && node.Props.ContainsKey("label")) {{
                        tb.Text = node.Props["label"].ToString();
                    }}
                    string ts = (node.Props != null && node.Props.ContainsKey("style")) ? node.Props["style"].ToString() : "body";
                    bool emphasized = (node.Props != null && node.Props.ContainsKey("emphasized") && node.Props["emphasized"] is bool emp && emp);
                    switch (ts) {{
{windows_cases}                        default:
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                    }}
                    if (node.Props != null && node.Props.ContainsKey("color")) {{
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) tb.Foreground = fg;
                    }}
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double tbOp)) {{
                        tb.Opacity = tbOp / 100.0;
                    }}
                    return tb;
"#;
"###
    );
    write_if_changed(&primitives_dir.join("text.rs"), &text_rs);

    // =====================================================
    // Build Icon Primitive (icon.rs)
    // =====================================================
    let defaults = icons_obj.get("_defaults");
    let default_ios_variant = defaults
        .and_then(|d| d.get("ios"))
        .and_then(|i| i.get("variant"))
        .and_then(|v| v.as_str())
        .unwrap_or("none");
    let default_ios_rendering = defaults
        .and_then(|d| d.get("ios"))
        .and_then(|i| i.get("rendering"))
        .and_then(|v| v.as_str())
        .unwrap_or("monochrome");
    let default_android_style = defaults
        .and_then(|d| d.get("android"))
        .and_then(|a| a.get("style"))
        .and_then(|v| v.as_str())
        .unwrap_or("rounded");

    let mut ios_icon_cases = String::new();
    let mut android_icon_cases = String::new();
    let mut gtk_icon_cases = String::new();
    let mut web_icon_cases = String::new();
    let mut windows_icon_cases = String::new();

    for (key, val) in icons_obj {
        if key.starts_with('_') {
            continue;
        }
        if !is_valid_snake_case(key) {
            panic!(
                "Invalid icon key \"{}\" in config/icons.json: keys must be snake_case.",
                key
            );
        }

        let global_svg = if let Some(s) = val.as_str() {
            if is_svg_path(s) { Some(s) } else { None }
        } else {
            val.get("svg").and_then(|v| v.as_str())
        };

        if let Some(svg_path) = global_svg {
            ios_icon_cases.push_str(&format!(
                "        case \"{}\": return (true, \"{}\")\n",
                key, key
            ));
            android_icon_cases.push_str(&format!(
                "        \"{}\" -> Pair(true, \"ic_{}\")\n",
                key, key
            ));
            gtk_icon_cases.push_str(&format!(
                "                \"{}\" => (true, \"{}-symbolic.svg\"),\n",
                key, key
            ));
            let svg_raw = read_svg_file(&manifest_dir, svg_path).unwrap_or_else(|| {
                format!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">{}</svg>"#,
                    builtin_web_svg(key)
                )
            });
            let escaped_svg = svg_raw.replace('\\', "\\\\").replace('`', "\\`").replace("${", "\\${");
            web_icon_cases.push_str(&format!(
                "                case \"{}\": el.innerHTML = `{}`; break;\n",
                key, escaped_svg
            ));
            windows_icon_cases.push_str(&format!(
                "                        case \"{}\": glyph = \"\\uE734\"; break;\n",
                key
            ));
        } else {
            // iOS
            let ios_val = val.get("ios").and_then(|v| v.as_str());
            if let Some(_path) = ios_val.filter(|s| is_svg_path(s)) {
                ios_icon_cases.push_str(&format!(
                    "        case \"{}\": return (true, \"{}\")\n",
                    key, key
                ));
            } else {
                let ios_sym = ios_val.unwrap_or("circle");
                ios_icon_cases.push_str(&format!(
                    "        case \"{}\": return (false, \"{}\")\n",
                    key, ios_sym
                ));
            }

            // Android
            let and_val = val.get("android").and_then(|v| v.as_str());
            if let Some(_path) = and_val.filter(|s| is_svg_path(s)) {
                android_icon_cases.push_str(&format!(
                    "        \"{}\" -> Pair(true, \"ic_{}\")\n",
                    key, key
                ));
            } else {
                let and_sym = and_val.unwrap_or("Star");
                let and_sym_mapped = if and_sym == "qr_code" { "QrCode" } else { and_sym };
                android_icon_cases.push_str(&format!(
                    "        \"{}\" -> Pair(false, \"{}\")\n",
                    key, and_sym_mapped
                ));
            }

            // Linux GTK4
            let lin_val = val.get("linux").and_then(|v| v.as_str());
            if let Some(_path) = lin_val.filter(|s| is_svg_path(s)) {
                gtk_icon_cases.push_str(&format!(
                    "                \"{}\" => (true, \"{}-symbolic.svg\"),\n",
                    key, key
                ));
            } else {
                let lin_sym = lin_val.unwrap_or("starred-symbolic");
                gtk_icon_cases.push_str(&format!(
                    "                \"{}\" => (false, \"{}\"),\n",
                    key, lin_sym
                ));
            }

            // Web
            let web_val = val.get("web").and_then(|v| v.as_str());
            if let Some(path) = web_val.filter(|s| is_svg_path(s)) {
                let svg_raw = read_svg_file(&manifest_dir, path).unwrap_or_else(|| {
                    format!(
                        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">{}</svg>"#,
                        builtin_web_svg(key)
                    )
                });
                let escaped_svg = svg_raw.replace('\\', "\\\\").replace('`', "\\`").replace("${", "\\${");
                web_icon_cases.push_str(&format!(
                    "                case \"{}\": el.innerHTML = `{}`; break;\n",
                    key, escaped_svg
                ));
            } else {
                let web_name = web_val.unwrap_or(key);
                let inner_svg = builtin_web_svg(web_name);
                web_icon_cases.push_str(&format!(
                    "                case \"{}\": el.innerHTML = `<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"${{size}}\" height=\"${{size}}\" viewBox=\"0 0 24 24\" fill=\"none\">{}</svg>`; break;\n",
                    key, inner_svg
                ));
            }

            // Windows
            let win_val = val.get("windows").and_then(|v| v.as_str());
            if let Some(_path) = win_val.filter(|s| is_svg_path(s)) {
                windows_icon_cases.push_str(&format!(
                    "                        case \"{}\": glyph = \"\\uE734\"; break;\n",
                    key
                ));
            } else {
                let win_glyph = win_val.unwrap_or("\u{E734}");
                let escaped_win: String = win_glyph
                    .chars()
                    .map(|c| format!("\\u{:04X}", c as u32))
                    .collect();
                windows_icon_cases.push_str(&format!(
                    "                        case \"{}\": glyph = \"{}\"; break;\n",
                    key, escaped_win
                ));
            }
        }
    }

    let icon_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/icons.json & config/colors.json
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;

pub struct Icon {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}

impl Icon {{
    /// Valid options: {valid_icons}
    pub fn new(name: impl Into<String>) -> Self {{
        Self {{
            props: vec![("name".to_string(), PropValue::String(name.into()))],
            children: vec![],
        }}
    }}

    /// Valid options: {valid_colors}
    pub fn color(mut self, color: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "color");
        self.props.push(("color".to_string(), PropValue::String(color.into())));
        self
    }}

    pub fn size(mut self, size: u32) -> Self {{
        self.props.retain(|(k, _)| k != "size");
        self.props.push(("size".to_string(), PropValue::Int(size)));
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        let pct = if opacity > 1.0 {{ opacity.clamp(0.0, 100.0) }} else {{ opacity.clamp(0.0, 1.0) * 100.0 }};
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }}

    /// Valid options: none, fill, circle, circle.fill, square, square.fill, slash, rounded, outlined, sharp, twotone
    pub fn variant(mut self, variant: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "variant");
        self.props.push(("variant".to_string(), PropValue::String(variant.into())));
        self
    }}

    /// Valid options: monochrome, hierarchical, palette, multicolor
    pub fn rendering(mut self, rendering: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "rendering");
        self.props.push(("rendering".to_string(), PropValue::String(rendering.into())));
        self
    }}
}}

impl IntoNode for Icon {{
    fn into_node(mut self) -> Node {{
        #[cfg(not(target_arch = "wasm32"))]
        {{
            let key = self.props.iter().find_map(|(k, v)| if k == "name" {{ if let PropValue::String(s) = v {{ Some(s.clone()) }} else {{ None }} }} else {{ None }}).unwrap_or_default();
            if !key.is_empty() {{
                for p in ["demo_app/config/icons.json", "config/icons.json"] {{
                    if let Ok(txt) = std::fs::read_to_string(p) {{
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {{
                            if let Some(entry) = json.get(&key) {{
                                if let Some(s) = entry.get("ios").and_then(|v| v.as_str()) {{
                                    self.props.push(("ios_symbol".to_string(), PropValue::String(s.to_string())));
                                }}
                                if let Some(s) = entry.get("android").and_then(|v| v.as_str()) {{
                                    self.props.push(("android_icon".to_string(), PropValue::String(s.to_string())));
                                }}
                                if let Some(s) = entry.get("linux").and_then(|v| v.as_str()) {{
                                    self.props.push(("linux_icon".to_string(), PropValue::String(s.to_string())));
                                }}
                                if let Some(s) = entry.get("windows").and_then(|v| v.as_str()) {{
                                    self.props.push(("windows_glyph".to_string(), PropValue::String(s.to_string())));
                                }}
                                if let Some(s) = entry.get("web").and_then(|v| v.as_str()) {{
                                    self.props.push(("web_icon".to_string(), PropValue::String(s.to_string())));
                                }}
                            }}
                        }}
                        break;
                    }}
                }}
            }}
        }}
        Node::Element {{
            name: "icon".to_string(),
            props: self.props,
            children: self.children,
        }}
    }}
}}

#[macro_export]
macro_rules! Icon {{
    ($name:expr) => {{{{
        $crate::engine::primitives::Icon::new($name)
    }}}}
}}

pub const ICON_IOS_SNIPPET: &str = r#"case "icon":
    let iconKey = node.props["name"] as? String ?? "sparkles"
    let colorName = node.props["color"] as? String ?? ""
    let size = CGFloat(node.props["size"] as? Int ?? 20)
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let variant = node.props["variant"] as? String ?? "{default_ios_variant}"
    let rendering = node.props["rendering"] as? String ?? "{default_ios_rendering}"
    let liveIos = node.props["ios_symbol"] as? String ?? ""
    let (isCustomAsset, baseSymbol): (Bool, String) = !liveIos.isEmpty ? (false, liveIos) : {{
        switch iconKey {{
{ios_icon_cases}        default: return (false, iconKey)
        }}
    }}()
    let resolvedColor = resolveAppColor(colorName)
    if isCustomAsset {{
        Image(baseSymbol)
            .renderingMode(.template)
            .resizable()
            .scaledToFit()
            .frame(width: size, height: size)
            .foregroundStyle(resolvedColor ?? Color.primary)
            .opacity(opacity)
    }} else {{
        let finalSymbol: String = {{
            if variant.isEmpty || variant == "none" || variant == "default" || variant == "outlined" || variant == "rounded" {{
                return baseSymbol
            }}
            let suffix = ".\(variant)"
            return baseSymbol.hasSuffix(suffix) ? baseSymbol : "\(baseSymbol)\(suffix)"
        }}()
        let mode: SymbolRenderingMode = {{
            switch rendering {{
            case "hierarchical": return .hierarchical
            case "palette": return .palette
            case "multicolor": return .multicolor
            default: return .monochrome
            }}
        }}()
        if let resolvedColor = resolvedColor {{
            Image(systemName: finalSymbol)
                .symbolRenderingMode(mode)
                .font(.system(size: size))
                .foregroundStyle(resolvedColor)
                .opacity(opacity)
        }} else {{
            Image(systemName: finalSymbol)
                .symbolRenderingMode(mode)
                .font(.system(size: size))
                .opacity(opacity)
        }}
    }}
"#;

pub const ICON_ANDROID_SNIPPET: &str = r#"
"icon" -> {{
    val iconKey = node.getString("name", "sparkles")
    val colorName = node.getString("color", "")
    val size = node.getInt("size", 24)
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    val liveAnd = node.getString("android_icon", "")
    val (isCustomDrawable, mappedName) = if (liveAnd.isNotEmpty()) Pair(false, liveAnd) else when (iconKey) {{
{android_icon_cases}        else -> Pair(false, "Star")
    }}
    val tintColor = resolveAppColor(colorName) ?: LocalContentColor.current
    val iconMod = Modifier.size(size.dp).let {{ if (opacity < 1f) it.graphicsLayer(alpha = opacity) else it }}
    if (isCustomDrawable) {{
        val ctx = androidx.compose.ui.platform.LocalContext.current
        val resId = ctx.resources.getIdentifier(mappedName, "drawable", ctx.packageName)
        if (resId != 0) {{
            Icon(
                painter = androidx.compose.ui.res.painterResource(id = resId),
                contentDescription = iconKey,
                tint = tintColor,
                modifier = iconMod
            )
        }}
    }} else {{
        val style = node.getString("variant", "{default_android_style}").lowercase()
        val vec = when (mappedName) {{
            "ArrowForward" -> AppIcons.ArrowForward
            "ArrowBack" -> AppIcons.ArrowBack
            "KeyboardArrowRight" -> AppIcons.KeyboardArrowRight
            "KeyboardArrowLeft" -> AppIcons.KeyboardArrowLeft
            "Home" -> AppIcons.Home
            "ShoppingCart" -> AppIcons.ShoppingCart
            "Settings" -> AppIcons.Settings
            "Person" -> AppIcons.Person
            "Search" -> AppIcons.Search
            "Add" -> AppIcons.Add
            "Check" -> AppIcons.Check
            "CheckCircle" -> AppIcons.CheckCircle
            "Close" -> AppIcons.Close
            "Delete" -> AppIcons.Delete
            "Edit" -> AppIcons.Edit
            "Favorite" -> AppIcons.Favorite
            "Notifications" -> AppIcons.Notifications
            "Info" -> AppIcons.Info
            "Warning" -> AppIcons.Warning
            "Email" -> AppIcons.Email
            "Lock" -> AppIcons.Lock
            "Share" -> AppIcons.Share
            "Refresh" -> AppIcons.Refresh
            "Build" -> AppIcons.Build
            "Coffee" -> AppIcons.Coffee
            "QrCode", "qr_code" -> AppIcons.QrCode
            else -> AppIcons.Star
        }}
        Icon(
            imageVector = vec,
            contentDescription = iconKey,
            tint = tintColor,
            modifier = iconMod
        )
    }}
}}
"#;

pub const ICON_GTK_SNIPPET: &str = r#"
        "icon" => {{
            let icon_key = node.get_string("name", "sparkles");
            let color = node.get_string("color", "");
            let size = node.get_int("size", 20);
            let opacity = node.get_int("opacity", 100);
            let live_lin = node.get_string("linux_icon", "");
            let (is_custom, mapped) = if !live_lin.is_empty() {{ (false, live_lin.as_str()) }} else {{ match icon_key.as_str() {{
{gtk_icon_cases}                _ => (false, "starred-symbolic"),
            }} }};
            let img = if is_custom {{
                let stem = mapped.trim_end_matches(".svg");
                let sym_name = if stem.ends_with("-symbolic") {{ stem.to_string() }} else {{ format!("{{}}-symbolic", stem) }};
                let p1 = std::path::PathBuf::from(format!("icons/{{}}.svg", sym_name));
                let p2 = std::path::PathBuf::from(format!("demo_app/platforms/Linux/icons/{{}}.svg", sym_name));
                let p3 = std::path::PathBuf::from(format!("icons/{{}}", mapped));
                let p4 = std::path::PathBuf::from(format!("demo_app/platforms/Linux/icons/{{}}", mapped));
                let found = if p1.exists() {{ Some(p1) }} else if p2.exists() {{ Some(p2) }} else if p3.exists() {{ Some(p3) }} else if p4.exists() {{ Some(p4) }} else {{ None }};
                if let Some(path) = found {{
                    let file = gtk4::gio::File::for_path(&path);
                    let gicon = gtk4::gio::FileIcon::new(&file);
                    gtk4::Image::from_gicon(&gicon)
                }} else {{
                    gtk4::Image::from_icon_name(&sym_name)
                }}
            }} else {{
                gtk4::Image::from_icon_name(mapped)
            }};
            img.set_pixel_size(size as i32);
            apply_gtk_fg_color(&img, &color);
            if opacity < 100 {{
                img.set_opacity((opacity as f64) / 100.0);
            }}
            img.upcast::<gtk4::Widget>()
        }}
"#;

pub const ICON_WEB_SNIPPET: &str = r#"
        case "icon": {{
            const iconKey = (node.props && (node.props.web_icon || node.props.name)) || "sparkles";
            const color = (node.props && node.props.color) || "";
            const size = (node.props && node.props.size) || 20;
            const opacity = (node.props && node.props.opacity !== undefined) ? (node.props.opacity / 100.0) : 1.0;
            const el = document.createElement("span");
            el.classList.add("nk-icon");
            el.style.display = "inline-flex";
            el.style.alignItems = "center";
            el.style.justifyContent = "center";
            el.style.width = size + "px";
            el.style.height = size + "px";
            applyWebFgColor(el, color);
            if (opacity < 1.0) el.style.opacity = opacity.toString();
            switch (iconKey) {{
{web_icon_cases}                default:
                    el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${{size}}" height="${{size}}" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="8"/></svg>`;
                    break;
            }}
            const svgEl = el.querySelector("svg");
            if (svgEl) {{
                svgEl.setAttribute("width", size);
                svgEl.setAttribute("height", size);
            }}
            return el;
        }}
"#;

pub const ICON_WINDOWS_SNIPPET: &str = r#"
                case "icon":
                    string iconKey = (node.Props != null && node.Props.ContainsKey("name")) ? node.Props["name"].ToString() : "sparkles";
                    double iconSize = (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double iSz)) ? iSz : 20.0;
                    string glyph = (node.Props != null && node.Props.ContainsKey("windows_glyph")) ? node.Props["windows_glyph"].ToString() : "";
                    if (string.IsNullOrEmpty(glyph)) switch (iconKey) {{
{windows_icon_cases}                        default: glyph = "\uE734"; break;
                    }}
                    var fi = new FontIcon {{ Glyph = glyph, FontSize = iconSize }};
                    if (node.Props != null && node.Props.ContainsKey("color")) {{
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) fi.Foreground = fg;
                    }}
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double fiOp)) {{
                        fi.Opacity = fiOp / 100.0;
                    }}
                    return fi;
"#;
"###
    );
    write_if_changed(&primitives_dir.join("icon.rs"), &icon_rs);

    // =====================================================
    // Build VStack, HStack, and Button Primitives from Blueprints
    // =====================================================
    let blueprints_dir = manifest_dir.join("../generator/blueprints");
    let extract_fn_body = |code: &str| -> String {
        if let (Some(s), Some(e)) = (code.find('{'), code.rfind('}')) {
            if s < e {
                return code[s + 1..e].trim().to_string();
            }
        }
        code.trim().to_string()
    };
    let read_blueprint_snippet = |tag: &str, platform: &str, filename: &str| -> String {
        let p = blueprints_dir.join(tag).join(filename);
        println!("cargo:rerun-if-changed={}", p.display());
        let content = fs::read_to_string(&p).unwrap_or_default();
        let body = extract_fn_body(&content);
        let indented = body.replace('\n', "\n    ");
        match platform {
            "ios" => format!("case \"{}\":\n    {}\n", tag, indented),
            "android" => format!("\"{}\" -> {{\n    {}\n}}\n", tag, indented),
            "linux" => format!("\"{}\" => {{\n    {}\n}}\n", tag, indented),
            "web" => format!("case \"{}\": {{\n    {}\n}}\n", tag, indented),
            "windows" => format!("case \"{}\": {{\n    {}\n}}\n", tag, indented),
            _ => body,
        }
    };

    let vstack_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/vstack
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;
use crate::engine::primitives::color::{{BackgroundValue, IntoBackground}};

pub struct VStack {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}

impl VStack {{
    pub fn new() -> Self {{
        Self {{
            props: vec![],
            children: vec![],
        }}
    }}

    pub fn spacing(mut self, spacing: u32) -> Self {{
        self.props.retain(|(k, _)| k != "spacing");
        self.props.push(("spacing".to_string(), PropValue::Int(spacing)));
        self
    }}

    /// Valid options: hug, fill
    pub fn width(mut self, width: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "width");
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
    }}

    /// Valid options: top, bottom, center, middle, left, right, space_between (or 2-arg `.align(.top, .left)`)
    pub fn align(mut self, align: impl Into<String>) -> Self {{
        let a = align.into();
        let existing_x = self.props.iter().find(|(k, _)| k == "align_x").and_then(|(_, v)| if let PropValue::String(s) = v {{ Some(s.clone()) }} else {{ None }});
        let existing_y = self.props.iter().find(|(k, _)| k == "align_y").and_then(|(_, v)| if let PropValue::String(s) = v {{ Some(s.clone()) }} else {{ None }});
        let (nx, ny): (Option<&str>, Option<&str>) = match a.as_str() {{
            "top" => (None, Some("top")),
            "bottom" => (None, Some("bottom")),
            "middle" => (None, Some("center")),
            "left" | "leading" => (Some("left"), None),
            "right" | "trailing" => (Some("right"), None),
            "space_between" => (None, Some("space_between")),
            "center" => {{
                if matches!(existing_y.as_deref(), Some("top" | "bottom" | "space_between")) && existing_x.is_none() {{
                    (Some("center"), None)
                }} else if matches!(existing_x.as_deref(), Some("left" | "right")) && existing_y.is_none() {{
                    (None, Some("center"))
                }} else {{
                    (Some("center"), Some("center"))
                }}
            }}
            "top_left" => (Some("left"), Some("top")),
            "top_center" => (Some("center"), Some("top")),
            "top_right" => (Some("right"), Some("top")),
            "center_left" | "middle_left" => (Some("left"), Some("center")),
            "center_right" | "middle_right" => (Some("right"), Some("center")),
            "bottom_left" => (Some("left"), Some("bottom")),
            "bottom_center" => (Some("center"), Some("bottom")),
            "bottom_right" => (Some("right"), Some("bottom")),
            _ => (None, None),
        }};
        if let Some(x) = nx {{
            self.props.retain(|(k, _)| k != "align_x");
            self.props.push(("align_x".to_string(), PropValue::String(x.to_string())));
        }}
        if let Some(y) = ny {{
            self.props.retain(|(k, _)| k != "align_y");
            self.props.push(("align_y".to_string(), PropValue::String(y.to_string())));
        }}
        self
    }}

    /// Valid options: {valid_colors} (or an inline hex color like "#1E293B", or `Gradient!(...)`)
    pub fn background(mut self, bg: impl IntoBackground) -> Self {{
        self.props.retain(|(k, _)| k != "background" && k != "background_gradient");
        match bg.into_background() {{
            BackgroundValue::Solid(c) => {{
                self.props.push(("background".to_string(), PropValue::String(c)));
            }}
            BackgroundValue::Gradient(g) => {{
                self.props.push(("background_gradient".to_string(), PropValue::Json(g.to_json_value())));
            }}
        }}
        self
    }}

    pub fn background_image(mut self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {{
        self.props.retain(|(k, _)| k != "background_image");
        self.props.push(("background_image".to_string(), PropValue::String(img.into_bg_image_url())));
        self
    }}

    #[allow(non_snake_case)]
    pub fn backgroundImage(self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {{
        self.background_image(img)
    }}

    pub fn padding(mut self, padding: u32) -> Self {{
        self.props.retain(|(k, _)| k != "padding");
        self.props.push(("padding".to_string(), PropValue::Int(padding)));
        self
    }}

    pub fn corner_radius(mut self, radius: u32) -> Self {{
        self.props.retain(|(k, _)| k != "corner_radius");
        self.props.push(("corner_radius".to_string(), PropValue::Int(radius)));
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        let pct = if opacity > 1.0 {{ opacity.clamp(0.0, 100.0) }} else {{ opacity.clamp(0.0, 1.0) * 100.0 }};
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }}

    pub fn child(mut self, child: impl IntoNode) -> Self {{
        self.children.push(child.into_node());
        self
    }}

    pub fn children(mut self, children: impl IntoChildren) -> Self {{
        self.children.extend(children.into_children());
        self
    }}
}}

impl IntoNode for VStack {{
    fn into_node(self) -> Node {{
        Node::Element {{
            name: "vstack".to_string(),
            props: self.props,
            children: self.children,
        }}
    }}
}}

pub const VSTACK_ANDROID_SNIPPET: &str = r#"
{vstack_android}"#;

pub const VSTACK_IOS_SNIPPET: &str = r#"
{vstack_ios}"#;

pub const VSTACK_GTK_SNIPPET: &str = r#"
{vstack_gtk}"#;

pub const VSTACK_WEB_SNIPPET: &str = r#"
{vstack_web}"#;

pub const VSTACK_WINDOWS_SNIPPET: &str = r#"
{vstack_win}"#;
"###,
        valid_colors = valid_colors,
        vstack_android = read_blueprint_snippet("vstack", "android", "android.kt"),
        vstack_ios = read_blueprint_snippet("vstack", "ios", "ios.swift"),
        vstack_gtk = read_blueprint_snippet("vstack", "linux", "linux.rs"),
        vstack_web = read_blueprint_snippet("vstack", "web", "web.js"),
        vstack_win = read_blueprint_snippet("vstack", "windows", "windows.cs"),
    );
    write_if_changed(&primitives_dir.join("vstack.rs"), &vstack_rs);

    let hstack_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/hstack
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;
use crate::engine::primitives::color::{{BackgroundValue, IntoBackground}};

pub struct HStack {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}

impl HStack {{
    pub fn on_delete<H, Args, S>(mut self, handler: H) -> Self
    where
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static,
    {{
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_delete".to_string(), PropValue::String(id)));
        self
    }}

    pub fn new() -> Self {{
        Self {{
            props: vec![],
            children: vec![],
        }}
    }}

    pub fn spacing(mut self, spacing: u32) -> Self {{
        self.props.retain(|(k, _)| k != "spacing");
        self.props.push(("spacing".to_string(), PropValue::Int(spacing)));
        self
    }}

    /// Valid options: hug, fill
    pub fn width(mut self, width: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "width");
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
    }}

    /// Valid options: top, bottom, center, middle, left, right, space_between (or 2-arg `.align(.top, .left)`)
    pub fn align(mut self, align: impl Into<String>) -> Self {{
        let a = align.into();
        let existing_x = self.props.iter().find(|(k, _)| k == "align_x").and_then(|(_, v)| if let PropValue::String(s) = v {{ Some(s.clone()) }} else {{ None }});
        let existing_y = self.props.iter().find(|(k, _)| k == "align_y").and_then(|(_, v)| if let PropValue::String(s) = v {{ Some(s.clone()) }} else {{ None }});
        let (nx, ny): (Option<&str>, Option<&str>) = match a.as_str() {{
            "top" => (None, Some("top")),
            "bottom" => (None, Some("bottom")),
            "middle" => (None, Some("center")),
            "left" | "leading" => (Some("left"), None),
            "right" | "trailing" => (Some("right"), None),
            "space_between" => (Some("space_between"), None),
            "center" => {{
                if matches!(existing_y.as_deref(), Some("top" | "bottom")) && existing_x.is_none() {{
                    (Some("center"), None)
                }} else if matches!(existing_x.as_deref(), Some("left" | "right" | "space_between")) && existing_y.is_none() {{
                    (None, Some("center"))
                }} else {{
                    (Some("center"), Some("center"))
                }}
            }}
            "top_left" => (Some("left"), Some("top")),
            "top_center" => (Some("center"), Some("top")),
            "top_right" => (Some("right"), Some("top")),
            "center_left" | "middle_left" => (Some("left"), Some("center")),
            "center_right" | "middle_right" => (Some("right"), Some("center")),
            "bottom_left" => (Some("left"), Some("bottom")),
            "bottom_center" => (Some("center"), Some("bottom")),
            "bottom_right" => (Some("right"), Some("bottom")),
            _ => (None, None),
        }};
        if let Some(x) = nx {{
            self.props.retain(|(k, _)| k != "align_x");
            self.props.push(("align_x".to_string(), PropValue::String(x.to_string())));
        }}
        if let Some(y) = ny {{
            self.props.retain(|(k, _)| k != "align_y");
            self.props.push(("align_y".to_string(), PropValue::String(y.to_string())));
        }}
        self
    }}

    pub fn wrap(mut self, wrap: bool) -> Self {{
        self.props.retain(|(k, _)| k != "wrap");
        self.props.push(("wrap".to_string(), PropValue::Bool(wrap)));
        self
    }}

    /// Valid options: {valid_colors} (or an inline hex color like "#1E293B", or `Gradient!(...)`)
    pub fn background(mut self, bg: impl IntoBackground) -> Self {{
        self.props.retain(|(k, _)| k != "background" && k != "background_gradient");
        match bg.into_background() {{
            BackgroundValue::Solid(c) => {{
                self.props.push(("background".to_string(), PropValue::String(c)));
            }}
            BackgroundValue::Gradient(g) => {{
                self.props.push(("background_gradient".to_string(), PropValue::Json(g.to_json_value())));
            }}
        }}
        self
    }}

    pub fn background_image(mut self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {{
        self.props.retain(|(k, _)| k != "background_image");
        self.props.push(("background_image".to_string(), PropValue::String(img.into_bg_image_url())));
        self
    }}

    #[allow(non_snake_case)]
    pub fn backgroundImage(self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {{
        self.background_image(img)
    }}

    pub fn padding(mut self, padding: u32) -> Self {{
        self.props.retain(|(k, _)| k != "padding");
        self.props.push(("padding".to_string(), PropValue::Int(padding)));
        self
    }}

    pub fn corner_radius(mut self, radius: u32) -> Self {{
        self.props.retain(|(k, _)| k != "corner_radius");
        self.props.push(("corner_radius".to_string(), PropValue::Int(radius)));
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        let pct = if opacity > 1.0 {{ opacity.clamp(0.0, 100.0) }} else {{ opacity.clamp(0.0, 1.0) * 100.0 }};
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }}

    pub fn child(mut self, child: impl IntoNode) -> Self {{
        self.children.push(child.into_node());
        self
    }}

    pub fn children(mut self, children: impl IntoChildren) -> Self {{
        self.children.extend(children.into_children());
        self
    }}
}}

impl IntoNode for HStack {{
    fn into_node(self) -> Node {{
        Node::Element {{
            name: "hstack".to_string(),
            props: self.props,
            children: self.children,
        }}
    }}
}}

pub const HSTACK_ANDROID_SNIPPET: &str = r#"
{hstack_android}"#;

pub const HSTACK_IOS_SNIPPET: &str = r#"
{hstack_ios}"#;

pub const HSTACK_GTK_SNIPPET: &str = r#"
{hstack_gtk}"#;

pub const HSTACK_WEB_SNIPPET: &str = r#"
{hstack_web}"#;

pub const HSTACK_WINDOWS_SNIPPET: &str = r#"
{hstack_win}"#;
"###,
        valid_colors = valid_colors,
        hstack_android = read_blueprint_snippet("hstack", "android", "android.kt"),
        hstack_ios = read_blueprint_snippet("hstack", "ios", "ios.swift"),
        hstack_gtk = read_blueprint_snippet("hstack", "linux", "linux.rs"),
        hstack_web = read_blueprint_snippet("hstack", "web", "web.js"),
        hstack_win = read_blueprint_snippet("hstack", "windows", "windows.cs"),
    );
    write_if_changed(&primitives_dir.join("hstack.rs"), &hstack_rs);

    let button_rs = format!(
        r###"// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/button
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;

pub struct Button {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}

impl Button {{
    pub fn new() -> Self {{
        Self {{
            props: vec![],
            children: vec![],
        }}
    }}

    /// Valid options: primary, secondary, text
    pub fn variant(mut self, variant: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "variant");
        self.props.push(("variant".to_string(), PropValue::String(variant.into())));
        self
    }}

    /// Valid options: {valid_colors} (or any inline hex color like "#FF5733")
    pub fn color(mut self, color: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "color");
        self.props.push(("color".to_string(), PropValue::String(color.into())));
        self
    }}

    pub fn opacity(mut self, opacity: f32) -> Self {{
        let pct = if opacity > 1.0 {{ opacity.clamp(0.0, 100.0) }} else {{ opacity.clamp(0.0, 1.0) * 100.0 }};
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }}

    /// Valid options: "fill", "hug"
    pub fn placement(mut self, placement: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "placement");
        self.props.push(("placement".to_string(), PropValue::String(placement.into())));
        self
    }}

    pub fn width(mut self, width: impl Into<String>) -> Self {{
        self.props.retain(|(k, _)| k != "width");
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
    }}

    pub fn action<H, Args, S>(mut self, handler: H) -> Self
    where
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static,
    {{
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("action".to_string(), PropValue::String(id)));
        self
    }}

    pub fn child(mut self, child: impl IntoNode) -> Self {{
        self.children.push(child.into_node());
        self
    }}

    pub fn children(mut self, children: impl IntoChildren) -> Self {{
        self.children.extend(children.into_children());
        self
    }}
}}

impl IntoNode for Button {{
    fn into_node(self) -> Node {{
        Node::Element {{
            name: "button".to_string(),
            props: self.props,
            children: self.children,
        }}
    }}
}}

pub const BUTTON_ANDROID_SNIPPET: &str = r#"
{button_android}"#;

pub const BUTTON_IOS_SNIPPET: &str = r#"
{button_ios}"#;

pub const BUTTON_GTK_SNIPPET: &str = r#"
{button_gtk}"#;

pub const BUTTON_WEB_SNIPPET: &str = r#"
{button_web}"#;

pub const BUTTON_WINDOWS_SNIPPET: &str = r#"
{button_win}"#;
"###,
        valid_colors = valid_colors,
        button_android = read_blueprint_snippet("button", "android", "android.kt"),
        button_ios = read_blueprint_snippet("button", "ios", "ios.swift"),
        button_gtk = read_blueprint_snippet("button", "linux", "linux.rs"),
        button_web = read_blueprint_snippet("button", "web", "web.js"),
        button_win = read_blueprint_snippet("button", "windows", "windows.cs"),
    );
    write_if_changed(&primitives_dir.join("button.rs"), &button_rs);

    let mod_rs = r#"pub mod color;
pub use color::*;
pub mod icon;
pub use icon::*;
pub mod page;
pub use page::*;
pub mod text;
pub use text::*;
pub mod button;
pub use button::*;
pub mod navigationstack;
pub use navigationstack::*;
pub mod hstack;
pub use hstack::*;
pub mod switch;
pub use switch::*;
pub mod vstack;
pub use vstack::*;
pub mod textinput;
pub use textinput::*;
pub mod image;
pub use image::*;
pub mod toolbartop;
pub use toolbartop::*;
pub mod section;
pub use section::*;
pub mod grid;
pub use grid::*;
pub mod spacer;
pub use spacer::*;
"#;
    write_if_changed(&primitives_dir.join("mod.rs"), mod_rs);
}
