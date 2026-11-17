use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
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
