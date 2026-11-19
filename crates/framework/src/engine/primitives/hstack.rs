// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/hstack
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;
use crate::engine::primitives::color::{BackgroundValue, IntoBackground};

pub struct HStack {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}

impl HStack {
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
            "space_between" => (Some("space_between"), None),
            "center" => {
                if matches!(existing_y.as_deref(), Some("top" | "bottom")) && existing_x.is_none() {
                    (Some("center"), None)
                } else if matches!(existing_x.as_deref(), Some("left" | "right" | "space_between")) && existing_y.is_none() {
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

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.props.retain(|(k, _)| k != "wrap");
        self.props.push(("wrap".to_string(), PropValue::Bool(wrap)));
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

impl IntoNode for HStack {
    fn into_node(self) -> Node {
        Node::Element {
            name: "hstack".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const HSTACK_ANDROID_SNIPPET: &str = r#"
"hstack" -> {
    val spacing = node.getInt("spacing", 0)
        val widthProp = node.getString("width", "")
        val alignX = node.getString("align_x", "left")
        val alignY = node.getString("align_y", "center")
        val isWrap = node.getBoolean("wrap", false)
        val isHeaderBar = node.getBoolean("is_header_bar", false)
        val isSpaceBetween = alignX == "space_between" || (isHeaderBar && node.children.size == 2)
        val hasFlexChild = node.children.any {
            it.getString("width", "") == "fill"
                || (it.name == "spacer" && it.getInt("size", 0) == 0)
                || (it.name == "vstack" && it.getString("width", "") != "hug")
        }
        val isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlexChild || isSpaceBetween))
        var rowMod = (if (widthProp == "hug") Modifier else modifier).then(Modifier.applyContainerStyle(node))
        if (isFill) {
            rowMod = rowMod.fillMaxWidth()
        }
        val vAlign = when (alignY) {
            "top" -> Alignment.Top
            "bottom" -> Alignment.Bottom
            else -> Alignment.CenterVertically
        }
        val hArrangement = when {
            isSpaceBetween -> Arrangement.SpaceBetween
            alignX == "center" -> Arrangement.spacedBy(spacing.dp, Alignment.CenterHorizontally)
            alignX == "right" -> Arrangement.spacedBy(spacing.dp, Alignment.End)
            else -> Arrangement.spacedBy(spacing.dp, Alignment.Start)
        }
    
        if (isWrap) {
            @OptIn(androidx.compose.foundation.layout.ExperimentalLayoutApi::class)
            androidx.compose.foundation.layout.FlowRow(
                modifier = rowMod,
                horizontalArrangement = hArrangement,
                verticalArrangement = Arrangement.spacedBy(spacing.dp)
            ) {
                node.children.forEach { child ->
                    DynamicNodeView(child, dispatch, Modifier.widthIn(min = 0.dp))
                }
            }
        } else {
            Row(
                modifier = rowMod,
                horizontalArrangement = hArrangement,
                verticalAlignment = vAlign
            ) {
                node.children.forEach { child ->
                    val childWidth = child.getString("width", "")
                    val childIsFlex = childWidth == "fill"
                        || (child.name == "spacer" && child.getInt("size", 0) == 0)
                        || (child.name == "vstack" && childWidth != "hug")
                    val childMod = if (childIsFlex) Modifier.weight(1f) else Modifier.widthIn(min = 0.dp)
                    DynamicNodeView(child, dispatch, childMod)
                }
            }
        }
}
"#;

pub const HSTACK_IOS_SNIPPET: &str = r#"
case "hstack":
    let spacing = node.props["spacing"] as? Int ?? 0
        let widthProp = node.props["width"] as? String ?? ""
        let alignX = node.props["align_x"] as? String ?? "left"
        let alignY = node.props["align_y"] as? String ?? "center"
        let isWrap = node.props["wrap"] as? Bool ?? false
        let isHeaderBar = node.props["is_header_bar"] as? Bool ?? false
        let isSpaceBetween = alignX == "space_between" || (isHeaderBar && node.children.count == 2)
        let hasFlexChild = node.children.contains {
            ($0.props["width"] as? String) == "fill"
                || ($0.name == "spacer" && ($0.props["size"] as? Int ?? 0) == 0)
                || ($0.name == "vstack" && ($0.props["width"] as? String ?? "") != "hug")
        }
        let isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlexChild || isSpaceBetween))
        let vAlign: VerticalAlignment = alignY == "top" ? .top : (alignY == "bottom" ? .bottom : .center)
        let frameAlign: Alignment = {
            switch (alignX, alignY) {
            case ("center", "top"): return .top
            case ("right", "top"): return .topTrailing
            case (_, "top"): return .topLeading
            case ("center", "bottom"): return .bottom
            case ("right", "bottom"): return .bottomTrailing
            case (_, "bottom"): return .bottomLeading
            case ("center", _): return .center
            case ("right", _): return .trailing
            default: return .leading
            }
        }()
    
        if isWrap {
            NativeFlowLayout(spacing: CGFloat(spacing), alignX: alignX, alignY: alignY) {
                ForEach(0..<node.children.count, id: \.self) { i in
                    DynamicNodeView(node: node.children[i], dispatch: dispatch)
                }
            }
            .frame(maxWidth: isFill ? .infinity : nil, alignment: frameAlign)
            .applyContainerStyle(node: node)
        } else {
            HStack(alignment: vAlign, spacing: CGFloat(spacing)) {
                ForEach(0..<node.children.count, id: \.self) { i in
                    if isSpaceBetween && i > 0 {
                        Spacer(minLength: CGFloat(spacing))
                    }
                    DynamicNodeView(node: node.children[i], dispatch: dispatch)
                }
            }
            .frame(maxWidth: isFill ? .infinity : nil, alignment: frameAlign)
            .applyContainerStyle(node: node)
        }
"#;

pub const HSTACK_GTK_SNIPPET: &str = r#"
"hstack" => {
    let spacing = node.get_int("spacing", 0);
        let width_prop = node.get_string("width", "");
        let align_x = node.get_string("align_x", "left");
        let align_y = node.get_string("align_y", "center");
        let is_wrap = node.get_bool("wrap", false);
        let is_hdr = node.get_bool("is_header_bar", false);
        let is_space_between = align_x == "space_between" || (is_hdr && node.children.len() == 2);
        let has_flex = node.children.iter().any(|c| {
            c.get_string("width", "") == "fill"
                || (c.name == "spacer" && c.get_int("size", 0) == 0)
                || (c.name == "vstack" && c.get_string("width", "") != "hug")
        });
        let is_fill = width_prop == "fill" || (width_prop != "hug" && (has_flex || is_space_between));
    
        if is_wrap {
            let fb = gtk4::FlowBox::new();
            fb.set_selection_mode(gtk4::SelectionMode::None);
            fb.set_column_spacing(spacing as u32);
            fb.set_row_spacing(spacing as u32);
            fb.set_hexpand(is_fill);
            apply_gtk_container_style(&fb, node);
            for child in &node.children {
                fb.insert(&parse_dynamic_widget(child, dispatch), -1);
            }
            return fb.upcast::<gtk4::Widget>();
        }
    
        let bx = gtk4::Box::new(gtk4::Orientation::Horizontal, spacing as i32);
        bx.set_hexpand(is_fill);
        if !is_fill {
            bx.set_halign(match align_x.as_str() {
                "center" => gtk4::Align::Center,
                "right" => gtk4::Align::End,
                _ => gtk4::Align::Start,
            });
        }
        let v_align = match align_y.as_str() {
            "top" => gtk4::Align::Start,
            "bottom" => gtk4::Align::End,
            _ => gtk4::Align::Center,
        };
        apply_gtk_container_style(&bx, node);
        for (i, child) in node.children.iter().enumerate() {
            if is_space_between && i > 0 {
                let sp = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                sp.set_hexpand(true);
                bx.append(&sp);
            }
            let cw = parse_dynamic_widget(child, dispatch);
            cw.set_valign(v_align);
            let cw_width = child.get_string("width", "");
            if cw_width == "fill" || (child.name == "vstack" && cw_width != "hug" && !is_space_between) {
                cw.set_hexpand(true);
            }
            bx.append(&cw);
        }
        bx.upcast::<gtk4::Widget>()
}
"#;

pub const HSTACK_WEB_SNIPPET: &str = r#"
case "hstack": {
    const p = node.props || {};
        const isHdr = Boolean(p.is_header_bar);
        const widthProp = p.width || "";
        const alignX = p.align_x || "left";
        const alignY = p.align_y || "center";
        const isWrap = Boolean(p.wrap);
        const isSpaceBetween = alignX === "space_between" || (isHdr && (node.children || []).length === 2);
        const hasFlex = (node.children || []).some(c => {
            const cw = (c.props && c.props.width) || "";
            return cw === "fill" || (c.name === "spacer" && !((c.props && c.props.size) > 0)) || (c.name === "vstack" && cw !== "hug");
        });
        const isFill = widthProp === "fill" || (widthProp !== "hug" && (hasFlex || isSpaceBetween));
        const el = document.createElement("div");
        el.classList.add("hstack");
        applyWebContainerStyle(el, node);
        el.style.width = isFill ? "100%" : "fit-content";
        el.style.flexWrap = isWrap ? "wrap" : "nowrap";
        el.style.alignItems = alignY === "top" ? "flex-start" : (alignY === "bottom" ? "flex-end" : "center");
        el.style.justifyContent = isSpaceBetween ? "space-between" : (alignX === "center" ? "center" : (alignX === "right" ? "flex-end" : "flex-start"));
        if (node.children) {
            node.children.forEach(c => {
                const childEl = renderNode(c);
                const cw = (c.props && c.props.width) || "";
                if (cw === "fill" || (c.name === "vstack" && cw !== "hug" && !isSpaceBetween)) {
                    childEl.style.flex = "1 1 0%";
                }
                el.appendChild(childEl);
            });
        }
        return el;
}
"#;

pub const HSTACK_WINDOWS_SNIPPET: &str = r#"
case "hstack": {
    double hSpace = (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double sp)) ? sp : 0;
        string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
        string alignX = (node.Props != null && node.Props.ContainsKey("align_x")) ? node.Props["align_x"].ToString() : "left";
        string alignY = (node.Props != null && node.Props.ContainsKey("align_y")) ? node.Props["align_y"].ToString() : "center";
        bool isHdr = (node.Props != null && node.Props.ContainsKey("is_header_bar") && node.Props["is_header_bar"] is bool bHdr && bHdr);
        int childCount = node.Children != null ? node.Children.Count : 0;
        bool isSpaceBetween = alignX == "space_between" || (isHdr && childCount == 2);
        bool hasFlex = false;
        if (node.Children != null) {
            foreach (var c in node.Children) {
                string cw = (c.Props != null && c.Props.ContainsKey("width")) ? c.Props["width"].ToString() : "";
                bool hasSize = c.Props != null && c.Props.ContainsKey("size") && double.TryParse(c.Props["size"].ToString(), out double sz) && sz > 0;
                if (cw == "fill" || (c.Name == "spacer" && !hasSize) || (c.Name == "vstack" && cw != "hug")) {
                    hasFlex = true;
                    break;
                }
            }
        }
        bool isFill = widthProp == "fill" || (widthProp != "hug" && (hasFlex || isSpaceBetween));
        var vAlign = alignY == "top" ? VerticalAlignment.Top : (alignY == "bottom" ? VerticalAlignment.Bottom : VerticalAlignment.Center);
    
        if (isFill && (hasFlex || isSpaceBetween) && node.Children != null) {
            var grid = new Grid { ColumnSpacing = hSpace, HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = vAlign };
            ApplyWindowsContainerStyle(grid, node);
            int colIdx = 0;
            for (int i = 0; i < childCount; i++) {
                if (isSpaceBetween && i > 0) {
                    grid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
                    colIdx++;
                }
                var c = node.Children[i];
                string cw = (c.Props != null && c.Props.ContainsKey("width")) ? c.Props["width"].ToString() : "";
                bool hasSize = c.Props != null && c.Props.ContainsKey("size") && double.TryParse(c.Props["size"].ToString(), out double sz) && sz > 0;
                bool cFlex = !isSpaceBetween && (cw == "fill" || (c.Name == "spacer" && !hasSize) || (c.Name == "vstack" && cw != "hug"));
                grid.ColumnDefinitions.Add(new ColumnDefinition { Width = cFlex ? new GridLength(1, GridUnitType.Star) : GridLength.Auto });
                var el = RenderNode(c);
                if (el is FrameworkElement fe) {
                    fe.VerticalAlignment = vAlign;
                    Grid.SetColumn(fe, colIdx);
                }
                grid.Children.Add(el);
                colIdx++;
            }
            return grid;
        }
    
        var hp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = hSpace };
        hp.HorizontalAlignment = isFill ? HorizontalAlignment.Stretch : (alignX == "center" ? HorizontalAlignment.Center : (alignX == "right" ? HorizontalAlignment.Right : HorizontalAlignment.Left));
        hp.VerticalAlignment = vAlign;
        ApplyWindowsContainerStyle(hp, node);
        if (node.Children != null) {
            foreach (var c in node.Children) {
                var el = RenderNode(c);
                if (el is FrameworkElement fe) fe.VerticalAlignment = vAlign;
                hp.Children.Add(el);
            }
        }
        return hp;
}
"#;
