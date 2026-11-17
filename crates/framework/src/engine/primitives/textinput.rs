
// ==========================================
// ⚠️ GENERATED CODE - DO NOT EDIT ⚠️
// ==========================================
#[allow(unused_imports)]
use crate::engine::core::*;
use crate::engine::ast::{Node, PropValue};
use crate::engine::builder::*;

pub struct TextInput {
    pub props: Vec<(String, PropValue)>,
    pub children: Vec<Node>,
}
impl TextInput {
    pub fn new() -> Self {
        Self {
            props: vec![],
            children: vec![],
        }
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.props.push(("value".to_string(), PropValue::String(value.into())));
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.props.push(("placeholder".to_string(), PropValue::String(placeholder.into())));
        self
    }
    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.props.push(("width".to_string(), PropValue::String(width.into())));
        self
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
}
impl IntoNode for TextInput {
    fn into_node(self) -> Node {
        Node::Element {
            name: "textinput".to_string(),
            props: self.props,
            children: self.children,
        }
    }
}

#[macro_export]
macro_rules! TextInput {
    () => {{
        $crate::engine::primitives::TextInput::new()
    }};
    ($val:expr) => {{
        $crate::engine::primitives::TextInput::new().value($val)
    }};
}

pub const TEXTINPUT_ANDROID_SNIPPET: &str = r#"
"textinput" -> {
    val externalValue = node.getString("value", "")
        val placeholder = node.getString("placeholder", "")
        val onChangeStr = node.getString("on_change", "")
        var localText by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(externalValue) }
        var lastSent by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(externalValue) }
        var isFocused by androidx.compose.runtime.remember { androidx.compose.runtime.mutableStateOf(false) }
    
        androidx.compose.runtime.LaunchedEffect(externalValue, isFocused) {
            if (!isFocused && externalValue != localText) {
                localText = externalValue
                lastSent = externalValue
            }
        }
    
        OutlinedTextField(
            value = localText,
            onValueChange = { newText ->
                localText = newText
                if (onChangeStr.isNotEmpty() && newText != lastSent) {
                    lastSent = newText
                    dispatch(onChangeStr, org.json.JSONObject().put("value", newText))
                }
            },
            placeholder = { Text(placeholder) },
            singleLine = true,
            shape = androidx.compose.foundation.shape.RoundedCornerShape(10.dp),
            modifier = modifier
                .fillMaxWidth()
                .onFocusChanged { isFocused = it.isFocused }
        )
}
"#;

pub const TEXTINPUT_IOS_SNIPPET: &str = r#"
case "textinput":
    let value = node.props["value"] as? String ?? ""
        let placeholder = node.props["placeholder"] as? String ?? ""
        let onChangeStr = node.props["on_change"] as? String ?? ""
        DynamicTextInputWrapper(value: value, placeholder: placeholder, onChangeAction: onChangeStr, dispatch: dispatch)
"#;

pub const TEXTINPUT_GTK_SNIPPET: &str = r#"
"textinput" => {
    let value = node.get_string("value", "");
        let placeholder = node.get_string("placeholder", "");
        let on_change_str = node.get_string("on_change", "");
        let entry = gtk4::Entry::new();
        entry.set_hexpand(true);
        entry.set_placeholder_text(Some(&placeholder));
        entry.set_text(&value);
        entry.set_widget_name(&on_change_str);
        let disp = dispatch.clone();
        let last_sent = std::rc::Rc::new(std::cell::RefCell::new(value));
        entry.connect_changed(move |e| {
            let is_focused = e.has_focus() || e.focus_child().is_some();
            if !is_focused {
                return;
            }
            let act = e.widget_name();
            let txt = e.text().to_string();
            if !act.is_empty() && *last_sent.borrow() != txt {
                *last_sent.borrow_mut() = txt.clone();
                disp(act.to_string(), Some(serde_json::json!({ "value": txt })));
            }
        });
        entry.upcast::<gtk4::Widget>()
}
"#;

pub const TEXTINPUT_WEB_SNIPPET: &str = r#"
case "textinput": {
    const { value = "", placeholder = "", on_change = "" } = node.props || {};
        const el = h("input", {
            className: "nk-textinput",
            attrs: { type: "text", placeholder: placeholder, value: value },
            style: {
                width: "100%",
                padding: "10px 12px",
                fontSize: "15px",
                borderRadius: "10px",
                border: "1px solid var(--rn-outline-variant, #d1d1d6)",
                backgroundColor: "var(--rn-surface, #ffffff)",
                color: "var(--rn-text, #000000)",
                outline: "none",
                boxSizing: "border-box",
            },
            onInput: (e) => {
                const newVal = e.target.value;
                if (el.__action && newVal !== el.__lastSent) {
                    el.__lastSent = newVal;
                    dispatch(el.__action, { value: newVal });
                }
            },
        });
        el.value = value;
        el.__lastSent = value;
        el.__action = on_change;
        return el;
}
"#;

pub const TEXTINPUT_WINDOWS_SNIPPET: &str = r#"
case "textinput": {
    string valStr = (node.Props != null && node.Props.ContainsKey("value")) ? node.Props["value"].ToString() : "";
        string placeholder = (node.Props != null && node.Props.ContainsKey("placeholder")) ? node.Props["placeholder"].ToString() : "";
        string onChange = (node.Props != null && node.Props.ContainsKey("on_change")) ? node.Props["on_change"].ToString() : "";
        var inputBox = new TextBox {
            Text = valStr,
            PlaceholderText = placeholder,
            HorizontalAlignment = HorizontalAlignment.Stretch,
            CornerRadius = new CornerRadius(6),
            Tag = onChange
        };
        string lastSent = valStr;
        inputBox.TextChanged += (s, e) => {
            if (inputBox.FocusState != FocusState.Unfocused && inputBox.Tag is string act && !string.IsNullOrEmpty(act) && inputBox.Text != lastSent) {
                lastSent = inputBox.Text;
                Dispatch(act, inputBox.Text);
            }
        };
        return inputBox;
}
"#;
