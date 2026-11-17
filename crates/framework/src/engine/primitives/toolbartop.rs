
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct ToolbarTop {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl ToolbarTop {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
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
impl IntoNode for ToolbarTop {
    fn into_node(mut self) -> Node {
        fn tag_toolbar_buttons(node: &mut Node) {
            if let Node::Element { name, props, children } = node {
                if name == "button" && !props.iter().any(|(k, _)| k == "placement") {
                    props.push(("placement".to_string(), PropValue::String("trailing".to_string())));
                }
                for child in children {
                    tag_toolbar_buttons(child);
                }
            }
        }
        for child in &mut self.children {
            tag_toolbar_buttons(child);
        }
        Node::Element {
            name: "toolbartop".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

pub const TOOLBARTOP_ANDROID_SNIPPET: &str = r#"
"toolbartop" -> {
    node.children.forEach { DynamicNodeView(it, dispatch) }
}
"#;

pub const TOOLBARTOP_IOS_SNIPPET: &str = r#"
case "toolbartop":
    ForEach(0..<node.children.count, id: \.self) { i in DynamicNodeView(node: node.children[i], dispatch: dispatch) }
"#;

pub const TOOLBARTOP_GTK_SNIPPET: &str = r#"
"toolbartop" => {
    let b = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        for c in &node.children { b.append(&parse_dynamic_widget(c, dispatch)); }
        b.upcast::<gtk4::Widget>()
}
"#;

pub const TOOLBARTOP_WEB_SNIPPET: &str = r#"
case "toolbartop": {
    return h("div", { className: "toolbartop", style: { display: "inline-flex", alignItems: "center", gap: "6px" } }, renderChildren(node));
}
"#;

pub const TOOLBARTOP_WINDOWS_SNIPPET: &str = r#"
case "toolbartop": {
    var sp = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 6 };
        if (node.Children != null) { foreach (var c in node.Children) sp.Children.Add(RenderNode(c)); }
        return sp;
}
"#;
