use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let spacing = node.get_int("spacing", 0);
    let width_prop = node.get_string("width", "");
    let align_x = node.get_string("align_x", "left");
    let align_y = node.get_string("align_y", "center");
    let is_wrap = node.get_bool("wrap", false);
    let is_hdr = node.get_bool("is_header_bar", false);
    let is_space_between = align_x == "space_between" || (is_hdr && node.children.len() == 2);
    let has_flex = node.children.iter().any(|c| {
        c.get_string("width", "") == "fill"
            || (c.name == "spacer" && c.get_int("size", 0) == 0)
            || (c.name == "vstack" && c.get_string("width", "") != "hug")
    });
    let is_fill = width_prop == "fill" || (width_prop != "hug" && (has_flex || is_space_between));

    if is_wrap {
        let fb = gtk4::FlowBox::new();
        fb.set_selection_mode(gtk4::SelectionMode::None);
        fb.set_column_spacing(spacing as u32);
        fb.set_row_spacing(spacing as u32);
        fb.set_hexpand(is_fill);
        apply_gtk_container_style(&fb, node);
        for child in &node.children {
            fb.insert(&parse_dynamic_widget(child, dispatch), -1);
        }
        return fb.upcast::<gtk4::Widget>();
    }

    let bx = gtk4::Box::new(gtk4::Orientation::Horizontal, spacing as i32);
    bx.set_hexpand(is_fill);
    if !is_fill {
        bx.set_halign(match align_x.as_str() {
            "center" => gtk4::Align::Center,
            "right" => gtk4::Align::End,
            _ => gtk4::Align::Start,
        });
    }
    let v_align = match align_y.as_str() {
        "top" => gtk4::Align::Start,
        "bottom" => gtk4::Align::End,
        _ => gtk4::Align::Center,
    };
    apply_gtk_container_style(&bx, node);
    for (i, child) in node.children.iter().enumerate() {
        if is_space_between && i > 0 {
            let sp = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            sp.set_hexpand(true);
            bx.append(&sp);
        }
        let cw = parse_dynamic_widget(child, dispatch);
        cw.set_valign(v_align);
        let cw_width = child.get_string("width", "");
        if cw_width == "fill" || (child.name == "vstack" && cw_width != "hug" && !is_space_between) {
            cw.set_hexpand(true);
        }
        bx.append(&cw);
    }
    bx.upcast::<gtk4::Widget>()
}
