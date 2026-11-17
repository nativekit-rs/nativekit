use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
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
