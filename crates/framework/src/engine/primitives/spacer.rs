
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Spacer {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Spacer {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn size(mut self, size: u32) -> Self {
        self.props.push(("size".to_string(), PropValue::Int(size)));
        self
    }
}
impl IntoNode for Spacer {
    fn into_node(self) -> Node {
        Node::Element {
            name: "spacer".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! Spacer {
    () => {{
        $crate::engine::primitives::Spacer {
            props: vec![],
            children: vec![],
        }
    }}
}

pub const SPACER_ANDROID_SNIPPET: &str = r#"
"spacer" -> {
    val size = node.getInt("size", 0)
        if (size > 0) {
            androidx.compose.foundation.layout.Spacer(modifier = modifier.size(size.dp))
        } else {
            androidx.compose.foundation.layout.Spacer(modifier = modifier)
        }
}
"#;

pub const SPACER_IOS_SNIPPET: &str = r#"
case "spacer":
    let size = node.props["size"] as? Int ?? 0
        if size > 0 {
            Spacer().frame(width: CGFloat(size), height: CGFloat(size))
        } else {
            Spacer(minLength: 0)
        }
"#;

pub const SPACER_GTK_SNIPPET: &str = r#"
"spacer" => {
    let size = node.get_int("size", 0);
        let bx = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        if size > 0 {
            bx.set_size_request(size as i32, size as i32);
        } else {
            bx.set_hexpand(true);
        }
        bx.upcast::<gtk4::Widget>()
}
"#;

pub const SPACER_WEB_SNIPPET: &str = r#"
case "spacer": {
    const size = (node.props && node.props.size) || 0;
        const el = document.createElement("div");
        if (size > 0) {
            el.style.width = size + "px";
            el.style.height = size + "px";
            el.style.flex = "0 0 auto";
        } else {
            el.style.flex = "1 1 auto";
        }
        return el;
}
"#;

pub const SPACER_WINDOWS_SNIPPET: &str = r#"
case "spacer": {
    var b = new Border();
        if (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double sz) && sz > 0) {
            b.Width = sz;
            b.Height = sz;
        } else {
            b.HorizontalAlignment = HorizontalAlignment.Stretch;
        }
        return b;
}
"#;
