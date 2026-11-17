use gtk4::prelude::*;
pub fn render(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> gtk4::Widget {
    let b = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    for c in &node.children { b.append(&parse_dynamic_widget(c, dispatch)); }
    b.upcast::<gtk4::Widget>()
}
