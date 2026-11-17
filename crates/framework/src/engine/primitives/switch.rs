
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct Switch {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl Switch {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn on_change<H, Args, S>(mut self, handler: H) -> Self 
    where 
        H: crate::engine::registry::ActionHandler<Args, S>,
        S: 'static
    {
        let id = crate::engine::registry::register_handler(handler);
        self.props.push(("on_change".to_string(), PropValue::String(id)));
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.props.push(("title".to_string(), PropValue::String(title.into())));
        self
    }
    pub fn value(mut self, value: bool) -> Self {
        self.props.push(("value".to_string(), PropValue::Bool(value)));
        self
    }
}
impl IntoNode for Switch {
    fn into_node(self) -> Node {
        Node::Element {
            name: "switch".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! Switch {
    () => {{
        $crate::engine::primitives::Switch {
            props: vec![],
            children: vec![],
        }
    }}
}

pub const SWITCH_ANDROID_SNIPPET: &str = r#"
"switch" -> {
    val on_change_str = node.getString("on_change", "")
        val value = node.getBoolean("value", false)
        val on_change_action: (Boolean) -> Unit = { newVal ->
            if (on_change_str.isNotEmpty()) {
                dispatch(on_change_str, org.json.JSONObject().put("value", newVal))
            }
        }
        val title = node.getString("title", "")
        if (title.isNotEmpty()) {
            Row(
                modifier = modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(title, style = MaterialTheme.typography.bodyLarge)
                Switch(checked = value, onCheckedChange = { on_change_action(it) })
            }
        } else {
            Switch(checked = value, onCheckedChange = { on_change_action(it) })
        }
}
"#;

pub const SWITCH_IOS_SNIPPET: &str = r#"
case "switch":
    let on_change_str = node.props["on_change"] as? String ?? ""
        let value = node.props["value"] as? Bool ?? false
        let title = node.props["title"] as? String ?? ""
        let binding = Binding<Bool>(
            get: { value },
            set: { newVal in
                if !on_change_str.isEmpty {
                    dispatch(on_change_str, ["value": newVal])
                }
            }
        )
        if !title.isEmpty {
            Toggle(title, isOn: binding)
        } else {
            Toggle(isOn: binding) { EmptyView() }.labelsHidden()
        }
"#;

pub const SWITCH_GTK_SNIPPET: &str = r#"
"switch" => {
    let value = node.get_bool("value", false);
        let on_change_str = node.get_string("on_change", "");
        let title = node.get_string("title", "");
        let sw = gtk4::Switch::new();
        sw.set_active(value);
        sw.set_valign(gtk4::Align::Center);
        if !on_change_str.is_empty() {
            let disp = dispatch.clone();
            sw.connect_state_set(move |_, _| {
                disp(on_change_str.clone(), None);
                gtk4::glib::Propagation::Proceed
            });
        }
        if !title.is_empty() {
            let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
            row.set_hexpand(true);
            let lbl = gtk4::Label::new(Some(&title));
            lbl.set_halign(gtk4::Align::Start);
            lbl.set_hexpand(true);
            row.append(&lbl);
            row.append(&sw);
            return row.upcast::<gtk4::Widget>();
        }
        sw.upcast::<gtk4::Widget>()
}
"#;

pub const SWITCH_WEB_SNIPPET: &str = r#"
case "switch": {
    const { on_change = "", value = false } = node.props || {};
        const el = h("input", {
            attrs: { type: "checkbox" },
            onChange: (e) => { if (on_change) dispatch(on_change, e.target.checked); },
        });
        el.checked = Boolean(value);
        return el;
}
"#;

pub const SWITCH_WINDOWS_SNIPPET: &str = r#"
case "switch": {
    var sw = new CheckBox();
        if (node.Props != null && node.Props.ContainsKey("is_on") && node.Props["is_on"] is bool b) {
            sw.IsChecked = b;
        }
        if (node.Props != null && node.Props.ContainsKey("on_toggle")) {
            string action = node.Props["on_toggle"].ToString();
            sw.Checked += (s, e) => Dispatch(action, "true");
            sw.Unchecked += (s, e) => Dispatch(action, "false");
        }
        return sw;
}
"#;
