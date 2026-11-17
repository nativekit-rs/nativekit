use crate::build_types::RustDef;

pub fn generate_rust_struct(struct_name: &str, def: &RustDef) -> String {
    let mut rust_code = format!(r#"
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{{Node, PropValue}};
use crate::engine::builder::*;

pub struct {} {{
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}}
"#, struct_name);

    rust_code.push_str(&format!("impl {} {{\n", struct_name));
    rust_code.push_str("    pub fn new() -> Self {\n        Self {\n            props: vec![],\n            children: vec![],\n        }\n    }\n");
    
    // Builders
    for (prop_name, prop_type) in &def.builder {
        let mut doc_comment = String::new();
        let mut rust_type = "impl Into<String>".to_string();
        let mut value_expr = format!("{}.into()", prop_name);
        let mut prop_val = "PropValue::String".to_string();
        
        if prop_type.starts_with("String(") {
            let inner = prop_type.trim_start_matches("String(").trim_end_matches(")");
            let variants: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            doc_comment = format!("    /// Valid options: {}\n", variants.join(", "));
        } else if prop_type == "Bool" {
            rust_type = "bool".to_string();
            value_expr = prop_name.to_string();
            prop_val = "PropValue::Bool".to_string();
        } else if prop_type == "u32" {
            rust_type = "u32".to_string();
            value_expr = prop_name.to_string();
            prop_val = "PropValue::Int".to_string();
        }
        
        if prop_type == "Action" {
            rust_code.push_str(&format!("    pub fn {prop_name}<H, Args, S>(mut self, handler: H) -> Self \n    where \n        H: crate::engine::registry::ActionHandler<Args, S>,\n        S: 'static\n    {{\n        let id = crate::engine::registry::register_handler(handler);\n        self.props.push((\"{prop_name}\".to_string(), PropValue::String(id)));\n        self\n    }}\n"));
        } else {
            rust_code.push_str(&doc_comment);
            rust_code.push_str(&format!("    pub fn {prop_name}(mut self, {prop_name}: {rust_type}) -> Self {{\n        self.props.push((\"{prop_name}\".to_string(), {prop_val}({value_expr})));\n        self\n    }}\n"));
        }
    }

    if struct_name == "Section" || struct_name == "Grid" {
        rust_code.push_str("    pub fn header(mut self, content: impl IntoNode) -> Self {\n        self.children.retain(|c| !matches!(c, Node::Element { name, .. } if name == \"section_header\"));\n        self.children.push(Node::Element { name: \"section_header\".to_string(), props: vec![], children: vec![content.into_node()] });\n        self\n    }\n");
        rust_code.push_str("    pub fn footer(mut self, content: impl IntoNode) -> Self {\n        self.children.retain(|c| !matches!(c, Node::Element { name, .. } if name == \"section_footer\"));\n        self.children.push(Node::Element { name: \"section_footer\".to_string(), props: vec![], children: vec![content.into_node()] });\n        self\n    }\n");
        rust_code.push_str("    pub fn prominent(mut self) -> Self {\n        self.props.retain(|(k, _)| k != \"prominent\");\n        self.props.push((\"prominent\".to_string(), PropValue::Bool(true)));\n        self\n    }\n");
    }
    if def.accepts_children {
        rust_code.push_str("    pub fn child(mut self, child: impl IntoNode) -> Self {\n        self.children.push(child.into_node());\n        self\n    }\n");
        rust_code.push_str("    pub fn children(mut self, children: impl IntoChildren) -> Self {\n        self.children.extend(children.into_children());\n        self\n    }\n");
    }

    rust_code.push_str("}\n");
    rust_code.push_str(&format!("impl IntoNode for {} {{\n    fn into_node(self) -> Node {{\n        Node::Element {{\n            name: \"{}\".to_string(),\n            props: self.props,\n            children: self.children,\n        }}\n    }}\n}}\n", struct_name, struct_name.to_lowercase()));

    if def.accepts_children && def.constructor.is_empty() {
        // Container macros (`Page!`, `VStack!`, `HStack!`, `Section!`, `Grid!`, `ToolbarTop!`, `NavigationStack!`, `Button!`)
        // are provided by `native_kit_macros`.
    } else if struct_name == "TextInput" {
        rust_code.push_str("
#[macro_export]
macro_rules! TextInput {
    () => {{
        $crate::engine::primitives::TextInput::new()
    }};
    ($val:expr) => {{
        $crate::engine::primitives::TextInput::new().value($val)
    }};
}
");
    } else if struct_name == "Text" {
        rust_code.push_str(&format!("
#[macro_export]
macro_rules! Text {{
    ($($arg:tt)*) => {{{{
        $crate::engine::primitives::Text {{
            props: vec![(\"label\".to_string(), $crate::engine::ast::PropValue::String(format!($($arg)*).into()))],
            children: vec![],
        }}
    }}}}
}}
"));
    } else {
        let mut macro_args = Vec::new();
        let mut struct_props = Vec::new();
        for (i, (arg_name, arg_type)) in def.constructor.iter().enumerate() {
            macro_args.push(format!("$arg{}:expr", i));
            if arg_type == "Bool" {
                struct_props.push(format!("(\"{}\".to_string(), $crate::engine::ast::PropValue::Bool($arg{}))", arg_name, i));
            } else {
                struct_props.push(format!("(\"{}\".to_string(), $crate::engine::ast::PropValue::String($arg{}.into()))", arg_name, i));
            }
        }
        
        let props_vec = if struct_props.is_empty() { "vec![]".to_string() } else { format!("vec![{}]", struct_props.join(", ")) };
        
        rust_code.push_str(&format!("
#[macro_export]
macro_rules! {} {{
    ({}) => {{{{
        $crate::engine::primitives::{} {{
            props: {},
            children: vec![],
        }}
    }}}}
}}
", struct_name, macro_args.join(", "), struct_name, props_vec));
    }

    rust_code
}
