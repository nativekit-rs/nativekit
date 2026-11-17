use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
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
