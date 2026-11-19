
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Grid {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Grid {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn item_max_width(mut self, item_max_width: u32) -> Self {
        self.props.push(("item_max_width".to_string(), PropValue::Int(item_max_width)));
        self
    }
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.props.push(("spacing".to_string(), PropValue::Int(spacing)));
        self
    }
    pub fn item_min_width(mut self, item_min_width: u32) -> Self {
        self.props.push(("item_min_width".to_string(), PropValue::Int(item_min_width)));
        self
    }
    pub fn columns(mut self, columns: u32) -> Self {
        self.props.push(("columns".to_string(), PropValue::Int(columns)));
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
    pub fn wrap(mut self) -> Self {
        self.props.retain(|(k, _)| k != "wrap");
        self.props.push(("wrap".to_string(), PropValue::Bool(true)));
        self
    }
    pub fn scroll_freely(mut self) -> Self {
        self.props.retain(|(k, _)| k != "scroll_freely");
        self.props.push(("scroll_freely".to_string(), PropValue::Bool(true)));
        self
    }
    pub fn uncontained(mut self) -> Self {
        self.props.retain(|(k, _)| k != "uncontained");
        self.props.push(("uncontained".to_string(), PropValue::Bool(true)));
        self
    }
    pub fn item_width(mut self, min: u32, max: u32) -> Self {
        let actual_min = min.min(max);
        let actual_max = min.max(max);
        self.props.retain(|(k, _)| k != "item_min_width" && k != "item_max_width");
        self.props.push(("item_min_width".to_string(), PropValue::Int(actual_min)));
        self.props.push(("item_max_width".to_string(), PropValue::Int(actual_max)));
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
impl IntoNode for Grid {
    fn into_node(self) -> Node {
        Node::Element {
            name: "grid".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const GRID_ANDROID_SNIPPET: &str = r#"
"grid" -> {
    val cols = maxOf(1, node.getInt("columns", 2))
        val hasExplicitCols = node.props.containsKey("columns")
        val spacing = node.getInt("spacing", 12)
        val isWrap = node.getBoolean("wrap", false)
        val scrollFreely = node.getBoolean("scroll_freely", false)
        val isUncontained = node.getBoolean("uncontained", false)
        val minW = node.getInt("item_min_width", 0)
        val maxW = node.getInt("item_max_width", 0)
        val hasItemWidth = minW > 0 && maxW > 0
        val hdrNode = node.children.firstOrNull { it.name == "section_header" }?.children?.firstOrNull()
        val ftrNode = node.children.firstOrNull { it.name == "section_footer" }?.children?.firstOrNull()
        val bodyChildren = node.children.filter { it.name != "section_header" && it.name != "section_footer" }
        val cardShape = androidx.compose.foundation.shape.RoundedCornerShape(16.dp)
    
        @Composable
        fun RenderGridCard(child: DynamicUINode, cardMod: Modifier) {
            if (isUncontained) {
                Box(modifier = cardMod) {
                    DynamicNodeView(child, dispatch, Modifier.fillMaxWidth())
                }
                return
            }
            val isBtn = child.name == "button"
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
                shape = cardShape,
                color = if (rowBrush != null) Color.Transparent else (rowBgColor ?: MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.55f)),
                modifier = cardMod.clip(cardShape)
                    .let { m -> if (rowBrush != null) m.background(brush = rowBrush) else m }
                    .let { m -> if (isBtn && actStr.isNotEmpty()) m.clickable { dispatch(actStr, null) } else m }
            ) {
                Box(modifier = Modifier.fillMaxWidth()) {
                    DynamicNodeView(cleanChild, dispatch, Modifier.fillMaxWidth())
                }
            }
        }
    
        Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(spacing.dp)) {
            if (hdrNode != null) {
                Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 4.dp)) { DynamicNodeView(hdrNode, dispatch, Modifier.fillMaxWidth()) }
            }
            if (isWrap) {
                BoxWithConstraints(modifier = Modifier.fillMaxWidth()) {
                    val effectiveCols = if (hasItemWidth && !hasExplicitCols) {
                        maxOf(1, ((maxWidth + spacing.dp) / (minW.dp + spacing.dp)).toInt())
                    } else {
                        cols
                    }
                    val rawItemWidth = (maxWidth - (spacing.dp * (effectiveCols - 1))) / effectiveCols
                    val cardWidth = if (hasItemWidth) rawItemWidth.coerceIn(minW.dp, maxW.dp) else rawItemWidth
                    val rows = bodyChildren.chunked(effectiveCols)
                    Column(modifier = Modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(spacing.dp)) {
                        rows.forEach { rowItems ->
                            Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(spacing.dp)) {
                                for (i in 0 until effectiveCols) {
                                    if (i < rowItems.size) {
                                        RenderGridCard(rowItems[i], if (hasItemWidth) Modifier.width(cardWidth) else Modifier.weight(1f))
                                    } else if (!hasItemWidth) {
                                        Spacer(modifier = Modifier.weight(1f))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                BoxWithConstraints(modifier = Modifier.fillMaxWidth()) {
                    val rawItemWidth = (maxWidth - (spacing.dp * (cols - 1))) / cols
                    val itemWidth = if (hasItemWidth) rawItemWidth.coerceIn(minW.dp, maxW.dp) else rawItemWidth
                    val listState = androidx.compose.foundation.lazy.rememberLazyListState()
                    val flingBehavior = if (scrollFreely) {
                        androidx.compose.foundation.gestures.ScrollableDefaults.flingBehavior()
                    } else {
                        androidx.compose.foundation.gestures.snapping.rememberSnapFlingBehavior(lazyListState = listState)
                    }
                    androidx.compose.foundation.lazy.LazyRow(
                        state = listState,
                        flingBehavior = flingBehavior,
                        contentPadding = androidx.compose.foundation.layout.PaddingValues(horizontal = 16.dp),
                        horizontalArrangement = Arrangement.spacedBy(spacing.dp),
                        modifier = Modifier
                            .fillMaxWidth()
                            .layout { measurable, constraints ->
                                val bleed = 16.dp.roundToPx()
                                val expandedMaxW = if (constraints.maxWidth == androidx.compose.ui.unit.Constraints.Infinity) {
                                    constraints.maxWidth
                                } else {
                                    constraints.maxWidth + bleed * 2
                                }
                                val placeable = measurable.measure(
                                    constraints.copy(
                                        minWidth = (constraints.minWidth + bleed * 2).coerceAtMost(expandedMaxW),
                                        maxWidth = expandedMaxW
                                    )
                                )
                                layout((placeable.width - bleed * 2).coerceAtLeast(0), placeable.height) {
                                    placeable.place(-bleed, 0)
                                }
                            }
                    ) {
                        items(bodyChildren.size) { idx ->
                            RenderGridCard(bodyChildren[idx], Modifier.width(itemWidth))
                        }
                    }
                }
            }
            if (ftrNode != null) {
                Box(modifier = Modifier.fillMaxWidth().padding(horizontal = 4.dp)) { DynamicNodeView(ftrNode, dispatch, Modifier.fillMaxWidth()) }
            }
        }
}
"#;

pub const GRID_IOS_SNIPPET: &str = r#"
case "grid":
    let cols = max(1, node.props["columns"] as? Int ?? 2)
        let hasExplicitCols = node.props["columns"] != nil
        let spacing = CGFloat(node.props["spacing"] as? Int ?? 12)
        let isWrap = node.props["wrap"] as? Bool ?? false
        let scrollFreely = node.props["scroll_freely"] as? Bool ?? false
        let isUncontained = node.props["uncontained"] as? Bool ?? false
        let minW = CGFloat(node.props["item_min_width"] as? Int ?? 0)
        let maxW = CGFloat(node.props["item_max_width"] as? Int ?? 0)
        let hasItemWidth = minW > 0 && maxW > 0
        let isProminent = node.props["prominent"] as? Bool ?? false
        let headerStr = node.props["header"] as? String ?? ""
        let footerStr = node.props["footer"] as? String ?? ""
        let headerNode = node.children.first { $0.name == "section_header" }?.children.first
        let footerNode = node.children.first { $0.name == "section_footer" }?.children.first
        let bodyChildren = node.children.filter { $0.name != "section_header" && $0.name != "section_footer" }
        
        let gridCols: [GridItem] = hasItemWidth
            ? (hasExplicitCols
                ? Array(repeating: GridItem(.flexible(minimum: minW, maximum: maxW), spacing: spacing), count: cols)
                : [GridItem(.adaptive(minimum: minW, maximum: maxW), spacing: spacing)])
            : Array(repeating: GridItem(.flexible(), spacing: spacing), count: cols)
    
        Section {
            // --- Body Content ---
            if isWrap {
                LazyVGrid(columns: gridCols, alignment: .leading, spacing: spacing) {
                    ForEach(0..<bodyChildren.count, id: \.self) { i in
                        let child = bodyChildren[i]
                        if isUncontained {
                            DynamicNodeView(node: child, dispatch: dispatch)
                                .frame(minWidth: hasItemWidth ? minW : nil, maxWidth: hasItemWidth ? maxW : .infinity, maxHeight: .infinity, alignment: .leading)
                        } else {
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
                            DynamicNodeView(node: cleanChild, dispatch: dispatch)
                                .frame(minWidth: hasItemWidth ? minW : nil, maxWidth: hasItemWidth ? maxW : .infinity, maxHeight: .infinity, alignment: .leading)
                                .background {
                                    if let grad = rowBgGrad {
                                        renderGradientView(grad)
                                    } else if !rowBgName.isEmpty, let col = resolveAppColor(rowBgName) {
                                        col
                                    } else {
                                        Color(uiColor: .secondarySystemGroupedBackground)
                                    }
                                }
                                .clipShape(RoundedRectangle(cornerRadius: 26, style: .continuous))
                        }
                    }
                }
                .listRowInsets(EdgeInsets(top: 6, leading: 0, bottom: 6, trailing: 0))
                .listRowBackground(Color.clear)
                .listRowSeparator(.hidden)
            } else {
                // Using standard HStack instead of LazyHStack prevents cards from popping
                // out of existence when moving past the scroll viewport bounds under scrollClipDisabled().
                let hScroll = ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: spacing) {
                        ForEach(0..<bodyChildren.count, id: \.self) { i in
                            let child = bodyChildren[i]
                            Group {
                                if isUncontained {
                                    DynamicNodeView(node: child, dispatch: dispatch)
                                        .frame(maxHeight: .infinity, alignment: .leading)
                                } else {
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
                                    DynamicNodeView(node: cleanChild, dispatch: dispatch)
                                        .frame(maxHeight: .infinity, alignment: .leading)
                                        .background {
                                            if let grad = rowBgGrad {
                                                renderGradientView(grad)
                                            } else if !rowBgName.isEmpty, let col = resolveAppColor(rowBgName) {
                                                col
                                            } else {
                                                Color(uiColor: .secondarySystemGroupedBackground)
                                            }
                                        }
                                        .clipShape(RoundedRectangle(cornerRadius: 26, style: .continuous))
                                }
                            }
                            .containerRelativeFrame(.horizontal) { length, _ in
                                let raw = max(0, (length - spacing * CGFloat(cols - 1)) / CGFloat(cols))
                                return hasItemWidth ? min(maxW, max(minW, raw)) : raw
                            }
                        }
                    }
                    .scrollTargetLayout()
                    .background(UnclipListCell())
                }
                .scrollClipDisabled()
                let scrollContent = Group {
                    if scrollFreely {
                        hScroll
                    } else {
                        hScroll.scrollTargetBehavior(.viewAligned)
                    }
                }
                scrollContent
                    .listRowInsets(EdgeInsets(top: 6, leading: 0, bottom: 6, trailing: 0))
                    .listRowBackground(Color.clear)
                    .listRowSeparator(.hidden)
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

pub const GRID_GTK_SNIPPET: &str = r#"
"grid" => {
    let cols = std::cmp::max(1, node.get_int("columns", 2)) as i32;
        let has_explicit_cols = node.props.contains_key("columns");
        let spacing = node.get_int("spacing", 12);
        let is_wrap = node.get_bool("wrap", false);
        let scroll_freely = node.get_bool("scroll_freely", false);
        let is_uncontained = node.get_bool("uncontained", false);
        let min_w = node.get_int("item_min_width", 0) as i32;
        let max_w = node.get_int("item_max_width", 0) as i32;
        let has_item_width = min_w > 0 && max_w > 0;
        let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
        if let Some(hdr) = node.children.iter().find(|c| c.name == "section_header").and_then(|c| c.children.first()) {
            let hw = parse_dynamic_widget(hdr, dispatch);
            hw.set_margin_start(4); hw.set_margin_end(4);
            outer.append(&hw);
        }
        let mut cards: Vec<gtk4::Widget> = Vec::new();
        for child in node.children.iter().filter(|c| c.name != "section_header" && c.name != "section_footer") {
            let card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
            card.set_hexpand(true);
            if is_uncontained {
                let cw = parse_dynamic_widget(child, dispatch);
                cw.set_hexpand(true);
                card.append(&cw);
            } else {
                card.add_css_class("card");
                card.set_overflow(gtk4::Overflow::Hidden);
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
                    apply_gtk_container_style(&card, &row_style_node);
                    clean_child.props.remove("background");
                    clean_child.props.remove("background_gradient");
                }
                let cw = parse_dynamic_widget(&clean_child, dispatch);
                cw.set_hexpand(true);
                card.append(&cw);
            }
            if has_item_width {
                card.set_size_request(min_w, -1);
                let clamp = libadwaita::Clamp::new();
                clamp.set_maximum_size(max_w);
                clamp.set_tightening_threshold(min_w);
                clamp.set_hexpand(true);
                clamp.set_child(Some(&card));
                cards.push(clamp.upcast::<gtk4::Widget>());
            } else {
                cards.push(card.upcast::<gtk4::Widget>());
            }
        }
        if is_wrap {
            if has_item_width && !has_explicit_cols {
                let flow = gtk4::FlowBox::new();
                flow.set_selection_mode(gtk4::SelectionMode::None);
                flow.set_column_spacing(spacing);
                flow.set_row_spacing(spacing);
                flow.set_homogeneous(true);
                flow.set_hexpand(true);
                for card in cards {
                    flow.append(&card);
                }
                outer.append(&flow);
            } else {
                let grid = gtk4::Grid::new();
                grid.set_column_homogeneous(true);
                grid.set_column_spacing(spacing);
                grid.set_row_spacing(spacing);
                for (i, card) in cards.into_iter().enumerate() {
                    grid.attach(&card, (i as i32) % cols, (i as i32) / cols, 1, 1);
                }
                outer.append(&grid);
            }
        } else if scroll_freely {
            let sw = gtk4::ScrolledWindow::new();
            sw.set_hexpand(true);
            sw.set_overflow(gtk4::Overflow::Visible);
            sw.set_hscrollbar_policy(gtk4::PolicyType::Automatic);
            sw.set_vscrollbar_policy(gtk4::PolicyType::Never);
            sw.set_propagate_natural_height(true);
            let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, spacing as i32);
            for card in cards {
                if !has_item_width {
                    card.set_size_request(160, -1);
                }
                row_box.append(&card);
            }
            sw.set_child(Some(&row_box));
            outer.append(&sw);
        } else {
            let carousel = libadwaita::Carousel::new();
            carousel.set_hexpand(true);
            carousel.set_overflow(gtk4::Overflow::Visible);
            carousel.set_allow_mouse_drag(true);
            carousel.set_allow_scroll_wheel(true);
            carousel.set_allow_long_swipes(true);
            carousel.set_spacing(spacing);
            let cols_usize = cols as usize;
            for chunk in cards.chunks(cols_usize) {
                let page_grid = gtk4::Grid::new();
                page_grid.set_column_homogeneous(true);
                page_grid.set_column_spacing(spacing);
                page_grid.set_hexpand(true);
                for col_idx in 0..cols_usize {
                    if let Some(card) = chunk.get(col_idx) {
                        page_grid.attach(card, col_idx as i32, 0, 1, 1);
                    } else {
                        let spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
                        spacer.set_hexpand(true);
                        page_grid.attach(&spacer, col_idx as i32, 0, 1, 1);
                    }
                }
                carousel.append(&page_grid);
            }
            outer.append(&carousel);
            let dots = libadwaita::CarouselIndicatorDots::new();
            dots.set_carousel(Some(&carousel));
            outer.append(&dots);
        }
        if let Some(ftr) = node.children.iter().find(|c| c.name == "section_footer").and_then(|c| c.children.first()) {
            let fw = parse_dynamic_widget(ftr, dispatch);
            fw.add_css_class("dim-label"); fw.set_margin_start(4);
            outer.append(&fw);
        }
        outer.upcast::<gtk4::Widget>()
}
"#;

pub const GRID_WEB_SNIPPET: &str = r#"
case "grid": {
    const cols = Math.max(1, (node.props && node.props.columns) || 2);
        const hasExplicitCols = Boolean(node.props && node.props.columns !== undefined);
        const spacing = (node.props && node.props.spacing) || 12;
        const isWrap = Boolean(node.props && node.props.wrap);
        const scrollFreely = Boolean(node.props && node.props.scroll_freely);
        const isUncontained = Boolean(node.props && node.props.uncontained);
        const minW = (node.props && node.props.item_min_width) || 0;
        const maxW = (node.props && node.props.item_max_width) || 0;
        const hasItemWidth = minW > 0 && maxW > 0;
        const children = node.children || [];
        const hdrNode = children.find(c => c.name === "section_header");
        const ftrNode = children.find(c => c.name === "section_footer");
        const bodyChildren = children.filter(c => c.name !== "section_header" && c.name !== "section_footer").map(child => {
            if (isUncontained) {
                return h("div", {
                    className: "nk-grid-card uncontained",
                    style: {
                        boxSizing: "border-box",
                        scrollSnapAlign: (isWrap || scrollFreely) ? "none" : "start",
                        ...(hasItemWidth ? { minWidth: `${minW}px`, maxWidth: `${maxW}px`, width: "100%" } : {}),
                    },
                }, renderNode(child));
            }
            const isBtn = child.name === "button";
            const act = (child.props && child.props.action) || "";
            const rowBg = (child.props && child.props.background) || "";
            const rowGrad = (child.props && child.props.background_gradient) || null;
            const hasRowBg = Boolean(rowBg || rowGrad);
            const cleanChild = hasRowBg
                ? { ...child, props: Object.fromEntries(Object.entries(child.props || {}).filter(([k]) => k !== "background" && k !== "background_gradient")) }
                : child;
            const cardEl = h("div", {
                className: "nk-grid-card" + (isBtn ? " clickable" : ""),
                style: {
                    padding: "0px",
                    borderRadius: "14px",
                    backgroundColor: "var(--rn-surface, #ffffff)",
                    border: "1px solid var(--rn-outline-variant, rgba(128,128,128,0.15))",
                    overflow: "hidden",
                    boxSizing: "border-box",
                    scrollSnapAlign: (isWrap || scrollFreely) ? "none" : "start",
                    cursor: isBtn ? "pointer" : "default",
                    ...(hasItemWidth ? { minWidth: `${minW}px`, maxWidth: `${maxW}px`, width: "100%" } : {}),
                },
                onClick: (isBtn && act) ? () => dispatch(act, null) : null,
            }, renderNode(cleanChild));
            if (rowGrad) applyWebGradient(cardEl, rowGrad);
            else if (rowBg) applyWebBgColor(cardEl, rowBg);
            return cardEl;
        });
        const wrapTemplate = hasItemWidth
            ? (hasExplicitCols ? `repeat(${cols}, minmax(${minW}px, ${maxW}px))` : `repeat(auto-fill, minmax(${minW}px, ${maxW}px))`)
            : `repeat(${cols}, minmax(0, 1fr))`;
        const scrollAutoCols = hasItemWidth
            ? `clamp(${minW}px, calc((100% - ${(cols - 1) * spacing}px) / ${cols}), ${maxW}px)`
            : `calc((100% - ${(cols - 1) * spacing}px) / ${cols})`;
        const gridStyle = isWrap
            ? { display: "grid", gridTemplateColumns: wrapTemplate, gap: `${spacing}px`, width: "100%" }
            : {
                display: "grid",
                gridAutoFlow: "column",
                gridAutoColumns: scrollAutoCols,
                gap: `${spacing}px`,
                width: "calc(100% + 32px)",
                marginLeft: "-16px",
                marginRight: "-16px",
                paddingLeft: "16px",
                paddingRight: "16px",
                scrollPaddingLeft: "16px",
                scrollPaddingRight: "16px",
                boxSizing: "border-box",
                overflowX: "auto",
                scrollSnapType: scrollFreely ? "none" : "x mandatory",
                scrollbarWidth: "none",
            };
        return h("div", { className: "nk-grid-wrap", style: { display: "flex", flexDirection: "column", gap: "8px", width: "100%" } }, [
            hdrNode && hdrNode.children[0] ? h("div", { style: { padding: "0 4px" } }, renderNode(hdrNode.children[0])) : null,
            h("div", { className: "nk-grid", style: gridStyle }, bodyChildren),
            ftrNode && ftrNode.children[0] ? h("div", { style: { padding: "0 4px", opacity: "0.7", fontSize: "12px" } }, renderNode(ftrNode.children[0])) : null,
        ]);
}
"#;

pub const GRID_WINDOWS_SNIPPET: &str = r#"
case "grid": {
    int cols = (node.Props != null && node.Props.ContainsKey("columns") && int.TryParse(node.Props["columns"].ToString(), out int cVal)) ? Math.Max(1, cVal) : 2;
        bool hasExplicitCols = (node.Props != null && node.Props.ContainsKey("columns"));
        double spacing = (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double sVal)) ? sVal : 12;
        bool isWrap = (node.Props != null && node.Props.ContainsKey("wrap") && node.Props["wrap"] is bool bWrap && bWrap);
        bool scrollFreely = (node.Props != null && node.Props.ContainsKey("scroll_freely") && node.Props["scroll_freely"] is bool bSf && bSf);
        bool isUncontained = (node.Props != null && node.Props.ContainsKey("uncontained") && node.Props["uncontained"] is bool bUnc && bUnc);
        double minW = (node.Props != null && node.Props.ContainsKey("item_min_width") && double.TryParse(node.Props["item_min_width"].ToString(), out double minVal)) ? minVal : 0;
        double maxW = (node.Props != null && node.Props.ContainsKey("item_max_width") && double.TryParse(node.Props["item_max_width"].ToString(), out double maxVal)) ? maxVal : 0;
        bool hasItemWidth = minW > 0 && maxW > 0;
        DynamicUINode hdrNode = null, ftrNode = null;
        var body = new List<DynamicUINode>();
        if (node.Children != null) {
            foreach (var c in node.Children) {
                if (c.Name == "section_header" && c.Children != null && c.Children.Count > 0) hdrNode = c.Children[0];
                else if (c.Name == "section_footer" && c.Children != null && c.Children.Count > 0) ftrNode = c.Children[0];
                else body.Add(c);
            }
        }
        var wrap = new StackPanel { Orientation = Orientation.Vertical, Spacing = 6, HorizontalAlignment = HorizontalAlignment.Stretch };
        bool isProminent = (node.Props != null && node.Props.ContainsKey("prominent") && node.Props["prominent"] is bool pr && pr);
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
        int count = body.Count;
        var cards = new List<Border>();
        for (int i = 0; i < count; i++) {
            var c = body[i];
            bool isBtn = (c.Name == "button");
            Border card;
            if (isUncontained) {
                card = new Border {
                    HorizontalAlignment = HorizontalAlignment.Stretch,
                    VerticalAlignment = VerticalAlignment.Stretch,
                    Child = RenderNode(c)
                };
            } else {
                Microsoft.UI.Xaml.Media.Brush rowBgBrush = ResolveNodeBackgroundBrush(c, true);
                card = new Border {
                    CornerRadius = new CornerRadius(8),
                    Padding = new Thickness(0),
                    HorizontalAlignment = HorizontalAlignment.Stretch,
                    VerticalAlignment = VerticalAlignment.Stretch,
                    Background = rowBgBrush ?? (Application.Current.Resources["CardBackgroundFillColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush),
                    BorderBrush = Application.Current.Resources["CardStrokeColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush,
                    BorderThickness = new Thickness(1),
                    Child = RenderNode(c)
                };
                if (isBtn && c.Props != null && c.Props.ContainsKey("action")) {
                    string act = c.Props["action"].ToString();
                    card.PointerPressed += (s, e) => { if (!string.IsNullOrEmpty(act)) Dispatch(act, ""); };
                }
            }
            if (hasItemWidth) {
                card.MinWidth = minW;
                card.MaxWidth = maxW;
            }
            cards.Add(card);
        }
        if (isWrap) {
            var grid = new Grid { ColumnSpacing = spacing, RowSpacing = spacing, HorizontalAlignment = HorizontalAlignment.Stretch };
            Action<int> layoutGrid = (int activeCols) => {
                grid.ColumnDefinitions.Clear();
                grid.RowDefinitions.Clear();
                for (int c = 0; c < activeCols; c++) {
                    var colDef = new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) };
                    if (hasItemWidth) {
                        colDef.MinWidth = minW;
                        colDef.MaxWidth = maxW;
                    }
                    grid.ColumnDefinitions.Add(colDef);
                }
                int rows = (count + activeCols - 1) / activeCols;
                for (int r = 0; r < rows; r++) grid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
                for (int i = 0; i < count; i++) {
                    var card = cards[i];
                    if (!grid.Children.Contains(card)) grid.Children.Add(card);
                    Grid.SetColumn(card, i % activeCols);
                    Grid.SetRow(card, i / activeCols);
                }
            };
            layoutGrid(cols);
            if (hasItemWidth && !hasExplicitCols) {
                grid.SizeChanged += (s, e) => {
                    if (e.NewSize.Width > 0) {
                        int autoCols = Math.Max(1, (int)((e.NewSize.Width + spacing) / (minW + spacing)));
                        if (autoCols != grid.ColumnDefinitions.Count) layoutGrid(autoCols);
                    }
                };
            }
            wrap.Children.Add(grid);
        } else {
            var sv = new ScrollViewer {
                HorizontalScrollMode = ScrollMode.Enabled,
                HorizontalScrollBarVisibility = ScrollBarVisibility.Hidden,
                VerticalScrollMode = ScrollMode.Disabled,
                VerticalScrollBarVisibility = ScrollBarVisibility.Disabled,
                HorizontalSnapPointsType = scrollFreely
                    ? Microsoft.UI.Xaml.Controls.SnapPointsType.None
                    : Microsoft.UI.Xaml.Controls.SnapPointsType.MandatorySingle,
                HorizontalSnapPointsAlignment = Microsoft.UI.Xaml.Controls.Primitives.SnapPointsAlignment.Near,
                HorizontalAlignment = HorizontalAlignment.Stretch,
                Margin = new Thickness(-16, 0, -16, 0)
            };
            var track = new StackPanel { Orientation = Orientation.Horizontal, Spacing = spacing, Padding = new Thickness(16, 0, 16, 0) };
            int pages = (count + cols - 1) / cols;
            var pageGrids = new List<Grid>();
            for (int p = 0; p < pages; p++) {
                var pg = new Grid { ColumnSpacing = spacing, HorizontalAlignment = HorizontalAlignment.Stretch };
                for (int c = 0; c < cols; c++) {
                    var colDef = new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) };
                    if (hasItemWidth) {
                        colDef.MinWidth = minW;
                        colDef.MaxWidth = maxW;
                    }
                    pg.ColumnDefinitions.Add(colDef);
                }
                for (int c = 0; c < cols; c++) {
                    int idx = p * cols + c;
                    if (idx < count) {
                        var card = cards[idx];
                        Grid.SetColumn(card, c);
                        pg.Children.Add(card);
                    }
                }
                pageGrids.Add(pg);
                track.Children.Add(pg);
            }
            sv.SizeChanged += (s, e) => {
                double gridW = Math.Max(0, e.NewSize.Width - 32);
                if (gridW > 0) {
                    if (hasItemWidth) {
                        double rawItemW = (gridW - spacing * (cols - 1)) / cols;
                        double clampedW = Math.Min(maxW, Math.Max(minW, rawItemW));
                        double pageW = clampedW * cols + spacing * (cols - 1);
                        foreach (var pg in pageGrids) pg.Width = pageW;
                    } else {
                        foreach (var pg in pageGrids) pg.Width = gridW;
                    }
                }
            };
            sv.Content = track;
            wrap.Children.Add(sv);
        }
        if (ftrNode != null) wrap.Children.Add(RenderNode(ftrNode));
        return wrap;
}
"#;
