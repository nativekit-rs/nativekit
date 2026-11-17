
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct ToolbarItem {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl ToolbarItem {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    /// Valid options: leading, trailing
    pub fn placement(mut self, placement: impl Into<String>) -> Self {
        self.props.push(("placement".to_string(), PropValue::String(placement.into())));
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
impl IntoNode for ToolbarItem {
    fn into_node(self) -> Node {
        Node::Element {
            name: "toolbaritem".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! ToolbarItem {
    ( $($child:expr)* ) => {{
        let mut children = Vec::new();
        $(
            children.extend($crate::engine::builder::IntoChildren::into_children($child));
        )*
        $crate::engine::primitives::ToolbarItem {
            props: vec![],
            children,
        }
    }}
}

pub const TOOLBARITEM_ANDROID_SNIPPET: &str = r#"
"toolbaritem" -> {
    node.children.forEach { DynamicNodeView(it, dispatch) }
}
"#;

pub const TOOLBARITEM_IOS_SNIPPET: &str = r#"
case "toolbaritem":
    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
"#;

pub const TOOLBARITEM_GTK_SNIPPET: &str = r#"
"toolbaritem" => {
    let b = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        for c in &node.children { b.append(&parse_dynamic_widget(c, dispatch)); }
        b.upcast::<gtk4::Widget>()
}
"#;

pub const TOOLBARITEM_WEB_SNIPPET: &str = r#"
case "toolbaritem": {
    return h("div", { className: "toolbaritem", style: { display: "inline-flex", alignItems: "center", gap: "6px" } }, renderChildren(node));
}
"#;

pub const TOOLBARITEM_WINDOWS_SNIPPET: &str = r#"
case "toolbaritem": {
    var sp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 6 };
        if (node.Children != null) { foreach (var c in node.Children) sp.Children.Add(RenderNode(c)); }
        return sp;
}
"#;
