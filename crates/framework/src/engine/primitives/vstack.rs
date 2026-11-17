// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/vstack
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;
use crate::engine::primitives::color::{BackgroundValue, IntoBackground};

pub struct VStack {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}

impl VStack {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.props.retain(|(k, _)| k != "spacing");
        self.props.push(("spacing".to_string(), PropValue::Int(spacing)));
        self
    }

    /// Valid options: hug, fill
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "width");
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
    }

    /// Valid options: top, bottom, center, middle, left, right, space_between (or 2-arg `.align(.top, .left)`)
    pub fn align(mut self, align: impl Into<String>) -> Self {
        let a = align.into();
        let existing_x = self.props.iter().find(|(k, _)| k == "align_x").and_then(|(_, v)| if let PropValue::String(s) = v { Some(s.clone()) } else { None });
        let existing_y = self.props.iter().find(|(k, _)| k == "align_y").and_then(|(_, v)| if let PropValue::String(s) = v { Some(s.clone()) } else { None });
        let (nx, ny): (Option<&str>, Option<&str>) = match a.as_str() {
            "top" => (None, Some("top")),
            "bottom" => (None, Some("bottom")),
            "middle" => (None, Some("center")),
            "left" | "leading" => (Some("left"), None),
            "right" | "trailing" => (Some("right"), None),
            "space_between" => (None, Some("space_between")),
            "center" => {
                if matches!(existing_y.as_deref(), Some("top" | "bottom" | "space_between")) && existing_x.is_none() {
                    (Some("center"), None)
                } else if matches!(existing_x.as_deref(), Some("left" | "right")) && existing_y.is_none() {
                    (None, Some("center"))
                } else {
                    (Some("center"), Some("center"))
                }
            }
            "top_left" => (Some("left"), Some("top")),
            "top_center" => (Some("center"), Some("top")),
            "top_right" => (Some("right"), Some("top")),
            "center_left" | "middle_left" => (Some("left"), Some("center")),
            "center_right" | "middle_right" => (Some("right"), Some("center")),
            "bottom_left" => (Some("left"), Some("bottom")),
            "bottom_center" => (Some("center"), Some("bottom")),
            "bottom_right" => (Some("right"), Some("bottom")),
            _ => (None, None),
        };
        if let Some(x) = nx {
            self.props.retain(|(k, _)| k != "align_x");
            self.props.push(("align_x".to_string(), PropValue::String(x.to_string())));
        }
        if let Some(y) = ny {
            self.props.retain(|(k, _)| k != "align_y");
            self.props.push(("align_y".to_string(), PropValue::String(y.to_string())));
        }
        self
    }

    /// Valid options: black, blue, brown, cyan, danger, fill, gray, green, grouped_background, indigo, mint, orange, pink, primary, purple, red, secondary, separator, success, surface, surface_secondary, teal, text_primary, text_secondary, text_tertiary, warning, white, yellow (or an inline hex color like "#1E293B", or `Gradient!(...)`)
    pub fn background(mut self, bg: impl IntoBackground) -> Self {
        self.props.retain(|(k, _)| k != "background" && k != "background_gradient");
        match bg.into_background() {
            BackgroundValue::Solid(c) => {
                self.props.push(("background".to_string(), PropValue::String(c)));
            }
            BackgroundValue::Gradient(g) => {
                self.props.push(("background_gradient".to_string(), PropValue::Json(g.to_json_value())));
            }
        }
        self
    }

    pub fn background_image(mut self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {
        self.props.retain(|(k, _)| k != "background_image");
        self.props.push(("background_image".to_string(), PropValue::String(img.into_bg_image_url())));
        self
    }

    #[allow(non_snake_case)]
    pub fn backgroundImage(self, img: impl crate::engine::primitives::color::IntoBackgroundImage) -> Self {
        self.background_image(img)
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.props.retain(|(k, _)| k != "padding");
        self.props.push(("padding".to_string(), PropValue::Int(padding)));
        self
    }

    pub fn corner_radius(mut self, radius: u32) -> Self {
        self.props.retain(|(k, _)| k != "corner_radius");
        self.props.push(("corner_radius".to_string(), PropValue::Int(radius)));
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        let pct = if opacity > 1.0 { opacity.clamp(0.0, 100.0) } else { opacity.clamp(0.0, 1.0) * 100.0 };
        self.props.retain(|(k, _)| k != "opacity");
        self.props.push(("opacity".to_string(), PropValue::Int(pct.round() as u32)));
        self
    }

    pub fn child(mut self, child: impl IntoNode) -> Self {
        self.children.push(child.into_node());
        self
    }

    pub fn children(mut self, children: impl IntoChildren) -> Self {
        self.children.extend(children.into_children());
        self
    }
}

impl IntoNode for VStack {
    fn into_node(self) -> Node {
        Node::Element {
            name: "vstack".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const VSTACK_ANDROID_SNIPPET: &str = r#"
"vstack" -> {
    val spacing = node.getInt("spacing", 0)
        val widthProp = node.getString("width", "")
        val alignX = node.getString("align_x", "left")
        val alignY = node.getString("align_y", "top")
        val bgImg = node.getString("background_image", "")
        val radius = node.getInt("corner_radius", 0)
        val pad = node.getInt("padding", 0)
        val isHug = widthProp == "hug"
        var colMod = (if (isHug) Modifier else modifier).then(Modifier.applyContainerStyle(node))
        if (!isHug && (widthProp == "fill" || modifier == Modifier)) {
            colMod = colMod.fillMaxWidth()
        }
        val hAlign = when (alignX) {
            "center" -> Alignment.CenterHorizontally
            "right" -> Alignment.End
            else -> Alignment.Start
        }
        val vArrangement = when (alignY) {
            "space_between" -> Arrangement.SpaceBetween
            "center" -> Arrangement.spacedBy(spacing.dp, Alignment.CenterVertically)
            "bottom" -> Arrangement.spacedBy(spacing.dp, Alignment.Bottom)
            else -> Arrangement.spacedBy(spacing.dp, Alignment.Top)
        }
        if (bgImg.isNotEmpty()) {
            var boxMod = if (!isHug && (widthProp == "fill" || modifier == Modifier)) modifier.fillMaxWidth() else (if (isHug) Modifier else modifier)
            if (radius > 0) boxMod = boxMod.clip(androidx.compose.foundation.shape.RoundedCornerShape(radius.dp))
            androidx.compose.foundation.layout.Box(modifier = boxMod) {
                DynamicNetworkOrAssetImage(
                    src = bgImg, widthDp = 0, heightDp = 0, cornerRadiusDp = radius,
                    isContain = false, opacity = 1f, modifier = Modifier.matchParentSize()
                )
                Column(
                    modifier = (if (isHug) Modifier else Modifier.fillMaxWidth()).padding(pad.dp),
                    verticalArrangement = vArrangement,
                    horizontalAlignment = hAlign
                ) {
                    node.children.forEach { child -> DynamicNodeView(child, dispatch) }
                }
            }
        } else {
            Column(
                modifier = colMod,
                verticalArrangement = vArrangement,
                horizontalAlignment = hAlign
            ) {
                node.children.forEach { child -> DynamicNodeView(child, dispatch) }
            }
        }
}
"#;

pub const VSTACK_IOS_SNIPPET: &str = r#"
case "vstack":
    let spacing = node.props["spacing"] as? Int ?? 0
        let widthProp = node.props["width"] as? String ?? ""
        let alignX = node.props["align_x"] as? String ?? "left"
        let alignY = node.props["align_y"] as? String ?? "top"
        let isFill = widthProp != "hug"
        let isSpaceBetween = alignY == "space_between"
        let hAlign: HorizontalAlignment = alignX == "center" ? .center : (alignX == "right" ? .trailing : .leading)
        let frameAlign: Alignment = {
            switch (alignX, alignY) {
            case ("center", "center"): return .center
            case ("right", "center"): return .trailing
            case (_, "center"): return .leading
            case ("center", "bottom"): return .bottom
            case ("right", "bottom"): return .bottomTrailing
            case (_, "bottom"): return .bottomLeading
            case ("center", _): return .top
            case ("right", _): return .topTrailing
            default: return .topLeading
            }
        }()
    
        VStack(alignment: hAlign, spacing: CGFloat(spacing)) {
            ForEach(0..<node.children.count, id: \.self) { i in
                if isSpaceBetween && i > 0 {
                    Spacer(minLength: CGFloat(spacing))
                }
                DynamicNodeView(node: node.children[i], dispatch: dispatch)
            }
        }
        .frame(maxWidth: isFill ? .infinity : nil, alignment: frameAlign)
        .applyContainerStyle(node: node)
"#;

pub const VSTACK_GTK_SNIPPET: &str = r#"
"vstack" => {
    let spacing = node.get_int("spacing", 0);
        let width_prop = node.get_string("width", "");
        let align_x = node.get_string("align_x", "left");
        let align_y = node.get_string("align_y", "top");
        let bg_img = node.get_string("background_image", "");
        let radius = node.get_int("corner_radius", 0);
        let is_fill = width_prop != "hug";
        let bx = gtk4::Box::new(gtk4::Orientation::Vertical, spacing as i32);
        bx.set_hexpand(is_fill);
        if !is_fill {
            bx.set_halign(match align_x.as_str() {
                "center" => gtk4::Align::Center,
                "right" => gtk4::Align::End,
                _ => gtk4::Align::Start,
            });
        }
        bx.set_valign(match align_y.as_str() {
            "center" => gtk4::Align::Center,
            "bottom" => gtk4::Align::End,
            _ => gtk4::Align::Start,
        });
        let h_align = match align_x.as_str() {
            "center" => gtk4::Align::Center,
            "right" => gtk4::Align::End,
            _ => gtk4::Align::Fill,
        };
        apply_gtk_container_style(&bx, node);
        for child in &node.children {
            let cw = parse_dynamic_widget(child, dispatch);
            if align_x == "center" || align_x == "right" {
                cw.set_halign(h_align);
            }
            bx.append(&cw);
        }
        if !bg_img.is_empty() {
            let overlay = gtk4::Overlay::new();
            if radius > 0 {
                overlay.set_overflow(gtk4::Overflow::Hidden);
                overlay.add_css_class("nk-dyn-bg-ov");
                let prov = gtk4::CssProvider::new();
                prov.load_from_data(&format!(".nk-dyn-bg-ov {{ border-radius: {}px; }}", radius));
                #[allow(deprecated)]
                overlay.style_context().add_provider(&prov, gtk4::STYLE_PROVIDER_PRIORITY_USER);
            }
            let pic = crate::build_gtk_image(&bg_img, 0, 0, radius, "cover", 100);
            overlay.set_child(Some(&pic));
            overlay.add_overlay(&bx);
            return overlay.upcast::<gtk4::Widget>();
        }
        bx.upcast::<gtk4::Widget>()
}
"#;

pub const VSTACK_WEB_SNIPPET: &str = r#"
case "vstack": {
    const p = node.props || {};
        const widthProp = p.width || "";
        const alignX = p.align_x || "left";
        const alignY = p.align_y || "top";
        const isFill = widthProp !== "hug";
        const el = document.createElement("div");
        el.classList.add("vstack");
        applyWebContainerStyle(el, node);
        el.style.width = isFill ? "100%" : "fit-content";
        el.style.alignItems = alignX === "center" ? "center" : (alignX === "right" ? "flex-end" : "stretch");
        el.style.justifyContent = alignY === "space_between" ? "space-between" : (alignY === "center" ? "center" : (alignY === "bottom" ? "flex-end" : "flex-start"));
        if (node.children) node.children.forEach(c => el.appendChild(renderNode(c)));
        return el;
}
"#;

pub const VSTACK_WINDOWS_SNIPPET: &str = r#"
case "vstack": {
    var vp = new StackPanel { Orientation = Orientation.Vertical };
        if (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double vSpace)) {
            vp.Spacing = vSpace;
        }
        string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
        string alignX = (node.Props != null && node.Props.ContainsKey("align_x")) ? node.Props["align_x"].ToString() : "left";
        string alignY = (node.Props != null && node.Props.ContainsKey("align_y")) ? node.Props["align_y"].ToString() : "top";
        vp.HorizontalAlignment = widthProp != "hug" ? HorizontalAlignment.Stretch : (alignX == "center" ? HorizontalAlignment.Center : (alignX == "right" ? HorizontalAlignment.Right : HorizontalAlignment.Left));
        vp.VerticalAlignment = alignY == "center" ? VerticalAlignment.Center : (alignY == "bottom" ? VerticalAlignment.Bottom : VerticalAlignment.Top);
        ApplyWindowsContainerStyle(vp, node);
        if (node.Children != null) {
            foreach (var c in node.Children) vp.Children.Add(RenderNode(c));
        }
        return vp;
}
"#;
