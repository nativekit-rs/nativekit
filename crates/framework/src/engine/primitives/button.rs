// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// Generated from config/colors.json & blueprints/button
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Button {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }

    /// Valid options: primary, secondary, text
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "variant");
        self.props.push(("variant".to_string(), PropValue::String(variant.into())));
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

    /// Valid options: "fill", "hug"
    pub fn placement(mut self, placement: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "placement");
        self.props.push(("placement".to_string(), PropValue::String(placement.into())));
        self
    }

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.props.retain(|(k, _)| k != "width");
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
    }

    pub fn action<H, Args, S>(mut self, handler: H) -> Self
    where
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static,
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("action".to_string(), PropValue::String(id)));
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

impl IntoNode for Button {
    fn into_node(self) -> Node {
        Node::Element {
            name: "button".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const BUTTON_ANDROID_SNIPPET: &str = r#"
"button" -> {
    val variant = node.getString("variant", "")
        val actionStr = node.getString("action", "")
        val colorName = node.getString("color", "")
        val opacity = node.getInt("opacity", 100).toFloat() / 100f
        val isFill = node.getString("width") == "fill"
        val customColor = resolveAppColor(colorName)
        var btnMod = if (isFill && modifier == Modifier) modifier.fillMaxWidth() else modifier
        if (opacity < 1f) btnMod = btnMod.graphicsLayer(alpha = opacity)
        val onClick: () -> Unit = { if (actionStr.isNotEmpty()) dispatch(actionStr, null) }
    
        when (variant) {
            "primary" -> Button(
                onClick = onClick,
                modifier = btnMod,
                colors = if (customColor != null) ButtonDefaults.buttonColors(containerColor = customColor) else ButtonDefaults.buttonColors()
            ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
            "secondary" -> FilledTonalButton(
                onClick = onClick,
                modifier = btnMod,
                colors = if (customColor != null) ButtonDefaults.filledTonalButtonColors(containerColor = customColor) else ButtonDefaults.filledTonalButtonColors()
            ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
            else -> TextButton(
                onClick = onClick,
                modifier = btnMod,
                colors = if (customColor != null) ButtonDefaults.textButtonColors(contentColor = customColor) else ButtonDefaults.textButtonColors()
            ) { node.children.forEach { DynamicNodeView(it, dispatch) } }
        }
}
"#;

pub const BUTTON_IOS_SNIPPET: &str = r#"
case "button":
    let variant = node.props["variant"] as? String ?? ""
        let action_str = node.props["action"] as? String ?? ""
        let colorName = node.props["color"] as? String ?? ""
        let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
        let isFill = (node.props["width"] as? String) == "fill"
        let inSection = node.props["in_section"] as? Bool ?? false
        let isPicker = node.props["is_picker"] as? Bool ?? false
        let customColor = resolveAppColor(colorName)
        let onTap: () -> Void = { if !action_str.isEmpty { dispatch(action_str, nil) } }
    
        if isPicker {
            let pTitle = node.props["picker_title"] as? String ?? "Select"
            let pVal = node.props["picker_value"] as? String ?? ""
            Button(action: onTap) {
                HStack {
                    Text(pTitle).foregroundStyle(Color.primary)
                    Spacer()
                    Text(pVal).foregroundStyle(Color.secondary)
                    Image(systemName: "chevron.right").font(.caption.weight(.semibold)).foregroundStyle(Color(uiColor: .tertiaryLabel))
                }
                .contentShape(Rectangle())
            }
            .buttonStyle(.plain)
        } else if inSection {
            Button(action: onTap) {
                HStack(spacing: 8) {
                    ForEach(0..<node.children.count, id: \.self) { i in
                        DynamicNodeView(node: node.children[i], dispatch: dispatch)
                    }
                    Spacer()
                }
                .contentShape(Rectangle())
            }
            .foregroundStyle(customColor ?? AppTheme.primary)
            .opacity(opacity)
        } else if variant == "primary" {
            Button(action: onTap) {
                HStack(spacing: 6) {
                    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
                }
                .fixedSize(horizontal: !isFill, vertical: false)
                .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            .buttonStyle(.borderedProminent)
            .tint(customColor)
            .opacity(opacity)
        } else if variant == "secondary" {
            Button(action: onTap) {
                HStack(spacing: 6) {
                    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
                }
                .fixedSize(horizontal: !isFill, vertical: false)
                .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            .buttonStyle(.bordered)
            .tint(customColor)
            .opacity(opacity)
        } else {
            Button(action: onTap) {
                HStack(spacing: 6) {
                    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
                }
                .fixedSize(horizontal: !isFill, vertical: false)
                .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            }
            .fixedSize(horizontal: !isFill, vertical: false)
            .frame(maxWidth: isFill ? .infinity : nil, alignment: .center)
            .buttonStyle(.borderless)
            .tint(customColor)
            .opacity(opacity)
        }
"#;

pub const BUTTON_GTK_SNIPPET: &str = r#"
"button" => {
    let btn = gtk4::Button::new();
        let variant = node.get_string("variant", "");
        let color = node.get_string("color", "");
        let opacity = node.get_int("opacity", 100);
        let is_fill = node.get_string("width", "") == "fill";
        btn.set_halign(if is_fill { gtk4::Align::Fill } else { gtk4::Align::Start });
        btn.set_hexpand(is_fill);
    
        if variant == "primary" {
            btn.add_css_class("suggested-action");
        } else if variant != "secondary" {
            btn.add_css_class("flat");
        }
        apply_gtk_fg_color(&btn, &color);
        if opacity < 100 {
            btn.set_opacity((opacity as f64) / 100.0);
        }
        if node.children.len() == 1 {
            let child_w = parse_dynamic_widget(&node.children[0], dispatch);
            child_w.set_halign(gtk4::Align::Center);
            btn.set_child(Some(&child_w));
        } else if !node.children.is_empty() {
            let box_w = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
            box_w.set_halign(gtk4::Align::Center);
            for c in &node.children {
                box_w.append(&parse_dynamic_widget(c, dispatch));
            }
            btn.set_child(Some(&box_w));
        }
        let action_str = node.get_string("action", "");
        btn.set_widget_name(&action_str);
        let disp = dispatch.clone();
        btn.connect_clicked(move |b| {
            let act = b.widget_name();
            if !act.is_empty() {
                disp(act.to_string(), None);
            }
        });
        btn.upcast::<gtk4::Widget>()
}
"#;

pub const BUTTON_WEB_SNIPPET: &str = r#"
case "button": {
    const { variant = "", action = "", color = "", opacity = 100, width = "" } = node.props || {};
        const isFill = width === "fill";
        const el = h("button", {
            className: variant === "primary" ? "primary" : (variant === "secondary" ? "secondary" : "text"),
            fgColor: (color && variant !== "primary") ? color : "",
            bgColor: (color && variant === "primary") ? color : "",
            style: {
                width: isFill ? "100%" : "fit-content",
                alignSelf: isFill ? "stretch" : "flex-start",
                flex: isFill ? "1" : "none",
                opacity: opacity < 100 ? String(opacity / 100.0) : "",
            },
            onClick: () => { if (el.__action) dispatch(el.__action, null); },
        }, renderChildren(node));
        el.__action = action;
        el.__hasListener = true;
        return el;
}
"#;

pub const BUTTON_WINDOWS_SNIPPET: &str = r#"
case "button": {
    var btn = new Button();
        string variant = (node.Props != null && node.Props.ContainsKey("variant")) ? node.Props["variant"].ToString() : "";
        if (variant == "primary") {
            try { btn.Style = Application.Current.Resources["AccentButtonStyle"] as Style; var pb = ResolveAppBrush("primary"); if (pb != null) btn.Background = pb; } catch { }
        } else if (variant != "secondary") {
            btn.Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent);
            btn.BorderThickness = new Thickness(0);
            var fg = ResolveAppBrush("primary");
            if (fg != null) btn.Foreground = fg;
        }
        bool isFill = (node.Props != null && node.Props.ContainsKey("width") && node.Props["width"].ToString() == "fill");
        btn.HorizontalAlignment = isFill ? HorizontalAlignment.Stretch : HorizontalAlignment.Left;
        btn.HorizontalContentAlignment = HorizontalAlignment.Center;
        if (node.Props != null && node.Props.ContainsKey("color")) {
            var brush = ResolveAppBrush(node.Props["color"].ToString());
            if (brush != null) {
                if (variant == "primary" || variant == "secondary") btn.Background = brush;
                else btn.Foreground = brush;
            }
        }
        if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double bOp)) {
            btn.Opacity = bOp / 100.0;
        }
        if (node.Children != null && node.Children.Count > 0) {
            btn.Content = RenderNode(node.Children[0]);
        }
        if (node.Props != null && node.Props.ContainsKey("action")) {
            btn.Tag = node.Props["action"].ToString();
            btn.Click += (s, e) => { if (btn.Tag is string act) Dispatch(act, ""); };
        }
        return btn;
}
"#;
