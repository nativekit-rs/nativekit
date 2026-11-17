
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Page {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Page {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn presentation(mut self, presentation: impl Into<String>) -> Self {
        self.props.push(("presentation".to_string(), PropValue::String(presentation.into())));
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.props.push(("subtitle".to_string(), PropValue::String(subtitle.into())));
        self
    }
    pub fn top_style(mut self, top_style: impl Into<String>) -> Self {
        self.props.push(("top_style".to_string(), PropValue::String(top_style.into())));
        self
    }
    pub fn large_title(mut self, large_title: bool) -> Self {
        self.props.push(("large_title".to_string(), PropValue::Bool(large_title)));
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.push(("title".to_string(), PropValue::String(title.into())));
        self
    }
    pub fn section_spacing(mut self, section_spacing: u32) -> Self {
        self.props.push(("section_spacing".to_string(), PropValue::Int(section_spacing)));
        self
    }
    pub fn on_back<H, Args, S>(mut self, handler: H) -> Self 
    where 
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_back".to_string(), PropValue::String(id)));
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
impl IntoNode for Page {
    fn into_node(self) -> Node {
        Node::Element {
            name: "page".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const PAGE_ANDROID_SNIPPET: &str = r#"
"page" -> {
    val title = node.getString("title", "")
        val subtitle = node.getString("subtitle", "")
        val largeTitle = node.getBoolean("large_title", false)
        val topStyle = node.getString("top_style", "soft")
        val onBackStr = node.getString("on_back", "")
        val presentation = node.getString("presentation", "push")
        val isSheet = presentation == "sheet"
        val isForm = presentation == "form" || presentation == "dialog" || presentation == "modal" || presentation == "fullscreen"
    
        val toolbarNode = node.children.firstOrNull { it.name == "toolbartop" }
        val toolbarItems = toolbarNode?.children ?: emptyList()
        val leadingItems = toolbarItems.filter {
            (it.getString("placement", "").ifEmpty { it.children.firstOrNull()?.getString("placement", "") ?: "trailing" }) == "leading"
        }
        val trailingItems = toolbarItems.filter {
            (it.getString("placement", "").ifEmpty { it.children.firstOrNull()?.getString("placement", "") ?: "trailing" }) != "leading"
        }
        val bodyChildren = node.children.filter { it.name != "toolbartop" && it.name != "toolbarbottom" }
        val sectionSpacing = node.getInt("section_spacing", 16)
    
        if (isSheet) {
            androidx.compose.foundation.layout.Column(
                modifier = androidx.compose.ui.Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 24.dp)
                    .padding(bottom = 32.dp),
                verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(sectionSpacing.dp)
            ) {
                bodyChildren.forEach { child ->
                    DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier.fillMaxWidth())
                }
            }
        } else {
            val titleBlock: @androidx.compose.runtime.Composable () -> Unit = {
                androidx.compose.foundation.layout.Column {
                    androidx.compose.material3.Text(title)
                    if (subtitle.isNotEmpty()) {
                        androidx.compose.material3.Text(subtitle, style = androidx.compose.material3.MaterialTheme.typography.bodySmall)
                    }
                }
            }
    
            val navIconBlock: @androidx.compose.runtime.Composable () -> Unit = {
                if (leadingItems.isNotEmpty()) {
                    androidx.compose.foundation.layout.Row(verticalAlignment = Alignment.CenterVertically) {
                        leadingItems.forEach { item -> DynamicNodeView(item, dispatch) }
                    }
                } else if (onBackStr.isNotEmpty()) {
                    androidx.compose.material3.IconButton(onClick = { dispatch(onBackStr, null) }) {
                        androidx.compose.material3.Icon(
                            imageVector = if (isForm) AppCloseIcon else AppBackIcon,
                            contentDescription = if (isForm) "Close" else "Back",
                            modifier = androidx.compose.ui.Modifier.size(24.dp)
                        )
                    }
                }
            }
    
            val actionsBlock: @androidx.compose.runtime.Composable androidx.compose.foundation.layout.RowScope.() -> Unit = {
                trailingItems.forEach { item -> DynamicNodeView(item, dispatch) }
            }
    
            val isLarge = largeTitle && topStyle != "hard"
            val scrollBehavior = if (isLarge) {
                androidx.compose.material3.TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
            } else {
                androidx.compose.material3.TopAppBarDefaults.pinnedScrollBehavior()
            }
            androidx.compose.material3.Scaffold(
                modifier = Modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
                topBar = {
                    if (isLarge) {
                        androidx.compose.material3.LargeTopAppBar(
                            title = titleBlock,
                            navigationIcon = navIconBlock,
                            actions = actionsBlock,
                            scrollBehavior = scrollBehavior
                        )
                    } else {
                        androidx.compose.material3.TopAppBar(
                            title = titleBlock,
                            navigationIcon = navIconBlock,
                            actions = actionsBlock,
                            scrollBehavior = scrollBehavior
                        )
                    }
                }
            ) { innerPadding ->
                androidx.compose.foundation.layout.Box(
                    modifier = androidx.compose.ui.Modifier
                        .fillMaxSize()
                        .verticalScroll(androidx.compose.foundation.rememberScrollState())
                        .padding(innerPadding)
                ) {
                    androidx.compose.foundation.layout.Column(
                        modifier = androidx.compose.ui.Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(sectionSpacing.dp)
                    ) {
                        bodyChildren.forEach { child ->
                            DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier.fillMaxWidth())
                        }
                    }
                }
            }
        }
}
"#;

pub const PAGE_IOS_SNIPPET: &str = r#"
case "page":
    let title = node.props["title"] as? String ?? ""
        let subtitle = node.props["subtitle"] as? String ?? ""
        let largeTitle = node.props["large_title"] as? Bool ?? false
        let topStyle = node.props["top_style"] as? String ?? "soft"
        let toolbarNode = node.children.first { $0.name == "toolbartop" }
        let toolbarItems = toolbarNode?.children ?? []
        let leadingItems = toolbarItems.filter { ($0.props["placement"] as? String ?? $0.children.first?.props["placement"] as? String ?? "trailing") == "leading" }
        let trailingItems = toolbarItems.filter { ($0.props["placement"] as? String ?? $0.children.first?.props["placement"] as? String ?? "trailing") != "leading" }
        let bodyChildren = node.children.filter { $0.name != "toolbartop" && $0.name != "toolbarbottom" }
        let sectionSpacing = CGFloat(node.props["section_spacing"] as? Int ?? 16)
    
        let content = List {
            ForEach(0..<bodyChildren.count, id: \.self) { i in
                let child = bodyChildren[i]
                if child.name == "section" || child.name == "grid" {
                    DynamicNodeView(node: child, dispatch: dispatch)
                } else {
                    DynamicNodeView(node: child, dispatch: dispatch)
                        .listRowInsets(EdgeInsets(top: 6, leading: 16, bottom: 6, trailing: 16))
                        .listRowBackground(Color.clear)
                        .listRowSeparator(.hidden)
                }
            }
        }
        .listStyle(.insetGrouped)
        .scrollDismissesKeyboard(.interactively)
        .listSectionSpacing(sectionSpacing)
    
        Group {
            if topStyle == "hard" {
                content.scrollEdgeEffect(.top, .hard)
            } else {
                content
            }
        }
        .navigationTitle(title)
        .navigationBarTitleDisplayMode(topStyle == "hard" ? .inline : (largeTitle ? .large : .inline))
        .navigationBarBackButtonHidden(!leadingItems.isEmpty)
        .toolbar {
            if !leadingItems.isEmpty {
                ToolbarItemGroup(placement: .topBarLeading) {
                    ForEach(0..<leadingItems.count, id: \.self) { i in
                        DynamicNodeView(node: leadingItems[i], dispatch: dispatch)
                    }
                }
            }
            if !subtitle.isEmpty {
                ToolbarItem(placement: .principal) {
                    VStack {
                        Text(title).font(.headline)
                        Text(subtitle).font(.caption)
                    }
                }
            }
            if !trailingItems.isEmpty {
                ToolbarItemGroup(placement: .topBarTrailing) {
                    ForEach(0..<trailingItems.count, id: \.self) { i in
                        DynamicNodeView(node: trailingItems[i], dispatch: dispatch)
                    }
                }
            }
        }
"#;

pub const PAGE_GTK_SNIPPET: &str = r#"
"page" => {
    let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        scrolled.set_hscrollbar_policy(gtk4::PolicyType::Never);
        scrolled.set_vscrollbar_policy(gtk4::PolicyType::Automatic);
    
        let section_spacing = node.get_int("section_spacing", 16) as i32;
        let page_box = gtk4::Box::new(gtk4::Orientation::Vertical, section_spacing);
        page_box.set_hexpand(true);
        page_box.set_vexpand(true);
        page_box.set_margin_start(16);
        page_box.set_margin_end(16);
        page_box.set_margin_top(16);
        page_box.set_margin_bottom(24);
        let title = node.get_string("title", "");
        if !title.is_empty() {
            let title_lbl = gtk4::Label::new(Some(&title));
            title_lbl.add_css_class("title-1");
            title_lbl.set_halign(gtk4::Align::Start);
            page_box.append(&title_lbl);
        }
        for child in &node.children {
            if child.name != "toolbartop" && child.name != "toolbarbottom" {
                page_box.append(&parse_dynamic_widget(child, dispatch));
            }
        }
        let click_gesture = gtk4::GestureClick::new();
        let pb_weak = page_box.downgrade();
        click_gesture.connect_released(move |_, _, x, y| {
            if let Some(pb) = pb_weak.upgrade() {
                let picked = pb.pick(x, y, gtk4::PickFlags::DEFAULT);
                let clicked_entry = picked.as_ref().map_or(false, |w| {
                    w.is::<gtk4::Entry>() || w.ancestor(gtk4::Entry::static_type()).is_some()
                });
                if !clicked_entry {
                    if let Some(root) = pb.root() {
                        root.set_focus(None::<&gtk4::Widget>);
                    }
                }
            }
        });
        page_box.add_controller(click_gesture);
        scrolled.set_child(Some(&page_box));
        scrolled.upcast::<gtk4::Widget>()
}
"#;

pub const PAGE_WEB_SNIPPET: &str = r#"
case "page": {
    const { title = "", subtitle = "", on_back = "", presentation = "push", top_style = "soft", section_spacing = 16 } = node.props || {};
        const isForm = ["form", "dialog", "modal", "fullscreen"].includes(presentation);
        const children = node.children || [];
        const toolbarNode = children.find(c => c.name === "toolbartop");
        const toolbarItems = (toolbarNode && toolbarNode.children) ? toolbarNode.children : [];
        const getPlacement = (item) => (item.props && item.props.placement) || (item.children && item.children[0] && item.children[0].props && item.children[0].props.placement) || "trailing";
        const leadingItems = toolbarItems.filter(i => getPlacement(i) === "leading").map(i => renderNode(i));
        const trailingItems = toolbarItems.filter(i => getPlacement(i) !== "leading").map(i => renderNode(i));
        const bodyChildren = children.filter(c => c.name !== "toolbartop" && c.name !== "toolbarbottom").map(c => renderNode(c));
    
        const backBtn = (leadingItems.length === 0 && on_back) ? h("button", {
            className: "back-button",
            html: isForm ? "&#10005;" : "&#8592; Back",
            style: { background: "none", border: "none", color: "var(--rn-primary, #007aff)", cursor: "pointer", fontSize: "15px" },
            onClick: () => dispatch(on_back, null),
        }) : null;
    
        const navbar = h("nav", {
            className: "native-navbar",
            style: {
                display: "flex",
                alignItems: "center",
                justifyContent: "space-between",
                padding: "12px 16px",
                minHeight: "44px",
                background: top_style === "hard" ? "var(--rn-surface, #ffffff)" : "transparent",
                borderBottom: top_style === "hard" ? "1px solid var(--rn-border, #e5e5ea)" : "none",
            },
        }, [
            h("div", { style: { display: "flex", alignItems: "center", gap: "8px", minWidth: "80px" } }, leadingItems.length > 0 ? leadingItems : backBtn),
            h("div", { style: { textAlign: "center", flex: "1" } }, [
                h("div", { className: "page-title", text: title, style: { fontWeight: "600", fontSize: "17px" } }),
                subtitle ? h("div", { text: subtitle, style: { fontSize: "12px", color: "gray" } }) : null,
            ]),
            h("div", { style: { display: "flex", alignItems: "center", justifyContent: "flex-end", gap: "8px", minWidth: "80px" } }, trailingItems),
        ]);
    
        const content = h("div", {
            className: "page-content",
            style: { flex: "1", overflowY: "auto", padding: "16px", boxSizing: "border-box", display: "flex", flexDirection: "column", gap: `${section_spacing}px` },
        }, bodyChildren);
    
        return h("div", {
            className: "page",
            style: { display: "flex", flexDirection: "column", width: "100%", height: "100%" },
        }, [navbar, content]);
}
"#;

pub const PAGE_WINDOWS_SNIPPET: &str = r#"
case "page": {
    var pageScroll = new ScrollViewer {
            IsTabStop = true,
            HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled,
            VerticalScrollBarVisibility = ScrollBarVisibility.Auto,
            HorizontalAlignment = HorizontalAlignment.Stretch,
            VerticalAlignment = VerticalAlignment.Stretch
        };
        int sectionSpacing = 16;
        if (node.Props != null && node.Props.ContainsKey("section_spacing") && int.TryParse(node.Props["section_spacing"].ToString(), out int sVal)) {
            sectionSpacing = sVal;
        }
        var pageContent = new StackPanel {
            HorizontalAlignment = HorizontalAlignment.Stretch,
            Margin = new Thickness(16, 16, 16, 24),
            Spacing = sectionSpacing,
            Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent)
        };
        pageContent.PointerPressed += (s, e) => { pageScroll.Focus(FocusState.Programmatic); };
    
        string pTitle = (node.Props != null && node.Props.ContainsKey("title")) ? node.Props["title"].ToString() : "";
        DynamicUINode toolbarNode = null;
        if (node.Children != null) {
            foreach (var c in node.Children) { if (c.Name == "toolbartop") { toolbarNode = c; break; } }
        }
    
        if (!string.IsNullOrEmpty(pTitle) || toolbarNode != null) {
            var headerGrid = new Grid { Margin = new Thickness(0, 0, 0, 16), Tag = "__page_header__" };
            headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            headerGrid.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
    
            var leadPanel = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8, VerticalAlignment = VerticalAlignment.Center, Margin = new Thickness(0, 0, 12, 0) };
            var trailPanel = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 8, VerticalAlignment = VerticalAlignment.Center };
    
            if (toolbarNode != null && toolbarNode.Children != null) {
                foreach (var item in toolbarNode.Children) {
                    string pl = (item.Props != null && item.Props.ContainsKey("placement")) ? item.Props["placement"].ToString() : "trailing";
                    if (pl == "leading") leadPanel.Children.Add(RenderNode(item));
                    else trailPanel.Children.Add(RenderNode(item));
                }
            }
            headerGrid.Children.Add(leadPanel);
            Grid.SetColumn(leadPanel, 0);
    
            if (!string.IsNullOrEmpty(pTitle)) {
                var titleTb = new TextBlock {
                    Text = pTitle,
                    FontSize = 28,
                    FontWeight = Microsoft.UI.Text.FontWeights.SemiBold,
                    VerticalAlignment = VerticalAlignment.Center
                };
                headerGrid.Children.Add(titleTb);
                Grid.SetColumn(titleTb, 1);
            }
    
            headerGrid.Children.Add(trailPanel);
            Grid.SetColumn(trailPanel, 2);
            pageContent.Children.Add(headerGrid);
        }
    
        if (node.Children != null) {
            foreach (var c in node.Children) {
                if (c.Name != "toolbartop" && c.Name != "toolbarbottom") pageContent.Children.Add(RenderNode(c));
            }
        }
        pageScroll.Content = pageContent;
        return pageScroll;
}
"#;
