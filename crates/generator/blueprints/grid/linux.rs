use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let cols = std::cmp::max(1, node.get_int("columns", 2)) as i32;
    let spacing = node.get_int("spacing", 12);
    let outer = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    if let Some(hdr) = node.children.iter().find(|c| c.name == "section_header").and_then(|c| c.children.first()) {
        let hw = parse_dynamic_widget(hdr, dispatch);
        hw.set_margin_start(4); hw.set_margin_end(4);
        outer.append(&hw);
    }
    let grid = gtk4::Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_column_spacing(spacing);
    grid.set_row_spacing(spacing);
    for (i, child) in node.children.iter().filter(|c| c.name != "section_header" && c.name != "section_footer").enumerate() {
        let cw = parse_dynamic_widget(child, dispatch);
        cw.set_hexpand(true);
        grid.attach(&cw, (i as i32) % cols, (i as i32) / cols, 1, 1);
    }
    outer.append(&grid);
    if let Some(ftr) = node.children.iter().find(|c| c.name == "section_footer").and_then(|c| c.children.first()) {
        let fw = parse_dynamic_widget(ftr, dispatch);
        fw.add_css_class("dim-label"); fw.set_margin_start(4);
        outer.append(&fw);
    }
    outer.upcast::<gtk4::Widget>()
}
