use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    if let Some(hdr) = node.children.iter().find(|c| c.name == "section_header").and_then(|c| c.children.first()) {
        let hw = parse_dynamic_widget(hdr, dispatch);
        hw.set_margin_start(4); hw.set_margin_end(4);
        outer.append(&hw);
    }
    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.add_css_class("boxed-list");
    for child in node.children.iter().filter(|c| c.name != "section_header" && c.name != "section_footer") {
        let row = gtk4::ListBoxRow::new();
        let row_bg = child.get_string("background", "");
        let has_row_bg = !row_bg.is_empty() || child.props.contains_key("background_gradient");
        let mut clean_child = child.clone();
        if has_row_bg {
            let mut row_style_props = serde_json::Map::new();
            if let Some(bg_v) = child.props.get("background") {
                row_style_props.insert("background".to_string(), bg_v.clone());
            }
            if let Some(grad_v) = child.props.get("background_gradient") {
                row_style_props.insert("background_gradient".to_string(), grad_v.clone());
            }
            let row_style_node = DynamicUINode {
                name: "row".to_string(),
                props: row_style_props,
                children: vec![],
            };
            apply_gtk_container_style(&row, &row_style_node);
            clean_child.props.remove("background");
            clean_child.props.remove("background_gradient");
        }
        if clean_child.name == "button" {
            row.set_activatable(true);
            row.set_widget_name(&clean_child.get_string("action", ""));
            let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            row_box.set_margin_start(14); row_box.set_margin_end(14); row_box.set_margin_top(12); row_box.set_margin_bottom(12);
            if clean_child.get_bool("is_picker", false) {
                let t_lbl = gtk4::Label::new(Some(&clean_child.get_string("picker_title", "Select")));
                t_lbl.set_halign(gtk4::Align::Start); t_lbl.set_hexpand(true);
                let v_lbl = gtk4::Label::new(Some(&format!("{}  ›", clean_child.get_string("picker_value", ""))));
                v_lbl.add_css_class("dim-label");
                row_box.append(&t_lbl); row_box.append(&v_lbl);
            } else {
                let col = clean_child.get_string("color", "primary");
                for gc in &clean_child.children {
                    let w = parse_dynamic_widget(gc, dispatch);
                    crate::generated_primitives::apply_gtk_fg_color(&w, &col);
                    row_box.append(&w);
                }
            }
            row.set_child(Some(&row_box));
        } else {
            row.set_activatable(false);
            let cw = parse_dynamic_widget(&clean_child, dispatch);
            cw.set_margin_start(14); cw.set_margin_end(14); cw.set_margin_top(12); cw.set_margin_bottom(12);
            row.set_child(Some(&cw));
        }
        list_box.append(&row);
    }
    let disp = dispatch.clone();
    list_box.connect_row_activated(move |_, row| {
        let act = row.widget_name();
        if !act.is_empty() { disp(act.to_string(), None); }
    });
    outer.append(&list_box);
    if let Some(ftr) = node.children.iter().find(|c| c.name == "section_footer").and_then(|c| c.children.first()) {
        let fw = parse_dynamic_widget(ftr, dispatch);
        fw.add_css_class("dim-label"); fw.set_margin_start(8);
        outer.append(&fw);
    }
    outer.upcast::<gtk4::Widget>()
}
