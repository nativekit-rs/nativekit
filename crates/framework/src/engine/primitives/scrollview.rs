
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct ScrollView {
    props: Vec<(String, PropValue)>,
    children: Vec<Node>,
}
impl ScrollView {
    pub fn axis(mut self, axis: impl Into<String>) -> Self {
        self.props.push(("axis".to_string(), PropValue::String(axis.into())));
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
impl IntoNode for ScrollView {
    fn into_node(self) -> Node {
        Node::Element {
            name: "scrollview".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ScrollView() -> ScrollView {
    ScrollView {
        props: vec![],
        children: vec![],
    }
}

pub const SCROLLVIEW_ANDROID_SNIPPET: &str = r#"
"scrollview" -> {
    val axisStr = node.getString("axis") ?: "vertical"
    if (axisStr == "horizontal") {
        val scrollState = androidx.compose.foundation.rememberScrollState()
        androidx.compose.foundation.layout.Row(modifier = androidx.compose.ui.Modifier.horizontalScroll(scrollState).fillMaxHeight()) {
            node.children.forEach { child ->
                DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier)
            }
        }
    } else {
        val scrollState = androidx.compose.foundation.rememberScrollState()
        androidx.compose.foundation.layout.Column(modifier = androidx.compose.ui.Modifier.verticalScroll(scrollState).fillMaxWidth()) {
            node.children.forEach { child ->
                DynamicNodeView(child, dispatch, androidx.compose.ui.Modifier)
            }
        }
    }
}

"#;

pub const SCROLLVIEW_IOS_SNIPPET: &str = r#"
case "scrollview":
    let axisStr = node.props["axis"] as? String ?? "vertical"
    let axis: Axis.Set = axisStr == "horizontal" ? .horizontal : .vertical
    AnyView(
        ScrollView(axis) {
            if axis == .horizontal {
                HStack {
                    ForEach(0..<node.children.count, id: \.self) { i in
                        DynamicNodeView(node: node.children[i], dispatch: dispatch)
                    }
                }.frame(maxHeight: .infinity, alignment: .top)
            } else {
                VStack {
                    ForEach(0..<node.children.count, id: \.self) { i in
                        DynamicNodeView(node: node.children[i], dispatch: dispatch)
                    }
                }.frame(maxWidth: .infinity, alignment: .leading)
            }
        }
    )

"#;

pub const SCROLLVIEW_WEB_SNIPPET: &str = r#"case "scrollview":
    const axis = (node.props && node.props.axis) || "";
    const el = document.createElement("div");
    el.classList.add("scrollview");

    if (node.children) { node.children.forEach(c => el.appendChild(renderNode(c))); }
    return el;

"#;
