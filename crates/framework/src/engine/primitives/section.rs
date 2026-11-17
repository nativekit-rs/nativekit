
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Section {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Section {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.props.push(("spacing".to_string(), PropValue::Int(spacing)));
        self
    }
    pub fn header(mut self, content: impl IntoNode) -> Self {
        self.children.retain(|c| !matches!(c, Node::Element { name, .. } if name == "section_header"));
        self.children.push(Node::Element { name: "section_header".to_string(), props: vec![], children: vec![content.into_node()] });
        self
    }
    pub fn footer(mut self, content: impl IntoNode) -> Self {
        self.children.retain(|c| !matches!(c, Node::Element { name, .. } if name == "section_footer"));
        self.children.push(Node::Element { name: "section_footer".to_string(), props: vec![], children: vec![content.into_node()] });
        self
    }
    pub fn prominent(mut self) -> Self {
        self.props.retain(|(k, _)| k != "prominent");
        self.props.push(("prominent".to_string(), PropValue::Bool(true)));
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
impl IntoNode for Section {
    fn into_node(self) -> Node {
        Node::Element {
            name: "section".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const SECTION_ANDROID_SNIPPET: &str = r#"
"section" -> {
    val hdrNode = node.children.firstOrNull { it.name == "section_header" }?.children?.firstOrNull()
        val ftrNode = node.children.firstOrNull { it.name == "section_footer" }?.children?.firstOrNull()
        val bodyChildren = node.children.filter { it.name != "section_header" && it.name != "section_footer" }
        val count = bodyChildren.size
        Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            if (hdrNode != null) {
                Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp)) {
                    DynamicNodeView(hdrNode, dispatch, Modifier.fillMaxWidth())
                }
            }
            Column(modifier = Modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(2.dp)) {
                for (i in 0 until count) {
                    val child = bodyChildren[i]
                    val shape = when {
                        count == 1 -> androidx.compose.foundation.shape.RoundedCornerShape(16.dp)
                        i == 0 -> androidx.compose.foundation.shape.RoundedCornerShape(topStart = 16.dp, topEnd = 16.dp, bottomStart = 4.dp, bottomEnd = 4.dp)
                        i == count - 1 -> androidx.compose.foundation.shape.RoundedCornerShape(topStart = 4.dp, topEnd = 4.dp, bottomStart = 16.dp, bottomEnd = 16.dp)
                        else -> androidx.compose.foundation.shape.RoundedCornerShape(4.dp)
                    }
                    val isBtn = child.name == "button"
                    val isPicker = child.getBoolean("is_picker", false)
                    val actStr = child.getString("action", "")
                    val rowBgName = child.getString("background", "")
                    val rowBgColor = resolveAppColor(rowBgName)
                    val rowBgGrad = child.props["background_gradient"] as? org.json.JSONObject
                    val rowBrush = if (rowBgGrad != null) resolveAppGradientBrush(rowBgGrad) else null
                    val hasRowBg = rowBgColor != null || rowBrush != null
                    val cleanChild = if (hasRowBg) {
                        DynamicUINode(
                            child.name,
                            child.props.filterKeys { it != "background" && it != "background_gradient" },
                            child.children,
                            child.inList
                        )
                    } else {
                        child
                    }
                    Surface(
                        shape = shape,
                        color = if (rowBrush != null) Color.Transparent else (rowBgColor ?: MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.55f)),
                        modifier = Modifier.fillMaxWidth().clip(shape)
                            .let { m -> if (rowBrush != null) m.background(brush = rowBrush) else m }
                            .let { m -> if (isBtn && actStr.isNotEmpty()) m.clickable { dispatch(actStr, null) } else m }
                    ) {
                        Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 14.dp)) {
                            if (isPicker) {
                                Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween, verticalAlignment = Alignment.CenterVertically) {
                                    Text(child.getString("picker_title", ""), style = MaterialTheme.typography.bodyLarge)
                                    Text(child.getString("picker_value", "") + "  ›", style = MaterialTheme.typography.bodyMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                }
                            } else if (isBtn) {
                                Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                                    CompositionLocalProvider(LocalContentColor provides (resolveAppColor(child.getString("color", "")) ?: MaterialTheme.colorScheme.primary)) {
                                        cleanChild.children.forEach { DynamicNodeView(it, dispatch) }
                                    }
                                }
                            } else {
                                DynamicNodeView(cleanChild, dispatch, Modifier.fillMaxWidth())
                            }
                        }
                    }
                }
            }
            if (ftrNode != null) {
                Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 12.dp)) {
                    DynamicNodeView(ftrNode, dispatch, Modifier.fillMaxWidth())
                }
            }
        }
}
"#;

pub const SECTION_IOS_SNIPPET: &str = r#"
case "section":
    let headerStr = (node.props["header"] as? String) ?? (node.props["title"] as? String) ?? ""
        let footerStr = node.props["footer"] as? String ?? ""
        let isProminent = node.props["prominent"] as? Bool ?? false
        let headerNode = node.children.first { $0.name == "section_header" }?.children.first
        let footerNode = node.children.first { $0.name == "section_footer" }?.children.first
        let bodyChildren = node.children.filter { $0.name != "section_header" && $0.name != "section_footer" }
    
        Section {
            ForEach(0..<bodyChildren.count, id: \.self) { i in
                let child = bodyChildren[i]
                let delAction = child.props["on_delete"] as? String ?? ""
                let rowBgName = child.props["background"] as? String ?? ""
                let rowBgGrad = child.props["background_gradient"] as? [String: Any]
                let hasRowBg = !rowBgName.isEmpty || rowBgGrad != nil
                let cleanChild = hasRowBg
                    ? DynamicUINode(
                        name: child.name,
                        props: child.props.filter { $0.key != "background" && $0.key != "background_gradient" },
                        children: child.children
                    )
                    : child
                if !delAction.isEmpty {
                    DynamicNodeView(node: cleanChild, dispatch: dispatch)
                        .applyListRowBackground(node: child)
                        .swipeActions(edge: .trailing, allowsFullSwipe: true) {
                            Button(role: .destructive) {
                                dispatch(delAction, nil)
                            } label: {
                                Label("Delete", systemImage: "trash")
                            }
                        }
                } else {
                    DynamicNodeView(node: cleanChild, dispatch: dispatch)
                        .applyListRowBackground(node: child)
                }
            }
        } header: {
            if let headerNode = headerNode {
                if headerNode.name == "text"
                    && headerNode.props["style"] == nil
                    && headerNode.props["emphasized"] == nil
                    && headerNode.props["color"] == nil
                    && headerNode.props["opacity"] == nil
                    && headerNode.props["width"] == nil
                    && headerNode.props["height"] == nil
                    && headerNode.props["width_px"] == nil
                    && headerNode.props["height_px"] == nil
                    && headerNode.props["align_x"] == nil,
                   let label = headerNode.props["label"] as? String {
                    Text(label)
                } else {
                    DynamicNodeView(node: headerNode, dispatch: dispatch)
                }
            } else if !headerStr.isEmpty {
                Text(headerStr)
            }
        } footer: {
            if let footerNode = footerNode {
                if footerNode.name == "text"
                    && footerNode.props["style"] == nil
                    && footerNode.props["emphasized"] == nil
                    && footerNode.props["color"] == nil
                    && footerNode.props["opacity"] == nil
                    && footerNode.props["width"] == nil
                    && footerNode.props["height"] == nil
                    && footerNode.props["width_px"] == nil
                    && footerNode.props["height_px"] == nil
                    && footerNode.props["align_x"] == nil,
                   let label = footerNode.props["label"] as? String {
                    Text(label)
                } else {
                    DynamicNodeView(node: footerNode, dispatch: dispatch)
                }
            } else if !footerStr.isEmpty {
                Text(footerStr)
            }
        }
        .headerProminence(isProminent ? .increased : .standard)
"#;

pub const SECTION_GTK_SNIPPET: &str = r#"
"section" => {
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        if let Some(hdr) = node.children.iter().find(|c| c.name == "section_header").and_then(|c| c.children.first()) {
            let hw = parse_dynamic_widget(hdr, dispatch);
            hw.set_margin_start(4); hw.set_margin_end(4);
            outer.append(&hw);
        }
        let list_box = gtk4::ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        list_box.add_css_class("boxed-list");
        for child in node.children.iter().filter(|c| c.name != "section_header" && c.name != "section_footer") {
            let row = gtk4::ListBoxRow::new();
            let row_bg = child.get_string("background", "");
            let has_row_bg = !row_bg.is_empty() || child.props.contains_key("background_gradient");
            let mut clean_child = child.clone();
            if has_row_bg {
                let mut row_style_props = serde_json::Map::new();
                if let Some(bg_v) = child.props.get("background") {
                    row_style_props.insert("background".to_string(), bg_v.clone());
                }
                if let Some(grad_v) = child.props.get("background_gradient") {
                    row_style_props.insert("background_gradient".to_string(), grad_v.clone());
                }
                let row_style_node = DynamicUINode {
                    name: "row".to_string(),
                    props: row_style_props,
                    children: vec![],
                };
                apply_gtk_container_style(&row, &row_style_node);
                clean_child.props.remove("background");
                clean_child.props.remove("background_gradient");
            }
            if clean_child.name == "button" {
                row.set_activatable(true);
                row.set_widget_name(&clean_child.get_string("action", ""));
                let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
                row_box.set_margin_start(14); row_box.set_margin_end(14); row_box.set_margin_top(12); row_box.set_margin_bottom(12);
                if clean_child.get_bool("is_picker", false) {
                    let t_lbl = gtk4::Label::new(Some(&clean_child.get_string("picker_title", "Select")));
                    t_lbl.set_halign(gtk4::Align::Start); t_lbl.set_hexpand(true);
                    let v_lbl = gtk4::Label::new(Some(&format!("{}  ›", clean_child.get_string("picker_value", ""))));
                    v_lbl.add_css_class("dim-label");
                    row_box.append(&t_lbl); row_box.append(&v_lbl);
                } else {
                    let col = clean_child.get_string("color", "primary");
                    for gc in &clean_child.children {
                        let w = parse_dynamic_widget(gc, dispatch);
                        crate::generated_primitives::apply_gtk_fg_color(&w, &col);
                        row_box.append(&w);
                    }
                }
                row.set_child(Some(&row_box));
            } else {
                row.set_activatable(false);
                let cw = parse_dynamic_widget(&clean_child, dispatch);
                cw.set_margin_start(14); cw.set_margin_end(14); cw.set_margin_top(12); cw.set_margin_bottom(12);
                row.set_child(Some(&cw));
            }
            list_box.append(&row);
        }
        let disp = dispatch.clone();
        list_box.connect_row_activated(move |_, row| {
            let act = row.widget_name();
            if !act.is_empty() { disp(act.to_string(), None); }
        });
        outer.append(&list_box);
        if let Some(ftr) = node.children.iter().find(|c| c.name == "section_footer").and_then(|c| c.children.first()) {
            let fw = parse_dynamic_widget(ftr, dispatch);
            fw.add_css_class("dim-label"); fw.set_margin_start(8);
            outer.append(&fw);
        }
        outer.upcast::<gtk4::Widget>()
}
"#;

pub const SECTION_WEB_SNIPPET: &str = r#"
case "section": {
    const children = node.children || [];
        const hdrNode = children.find(c => c.name === "section_header");
        const ftrNode = children.find(c => c.name === "section_footer");
        const bodyChildren = children.filter(c => c.name !== "section_header" && c.name !== "section_footer");
        const rows = bodyChildren.map((child, idx, arr) => {
            const isLast = idx === arr.length - 1;
            const isBtn = child.name === "button";
            const isPicker = Boolean(child.props && child.props.is_picker);
            const act = (child.props && child.props.action) || "";
            const col = (child.props && child.props.color) || (isBtn && !isPicker ? "primary" : "");
            const rowBg = (child.props && child.props.background) || "";
            const rowGrad = (child.props && child.props.background_gradient) || null;
            const hasRowBg = Boolean(rowBg || rowGrad);
            const cleanChild = hasRowBg
                ? { ...child, props: Object.fromEntries(Object.entries(child.props || {}).filter(([k]) => k !== "background" && k !== "background_gradient")) }
                : child;
            const rowContent = isPicker
                ? [
                    h("span", { text: child.props.picker_title || "Select", style: { fontWeight: "400", color: "var(--rn-text)" } }),
                    h("span", { text: (child.props.picker_value || "") + "  ›", style: { color: "var(--rn-on-surface-variant, #8e8e93)", fontSize: "15px" } })
                  ]
                : (isBtn ? renderChildren(cleanChild) : renderNode(cleanChild));
            const rowEl = h("div", {
                className: "nk-section-row" + (isBtn ? " clickable" : ""),
                fgColor: col,
                style: {
                    padding: "13px 16px",
                    borderBottom: isLast ? "none" : "1px solid var(--rn-outline-variant, rgba(128,128,128,0.2))",
                    display: "flex", alignItems: "center", justifyContent: isPicker ? "space-between" : "flex-start",
                    gap: "8px", width: "100%", cursor: isBtn ? "pointer" : "default",
                },
                onClick: (isBtn && act) ? () => dispatch(act, null) : null,
            }, rowContent);
            if (rowGrad) applyWebGradient(rowEl, rowGrad);
            else if (rowBg) applyWebBgColor(rowEl, rowBg);
            return rowEl;
        });
        return h("div", { className: "nk-section", style: { display: "flex", flexDirection: "column", gap: "6px", width: "100%" } }, [
            hdrNode && hdrNode.children[0] ? h("div", { style: { padding: "0 8px" } }, renderNode(hdrNode.children[0])) : null,
            h("div", { style: { backgroundColor: "var(--rn-surface, #ffffff)", borderRadius: "14px", overflow: "hidden", border: "1px solid var(--rn-outline-variant, rgba(128,128,128,0.15))" } }, rows),
            ftrNode && ftrNode.children[0] ? h("div", { style: { padding: "0 12px", opacity: "0.7", fontSize: "12px" } }, renderNode(ftrNode.children[0])) : null,
        ]);
}
"#;

pub const SECTION_WINDOWS_SNIPPET: &str = r#"
case "section": {
    var wrap = new StackPanel { Orientation = Orientation.Vertical, Spacing = 6, HorizontalAlignment = HorizontalAlignment.Stretch };
        bool isProminent = (node.Props != null && node.Props.ContainsKey("prominent") && node.Props["prominent"] is bool pr && pr);
        DynamicUINode hdrNode = null, ftrNode = null;
        var body = new List<DynamicUINode>();
        if (node.Children != null) {
            foreach (var c in node.Children) {
                if (c.Name == "section_header" && c.Children != null && c.Children.Count > 0) hdrNode = c.Children[0];
                else if (c.Name == "section_footer" && c.Children != null && c.Children.Count > 0) ftrNode = c.Children[0];
                else body.Add(c);
            }
        }
        if (hdrNode != null) {
            var hdrEl = RenderNode(hdrNode);
            if (hdrNode.Name == "text" && hdrEl is TextBlock hdrTb) {
                bool hasStyle = hdrNode.Props != null && hdrNode.Props.ContainsKey("style");
                bool hasEmp = hdrNode.Props != null && hdrNode.Props.ContainsKey("emphasized");
                bool hasCol = hdrNode.Props != null && hdrNode.Props.ContainsKey("color");
                if (!hasStyle && !hasEmp) {
                    hdrTb.FontSize = isProminent ? 20 : 13;
                    hdrTb.FontWeight = Microsoft.UI.Text.FontWeights.SemiBold;
                }
                if (!hasCol && !isProminent) {
                    var secFg = ResolveAppBrush("text_secondary");
                    if (secFg != null) hdrTb.Foreground = secFg;
                }
            }
            wrap.Children.Add(hdrEl);
        }
        var rowsPanel = new StackPanel { Orientation = Orientation.Vertical, Spacing = 2, HorizontalAlignment = HorizontalAlignment.Stretch };
        int count = body.Count;
        for (int i = 0; i < count; i++) {
            var c = body[i];
            var rad = count == 1 ? new CornerRadius(8) : (i == 0 ? new CornerRadius(8, 8, 2, 2) : (i == count - 1 ? new CornerRadius(2, 2, 8, 8) : new CornerRadius(2)));
            bool isBtn = (c.Name == "button");
            Microsoft.UI.Xaml.Media.Brush rowBgBrush = ResolveNodeBackgroundBrush(c, true);
            UIElement rowChild;
            if (isBtn && c.Children != null && c.Children.Count > 0) {
                var sp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8 };
                foreach (var gc in c.Children) {
                    var r = RenderNode(gc);
                    if (r is TextBlock rowTb) {
                        var fg = ResolveAppBrush((c.Props != null && c.Props.ContainsKey("color")) ? c.Props["color"].ToString() : "primary");
                        if (fg != null) rowTb.Foreground = fg;
                    }
                    sp.Children.Add(r);
                }
                rowChild = sp;
            } else {
                rowChild = RenderNode(c);
            }
            var card = new Border {
                CornerRadius = rad, Padding = new Thickness(16, 12, 16, 12), HorizontalAlignment = HorizontalAlignment.Stretch,
                Background = rowBgBrush ?? (Application.Current.Resources["CardBackgroundFillColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush),
                BorderBrush = Application.Current.Resources["CardStrokeColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush,
                BorderThickness = new Thickness(1), Child = rowChild
            };
            if (isBtn && c.Props != null && c.Props.ContainsKey("action")) {
                string act = c.Props["action"].ToString();
                card.PointerPressed += (s, e) => { if (!string.IsNullOrEmpty(act)) Dispatch(act, ""); };
            }
            rowsPanel.Children.Add(card);
        }
        wrap.Children.Add(rowsPanel);
        if (ftrNode != null) wrap.Children.Add(RenderNode(ftrNode));
        return wrap;
}
"#;
