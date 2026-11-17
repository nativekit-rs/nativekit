
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Image {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Image {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    /// Valid options: cover, contain
    pub fn content_mode(mut self, content_mode: impl Into<String>) -> Self {
        self.props.push(("content_mode".to_string(), PropValue::String(content_mode.into())));
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.props.push(("width".to_string(), PropValue::Int(width)));
        self
    }
    pub fn opacity(mut self, opacity: u32) -> Self {
        self.props.push(("opacity".to_string(), PropValue::Int(opacity)));
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.props.push(("height".to_string(), PropValue::Int(height)));
        self
    }
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.props.push(("corner_radius".to_string(), PropValue::Int(corner_radius)));
        self
    }
    pub fn size(mut self, size: u32) -> Self {
        self.props.push(("size".to_string(), PropValue::Int(size)));
        self
    }
    pub fn alt(mut self, alt: impl Into<String>) -> Self {
        self.props.push(("alt".to_string(), PropValue::String(alt.into())));
        self
    }
    /// Sets the aspect ratio (default `1 / 1` when `.size(...)` is used). Accepts `16 / 9`, `(16, 9)`, `1.777`, or `"16/9"`.
    pub fn aspect_ratio(mut self, ratio: impl IntoAspectRatio) -> Self {
        self.props.retain(|(k, _)| k != "aspect_ratio");
        self.props.push(("aspect_ratio".to_string(), PropValue::String(format!("{:.4}", ratio.into_aspect_ratio()))));
        self
    }
    #[allow(non_snake_case)]
    pub fn aspectRatio(self, ratio: impl IntoAspectRatio) -> Self {
        self.aspect_ratio(ratio)
    }
}
impl IntoNode for Image {
    fn into_node(self) -> Node {
        Node::Element {
            name: "image".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! Image {
    ($arg0:expr) => {{
        $crate::engine::primitives::Image {
            props: vec![("src".to_string(), $crate::engine::ast::PropValue::String($arg0.into()))],
            children: vec![],
        }
    }}
}

pub const IMAGE_ANDROID_SNIPPET: &str = r#"
"image" -> {
    val src = node.getString("src", "")
        val size = node.getInt("size", 0)
        val rawW = node.getInt("width", 0)
        val rawH = node.getInt("height", 0)
        val aspect = node.getString("aspect_ratio", "").toFloatOrNull() ?: 0f
        val w = when {
            rawW > 0 -> rawW
            size > 0 -> size
            rawH > 0 && aspect > 0f -> maxOf(1, kotlin.math.round(rawH * aspect).toInt())
            else -> 0
        }
        val h = when {
            rawH > 0 -> rawH
            aspect > 0f && w > 0 -> maxOf(1, kotlin.math.round(w / aspect).toInt())
            size > 0 -> size
            else -> if (aspect > 0f) 0 else 180
        }
        val radius = node.getInt("corner_radius", 0)
        val mode = node.getString("content_mode", "cover")
        val opacity = node.getInt("opacity", 100).toFloat() / 100f
        val imgMod = if (w == 0 && h == 0 && aspect > 0f) modifier.fillMaxWidth().aspectRatio(aspect) else modifier
        DynamicNetworkOrAssetImage(
            src = src,
            widthDp = w,
            heightDp = h,
            cornerRadiusDp = radius,
            isContain = (mode == "contain" || mode == "fit"),
            opacity = opacity,
            modifier = imgMod
        )
}
"#;

pub const IMAGE_IOS_SNIPPET: &str = r#"
case "image":
    let src = node.props["src"] as? String ?? ""
        let size = node.props["size"] as? Int ?? 0
        let rawW = node.props["width"] as? Int ?? 0
        let rawH = node.props["height"] as? Int ?? 0
        let aspect = Double(node.props["aspect_ratio"] as? String ?? "") ?? 0.0
        let w: Int = {
            if rawW > 0 { return rawW }
            if size > 0 { return size }
            if rawH > 0 && aspect > 0 { return max(1, Int((Double(rawH) * aspect).rounded())) }
            return 0
        }()
        let h: Int = {
            if rawH > 0 { return rawH }
            if aspect > 0 && w > 0 { return max(1, Int((Double(w) / aspect).rounded())) }
            if size > 0 { return size }
            return aspect > 0 ? 0 : 180
        }()
        let radius = CGFloat(node.props["corner_radius"] as? Int ?? 0)
        let mode = node.props["content_mode"] as? String ?? "cover"
        let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
        let isContain = (mode == "contain" || mode == "fit")
    
        let content = Group {
            if src.hasPrefix("http://") || src.hasPrefix("https://"), let url = URL(string: src) {
                AsyncImage(url: url) { phase in
                    switch phase {
                    case .success(let image):
                        if isContain { image.resizable().scaledToFit() }
                        else { image.resizable().scaledToFill() }
                    default:
                        RoundedRectangle(cornerRadius: radius, style: .continuous)
                            .fill(Color.secondary.opacity(0.14))
                            .overlay(ProgressView())
                    }
                }
            } else {
                let cleanName = ((src as NSString).lastPathComponent as NSString).deletingPathExtension
                if UIImage(named: cleanName) != nil {
                    if isContain { Image(cleanName).resizable().scaledToFit() }
                    else { Image(cleanName).resizable().scaledToFill() }
                } else if let devUrl = URL(string: "http://127.0.0.1:5824/" + src.replacingOccurrences(of: "./", with: "").trimmingCharacters(in: CharacterSet(charactersIn: "/"))) {
                    AsyncImage(url: devUrl) { phase in
                        if case .success(let image) = phase {
                            if isContain { image.resizable().scaledToFit() } else { image.resizable().scaledToFill() }
                        } else {
                            RoundedRectangle(cornerRadius: radius, style: .continuous).fill(Color.secondary.opacity(0.14))
                        }
                    }
                }
            }
        }
    
        if w == 0 && h == 0 && aspect > 0 {
            Color.clear
                .aspectRatio(CGFloat(aspect), contentMode: .fit)
                .frame(maxWidth: .infinity)
                .overlay(content)
                .clipped()
                .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
                .opacity(opacity)
        } else {
            content
                .frame(width: w > 0 ? CGFloat(w) : nil, height: CGFloat(h))
                .frame(maxWidth: w > 0 ? nil : .infinity)
                .clipped()
                .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
                .opacity(opacity)
        }
"#;

pub const IMAGE_GTK_SNIPPET: &str = r#"
"image" => {
    let src = node.get_string("src", "");
        let size = node.get_int("size", 0);
        let raw_w = node.get_int("width", 0);
        let raw_h = node.get_int("height", 0);
        let aspect = node.get_string("aspect_ratio", "").parse::<f32>().unwrap_or(0.0);
        let w = if raw_w > 0 {
            raw_w
        } else if size > 0 {
            size
        } else if raw_h > 0 && aspect > 0.0 {
            ((raw_h as f32) * aspect).round().max(1.0) as u32
        } else {
            0
        };
        let h = if raw_h > 0 {
            raw_h
        } else if aspect > 0.0 && w > 0 {
            ((w as f32) / aspect).round().max(1.0) as u32
        } else if size > 0 {
            size
        } else if aspect > 0.0 {
            (320.0 / aspect).round().max(1.0) as u32
        } else {
            180
        };
        let radius = node.get_int("corner_radius", 0);
        let mode = node.get_string("content_mode", "cover");
        let opacity = node.get_int("opacity", 100);
        crate::build_gtk_image(&src, w, h, radius, &mode, opacity)
}
"#;

pub const IMAGE_WEB_SNIPPET: &str = r#"
case "image": {
    const { src = "", alt = "", width = 0, height = 0, size = 0, aspect_ratio = "", corner_radius = 0, content_mode = "cover", opacity = 100 } = node.props || {};
        const aspect = parseFloat(aspect_ratio) || 0;
        const finalW = width || size || (height > 0 && aspect > 0 ? Math.max(1, Math.round(height * aspect)) : 0);
        const finalH = height || (aspect > 0 && finalW > 0 ? Math.max(1, Math.round(finalW / aspect)) : 0) || size || (aspect > 0 ? 0 : 180);
        return h("img", {
            className: "nk-image",
            attrs: { src: src, alt: alt, loading: "lazy" },
            style: {
                width: finalW > 0 ? `${finalW}px` : "100%",
                height: finalH > 0 ? `${finalH}px` : "auto",
                ...(aspect > 0 ? { aspectRatio: String(aspect) } : {}),
                objectFit: (content_mode === "contain" || content_mode === "fit") ? "contain" : "cover",
                borderRadius: corner_radius > 0 ? `${corner_radius}px` : "0px",
                opacity: opacity < 100 ? String(opacity / 100.0) : "1",
                display: "block",
                flexShrink: "0",
            },
        });
}
"#;

pub const IMAGE_WINDOWS_SNIPPET: &str = r#"
case "image": {
    return BuildWindowsImage(node);
}
"#;
