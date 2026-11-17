use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let spacing = node.get_int("spacing", 0);
    let width_prop = node.get_string("width", "");
    let align_x = node.get_string("align_x", "left");
    let align_y = node.get_string("align_y", "top");
    let bg_img = node.get_string("background_image", "");
    let radius = node.get_int("corner_radius", 0);
    let is_fill = width_prop != "hug";
    let bx = gtk4::Box::new(gtk4::Orientation::Vertical, spacing as i32);
    bx.set_hexpand(is_fill);
    if !is_fill {
        bx.set_halign(match align_x.as_str() {
            "center" => gtk4::Align::Center,
            "right" => gtk4::Align::End,
            _ => gtk4::Align::Start,
        });
    }
    bx.set_valign(match align_y.as_str() {
        "center" => gtk4::Align::Center,
        "bottom" => gtk4::Align::End,
        _ => gtk4::Align::Start,
    });
    let h_align = match align_x.as_str() {
        "center" => gtk4::Align::Center,
        "right" => gtk4::Align::End,
        _ => gtk4::Align::Fill,
    };
    apply_gtk_container_style(&bx, node);
    for child in &node.children {
        let cw = parse_dynamic_widget(child, dispatch);
        if align_x == "center" || align_x == "right" {
            cw.set_halign(h_align);
        }
        bx.append(&cw);
    }
    if !bg_img.is_empty() {
        let overlay = gtk4::Overlay::new();
        if radius > 0 {
            overlay.set_overflow(gtk4::Overflow::Hidden);
            overlay.add_css_class("nk-dyn-bg-ov");
            let prov = gtk4::CssProvider::new();
            prov.load_from_data(&format!(".nk-dyn-bg-ov {{ border-radius: {}px; }}", radius));
            #[allow(deprecated)]
            overlay.style_context().add_provider(&prov, gtk4::STYLE_PROVIDER_PRIORITY_USER);
        }
        let pic = crate::build_gtk_image(&bg_img, 0, 0, radius, "cover", 100);
        overlay.set_child(Some(&pic));
        overlay.add_overlay(&bx);
        return overlay.upcast::<gtk4::Widget>();
    }
    bx.upcast::<gtk4::Widget>()
}
