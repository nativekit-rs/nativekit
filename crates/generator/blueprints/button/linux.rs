use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let btn = gtk4::Button::new();
    let variant = node.get_string("variant", "");
    let color = node.get_string("color", "");
    let opacity = node.get_int("opacity", 100);
    let is_fill = node.get_string("width", "") == "fill";
    btn.set_halign(if is_fill { gtk4::Align::Fill } else { gtk4::Align::Start });
    btn.set_hexpand(is_fill);

    if variant == "primary" {
        btn.add_css_class("suggested-action");
    } else if variant != "secondary" {
        btn.add_css_class("flat");
    }
    apply_gtk_fg_color(&btn, &color);
    if opacity < 100 {
        btn.set_opacity((opacity as f64) / 100.0);
    }
    if node.children.len() == 1 {
        let child_w = parse_dynamic_widget(&node.children[0], dispatch);
        child_w.set_halign(gtk4::Align::Center);
        btn.set_child(Some(&child_w));
    } else if !node.children.is_empty() {
        let box_w = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        box_w.set_halign(gtk4::Align::Center);
        for c in &node.children {
            box_w.append(&parse_dynamic_widget(c, dispatch));
        }
        btn.set_child(Some(&box_w));
    }
    let action_str = node.get_string("action", "");
    btn.set_widget_name(&action_str);
    let disp = dispatch.clone();
    btn.connect_clicked(move |b| {
        let act = b.widget_name();
        if !act.is_empty() {
            disp(act.to_string(), None);
        }
    });
    btn.upcast::<gtk4::Widget>()
}
