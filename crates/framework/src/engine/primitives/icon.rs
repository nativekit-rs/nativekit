// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/icons.json & config/colors.json
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Icon {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}

impl Icon {
    /// Valid options: arrow_left, arrow_right, bell, bolt, brand_spark, cart, check, check_circle, chevron_left, chevron_right, close, coffee, edit, heart, home, info, lock, mail, navigate_back, orders, plus, qrcode, refresh, search, settings, share, sparkles, star, trash, user, warning
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            props: vec![("name".to_string(), PropValue::String(name.into()))],
            children: vec![],
        }
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "color");
        self.props.push(("color".to_string(), PropValue::String(color.into())));
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.props.retain(|(k, _)| k != "size");
        self.props.push(("size".to_string(), PropValue::Int(size)));
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        let pct = if opacity > 1.0 { opacity.clamp(0.0, 100.0) } else { opacity.clamp(0.0, 1.0) * 100.0 };
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }

    /// Valid options: none, fill, circle, circle.fill, square, square.fill, slash, rounded, outlined, sharp, twotone
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "variant");
        self.props.push(("variant".to_string(), PropValue::String(variant.into())));
        self
    }

    /// Valid options: monochrome, hierarchical, palette, multicolor
    pub fn rendering(mut self, rendering: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "rendering");
        self.props.push(("rendering".to_string(), PropValue::String(rendering.into())));
        self
    }
}

impl IntoNode for Icon {
    fn into_node(mut self) -> Node {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let key = self.props.iter().find_map(|(k, v)| if k == "name" { if let PropValue::String(s) = v { Some(s.clone()) } else { None } } else { None }).unwrap_or_default();
            if !key.is_empty() {
                for p in ["demo_app/config/icons.json", "config/icons.json"] {
                    if let Ok(txt) = std::fs::read_to_string(p) {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {
                            if let Some(entry) = json.get(&key) {
                                if let Some(s) = entry.get("ios").and_then(|v| v.as_str()) {
                                    self.props.push(("ios_symbol".to_string(), PropValue::String(s.to_string())));
                                }
                                if let Some(s) = entry.get("android").and_then(|v| v.as_str()) {
                                    self.props.push(("android_icon".to_string(), PropValue::String(s.to_string())));
                                }
                                if let Some(s) = entry.get("linux").and_then(|v| v.as_str()) {
                                    self.props.push(("linux_icon".to_string(), PropValue::String(s.to_string())));
                                }
                                if let Some(s) = entry.get("windows").and_then(|v| v.as_str()) {
                                    self.props.push(("windows_glyph".to_string(), PropValue::String(s.to_string())));
                                }
                                if let Some(s) = entry.get("web").and_then(|v| v.as_str()) {
                                    self.props.push(("web_icon".to_string(), PropValue::String(s.to_string())));
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        Node::Element {
            name: "icon".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! Icon {
    ($name:expr) => {{
        $crate::engine::primitives::Icon::new($name)
    }}
}

pub const ICON_IOS_SNIPPET: &str = r#"case "icon":
    let iconKey = node.props["name"] as? String ?? "sparkles"
    let colorName = node.props["color"] as? String ?? ""
    let size = CGFloat(node.props["size"] as? Int ?? 20)
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let variant = node.props["variant"] as? String ?? "none"
    let rendering = node.props["rendering"] as? String ?? "monochrome"
    let liveIos = node.props["ios_symbol"] as? String ?? ""
    let (isCustomAsset, baseSymbol): (Bool, String) = !liveIos.isEmpty ? (false, liveIos) : {
        switch iconKey {
        case "arrow_left": return (false, "arrow.left")
        case "arrow_right": return (false, "arrow.right")
        case "bell": return (false, "bell")
        case "bolt": return (false, "bolt")
        case "brand_spark": return (true, "brand_spark")
        case "cart": return (false, "cart")
        case "check": return (false, "checkmark")
        case "check_circle": return (false, "checkmark.circle")
        case "chevron_left": return (false, "chevron.left")
        case "chevron_right": return (false, "chevron.right")
        case "close": return (false, "xmark")
        case "coffee": return (false, "mug")
        case "edit": return (false, "pencil")
        case "heart": return (false, "heart")
        case "home": return (false, "house")
        case "info": return (false, "info.circle")
        case "lock": return (false, "lock")
        case "mail": return (false, "envelope")
        case "navigate_back": return (false, "chevron.left")
        case "orders": return (false, "shippingbox")
        case "plus": return (false, "plus")
        case "qrcode": return (false, "qrcode")
        case "refresh": return (false, "arrow.clockwise")
        case "search": return (false, "magnifyingglass")
        case "settings": return (false, "gearshape")
        case "share": return (false, "square.and.arrow.up")
        case "sparkles": return (false, "sparkles")
        case "star": return (false, "star")
        case "trash": return (false, "trash")
        case "user": return (false, "person")
        case "warning": return (false, "exclamationmark.triangle")
        default: return (false, iconKey)
        }
    }()
    let resolvedColor = resolveAppColor(colorName)
    if isCustomAsset {
        Image(baseSymbol)
            .renderingMode(.template)
            .resizable()
            .scaledToFit()
            .frame(width: size, height: size)
            .foregroundStyle(resolvedColor ?? Color.primary)
            .opacity(opacity)
    } else {
        let finalSymbol: String = {
            if variant.isEmpty || variant == "none" || variant == "default" || variant == "outlined" || variant == "rounded" {
                return baseSymbol
            }
            let suffix = ".\(variant)"
            return baseSymbol.hasSuffix(suffix) ? baseSymbol : "\(baseSymbol)\(suffix)"
        }()
        let mode: SymbolRenderingMode = {
            switch rendering {
            case "hierarchical": return .hierarchical
            case "palette": return .palette
            case "multicolor": return .multicolor
            default: return .monochrome
            }
        }()
        if let resolvedColor = resolvedColor {
            Image(systemName: finalSymbol)
                .symbolRenderingMode(mode)
                .font(.system(size: size))
                .foregroundStyle(resolvedColor)
                .opacity(opacity)
        } else {
            Image(systemName: finalSymbol)
                .symbolRenderingMode(mode)
                .font(.system(size: size))
                .opacity(opacity)
        }
    }
"#;

pub const ICON_ANDROID_SNIPPET: &str = r#"
"icon" -> {
    val iconKey = node.getString("name", "sparkles")
    val colorName = node.getString("color", "")
    val size = node.getInt("size", 24)
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    val liveAnd = node.getString("android_icon", "")
    val (isCustomDrawable, mappedName) = if (liveAnd.isNotEmpty()) Pair(false, liveAnd) else when (iconKey) {
        "arrow_left" -> Pair(false, "ArrowBack")
        "arrow_right" -> Pair(false, "ArrowForward")
        "bell" -> Pair(false, "Notifications")
        "bolt" -> Pair(false, "Build")
        "brand_spark" -> Pair(true, "ic_brand_spark")
        "cart" -> Pair(false, "ShoppingCart")
        "check" -> Pair(false, "Check")
        "check_circle" -> Pair(false, "CheckCircle")
        "chevron_left" -> Pair(false, "KeyboardArrowLeft")
        "chevron_right" -> Pair(false, "KeyboardArrowRight")
        "close" -> Pair(false, "Close")
        "coffee" -> Pair(false, "Coffee")
        "edit" -> Pair(false, "Edit")
        "heart" -> Pair(false, "Favorite")
        "home" -> Pair(false, "Home")
        "info" -> Pair(false, "Info")
        "lock" -> Pair(false, "Lock")
        "mail" -> Pair(false, "Email")
        "navigate_back" -> Pair(false, "ArrowBack")
        "orders" -> Pair(false, "ShoppingCart")
        "plus" -> Pair(false, "Add")
        "qrcode" -> Pair(false, "QrCode")
        "refresh" -> Pair(false, "Refresh")
        "search" -> Pair(false, "Search")
        "settings" -> Pair(false, "Settings")
        "share" -> Pair(false, "Share")
        "sparkles" -> Pair(false, "Star")
        "star" -> Pair(false, "Star")
        "trash" -> Pair(false, "Delete")
        "user" -> Pair(false, "Person")
        "warning" -> Pair(false, "Warning")
        else -> Pair(false, "Star")
    }
    val tintColor = resolveAppColor(colorName) ?: LocalContentColor.current
    val iconMod = Modifier.size(size.dp).let { if (opacity < 1f) it.graphicsLayer(alpha = opacity) else it }
    if (isCustomDrawable) {
        val ctx = androidx.compose.ui.platform.LocalContext.current
        val resId = ctx.resources.getIdentifier(mappedName, "drawable", ctx.packageName)
        if (resId != 0) {
            Icon(
                painter = androidx.compose.ui.res.painterResource(id = resId),
                contentDescription = iconKey,
                tint = tintColor,
                modifier = iconMod
            )
        }
    } else {
        val style = node.getString("variant", "rounded").lowercase()
        val vec = when (mappedName) {
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
        }
        Icon(
            imageVector = vec,
            contentDescription = iconKey,
            tint = tintColor,
            modifier = iconMod
        )
    }
}
"#;

pub const ICON_GTK_SNIPPET: &str = r#"
        "icon" => {
            let icon_key = node.get_string("name", "sparkles");
            let color = node.get_string("color", "");
            let size = node.get_int("size", 20);
            let opacity = node.get_int("opacity", 100);
            let live_lin = node.get_string("linux_icon", "");
            let (is_custom, mapped) = if !live_lin.is_empty() { (false, live_lin.as_str()) } else { match icon_key.as_str() {
                "arrow_left" => (false, "go-previous-symbolic"),
                "arrow_right" => (false, "go-next-symbolic"),
                "bell" => (false, "preferences-system-notifications-symbolic"),
                "bolt" => (false, "weather-storm-symbolic"),
                "brand_spark" => (true, "brand_spark-symbolic.svg"),
                "cart" => (false, "shopping-cart-symbolic"),
                "check" => (false, "object-select-symbolic"),
                "check_circle" => (false, "emblem-ok-symbolic"),
                "chevron_left" => (false, "go-previous-symbolic"),
                "chevron_right" => (false, "go-next-symbolic"),
                "close" => (false, "window-close-symbolic"),
                "coffee" => (false, "emoji-food-symbolic"),
                "edit" => (false, "document-edit-symbolic"),
                "heart" => (true, "heart-symbolic.svg"),
                "home" => (false, "go-home-symbolic"),
                "info" => (false, "dialog-information-symbolic"),
                "lock" => (false, "changes-prevent-symbolic"),
                "mail" => (false, "mail-unread-symbolic"),
                "navigate_back" => (false, "go-previous-symbolic"),
                "orders" => (false, "package-x-generic-symbolic"),
                "plus" => (false, "list-add-symbolic"),
                "qrcode" => (false, "qr-code-symbolic"),
                "refresh" => (false, "view-refresh-symbolic"),
                "search" => (false, "system-search-symbolic"),
                "settings" => (false, "preferences-system-symbolic"),
                "share" => (false, "emblem-shared-symbolic"),
                "sparkles" => (false, "starred-symbolic"),
                "star" => (false, "starred-symbolic"),
                "trash" => (false, "user-trash-symbolic"),
                "user" => (false, "avatar-default-symbolic"),
                "warning" => (false, "dialog-warning-symbolic"),
                _ => (false, "starred-symbolic"),
            } };
            let img = if is_custom {
                let stem = mapped.trim_end_matches(".svg");
                let sym_name = if stem.ends_with("-symbolic") { stem.to_string() } else { format!("{}-symbolic", stem) };
                let p1 = std::path::PathBuf::from(format!("icons/{}.svg", sym_name));
                let p2 = std::path::PathBuf::from(format!("demo_app/platforms/Linux/icons/{}.svg", sym_name));
                let p3 = std::path::PathBuf::from(format!("icons/{}", mapped));
                let p4 = std::path::PathBuf::from(format!("demo_app/platforms/Linux/icons/{}", mapped));
                let found = if p1.exists() { Some(p1) } else if p2.exists() { Some(p2) } else if p3.exists() { Some(p3) } else if p4.exists() { Some(p4) } else { None };
                if let Some(path) = found {
                    let file = gtk4::gio::File::for_path(&path);
                    let gicon = gtk4::gio::FileIcon::new(&file);
                    gtk4::Image::from_gicon(&gicon)
                } else {
                    gtk4::Image::from_icon_name(&sym_name)
                }
            } else {
                gtk4::Image::from_icon_name(mapped)
            };
            img.set_pixel_size(size as i32);
            apply_gtk_fg_color(&img, &color);
            if opacity < 100 {
                img.set_opacity((opacity as f64) / 100.0);
            }
            img.upcast::<gtk4::Widget>()
        }
"#;

pub const ICON_WEB_SNIPPET: &str = r#"
        case "icon": {
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
            switch (iconKey) {
                case "arrow_left": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M19 12H5M11 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "arrow_right": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M5 12h14M13 6l6 6-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "bell": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "bolt": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" fill="currentColor"/></svg>`; break;
                case "brand_spark": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
  <path d="M12 2L14.8 9.2L22 12L14.8 14.8L12 22L9.2 14.8L2 12L9.2 9.2L12 2Z"/>
</svg>
`; break;
                case "cart": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M6 6h15l-1.5 9h-12z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" fill="none"/><circle cx="9" cy="19" r="1.5" fill="currentColor"/><circle cx="17" cy="19" r="1.5" fill="currentColor"/><path d="M3 3h2l1 3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/></svg>`; break;
                case "check": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M5 13l4 4L19 7" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "check_circle": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="8" fill="currentColor"/></svg>`; break;
                case "chevron_left": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M15 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "chevron_right": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M9 6l6 6-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "close": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M6 6l12 12M18 6L6 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>`; break;
                case "coffee": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="8" fill="currentColor"/></svg>`; break;
                case "edit": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M12 20h9M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "heart": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" fill="currentColor"/></svg>`; break;
                case "home": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M3 10.5L12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1v-9.5z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "info": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M12 16v-4M12 8h.01" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>`; break;
                case "lock": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><rect x="5" y="11" width="14" height="10" rx="2" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M8 11V7a4 4 0 0 1 8 0v4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/></svg>`; break;
                case "mail": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><rect x="3" y="5" width="18" height="14" rx="2" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M3 7l9 6 9-6" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "navigate_back": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M19 12H5M11 6l-6 6 6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "orders": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M6 6h15l-1.5 9h-12z" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round" fill="none"/><circle cx="9" cy="19" r="1.5" fill="currentColor"/><circle cx="17" cy="19" r="1.5" fill="currentColor"/><path d="M3 3h2l1 3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/></svg>`; break;
                case "plus": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>`; break;
                case "qrcode": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M5 12h14M13 6l6 6-6 6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "refresh": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M21 12a9 9 0 1 1-2.64-6.36L21 8M21 3v5h-5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "search": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="11" cy="11" r="7" stroke="currentColor" stroke-width="2" fill="none"/><path d="M20 20l-4.2-4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>`; break;
                case "settings": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9c.26.604.852.997 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" stroke="currentColor" stroke-width="1.6" fill="none"/></svg>`; break;
                case "share": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M4 12v7a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7M16 6l-4-4-4 4M12 2v13" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "sparkles": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M12 2L14.6 9.4L22 12L14.6 14.6L12 22L9.4 14.6L2 12L9.4 9.4L12 2Z" fill="currentColor"/></svg>`; break;
                case "star": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" fill="currentColor"/></svg>`; break;
                case "trash": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M4 7h16M10 11v6M14 11v6M6 7l1 12a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-12M9 7V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                case "user": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><circle cx="12" cy="8" r="4" stroke="currentColor" stroke-width="1.8" fill="none"/><path d="M4 20c1.8-3.6 5-5.5 8-5.5s6.2 1.9 8 5.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" fill="none"/></svg>`; break;
                case "warning": el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="none"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0zM12 9v4M12 17h.01" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" fill="none"/></svg>`; break;
                default:
                    el.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="${size}" viewBox="0 0 24 24" fill="currentColor"><circle cx="12" cy="12" r="8"/></svg>`;
                    break;
            }
            const svgEl = el.querySelector("svg");
            if (svgEl) {
                svgEl.setAttribute("width", size);
                svgEl.setAttribute("height", size);
            }
            return el;
        }
"#;

pub const ICON_WINDOWS_SNIPPET: &str = r#"
                case "icon":
                    string iconKey = (node.Props != null && node.Props.ContainsKey("name")) ? node.Props["name"].ToString() : "sparkles";
                    double iconSize = (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double iSz)) ? iSz : 20.0;
                    string glyph = (node.Props != null && node.Props.ContainsKey("windows_glyph")) ? node.Props["windows_glyph"].ToString() : "";
                    if (string.IsNullOrEmpty(glyph)) switch (iconKey) {
                        case "arrow_left": glyph = "\uE72B"; break;
                        case "arrow_right": glyph = "\uE72A"; break;
                        case "bell": glyph = "\uEA8F"; break;
                        case "bolt": glyph = "\uE945"; break;
                        case "brand_spark": glyph = "\uE734"; break;
                        case "cart": glyph = "\uE7BF"; break;
                        case "check": glyph = "\uE73E"; break;
                        case "check_circle": glyph = "\uE73E"; break;
                        case "chevron_left": glyph = "\uE76B"; break;
                        case "chevron_right": glyph = "\uE76C"; break;
                        case "close": glyph = "\uE711"; break;
                        case "coffee": glyph = "\uEC32"; break;
                        case "edit": glyph = "\uE70F"; break;
                        case "heart": glyph = "\uEB51"; break;
                        case "home": glyph = "\uE80F"; break;
                        case "info": glyph = "\uE946"; break;
                        case "lock": glyph = "\uE72E"; break;
                        case "mail": glyph = "\uE715"; break;
                        case "navigate_back": glyph = "\uE72B"; break;
                        case "orders": glyph = "\uE7BF"; break;
                        case "plus": glyph = "\uE710"; break;
                        case "qrcode": glyph = "\uED14"; break;
                        case "refresh": glyph = "\uE72C"; break;
                        case "search": glyph = "\uE721"; break;
                        case "settings": glyph = "\uE713"; break;
                        case "share": glyph = "\uE72D"; break;
                        case "sparkles": glyph = "\uE734"; break;
                        case "star": glyph = "\uE734"; break;
                        case "trash": glyph = "\uE74D"; break;
                        case "user": glyph = "\uE77B"; break;
                        case "warning": glyph = "\uE7BA"; break;
                        default: glyph = "\uE734"; break;
                    }
                    var fi = new FontIcon { Glyph = glyph, FontSize = iconSize };
                    if (node.Props != null && node.Props.ContainsKey("color")) {
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) fi.Foreground = fg;
                    }
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double fiOp)) {
                        fi.Opacity = fiOp / 100.0;
                    }
                    return fi;
"#;
