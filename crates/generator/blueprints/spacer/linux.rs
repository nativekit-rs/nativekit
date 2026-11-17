use gtk4::prelude::*;

pub fn render(node: &DynamicUINode, _dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let size = node.get_int("size", 0);
    let bx = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    if size > 0 {
        bx.set_size_request(size as i32, size as i32);
    } else {
        bx.set_hexpand(true);
    }
    bx.upcast::<gtk4::Widget>()
}
