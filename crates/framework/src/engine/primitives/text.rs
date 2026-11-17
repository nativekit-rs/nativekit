// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/typography.json & config/colors.json
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Text {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}

impl Text {
    pub fn on_delete<H, Args, S>(mut self, handler: H) -> Self
    where
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static,
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_delete".to_string(), PropValue::String(id)));
        self
    }
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }

    /// Valid options: body, caption1, caption2, footnote, headline, large_title, subhead, title1, title2, title3
    pub fn font(mut self, style: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "style");
        self.props.push(("style".to_string(), PropValue::String(style.into())));
        self
    }

    /// Applies the platform's emphasized weight for this text style.
    pub fn emphasized(mut self) -> Self {
        self.props.retain(|(k, _)| k != "emphasized");
        self.props.push(("emphasized".to_string(), PropValue::Bool(true)));
        self
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow (or any inline hex color like "#FF5733")
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "color");
        self.props.push(("color".to_string(), PropValue::String(color.into())));
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        let pct = if opacity > 1.0 { opacity.clamp(0.0, 100.0) } else { opacity.clamp(0.0, 1.0) * 100.0 };
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }
}

impl IntoNode for Text {
    fn into_node(self) -> Node {
        Node::Element {
            name: "text".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! Text {
    ($($arg:tt)*) => {{
        $crate::engine::primitives::Text {
            props: vec![("label".to_string(), $crate::engine::ast::PropValue::String(format!($($arg)*).into()))],
            children: vec![],
        }
    }}
}

pub const TEXT_IOS_SNIPPET: &str = r#"case "text":
    let label = node.props["label"] as? String ?? ""
    let style = node.props["style"] as? String ?? "body"
    let emphasized = node.props["emphasized"] as? Bool ?? false
    let colorName = node.props["color"] as? String ?? ""
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let font: Font = {
        switch style {
        case "body": return Font.body.weight(emphasized ? .semibold : .regular)
        case "caption1": return Font.caption.weight(emphasized ? .semibold : .regular)
        case "caption2": return Font.caption2.weight(emphasized ? .semibold : .regular)
        case "footnote": return Font.footnote.weight(emphasized ? .semibold : .regular)
        case "headline": return Font.headline.weight(emphasized ? .semibold : .semibold)
        case "large_title": return Font.largeTitle.weight(emphasized ? .bold : .regular)
        case "subhead": return Font.subheadline.weight(emphasized ? .semibold : .regular)
        case "title1": return Font.title.weight(emphasized ? .bold : .regular)
        case "title2": return Font.title2.weight(emphasized ? .bold : .regular)
        case "title3": return Font.title3.weight(emphasized ? .semibold : .regular)
        default: return Font.body.weight(emphasized ? .semibold : .regular)
        }
    }()
    if let resolvedColor = resolveAppColor(colorName) {
        Text(label)
            .font(font)
            .foregroundStyle(resolvedColor)
            .opacity(opacity)
    } else {
        Text(label)
            .font(font)
            .opacity(opacity)
    }
"#;

pub const TEXT_ANDROID_SNIPPET: &str = r#"
"text" -> {
    val label = node.getString("label") ?: ""
    val styleName = node.getString("style").ifEmpty { "body" }
    val emphasized = node.getBoolean("emphasized", false)
    val colorName = node.getString("color", "")
    val opacity = node.getInt("opacity", 100).toFloat() / 100f
    val textStyle = when (styleName) {
        "body" -> MaterialTheme.typography.bodyLarge.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        "caption1" -> MaterialTheme.typography.labelMedium.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        "caption2" -> MaterialTheme.typography.labelSmall.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        "footnote" -> MaterialTheme.typography.bodySmall.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        "headline" -> MaterialTheme.typography.titleMedium.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.Bold else androidx.compose.ui.text.font.FontWeight.SemiBold)
        "large_title" -> MaterialTheme.typography.displaySmall.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.Bold else androidx.compose.ui.text.font.FontWeight.Normal)
        "subhead" -> MaterialTheme.typography.titleSmall.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        "title1" -> MaterialTheme.typography.headlineLarge.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.Bold else androidx.compose.ui.text.font.FontWeight.Normal)
        "title2" -> MaterialTheme.typography.headlineMedium.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.Bold else androidx.compose.ui.text.font.FontWeight.Normal)
        "title3" -> MaterialTheme.typography.headlineSmall.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
        else -> MaterialTheme.typography.bodyLarge.copy(fontWeight = if (emphasized) androidx.compose.ui.text.font.FontWeight.SemiBold else androidx.compose.ui.text.font.FontWeight.Normal)
    }
    val resolvedColor = resolveAppColor(colorName) ?: androidx.compose.ui.graphics.Color.Unspecified
    Text(
        text = label,
        style = textStyle,
        color = resolvedColor,
        modifier = if (opacity < 1f) Modifier.graphicsLayer(alpha = opacity) else Modifier
    )
}
"#;

pub const TEXT_GTK_SNIPPET: &str = r#"
        "text" => {
            let label_str = node.get_string("label", "");
            let style = node.get_string("style", "body");
            let emphasized = node.get_bool("emphasized", false);
            let color = node.get_string("color", "");
            let opacity = node.get_int("opacity", 100);
            let lbl = gtk4::Label::new(Some(&label_str));
            lbl.set_wrap(true);
            lbl.set_halign(gtk4::Align::Start);
            let (css_class, weight_class) = match style.as_str() {
                "body" => ("body", if emphasized { "weight-semibold" } else { "weight-regular" }),
                "caption1" => ("caption-heading", if emphasized { "weight-bold" } else { "weight-regular" }),
                "caption2" => ("caption", if emphasized { "weight-semibold" } else { "weight-light" }),
                "footnote" => ("caption", if emphasized { "weight-semibold" } else { "weight-regular" }),
                "headline" => ("heading", if emphasized { "weight-bold" } else { "weight-semibold" }),
                "large_title" => ("title-1", if emphasized { "weight-heavy" } else { "weight-bold" }),
                "subhead" => ("title-4", if emphasized { "weight-semibold" } else { "weight-regular" }),
                "title1" => ("title-1", if emphasized { "weight-bold" } else { "weight-regular" }),
                "title2" => ("title-2", if emphasized { "weight-bold" } else { "weight-regular" }),
                "title3" => ("title-3", if emphasized { "weight-semibold" } else { "weight-regular" }),
                _ => ("body", if emphasized { "weight-semibold" } else { "weight-regular" }),
            };
            lbl.add_css_class(css_class);
            lbl.add_css_class(weight_class);
            apply_gtk_fg_color(&lbl, &color);
            if opacity < 100 {
                lbl.set_opacity((opacity as f64) / 100.0);
            }
            lbl.upcast::<gtk4::Widget>()
        }
"#;

pub const TEXT_WEB_SNIPPET: &str = r#"
        case "text": {
            const label = (node.props && node.props.label) || "";
            const style = (node.props && node.props.style) || "body";
            const emphasized = (node.props && node.props.emphasized) || false;
            const color = (node.props && node.props.color) || "";
            const opacity = (node.props && node.props.opacity !== undefined) ? (node.props.opacity / 100.0) : 1.0;
            const el = document.createElement("span");
            switch (style) {
        case "body": el.classList.add("body", emphasized ? "weight-semibold" : "weight-regular"); break;
        case "caption1": el.classList.add("caption-1", emphasized ? "weight-semibold" : "weight-regular"); break;
        case "caption2": el.classList.add("caption-2", emphasized ? "weight-semibold" : "weight-regular"); break;
        case "footnote": el.classList.add("footnote", emphasized ? "weight-semibold" : "weight-regular"); break;
        case "headline": el.classList.add("headline", emphasized ? "weight-semibold" : "weight-semibold"); break;
        case "large_title": el.classList.add("large-title", emphasized ? "weight-bold" : "weight-regular"); break;
        case "subhead": el.classList.add("subhead", emphasized ? "weight-semibold" : "weight-regular"); break;
        case "title1": el.classList.add("title-1", emphasized ? "weight-bold" : "weight-regular"); break;
        case "title2": el.classList.add("title-2", emphasized ? "weight-bold" : "weight-regular"); break;
        case "title3": el.classList.add("title-3", emphasized ? "weight-semibold" : "weight-regular"); break;
                default:
                    el.classList.add("body", emphasized ? "weight-semibold" : "weight-regular");
                    break;
            }
            applyWebFgColor(el, color);
            if (opacity < 1.0) el.style.opacity = opacity.toString();
            el.textContent = label;
            return el;
        }
"#;

pub const TEXT_WINDOWS_SNIPPET: &str = r#"
                case "text":
                    var tb = new TextBlock { TextWrapping = TextWrapping.Wrap };
                    if (node.Props != null && node.Props.ContainsKey("label")) {
                        tb.Text = node.Props["label"].ToString();
                    }
                    string ts = (node.Props != null && node.Props.ContainsKey("style")) ? node.Props["style"].ToString() : "body";
                    bool emphasized = (node.Props != null && node.Props.ContainsKey("emphasized") && node.Props["emphasized"] is bool emp && emp);
                    switch (ts) {
                        case "body":
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "caption1":
                            tb.FontSize = 12;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "caption2":
                            tb.FontSize = 11;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "footnote":
                            tb.FontSize = 13;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "headline":
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.SemiBold;
                            break;
                        case "large_title":
                            tb.FontSize = 34;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "subhead":
                            tb.FontSize = 15;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title1":
                            tb.FontSize = 28;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title2":
                            tb.FontSize = 22;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title3":
                            tb.FontSize = 20;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        default:
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                    }
                    if (node.Props != null && node.Props.ContainsKey("color")) {
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) tb.Foreground = fg;
                    }
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double tbOp)) {
                        tb.Opacity = tbOp / 100.0;
                    }
                    return tb;
"#;
