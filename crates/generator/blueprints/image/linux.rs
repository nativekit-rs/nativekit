use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, _dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let src = node.get_string("src", "");
    let size = node.get_int("size", 0);
    let w = node.get_int("width", size);
    let h = node.get_int("height", if size > 0 { size } else { 180 });
    let radius = node.get_int("corner_radius", 0);
    let mode = node.get_string("content_mode", "cover");
    let opacity = node.get_int("opacity", 100);
    crate::build_gtk_image(&src, w, h, radius, &mode, opacity)
}
