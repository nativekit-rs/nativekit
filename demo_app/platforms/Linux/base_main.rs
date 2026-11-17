
#[cfg(feature = "prod")]
static mut APP_PTR: *mut std::ffi::c_void = std::ptr::null_mut();

#[cfg(feature = "prod")]
extern "C" fn prod_refresh_callback() {
    gtk4::glib::idle_add_once(|| {
        unsafe {
            if !APP_PTR.is_null() {
                let ui_c_str = demo_app::get_ui(APP_PTR);
                if !ui_c_str.is_null() {
                    let ui_str = std::ffi::CStr::from_ptr(ui_c_str).to_str().unwrap_or("");
                    update_ui(ui_str);
                }
            }
        }
    });
}

use libadwaita::prelude::*;
use libadwaita::Application;
use gtk4::Widget;
use libadwaita as adw;

#[derive(Clone, PartialEq)]
pub struct DynamicUINode {
    pub name: String,
    pub props: serde_json::Map<String, serde_json::Value>,
    pub children: Vec<DynamicUINode>,
}

impl DynamicUINode {
    pub fn get_string(&self, key: &str, default: &str) -> String {
        self.props.get(key).and_then(|v| v.as_str()).unwrap_or(default).to_string()
    }
    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        self.props.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    }
    pub fn get_int(&self, key: &str, default: u32) -> u32 {
        self.props.get(key).and_then(|v| v.as_u64()).map(|v| v as u32).unwrap_or(default)
    }
    pub fn get_u32(&self, key: &str, default: u32) -> u32 {
        self.get_int(key, default)
    }
}

fn parse_dynamic_node(b64: &str, existing: Option<&DynamicUINode>) -> DynamicUINode {
    use base64::Engine;
    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(b64.trim().trim_matches('"')) {
        let mut offset = 0;
        let read_u8 = |o: &mut usize| -> u8 { let v = bytes.get(*o).copied().unwrap_or(0); *o += 1; v };
        let read_u16 = |o: &mut usize| -> u16 { 
            let mut b = [0; 2]; 
            if *o + 2 <= bytes.len() { b.copy_from_slice(&bytes[*o..*o+2]); }
            *o += 2; u16::from_le_bytes(b) 
        };
        
        if bytes.is_empty() { return DynamicUINode { name: "error".to_string(), props: serde_json::Map::new(), children: vec![] }; }
        
        let header = read_u8(&mut offset);
        if header == 0 {
            return decode_node(&bytes, &mut offset);
        } else if header == 1 {
            if let Some(mut current_root) = existing.cloned() {
                let patch_count = read_u16(&mut offset);
                for _ in 0..patch_count {
                    let ptype = read_u8(&mut offset);
                    let path_len = read_u16(&mut offset);
                    let mut path = vec![];
                    for _ in 0..path_len { path.push(read_u16(&mut offset) as usize); }
                    current_root = apply_patch(current_root, &path, 0, ptype, &bytes, &mut offset);
                }
                return current_root;
            }
        }
    }
    DynamicUINode { name: "error".to_string(), props: serde_json::Map::new(), children: vec![] }
}

fn apply_patch(mut node: DynamicUINode, path: &[usize], path_idx: usize, ptype: u8, data: &[u8], offset: &mut usize) -> DynamicUINode {
    if path_idx == path.len() {
        if ptype == 0 { return decode_node(data, offset); }
        else if ptype == 1 {
            let read_u8 = |o: &mut usize| -> u8 { let v = data.get(*o).copied().unwrap_or(0); *o += 1; v };
            let read_u16 = |o: &mut usize| -> u16 { 
                let mut b = [0; 2]; 
                if *o + 2 <= data.len() { b.copy_from_slice(&data[*o..*o+2]); }
                *o += 2; u16::from_le_bytes(b) 
            };
            let read_u32 = |o: &mut usize| -> u32 { 
                let mut b = [0; 4]; 
                if *o + 4 <= data.len() { b.copy_from_slice(&data[*o..*o+4]); }
                *o += 4; u32::from_le_bytes(b) 
            };
            let read_str = |o: &mut usize| -> String {
                let len = read_u16(o) as usize;
                let s = if *o + len <= data.len() { String::from_utf8_lossy(&data[*o..*o+len]).to_string() } else { "".to_string() };
                *o += len; s
            };
            let prop_count = read_u16(offset);
            for _ in 0..prop_count {
                let k = read_str(offset);
                let vtype = read_u8(offset);
                if vtype == 0 { node.props.insert(k, serde_json::Value::String(read_str(offset))); }
                else if vtype == 1 { node.props.insert(k, serde_json::Value::Bool(read_u8(offset) != 0)); }
                else if vtype == 2 { node.props.insert(k, serde_json::Value::Number(read_u32(offset).into())); }
                else if vtype == 3 { node.props.insert(k, serde_json::Value::String(read_str(offset))); }
                else if vtype == 4 { node.props.insert(k, serde_json::Value::String(read_str(offset))); }
                else if vtype == 5 {
                    let len = read_u32(offset) as usize;
                    if *offset + len <= data.len() {
                        if let Ok(j) = serde_json::from_slice(&data[*offset..*offset+len]) {
                            node.props.insert(k, j);
                        }
                    }
                    *offset += len;
                }
            }
            return node;
        } else if ptype == 2 {
            node.children.push(decode_node(data, offset));
            return node;
        } else if ptype == 3 {
            let read_u16 = |o: &mut usize| -> u16 { 
                let mut b = [0; 2]; 
                if *o + 2 <= data.len() { b.copy_from_slice(&data[*o..*o+2]); }
                *o += 2; u16::from_le_bytes(b) 
            };
            let len = read_u16(offset) as usize;
            node.children.truncate(len);
            return node;
        }
        return node;
    }
    let child_idx = path[path_idx];
    if child_idx < node.children.len() {
        let child = node.children.remove(child_idx);
        let new_child = apply_patch(child, path, path_idx + 1, ptype, data, offset);
        node.children.insert(child_idx, new_child);
    }
    node
}

fn decode_node(data: &[u8], offset: &mut usize) -> DynamicUINode {
    let read_u8 = |o: &mut usize| -> u8 { let v = data.get(*o).copied().unwrap_or(0); *o += 1; v };
    let read_u16 = |o: &mut usize| -> u16 { 
        let mut b = [0; 2]; 
        if *o + 2 <= data.len() { b.copy_from_slice(&data[*o..*o+2]); }
        *o += 2; u16::from_le_bytes(b) 
    };
    let read_u32 = |o: &mut usize| -> u32 { 
        let mut b = [0; 4]; 
        if *o + 4 <= data.len() { b.copy_from_slice(&data[*o..*o+4]); }
        *o += 4; u32::from_le_bytes(b) 
    };
    let read_str = |o: &mut usize| -> String {
        let len = read_u16(o) as usize;
        let s = if *o + len <= data.len() { String::from_utf8_lossy(&data[*o..*o+len]).to_string() } else { "".to_string() };
        *o += len; s
    };

    let ntype = read_u8(offset);
    if ntype == 1 {
        let name = read_str(offset).to_lowercase();
        let props_count = read_u16(offset);
        let mut props = serde_json::Map::new();
        for _ in 0..props_count {
            let k = read_str(offset);
            let v_type = read_u8(offset);
            match v_type {
                0 => { props.insert(k, serde_json::Value::String(read_str(offset))); }
                1 => { props.insert(k, serde_json::Value::Bool(read_u8(offset) != 0)); }
                2 => { props.insert(k, serde_json::Value::Number(serde_json::Number::from(read_u32(offset)))); }
                3 | 4 => { props.insert(k, serde_json::Value::String(read_str(offset))); }
                5 => {
                    let len = read_u32(offset) as usize;
                    if *offset + len <= data.len() {
                        if let Ok(j) = serde_json::from_slice(&data[*offset..*offset+len]) {
                            props.insert(k, j);
                        }
                    }
                    *offset += len;
                }
                _ => {}
            }
        }
        
        let children_count = read_u16(offset);
        let mut children = Vec::with_capacity(children_count as usize);
        for _ in 0..children_count {
            children.push(decode_node(data, offset));
        }
        DynamicUINode { name, props, children }
    } else {
        let len = read_u32(offset) as usize;
        *offset += len;
        DynamicUINode { name: "raw".to_string(), props: serde_json::Map::new(), children: vec![] }
    }
}

fn map_adwaita_icon(icon: &str) -> &'static str {
    match icon.to_lowercase().as_str() {
        "house" | "home" => "go-home-symbolic",
        "bag" | "orders" | "cart" | "shop" => "package-x-generic-symbolic",
        "gear" | "settings" => "preferences-system-symbolic",
        "person" | "user" | "profile" => "avatar-default-symbolic",
        _ => "starred-symbolic",
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
static LAST_APPLIED_TS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

static CURRENT_ACTIVE_TAB_IDX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(999);

fn setup_view_switcher(node: &DynamicUINode) {
    if node.name != "tabbar" {
        HEADER_BAR.with(|h| {
            if let Some(header) = h.borrow().as_ref() {
                WINDOW_TITLE.with(|t| {
                    if let Some(w_title) = t.borrow().as_ref() {
                        header.set_title_widget(Some(w_title));
                    }
                });
            }
        });
        return;
    }

    let active_index = node.get_u32("active_index", 0) as usize;
    CURRENT_ACTIVE_TAB_IDX.store(active_index, std::sync::atomic::Ordering::SeqCst);
    let tabs_json = node.props.get("tabs");

    let mut actions = Vec::new();
    let mut tab_defs = Vec::new();
    if let Some(serde_json::Value::Array(tabs)) = tabs_json {
        for tab_val in tabs {
            let title = tab_val.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let icon = tab_val.get("linux_icon").or_else(|| tab_val.get("icon")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let action = tab_val.get("action").and_then(|v| v.as_str()).unwrap_or("").to_string();
            actions.push(action);
            tab_defs.push((title, icon));
        }
    }

    TAB_ACTIONS.with(|ta| {
        *ta.borrow_mut() = actions;
    });

    let child_page = node.children.first();
    let (_, _, on_back, _) = child_page.map(get_active_page_info).unwrap_or_default();
    let is_drilled_in = !on_back.is_empty();

    VIEW_STACK.with(|vs| {
        let mut vs_borrow = vs.borrow_mut();
        if vs_borrow.is_none() && !tab_defs.is_empty() {
            let stack = adw::ViewStack::new();
            for (i, (title, icon)) in tab_defs.iter().enumerate() {
                let dummy = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
                let name = format!("tab_{}", i);
                let icon_name = if icon.ends_with("-symbolic") { icon.as_str() } else { map_adwaita_icon(icon) };
                let page = stack.add_titled(&dummy, Some(&name), title);
                page.set_icon_name(Some(icon_name));
            }

            let target_name = format!("tab_{}", active_index);
            stack.set_visible_child_name(&target_name);

            stack.connect_visible_child_name_notify(|s| {
                if let Some(name) = s.visible_child_name() {
                    if let Some(idx_str) = name.strip_prefix("tab_") {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            let cur = CURRENT_ACTIVE_TAB_IDX.load(std::sync::atomic::Ordering::SeqCst);
                            if idx != cur {
                                CURRENT_ACTIVE_TAB_IDX.store(idx, std::sync::atomic::Ordering::SeqCst);
                                let act_opt = TAB_ACTIONS.with(|ta| ta.borrow().get(idx).cloned());
                                let disp_opt = CURRENT_DISPATCH.with(|d| d.borrow().clone());
                                if let (Some(act), Some(disp)) = (act_opt, disp_opt) {
                                    if !act.is_empty() {
                                        disp(act, None);
                                    }
                                }
                            }
                        }
                    }
                }
            });

            let switcher = adw::ViewSwitcher::builder()
                .stack(&stack)
                .policy(adw::ViewSwitcherPolicy::Wide)
                .build();

            VIEW_SWITCHER.with(|vsw| {
                *vsw.borrow_mut() = Some(switcher);
            });
            *vs_borrow = Some(stack);
        }

        if let Some(stack) = vs_borrow.as_ref() {
            for (i, (title, icon)) in tab_defs.iter().enumerate() {
                let name = format!("tab_{}", i);
                if let Some(child) = stack.child_by_name(&name) {
                    let page = stack.page(&child);
                    page.set_title(Some(title));
                    let ic = if icon.ends_with("-symbolic") { icon.as_str() } else { map_adwaita_icon(icon) };
                    page.set_icon_name(Some(ic));
                }
            }
            let target_name = format!("tab_{}", active_index);
            if stack.visible_child_name().as_deref() != Some(target_name.as_str()) {
                stack.set_visible_child_name(&target_name);
            }
        }
    });

    HEADER_BAR.with(|h| {
        if let Some(header) = h.borrow().as_ref() {
            if is_drilled_in {
                WINDOW_TITLE.with(|t| {
                    if let Some(w_title) = t.borrow().as_ref() {
                        if header.title_widget().as_ref() != Some(w_title.upcast_ref()) {
                            header.set_title_widget(Some(w_title));
                        }
                    }
                });
            } else {
                VIEW_SWITCHER.with(|vsw| {
                    if let Some(switcher) = vsw.borrow().as_ref() {
                        if header.title_widget().as_ref() != Some(switcher.upcast_ref()) {
                            header.set_title_widget(Some(switcher));
                        }
                    }
                });
            }
        }
    });
}

#[derive(Clone)]
struct ModalLayerInfo {
    pages: Vec<DynamicUINode>,
    kind: String,
    on_pop: String,
    on_dismiss: String,
}

fn extract_nav_layers(nav_node: &DynamicUINode) -> (Vec<DynamicUINode>, Vec<ModalLayerInfo>) {
    let mut window_pages = Vec::new();
    let mut modal_layers = Vec::new();
    if nav_node.children.is_empty() {
        return (window_pages, modal_layers);
    }

    let has_overlay = nav_node.get_bool("has_overlay", false);
    let window_count = nav_node.get_u32(
        "window_count",
        if has_overlay { nav_node.children.len().saturating_sub(1) as u32 } else { nav_node.children.len() as u32 },
    ) as usize;
    let win_clamped = window_count.min(nav_node.children.len());
    for i in 0..win_clamped {
        window_pages.push(nav_node.children[i].clone());
    }

    let default_pop = nav_node.get_string("on_pop", "");
    let default_dismiss = {
        let d = nav_node.get_string("on_dismiss", "");
        if d.is_empty() { default_pop.clone() } else { d }
    };

    let mut cur_nav = nav_node.clone();
    let mut cur_win_count = win_clamped;
    let mut cur_has_overlay = has_overlay;

    while cur_has_overlay && cur_nav.children.len() > cur_win_count {
        let kind = cur_nav.get_string("overlay_kind", "sheet");
        let on_pop = {
            let p = cur_nav.get_string("on_pop", "");
            if p.is_empty() { default_pop.clone() } else { p }
        };
        let on_dismiss = {
            let d = cur_nav.get_string("on_dismiss", "");
            if d.is_empty() {
                if default_dismiss.is_empty() { on_pop.clone() } else { default_dismiss.clone() }
            } else {
                d
            }
        };

        let first_overlay = cur_nav.children[cur_win_count].clone();
        if first_overlay.name == "navigationstack" {
            let next_has_overlay = first_overlay.get_bool("has_overlay", false);
            let next_count = first_overlay.children.len();
            let next_win_count = (first_overlay.get_u32(
                "window_count",
                if next_has_overlay { next_count.saturating_sub(1) as u32 } else { next_count as u32 },
            ) as usize)
                .min(next_count);

            let mut layer_pages = Vec::new();
            for i in 0..next_win_count {
                layer_pages.push(first_overlay.children[i].clone());
            }
            let layer_pop = {
                let p = first_overlay.get_string("on_pop", "");
                if p.is_empty() { on_pop.clone() } else { p }
            };
            let layer_dismiss = {
                let d = first_overlay.get_string("on_dismiss", "");
                if d.is_empty() { on_dismiss.clone() } else { d }
            };

            if !layer_pages.is_empty() {
                modal_layers.push(ModalLayerInfo {
                    pages: layer_pages,
                    kind,
                    on_pop: layer_pop,
                    on_dismiss: layer_dismiss,
                });
            }

            cur_nav = first_overlay;
            cur_win_count = next_win_count;
            cur_has_overlay = next_has_overlay;
        } else {
            let mut layer_pages = Vec::new();
            for i in cur_win_count..cur_nav.children.len() {
                layer_pages.push(cur_nav.children[i].clone());
            }
            if !layer_pages.is_empty() {
                modal_layers.push(ModalLayerInfo {
                    pages: layer_pages,
                    kind,
                    on_pop,
                    on_dismiss,
                });
            }
            break;
        }
    }

    (window_pages, modal_layers)
}

#[derive(Clone)]
struct ActiveModalDialog {
    dialog: adw::Dialog,
    nav_view: adw::NavigationView,
    kind: String,
    titles: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    on_pop: std::rc::Rc<std::cell::RefCell<String>>,
    on_dismiss: std::rc::Rc<std::cell::RefCell<String>>,
    closed_by_rust: std::rc::Rc<std::cell::Cell<bool>>,
}

fn build_modal_page_widget(
    page: &DynamicUINode,
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>,
) -> gtk4::Widget {
    let body_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    body_box.set_margin_start(24);
    body_box.set_margin_end(24);
    body_box.set_margin_top(8);
    body_box.set_margin_bottom(24);

    if page.name == "page" {
        for child in &page.children {
            body_box.append(&parse_dynamic_widget(child, dispatch));
        }
    } else {
        body_box.append(&parse_dynamic_widget(page, dispatch));
    }
    body_box.upcast::<gtk4::Widget>()
}

fn update_modal_page_widget(
    body_widget: &gtk4::Widget,
    page: &DynamicUINode,
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>,
) -> bool {
    let Some(box_w) = body_widget.downcast_ref::<gtk4::Box>() else {
        return false;
    };
    let mut existing = Vec::new();
    let mut c_opt = box_w.first_child();
    while let Some(c) = c_opt {
        existing.push(c.clone());
        c_opt = c.next_sibling();
    }
    if page.name == "page" {
        if existing.len() != page.children.len() {
            return false;
        }
        for i in 0..existing.len() {
            if !update_existing_widget(&existing[i], &page.children[i], dispatch) {
                return false;
            }
        }
        true
    } else {
        if existing.len() != 1 {
            return false;
        }
        update_existing_widget(&existing[0], page, dispatch)
    }
}

fn build_modal_nav_page(
    page: &DynamicUINode,
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>,
) -> adw::NavigationPage {
    let modal_title = page.get_string("title", "");
    let modal_subtitle = page.get_string("subtitle", "");
    let top_style = page.get_string("top_style", "soft");

    let header = adw::HeaderBar::new();
    if top_style != "hard" {
        header.add_css_class("flat");
    }
    if !modal_subtitle.is_empty() {
        let win_title = adw::WindowTitle::new(&modal_title, &modal_subtitle);
        header.set_title_widget(Some(&win_title));
    }

    let body_widget = build_modal_page_widget(page, dispatch);
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&body_widget));

    adw::NavigationPage::new(&toolbar_view, &modal_title)
}

fn sync_modals(
    modal_layers: &[ModalLayerInfo],
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>,
) {
    let mut mismatch_from = modal_layers.len();
    ACTIVE_DIALOGS.with(|dialogs| {
        let list = dialogs.borrow();
        for (idx, entry) in list.iter().enumerate() {
            if idx >= modal_layers.len() || entry.kind != modal_layers[idx].kind {
                mismatch_from = idx;
                break;
            }
        }
    });

    loop {
        let popped = ACTIVE_DIALOGS.with(|dialogs| {
            let mut list = dialogs.borrow_mut();
            if list.len() > mismatch_from {
                list.pop()
            } else {
                None
            }
        });
        if let Some(removed) = popped {
            removed.closed_by_rust.set(true);
            removed.dialog.close();
            CLOSING_DIALOGS.with(|c| c.borrow_mut().push(removed));
        } else {
            break;
        }
    }

    let existing_entries: Vec<ActiveModalDialog> = ACTIVE_DIALOGS.with(|d| d.borrow().clone());
    for (idx, entry) in existing_entries.iter().enumerate() {
        if let Some(layer) = modal_layers.get(idx) {
            *entry.on_pop.borrow_mut() = layer.on_pop.clone();
            *entry.on_dismiss.borrow_mut() = layer.on_dismiss.clone();

            let cur_depth = layer.pages.len();
            let prev_depth = entry.titles.borrow().len();

            if cur_depth > prev_depth {
                SYNCING_MODALS.with(|s| s.set(true));
                for i in prev_depth..cur_depth {
                    let p = &layer.pages[i];
                    let title = p.get_string("title", "");
                    entry.titles.borrow_mut().push(title.clone());
                    let nav_page = build_modal_nav_page(p, dispatch);
                    entry.nav_view.push(&nav_page);
                    entry.dialog.set_title(&title);
                }
                SYNCING_MODALS.with(|s| s.set(false));
            } else if cur_depth < prev_depth && cur_depth > 0 {
                SYNCING_MODALS.with(|s| s.set(true));
                while entry.titles.borrow().len() > cur_depth {
                    entry.titles.borrow_mut().pop();
                    entry.nav_view.pop();
                }
                if let Some(last_t) = entry.titles.borrow().last() {
                    entry.dialog.set_title(last_t);
                }
                SYNCING_MODALS.with(|s| s.set(false));
            } else if let Some(active_page) = layer.pages.last() {
                let cur_title = active_page.get_string("title", "");
                let prev_title = entry.titles.borrow().last().cloned().unwrap_or_default();
                if cur_title != prev_title {
                    if let Some(last_slot) = entry.titles.borrow_mut().last_mut() {
                        *last_slot = cur_title.clone();
                    }
                    entry.dialog.set_title(&cur_title);
                    if let Some(vis_page) = entry.nav_view.visible_page() {
                        vis_page.set_title(&cur_title);
                        if let Some(tb) = vis_page.child().and_then(|w| w.downcast::<adw::ToolbarView>().ok()) {
                            let new_body = build_modal_page_widget(active_page, dispatch);
                            tb.set_content(Some(&new_body));
                        }
                    }
                } else if let Some(vis_page) = entry.nav_view.visible_page() {
                    if let Some(tb) = vis_page.child().and_then(|w| w.downcast::<adw::ToolbarView>().ok()) {
                        let updated = if let Some(content) = tb.content() {
                            update_modal_page_widget(&content, active_page, dispatch)
                        } else {
                            false
                        };
                        if !updated {
                            let new_body = build_modal_page_widget(active_page, dispatch);
                            tb.set_content(Some(&new_body));
                        }
                    }
                }
            }
        }
    }

    let Some(win) = MAIN_WINDOW.with(|w| w.borrow().clone()) else {
        return;
    };
    for idx in existing_entries.len()..modal_layers.len() {
        let layer = &modal_layers[idx];
        let is_form = layer.kind == "form" || layer.kind == "dialog" || layer.kind == "modal" || layer.kind == "fullscreen";

        let dialog = adw::Dialog::new();
        if is_form {
            dialog.set_presentation_mode(adw::DialogPresentationMode::Floating);
            dialog.set_content_width(480);
            dialog.set_content_height(500);
            dialog.set_follows_content_size(false);
        } else {
            dialog.set_presentation_mode(adw::DialogPresentationMode::BottomSheet);
            dialog.set_follows_content_size(true);
        }

        let nav_view = adw::NavigationView::new();
        let mut initial_titles = Vec::new();
        for (i, page) in layer.pages.iter().enumerate() {
            let title = page.get_string("title", "");
            initial_titles.push(title.clone());
            let nav_page = build_modal_nav_page(page, dispatch);
            if i == 0 {
                nav_view.add(&nav_page);
            } else {
                nav_view.push(&nav_page);
            }
            dialog.set_title(&title);
        }

        dialog.set_child(Some(&nav_view));

        let titles_rc = std::rc::Rc::new(std::cell::RefCell::new(initial_titles));
        let on_pop_rc = std::rc::Rc::new(std::cell::RefCell::new(layer.on_pop.clone()));
        let on_dismiss_rc = std::rc::Rc::new(std::cell::RefCell::new(layer.on_dismiss.clone()));
        let closed_by_rust_rc = std::rc::Rc::new(std::cell::Cell::new(false));

        {
            let titles_pop = titles_rc.clone();
            let on_pop_cell = on_pop_rc.clone();
            let closed_by_rust_pop = closed_by_rust_rc.clone();
            let dialog_weak = dialog.downgrade();
            nav_view.connect_popped(move |nv, _| {
                if let Some(vis) = nv.visible_page() {
                    if let Some(d) = dialog_weak.upgrade() {
                        d.set_title(&vis.title());
                    }
                }
                if !SYNCING_MODALS.with(|s| s.get()) && !closed_by_rust_pop.get() {
                    titles_pop.borrow_mut().pop();
                    let act = on_pop_cell.borrow().clone();
                    if !act.is_empty() {
                        if let Some(disp) = CURRENT_DISPATCH.with(|d| d.borrow().clone()) {
                            disp(act, None);
                        }
                    }
                }
            });
        }

        {
            let on_dismiss_cell = on_dismiss_rc.clone();
            let closed_by_rust_unmap = closed_by_rust_rc.clone();
            let d_weak_for_unmap = dialog.downgrade();
            dialog.connect_unmap(move |_| {
                if let Some(d) = d_weak_for_unmap.upgrade() {
                    CLOSING_DIALOGS.with(|c| {
                        c.borrow_mut().retain(|x| x.dialog != d);
                    });
                    if !closed_by_rust_unmap.get() {
                        closed_by_rust_unmap.set(true);
                        let mut to_close = Vec::new();
                        ACTIVE_DIALOGS.with(|dialogs| {
                            let mut list = dialogs.borrow_mut();
                            if let Some(pos) = list.iter().position(|x| x.dialog == d) {
                                let drained: Vec<ActiveModalDialog> = list.drain(pos..).collect();
                                for extra in drained.into_iter().skip(1) {
                                    extra.closed_by_rust.set(true);
                                    to_close.push(extra.dialog);
                                }
                            }
                        });
                        for extra_d in to_close {
                            extra_d.close();
                        }
                        let act = on_dismiss_cell.borrow().clone();
                        if !act.is_empty() {
                            if let Some(disp) = CURRENT_DISPATCH.with(|disp| disp.borrow().clone()) {
                                disp(act, None);
                            }
                        }
                    }
                }
            });
        }

        let entry = ActiveModalDialog {
            dialog: dialog.clone(),
            nav_view,
            kind: layer.kind.clone(),
            titles: titles_rc,
            on_pop: on_pop_rc,
            on_dismiss: on_dismiss_rc,
            closed_by_rust: closed_by_rust_rc,
        };
        ACTIVE_DIALOGS.with(|d| d.borrow_mut().push(entry));
        dialog.present(&win);
    }
}


thread_local! {
    static MAIN_NAV_TITLES: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    static SYNCING_MAIN_NAV: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static MAIN_PAGE_SEQ: std::cell::Cell<usize> = std::cell::Cell::new(0);
    static MAIN_NAV_TAGS: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
}

fn next_page_tag() -> String {
    MAIN_PAGE_SEQ.with(|seq| {
        let id = seq.get() + 1;
        seq.set(id);
        format!("nav_page_{}", id)
    })
}

pub fn build_main_navigation_stack(
    node: &DynamicUINode,
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>,
) -> gtk4::Widget {
    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let nav_view = adw::NavigationView::new();
    nav_view.set_hexpand(true);
    nav_view.set_vexpand(true);

    let (window_pages, modal_layers) = extract_nav_layers(node);
    let mut titles = Vec::new();
    let mut tags = Vec::new();

    SYNCING_MAIN_NAV.with(|s| s.set(true));
    for (i, page_node) in window_pages.iter().enumerate() {
        let title = page_node.get_string("title", "");
        let tag = next_page_tag();
        titles.push(title.clone());
        tags.push(tag.clone());
        let child_widget = parse_dynamic_widget(page_node, dispatch);
        let nav_page = adw::NavigationPage::builder()
            .title(&title)
            .tag(&tag)
            .child(&child_widget)
            .build();
        if i == 0 {
            nav_view.add(&nav_page);
        } else {
            nav_view.push(&nav_page);
        }
    }
    SYNCING_MAIN_NAV.with(|s| s.set(false));

    MAIN_NAV_TITLES.with(|t| *t.borrow_mut() = titles);
    MAIN_NAV_TAGS.with(|t| *t.borrow_mut() = tags);

    overlay.set_child(Some(&nav_view));
    sync_modals(&modal_layers, dispatch);
    overlay.upcast::<gtk4::Widget>()
}


pub fn build_gtk_image(src: &str, w: u32, h: u32, radius: u32, mode: &str, opacity: u32) -> gtk4::Widget {
    let pic = gtk4::Picture::new();
    pic.set_can_shrink(true);
    pic.set_content_fit(if mode == "contain" || mode == "fit" { gtk4::ContentFit::Contain } else { gtk4::ContentFit::Cover });
    if w > 0 {
        pic.set_size_request(w as i32, h as i32);
        pic.set_halign(gtk4::Align::Start);
    } else {
        pic.set_size_request(-1, if h > 0 { h as i32 } else { -1 });
        pic.set_hexpand(true);
    }
    if opacity < 100 {
        pic.set_opacity((opacity as f64) / 100.0);
    }
    if radius > 0 {
        pic.set_overflow(gtk4::Overflow::Hidden);
        pic.add_css_class("nk-dyn-img");
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".nk-dyn-img {{ border-radius: {}px; }}", radius));
        #[allow(deprecated)]
        pic.style_context().add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_USER);
    }

    if src.starts_with("http://") || src.starts_with("https://") {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        src.hash(&mut hasher);
        let cache_path = std::env::temp_dir().join(format!("nk_img_{:x}.img", hasher.finish()));
        if cache_path.exists() {
            let file = gtk4::gio::File::for_path(&cache_path);
            pic.set_file(Some(&file));
        } else {
            let url_str = src.to_string();
            let path_clone = cache_path.clone();
            let (tx, rx) = std::sync::mpsc::channel::<std::path::PathBuf>();
            std::thread::spawn(move || {
                if let Ok(resp) = reqwest::blocking::get(&url_str) {
                    if let Ok(b) = resp.bytes() {
                        if std::fs::write(&path_clone, &b).is_ok() {
                            let _ = tx.send(path_clone);
                        }
                    }
                }
            });
            let pic_weak = pic.downgrade();
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(30), move || {
                if let Ok(saved_path) = rx.try_recv() {
                    if let Some(p) = pic_weak.upgrade() {
                        let file = gtk4::gio::File::for_path(&saved_path);
                        p.set_file(Some(&file));
                    }
                    return gtk4::glib::ControlFlow::Break;
                }
                if pic_weak.upgrade().is_none() {
                    return gtk4::glib::ControlFlow::Break;
                }
                gtk4::glib::ControlFlow::Continue
            });
        }
    } else if !src.is_empty() {
        let clean = src.trim_start_matches("./").trim_start_matches('/');
        for cand in [clean.to_string(), format!("demo_app/{}", clean), format!("../../{}", clean)] {
            if std::path::Path::new(&cand).exists() {
                let file = gtk4::gio::File::for_path(&cand);
                pic.set_file(Some(&file));
                break;
            }
        }
    }

    pic.upcast::<gtk4::Widget>()
}

mod generated_primitives;
use generated_primitives::*;

fn parse_dynamic_widget(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> Widget {
    let name = node.name.to_lowercase();
    match name.as_str() {
        "text" | "icon" | "button" | "hstack" | "vstack" | "switch" | "navigationstack" | "page" => parse_generated_primitive(node, dispatch),
        "tabbar" => {
            let tab_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
            tab_box.set_hexpand(true);
            tab_box.set_vexpand(true);

            if !node.children.is_empty() {
                let child_w = parse_dynamic_widget(&node.children[0], dispatch);
                tab_box.append(&child_w);
            }
            tab_box.upcast::<gtk4::Widget>()
        }
        "list" | "if" => {
            let box_widget = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
            for child in &node.children {
                box_widget.append(&parse_dynamic_widget(child, dispatch));
            }
            box_widget.upcast::<Widget>()
        }
        _ => parse_generated_primitive(node, dispatch)
    }
}

fn update_existing_widget(
    widget: &gtk4::Widget,
    node: &DynamicUINode,
    dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>
) -> bool {
    let name = node.name.to_lowercase();
    
    let is_match = match name.as_str() {
        "text" => widget.is::<gtk4::Label>(),
        "textinput" => widget.is::<gtk4::Entry>(),
        "image" => widget.is::<gtk4::Picture>(),
        "icon" => widget.is::<gtk4::Image>(),
        "button" => widget.is::<gtk4::Button>(),
        "switch" => widget.is::<gtk4::Switch>(),
        "navigationstack" => widget.is::<gtk4::Overlay>(),
        "tabbar" => widget.is::<gtk4::Box>(),
        "page" => widget.is::<gtk4::ScrolledWindow>() || widget.is::<gtk4::Box>(),
        "hstack" | "vstack" | "list" | "if" => widget.is::<gtk4::Box>(),
        _ => widget.is::<gtk4::Box>(),
    };
    
    if !is_match {
        return false;
    }
    
    match name.as_str() {
        "icon" => {
            let img = widget.downcast_ref::<gtk4::Image>().unwrap();
            let size = node.get_int("size", 20);
            let opacity = node.get_int("opacity", 100);
            img.set_pixel_size(size as i32);
            img.set_opacity((opacity as f64) / 100.0);
            let fresh = parse_dynamic_widget(node, dispatch);
            let fresh_classes = fresh.css_classes();
            if img.css_classes() != fresh_classes {
                let class_strs: Vec<&str> = fresh_classes.iter().map(|s| s.as_str()).collect();
                img.set_css_classes(&class_strs);
            }
        }
        "tabbar" => {
            let box_w = widget.downcast_ref::<gtk4::Box>().unwrap();
            if node.children.is_empty() {
                return true;
            }
            if let Some(first_child) = box_w.first_child() {
                return update_existing_widget(&first_child, &node.children[0], dispatch);
            }
            return false;
        }
        "navigationstack" => {
            if let Some(overlay) = widget.downcast_ref::<gtk4::Overlay>() {
                if let Some(child_w) = overlay.child() {
                    if let Some(nav_view) = child_w.downcast_ref::<adw::NavigationView>() {
                        let (window_pages, modal_layers) = extract_nav_layers(node);
                        let cur_depth = window_pages.len();
                        let prev_depth = MAIN_NAV_TITLES.with(|t| t.borrow().len());

                        if cur_depth > prev_depth {
                            SYNCING_MAIN_NAV.with(|s| s.set(true));
                            for i in prev_depth..cur_depth {
                                let p = &window_pages[i];
                                let title = p.get_string("title", "");
                                let tag = next_page_tag();
                                MAIN_NAV_TITLES.with(|t| t.borrow_mut().push(title.clone()));
                                MAIN_NAV_TAGS.with(|t| t.borrow_mut().push(tag.clone()));
                                let w = parse_dynamic_widget(p, dispatch);
                                let nav_page = adw::NavigationPage::builder()
                                    .title(&title)
                                    .tag(&tag)
                                    .child(&w)
                                    .build();
                                nav_view.push(&nav_page);
                            }
                            SYNCING_MAIN_NAV.with(|s| s.set(false));
                        } else if cur_depth < prev_depth && cur_depth > 0 {
                            if let Some(active_p) = window_pages.last() {
                                let target_tag = MAIN_NAV_TAGS.with(|t| t.borrow().get(cur_depth - 1).cloned().unwrap_or_default());
                                if let Some(target_page) = nav_view.find_page(&target_tag) {
                                    if let Some(w) = target_page.child() {
                                        if !update_existing_widget(&w, active_p, dispatch) {
                                            let new_w = parse_dynamic_widget(active_p, dispatch);
                                            target_page.set_child(Some(&new_w));
                                        }
                                    }
                                }
                            }
                            SYNCING_MAIN_NAV.with(|s| s.set(true));
                            MAIN_NAV_TITLES.with(|t| {
                                let mut list = t.borrow_mut();
                                while list.len() > cur_depth {
                                    list.pop();
                                    nav_view.pop();
                                }
                            });
                            MAIN_NAV_TAGS.with(|t| t.borrow_mut().truncate(cur_depth));
                            SYNCING_MAIN_NAV.with(|s| s.set(false));
                        } else if let Some(active_p) = window_pages.last() {
                            let cur_title = active_p.get_string("title", "");
                            let prev_title = MAIN_NAV_TITLES.with(|t| t.borrow().last().cloned().unwrap_or_default());
                            if cur_title != prev_title {
                                let tag = next_page_tag();
                                MAIN_NAV_TITLES.with(|t| {
                                    if let Some(last) = t.borrow_mut().last_mut() { *last = cur_title.clone(); }
                                });
                                MAIN_NAV_TAGS.with(|t| {
                                    if let Some(last) = t.borrow_mut().last_mut() { *last = tag.clone(); }
                                });
                                let w = parse_dynamic_widget(active_p, dispatch);
                                let nav_page = adw::NavigationPage::builder()
                                    .title(&cur_title)
                                    .tag(&tag)
                                    .child(&w)
                                    .build();
                                SYNCING_MAIN_NAV.with(|s| s.set(true));
                                nav_view.replace(&[nav_page]);
                                SYNCING_MAIN_NAV.with(|s| s.set(false));
                            } else if let Some(vis_page) = nav_view.visible_page() {
                                if let Some(w) = vis_page.child() {
                                    if !update_existing_widget(&w, active_p, dispatch) {
                                        let new_w = parse_dynamic_widget(active_p, dispatch);
                                        vis_page.set_child(Some(&new_w));
                                    }
                                }
                            }
                        }

                        sync_modals(&modal_layers, dispatch);
                        return true;
                    }
                }
            }
            return false;
        }
        "page" => {
            let box_w = if let Some(scrolled) = widget.downcast_ref::<gtk4::ScrolledWindow>() {
                scrolled.child().and_then(|c| {
                    if let Ok(bx) = c.clone().downcast::<gtk4::Box>() {
                        Some(bx)
                    } else if let Ok(vp) = c.downcast::<gtk4::Viewport>() {
                        vp.child().and_then(|inner| inner.downcast::<gtk4::Box>().ok())
                    } else {
                        None
                    }
                })
            } else {
                widget.downcast_ref::<gtk4::Box>().cloned()
            };
            let Some(box_w) = box_w else { return false; };
            let title = node.get_string("title", "");
            let offset = if !title.is_empty() { 1 } else { 0 };

            let mut existing = Vec::new();
            let mut c_opt = box_w.first_child();
            while let Some(c) = c_opt {
                existing.push(c.clone());
                c_opt = c.next_sibling();
            }

            let body_children: Vec<&DynamicUINode> = node.children.iter().filter(|c| c.name != "toolbartop" && c.name != "toolbarbottom").collect();
            if existing.len().saturating_sub(offset) != body_children.len() {
                return false;
            }
            for i in 0..body_children.len() {
                let ok = update_existing_widget(&existing[i + offset], body_children[i], dispatch);
                if !ok {
                    return false;
                }
            }
        }
        "textinput" => {
            let entry = widget.downcast_ref::<gtk4::Entry>().unwrap();
            let new_val = node.get_string("value", "");
            let is_focused = entry.has_focus() || entry.focus_child().is_some();
            if !is_focused && entry.text() != new_val.as_str() {
                entry.set_widget_name("");
                entry.set_text(&new_val);
            }
            entry.set_widget_name(&node.get_string("on_change", ""));
        }
        "image" => {
            return true;
        }
        "text" => {
            let lbl = widget.downcast_ref::<gtk4::Label>().unwrap();
            lbl.set_label(&node.get_string("label", ""));
            let fresh = parse_dynamic_widget(node, dispatch);
            let fresh_classes = fresh.css_classes();
            if lbl.css_classes() != fresh_classes {
                let class_strs: Vec<&str> = fresh_classes.iter().map(|s| s.as_str()).collect();
                lbl.set_css_classes(&class_strs);
            }
            let color = node.get_string("color", "");
            apply_gtk_fg_color(lbl, &color);
        }
        "switch" => {
            let sw = widget.downcast_ref::<gtk4::Switch>().unwrap();
            sw.set_active(node.get_bool("is_on", false));
        }
        "button" => {
            let btn = widget.downcast_ref::<gtk4::Button>().unwrap();
            let width_prop = node.get_string("width", "");
            if width_prop == "fill" {
                btn.set_halign(gtk4::Align::Fill);
                btn.set_hexpand(true);
            } else {
                btn.set_halign(gtk4::Align::Start);
                btn.set_hexpand(false);
            }
            btn.set_widget_name(&node.get_string("action", ""));
            let color = node.get_string("color", "");
            apply_gtk_fg_color(btn, &color);
            if !node.children.is_empty() {
                if let Some(existing_child) = btn.child() {
                    let ok = update_existing_widget(&existing_child, &node.children[0], dispatch);
                    if !ok {
                        let new_w = parse_dynamic_widget(&node.children[0], dispatch);
                        new_w.set_halign(gtk4::Align::Center);
                        btn.set_child(Some(&new_w));
                    } else {
                        existing_child.set_halign(gtk4::Align::Center);
                    }
                } else {
                    let new_w = parse_dynamic_widget(&node.children[0], dispatch);
                    new_w.set_halign(gtk4::Align::Center);
                    btn.set_child(Some(&new_w));
                }
            } else {
                btn.set_child(None::<&gtk4::Widget>);
            }
        }
        "hstack" | "vstack" | "list" | "if" | _ => {
            let box_w = widget.downcast_ref::<gtk4::Box>().unwrap();
            if name == "hstack" || name == "vstack" {
                apply_gtk_container_style(box_w, node);
            }
            
            let mut existing = Vec::new();
            let mut c_opt = box_w.first_child();
            while let Some(c) = c_opt {
                existing.push(c.clone());
                c_opt = c.next_sibling();
            }
            
            if existing.len() != node.children.len() {
                return false; 
            }
            
            for i in 0..existing.len() {
                let ok = update_existing_widget(&existing[i], &node.children[i], dispatch);
                if !ok {
                    return false;
                }
            }
        }
    }
    
    true
}

static SERVER_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn active_server_url() -> &'static str {
    SERVER_URL.get_or_init(|| {
        let candidates = [
            /* CANDIDATE_URLS_HERE */
        ];
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(500))
            .build()
            .unwrap();
        for url in &candidates {
            if let Ok(resp) = client.get(&format!("{}/api/live_reload", url)).send() {
                if resp.status().is_success() {
                    println!("Connected to NativeKit dev server at {}", url);
                    return url.to_string();
                }
            }
        }
        candidates[0].to_string()
    })
}

thread_local! {
    static BRAND_CSS_PROVIDER: std::cell::RefCell<Option<gtk4::CssProvider>> = std::cell::RefCell::new(None);
    static CURRENT_ACCENT_MODE: std::cell::RefCell<String> = std::cell::RefCell::new(String::from("app"));
    static MAIN_WINDOW: std::cell::RefCell<Option<adw::ApplicationWindow>> = std::cell::RefCell::new(None);
    static ACTIVE_DIALOGS: std::cell::RefCell<Vec<ActiveModalDialog>> = std::cell::RefCell::new(Vec::new());
    static CLOSING_DIALOGS: std::cell::RefCell<Vec<ActiveModalDialog>> = std::cell::RefCell::new(Vec::new());
    static SYNCING_MODALS: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static CONTENT_CONTAINER: std::cell::RefCell<Option<gtk4::Box>> = std::cell::RefCell::new(None);
    static CURRENT_RUST_NODE: std::cell::RefCell<Option<DynamicUINode>> = std::cell::RefCell::new(None);
    static HEADER_BAR: std::cell::RefCell<Option<adw::HeaderBar>> = std::cell::RefCell::new(None);
    static WINDOW_TITLE: std::cell::RefCell<Option<adw::WindowTitle>> = std::cell::RefCell::new(None);
    static VIEW_STACK: std::cell::RefCell<Option<adw::ViewStack>> = std::cell::RefCell::new(None);
    static VIEW_SWITCHER: std::cell::RefCell<Option<adw::ViewSwitcher>> = std::cell::RefCell::new(None);
    static TAB_ACTIONS: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    static BACK_BUTTON: std::cell::RefCell<Option<gtk4::Button>> = std::cell::RefCell::new(None);
    static LEADING_TB_BOX: std::cell::RefCell<Option<gtk4::Box>> = std::cell::RefCell::new(None);
    static TRAILING_TB_BOX: std::cell::RefCell<Option<gtk4::Box>> = std::cell::RefCell::new(None);
    static CURRENT_BACK_ACTION: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    static CURRENT_DISPATCH: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>>> = std::cell::RefCell::new(None);
}

fn get_active_toolbar_items(node: &DynamicUINode) -> (Vec<DynamicUINode>, Vec<DynamicUINode>) {
    if node.name == "tabbar" {
        if let Some(first) = node.children.first() { return get_active_toolbar_items(first); }
    } else if node.name == "navigationstack" {
        let (window_pages, _) = extract_nav_layers(node);
        if let Some(active) = window_pages.last() { return get_active_toolbar_items(active); }
    } else if node.name == "page" {
        let mut lead = Vec::new();
        let mut trail = Vec::new();
        if let Some(tb) = node.children.iter().find(|c| c.name == "toolbartop") {
            for item in &tb.children {
                let p = item.get_string("placement", "");
                let p = if p.is_empty() { item.children.first().map(|c| c.get_string("placement", "trailing")).unwrap_or_else(|| "trailing".to_string()) } else { p };
                if p == "leading" { lead.push(item.clone()); } else { trail.push(item.clone()); }
            }
        }
        return (lead, trail);
    }
    (Vec::new(), Vec::new())
}

fn get_active_page_info(node: &DynamicUINode) -> (String, String, String, String) {
    if node.name == "tabbar" {
        if let Some(first_child) = node.children.first() {
            return get_active_page_info(first_child);
        }
    } else if node.name == "navigationstack" {
        let on_pop = node.get_string("on_pop", "");
        let (window_pages, _) = extract_nav_layers(node);
        if let Some(active) = window_pages.last() {
            let title = active.get_string("title", "");
            let subtitle = active.get_string("subtitle", "");
            let top_style = active.get_string("top_style", "soft");
            let mut on_back = active.get_string("on_back", "");
            if on_back.is_empty() && window_pages.len() > 1 {
                on_back = on_pop;
            }
            return (title, subtitle, on_back, top_style);
        }
    } else if node.name == "page" {
        return (
            node.get_string("title", ""),
            node.get_string("subtitle", ""),
            node.get_string("on_back", ""),
            node.get_string("top_style", "soft"),
        );
    }
    ("".to_string(), "".to_string(), "".to_string(), "soft".to_string())
}

fn update_ui(ui_str: &str) {
    CONTENT_CONTAINER.with(|w| {
        if let Some(container) = w.borrow().as_ref() {
            load_css();
            let node = CURRENT_RUST_NODE.with(|r| parse_dynamic_node(ui_str, r.borrow().as_ref()));
            if node.name == "tabbar" {
                let tm = node.get_string("theme_mode", "system");
                let am = node.get_string("accent_mode", "app");
                CURRENT_ACCENT_MODE.with(|m| *m.borrow_mut() = am);
                load_css();
                let sm = adw::StyleManager::default();
                match tm.as_str() {
                    "light" => sm.set_color_scheme(adw::ColorScheme::ForceLight),
                    "dark" => sm.set_color_scheme(adw::ColorScheme::ForceDark),
                    _ => sm.set_color_scheme(adw::ColorScheme::Default),
                }
            }
            let is_duplicate = CURRENT_RUST_NODE.with(|r| r.borrow().as_ref() == Some(&node));
            if is_duplicate && container.first_child().is_some() {
                return;
            }
            CURRENT_RUST_NODE.with(|r| *r.borrow_mut() = Some(node.clone()));
            
            let dispatch: std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)> = std::rc::Rc::new(move |action, payload| {
                #[cfg(feature = "prod")]
                {
                    let action_c = std::ffi::CString::new(action).unwrap();
                    let payload_c = payload.map(|p| std::ffi::CString::new(p.to_string()).unwrap());
                    unsafe {
                        let p_ptr = payload_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
                        let ui_c_str = demo_app::dispatch_event(APP_PTR, action_c.as_ptr(), p_ptr);
                        if !ui_c_str.is_null() {
                            let fresh_ui = std::ffi::CStr::from_ptr(ui_c_str).to_str().unwrap_or("").to_string();
                            gtk4::glib::idle_add_once(move || {
                                update_ui(&fresh_ui);
                            });
                        }
                    }
                    return;
                }
                #[cfg(not(feature = "prod"))]
                {
                    let action_clone = action.clone();
                
                static SENDER: std::sync::OnceLock<std::sync::mpsc::Sender<(String, Option<serde_json::Value>)>> = std::sync::OnceLock::new();
                
                let tx = SENDER.get_or_init(|| {
                    let (tx, rx) = std::sync::mpsc::channel::<(String, Option<serde_json::Value>)>();
                    std::thread::spawn(move || {
                        let server_url = active_server_url();
                        let client = reqwest::blocking::Client::builder()
                            .pool_idle_timeout(std::time::Duration::from_secs(15))
                            .build().unwrap();
                        
                        while let Ok((act, p)) = rx.recv() {
                            let mut body = serde_json::Map::new();
                            body.insert("action".to_string(), serde_json::Value::String(act));
                            if let Some(payload_val) = p {
                                body.insert("payload".to_string(), payload_val);
                            }
                            if let Ok(ui_resp) = client.post(&format!("{}/api/dispatch", server_url)).json(&body).send() {
                                if let Some(ts_hdr) = ui_resp.headers().get("X-Reload-Timestamp").and_then(|h| h.to_str().ok()).and_then(|s| s.parse::<u64>().ok()) {
                                    LAST_APPLIED_TS.store(ts_hdr, std::sync::atomic::Ordering::SeqCst);
                                }
                                if let Ok(fresh_ui) = ui_resp.text() {
                                    gtk4::glib::idle_add_once(move || {
                                        update_ui(&fresh_ui);
                                    });
                                }
                            }
                        }
                    });
                    tx
                });
                
                let _ = tx.send((action_clone, payload));
                }
            });

            CURRENT_DISPATCH.with(|d| {
                *d.borrow_mut() = Some(dispatch.clone());
            });

            let (title, subtitle, on_back, top_style) = get_active_page_info(&node);

            WINDOW_TITLE.with(|t| {
                if let Some(w_title) = t.borrow().as_ref() {
                    let target_title = if title.is_empty() { "/* APP_TITLE_HERE */" } else { &title };
                    if w_title.title() != target_title {
                        w_title.set_title(target_title);
                    }
                    if w_title.subtitle() != subtitle {
                        w_title.set_subtitle(&subtitle);
                    }
                }
            });

            setup_view_switcher(&node);

            HEADER_BAR.with(|h| {
                if let Some(header) = h.borrow().as_ref() {
                    let is_flat = header.has_css_class("flat");
                    if top_style == "hard" && is_flat {
                        header.remove_css_class("flat");
                    } else if top_style != "hard" && !is_flat {
                        header.add_css_class("flat");
                    }
                }
            });

            let (lead_items, trail_items) = get_active_toolbar_items(&node);
            LEADING_TB_BOX.with(|b| if let Some(bx) = b.borrow().as_ref() {
                while let Some(c) = bx.first_child() { bx.remove(&c); }
                for item in &lead_items { bx.append(&parse_dynamic_widget(item, &dispatch)); }
            });
            TRAILING_TB_BOX.with(|b| if let Some(bx) = b.borrow().as_ref() {
                while let Some(c) = bx.first_child() { bx.remove(&c); }
                for item in &trail_items { bx.append(&parse_dynamic_widget(item, &dispatch)); }
            });
            let has_back = !on_back.is_empty() && lead_items.is_empty();
            CURRENT_BACK_ACTION.with(|a| {
                *a.borrow_mut() = if has_back { Some(on_back) } else { None };
            });
            BACK_BUTTON.with(|b| {
                if let Some(btn) = b.borrow().as_ref() {
                    if btn.is_visible() != has_back {
                        btn.set_visible(has_back);
                    }
                }
            });
            
            if let Some(existing) = container.first_child() {
                let ok = update_existing_widget(&existing, &node, &dispatch);
                if !ok {
                    while let Some(c) = container.first_child() {
                        container.remove(&c);
                    }
                    let widget = parse_dynamic_widget(&node, &dispatch);
                    container.append(&widget);
                }
            } else {
                let widget = parse_dynamic_widget(&node, &dispatch);
                container.append(&widget);
            }
        }
    });
}

fn build_ui(app: &Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("/* APP_TITLE_HERE */")
        .default_width(540)
        .default_height(640)
        .build();

    let display = gtk4::gdk::Display::default().unwrap();
    if let Some(settings) = gtk4::Settings::for_display(&display).into() {
        settings.set_gtk_icon_theme_name(Some("Adwaita"));
    }
    let icon_theme = gtk4::IconTheme::for_display(&display);
    icon_theme.add_search_path("/opt/homebrew/share/icons");
    icon_theme.add_search_path("/usr/local/share/icons");
    icon_theme.add_search_path("/usr/share/icons");
    icon_theme.add_search_path(".");
    icon_theme.add_search_path("hicolor");
    icon_theme.add_search_path("icons");
    icon_theme.add_search_path("demo_app/platforms/Linux/icons");
    icon_theme.add_search_path("platforms/Linux/icons");
    window.set_icon_name(Some("icon"));

    let header = adw::HeaderBar::new();
    header.add_css_class("flat");
    
    let back_btn = gtk4::Button::from_icon_name("go-previous-symbolic");
    back_btn.set_visible(false);
    back_btn.connect_clicked(|_| {
        let act = CURRENT_BACK_ACTION.with(|a| a.borrow().clone());
        let disp = CURRENT_DISPATCH.with(|d| d.borrow().clone());
        if let (Some(a), Some(d)) = (act, disp) {
            if !a.is_empty() {
                d(a, None);
            }
        }
    });
    let leading_tb = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let trailing_tb = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    header.pack_start(&back_btn);
    header.pack_start(&leading_tb);
    header.pack_end(&trailing_tb);
    LEADING_TB_BOX.with(|b| *b.borrow_mut() = Some(leading_tb));
    TRAILING_TB_BOX.with(|b| *b.borrow_mut() = Some(trailing_tb));

    let title_widget = adw::WindowTitle::new("/* APP_TITLE_HERE */", "");
    header.set_title_widget(Some(&title_widget));

    let root_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    root_box.append(&header);

    let container_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container_box.set_hexpand(true);
    container_box.set_vexpand(true);
    root_box.append(&container_box);
    
    window.set_content(Some(&root_box));

    MAIN_WINDOW.with(|mw| {
        *mw.borrow_mut() = Some(window.clone());
    });
    HEADER_BAR.with(|h| {
        *h.borrow_mut() = Some(header.clone());
    });
    CONTENT_CONTAINER.with(|w| {
        *w.borrow_mut() = Some(container_box.clone());
    });
    WINDOW_TITLE.with(|t| {
        *t.borrow_mut() = Some(title_widget.clone());
    });
    BACK_BUTTON.with(|b| {
        *b.borrow_mut() = Some(back_btn.clone());
    });

    #[cfg(feature = "prod")]
    unsafe {
        if APP_PTR.is_null() {
            APP_PTR = demo_app::init_app();
            demo_app::register_refresh_callback(prod_refresh_callback);
        }
        let ui_c_str = demo_app::get_ui(APP_PTR);
        if !ui_c_str.is_null() {
            let ui_str = std::ffi::CStr::from_ptr(ui_c_str).to_str().unwrap_or("");
            update_ui(ui_str);
        }
    }

    // Initial fetch and live reload background loop
    #[cfg(not(feature = "prod"))]
    std::thread::spawn(|| {
        let server_url = active_server_url();
        let client = reqwest::blocking::Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();
            
        let mut last_ts = 0;
            
        if let Ok(resp) = client.get(&format!("{}/api/ui_layout", server_url)).send() {
            if let Ok(ui_str) = resp.text() {
                gtk4::glib::idle_add_once(move || {
                    update_ui(&ui_str);
                });
            }
        }
        
        let poll_client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(25))
            .build()
            .unwrap();
        loop {
            let wait_ts = std::cmp::max(last_ts, LAST_APPLIED_TS.load(std::sync::atomic::Ordering::SeqCst));
            if let Ok(resp) = poll_client.get(&format!("{}/api/live_reload?wait_for={}", server_url, wait_ts)).send() {
                if let Ok(json_str) = resp.text() {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(ts) = json.get("timestamp").and_then(|t| t.as_u64()) {
                            let applied = LAST_APPLIED_TS.load(std::sync::atomic::Ordering::SeqCst);
                            last_ts = ts;
                            if ts > applied {
                                LAST_APPLIED_TS.store(ts, std::sync::atomic::Ordering::SeqCst);
                                if let Ok(ui_resp) = client.get(&format!("{}/api/ui_layout", server_url)).send() {
                                    if let Ok(fresh_ui) = ui_resp.text() {
                                        gtk4::glib::idle_add_once(move || {
                                            update_ui(&fresh_ui);
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    });

    window.present();
}

fn load_css() {
    let display = gtk4::gdk::Display::default().unwrap();
    thread_local! {
        static BASE_INIT: std::cell::Cell<bool> = std::cell::Cell::new(false);
    }
    BASE_INIT.with(|init| {
        if !init.get() {
            let base_prov = gtk4::CssProvider::new();
            let base_css = std::fs::read_to_string("theme.css")
                .or_else(|_| std::fs::read_to_string("demo_app/platforms/Linux/theme.css"))
                .unwrap_or_default();
            base_prov.load_from_data(&base_css);
            gtk4::style_context_add_provider_for_display(&display, &base_prov, gtk4::STYLE_PROVIDER_PRIORITY_USER);
            init.set(true);
        }
    });

    let brand_prov = BRAND_CSS_PROVIDER.with(|p| {
        let mut b = p.borrow_mut();
        if b.is_none() {
            let prov = gtk4::CssProvider::new();
            gtk4::style_context_add_provider_for_display(&display, &prov, gtk4::STYLE_PROVIDER_PRIORITY_USER + 1);
            *b = Some(prov);
        }
        b.clone().unwrap()
    });

    let am = CURRENT_ACCENT_MODE.with(|m| m.borrow().clone());
    if am == "system" {
        // NIL: Clear brand CSS completely so vanilla Libadwaita system accent takes over!
        brand_prov.load_from_data("");
    } else {
        let is_dark = adw::StyleManager::default().is_dark();
        let file = if is_dark { "brand-dark.css" } else { "brand-light.css" };
        let alt = format!("demo_app/platforms/Linux/{}", file);
        let css = std::fs::read_to_string(file).or_else(|_| std::fs::read_to_string(&alt)).unwrap_or_default();
        brand_prov.load_from_data(&css);
    }
}

fn main() {
    std::env::set_var("XDG_DATA_DIRS", "/opt/homebrew/share:/usr/local/share:/usr/share");
    let app = Application::builder()
        .application_id("/* APP_ID_HERE */")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_startup(|_| {
        load_css();
        adw::StyleManager::default().connect_dark_notify(|_| load_css());
    });

    app.connect_activate(build_ui);
    app.run();
}
