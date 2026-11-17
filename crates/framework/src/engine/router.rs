pub fn resolve_tab_icon_props(key: &str) -> (String, String, String, String, String) {
    #[cfg(not(target_arch = "wasm32"))]
    for p in ["demo_app/config/icons.json", "config/icons.json"] {
        if let Ok(txt) = std::fs::read_to_string(p) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let Some(entry) = json.get(key) {
                    let ios = entry.get("ios").and_then(|v| v.as_str()).unwrap_or(key).to_string();
                    let and = entry.get("android").and_then(|v| v.as_str()).unwrap_or(key).to_string();
                    let lin = entry.get("linux").and_then(|v| v.as_str()).unwrap_or(key).to_string();
                    let win = entry.get("windows").and_then(|v| v.as_str()).unwrap_or(key).to_string();
                    let web = entry.get("web").and_then(|v| v.as_str()).unwrap_or(key).to_string();
                    return (ios, and, lin, win, web);
                }
            }
            break;
        }
    }
    (key.to_string(), key.to_string(), key.to_string(), key.to_string(), key.to_string())
}

use std::collections::HashMap;
use crate::engine::ast::Node;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OverlayState {
    pub kind: String,
    pub path: String,
    pub stack: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Router {
    pub current: String,
    pub history: Vec<String>,
    pub window: Vec<String>,
    pub overlay: Option<OverlayState>,
    #[serde(default)]
    pub sheet_patterns: Vec<String>,
    #[serde(default)]
    pub tab_histories: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub active_tab: Option<String>,
}

use std::sync::{Mutex, OnceLock};

pub fn global_router() -> &'static Mutex<Router> {
    static ROUTER: OnceLock<Mutex<Router>> = OnceLock::new();
    ROUTER.get_or_init(|| Mutex::new(Router::default()))
}

pub fn registered_tab_roots() -> &'static Mutex<Vec<String>> {
    static TABS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    TABS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn register_tab_root(pattern: &str) {
    let mut p = registered_tab_roots().lock().unwrap();
    if !p.iter().any(|x| x == pattern) {
        p.push(pattern.to_string());
    }
}

pub fn registered_sheet_patterns() -> &'static Mutex<Vec<String>> {
    static PATTERNS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    PATTERNS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn register_sheet_pattern(pattern: &str) {
    let mut p = registered_sheet_patterns().lock().unwrap();
    if !p.iter().any(|x| x == pattern) {
        p.push(pattern.to_string());
    }
}

static EPHEMERAL_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

pub fn ephemeral_views() -> &'static Mutex<HashMap<String, Node>> {
    static VIEWS: OnceLock<Mutex<HashMap<String, Node>>> = OnceLock::new();
    VIEWS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_ephemeral_view(node: Node) -> String {
    let id = EPHEMERAL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let path = format!("__ephemeral_{}", id);
    let mut views = ephemeral_views().lock().unwrap();
    views.insert(path.clone(), node);
    path
}

pub fn get_ephemeral_view(path: &str) -> Option<Node> {
    let views = ephemeral_views().lock().unwrap();
    views.get(path).cloned()
}

pub trait ToNavigationTarget {
    fn apply_to_router(self, router: &mut Router);
}

impl ToNavigationTarget for &str {
    fn apply_to_router(self, router: &mut Router) {
        router.navigate(self);
    }
}

impl ToNavigationTarget for String {
    fn apply_to_router(self, router: &mut Router) {
        router.navigate(self);
    }
}

impl ToNavigationTarget for &String {
    fn apply_to_router(self, router: &mut Router) {
        router.navigate(self.as_str());
    }
}

impl ToNavigationTarget for i32 {
    fn apply_to_router(self, router: &mut Router) {
        if self < 0 {
            router.pop_n(self.unsigned_abs() as usize);
        }
    }
}

impl ToNavigationTarget for isize {
    fn apply_to_router(self, router: &mut Router) {
        if self < 0 {
            router.pop_n(self.unsigned_abs());
        }
    }
}

impl ToNavigationTarget for Node {
    fn apply_to_router(self, router: &mut Router) {
        let path = register_ephemeral_view(self);
        router.navigate(path);
    }
}

impl ToNavigationTarget for crate::engine::primitives::Page {
    fn apply_to_router(self, router: &mut Router) {
        let node = crate::engine::builder::IntoNode::into_node(self);
        let path = register_ephemeral_view(node);
        router.navigate(path);
    }
}

impl ToNavigationTarget for crate::engine::primitives::VStack {
    fn apply_to_router(self, router: &mut Router) {
        let node = crate::engine::builder::IntoNode::into_node(self);
        let path = register_ephemeral_view(node);
        router.navigate(path);
    }
}

impl ToNavigationTarget for crate::engine::primitives::HStack {
    fn apply_to_router(self, router: &mut Router) {
        let node = crate::engine::builder::IntoNode::into_node(self);
        let path = register_ephemeral_view(node);
        router.navigate(path);
    }
}

impl ToNavigationTarget for fn() -> Node {
    fn apply_to_router(self, router: &mut Router) {
        let node = self();
        let path = register_ephemeral_view(node);
        router.navigate(path);
    }
}

impl ToNavigationTarget for fn() -> crate::engine::primitives::Page {
    fn apply_to_router(self, router: &mut Router) {
        let node = crate::engine::builder::IntoNode::into_node(self());
        let path = register_ephemeral_view(node);
        router.navigate(path);
    }
}

pub fn navigate_view(view: impl crate::engine::builder::IntoNode) {
    let node = view.into_node();
    navigate(node);
}

pub fn navigate(target: impl ToNavigationTarget) {
    {
        let mut r = global_router().lock().unwrap();
        target.apply_to_router(&mut r);
    }
    crate::engine::dev_server::notify_reload();
}

pub fn switch_tab(tab: impl Into<String>) {
    let t = tab.into();
    {
        let mut r = global_router().lock().unwrap();
        r.switch_tab(&t);
    }
    crate::engine::dev_server::notify_reload();
}

pub fn dismiss() {
    {
        let mut r = global_router().lock().unwrap();
        r.dismiss();
    }
    crate::engine::dev_server::notify_reload();
}


pub fn dev_router_snapshot_path() -> std::path::PathBuf {
    std::env::temp_dir().join("native_kit_dev_router.json")
}

pub fn restore_dev_router() {
    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    {
        let path = dev_router_snapshot_path();
        if let Ok(json_str) = std::fs::read_to_string(&path) {
            if let Ok(mut restored) = serde_json::from_str::<Router>(&json_str) {
                restored.history.retain(|p| !p.starts_with("__ephemeral_"));
                if restored.history.is_empty() {
                    let root = restored.active_tab.clone().unwrap_or_else(|| "/".to_string());
                    restored.history.push(root);
                }
                for stack in restored.tab_histories.values_mut() {
                    stack.retain(|p| !p.starts_with("__ephemeral_"));
                }
                if let Some(last) = restored.history.last() {
                    restored.current = last.clone();
                }
                restored.window = restored.history.clone();
                if let Ok(mut lock) = global_router().lock() {
                    *lock = restored;
                }
            }
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new("/")
    }
}

impl Router {
    pub fn save_dev_snapshot(&self) {
        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        {
            if let Ok(json_str) = serde_json::to_string(self) {
                let _ = std::fs::write(dev_router_snapshot_path(), json_str);
            }
        }
    }

    pub fn new(initial: impl Into<String>) -> Self {
        let init = initial.into();
        Self {
            current: init.clone(),
            history: vec![init.clone()],
            window: vec![init],
            overlay: None,
            sheet_patterns: Vec::new(),
            tab_histories: HashMap::new(),
            active_tab: None,
        }
    }

    pub fn switch_tab(&mut self, tab_pattern: &str) {
        let prev_tab = self.active_tab.clone().unwrap_or_else(|| "/".to_string());
        self.tab_histories.insert(prev_tab.clone(), self.history.clone());

        if tab_pattern == prev_tab {
            self.history = vec![tab_pattern.to_string()];
            self.current = tab_pattern.to_string();
            self.tab_histories.insert(tab_pattern.to_string(), self.history.clone());
        } else {
            self.active_tab = Some(tab_pattern.to_string());
            if let Some(hist) = self.tab_histories.get(tab_pattern).cloned() {
                if hist.is_empty() {
                    self.history = vec![tab_pattern.to_string()];
                    self.current = tab_pattern.to_string();
                } else {
                    self.history = hist;
                    self.current = self.history.last().cloned().unwrap_or_else(|| tab_pattern.to_string());
                }
            } else {
                self.history = vec![tab_pattern.to_string()];
                self.current = tab_pattern.to_string();
                self.tab_histories.insert(tab_pattern.to_string(), self.history.clone());
            }
        }
        self.window = self.history.clone();
        self.overlay = None;
        self.save_dev_snapshot();
    }

    pub fn navigate(&mut self, path: impl Into<String>) {
        let p = path.into();
        let registered_tabs = registered_tab_roots().lock().map(|t| t.clone()).unwrap_or_default();

        if registered_tabs.iter().any(|t| t == &p) {
            self.switch_tab(&p);
            return;
        }

        if p == "/" || p == "root" {
            if registered_tabs.iter().any(|t| t == "/") {
                self.switch_tab("/");
            } else {
                self.pop_to_root();
            }
            return;
        }

        let mut matching_tab_root: Option<String> = None;
        let mut best_tab_len = 0;
        for tab in &registered_tabs {
            if *tab != "/" && p.starts_with(&format!("{}/", tab)) {
                if tab.len() > best_tab_len {
                    best_tab_len = tab.len();
                    matching_tab_root = Some(tab.clone());
                }
            }
        }

        if let Some(tab_root) = matching_tab_root {
            let cur_tab = self.active_tab.clone().unwrap_or_else(|| "/".to_string());
            if cur_tab != tab_root {
                self.tab_histories.insert(cur_tab, self.history.clone());
                self.active_tab = Some(tab_root.clone());
                let mut tab_stack = self.tab_histories.get(&tab_root).cloned().unwrap_or_default();
                if tab_stack.is_empty() || tab_stack.first() != Some(&tab_root) {
                    tab_stack = vec![tab_root.clone()];
                }
                self.history = tab_stack;
            }
            if !self.history.iter().any(|x| x == &tab_root) {
                self.history.insert(0, tab_root.clone());
            }
        }

        if let Some(pos) = self.history.iter().rposition(|x| x == &p) {
            self.history.truncate(pos + 1);
            self.current = p;
            self.window = self.history.clone();
            self.overlay = None;
            self.save_dev_snapshot();
            return;
        }

        self.history.push(p.clone());
        self.current = p;
        self.window = self.history.clone();
        self.overlay = None;
        self.save_dev_snapshot();
    }

    pub fn dismiss(&mut self) -> bool {
        let global_patterns = registered_sheet_patterns().lock().map(|p| p.clone()).unwrap_or_default();
        // Find the start index of the top-most sheet in history
        let mut last_sheet_idx = None;
        for (i, path) in self.history.iter().enumerate() {
            if i > 0 {
                for pattern in self.sheet_patterns.iter().chain(global_patterns.iter()) {
                    if match_route(pattern, path).is_some() {
                        last_sheet_idx = Some(i);
                        break;
                    }
                }
            }
        }

        if let Some(idx) = last_sheet_idx {
            self.history.truncate(idx);
            if let Some(last) = self.history.last() {
                self.current = last.clone();
            }
            self.window = self.history.clone();
            self.overlay = None;
            self.save_dev_snapshot();
            return true;
        }

        if self.history.len() > 1 {
            self.history.pop();
            if let Some(last) = self.history.last() {
                self.current = last.clone();
            }
            self.window = self.history.clone();
            self.save_dev_snapshot();
            return true;
        }

        false
    }

    pub fn pop(&mut self) -> bool {
        if self.history.len() > 1 {
            self.history.pop();
            if let Some(prev) = self.history.last() {
                self.current = prev.clone();
            }
            self.window = self.history.clone();
            self.overlay = None;
            self.save_dev_snapshot();
            return true;
        }
        false
    }

    /// Pop `n` screens back from history (e.g. pop_n(2) or navigate(-2))
    pub fn pop_n(&mut self, n: usize) -> bool {
        let mut popped = false;
        for _ in 0..n {
            if self.pop() {
                popped = true;
            } else {
                break;
            }
        }
        popped
    }

    /// Pop all the way back to the root screen
    pub fn pop_to_root(&mut self) -> bool {
        let changed = self.history.len() > 1;
        self.history.truncate(1);
        if let Some(root) = self.history.first() {
            self.current = root.clone();
        }
        self.window = self.history.clone();
        self.overlay = None;
        self.save_dev_snapshot();
        changed
    }

    /// Pop back until reaching the target path
    pub fn pop_to(&mut self, target_path: &str) -> bool {
        if let Some(pos) = self.history.iter().rposition(|p| p == target_path) {
            self.history.truncate(pos + 1);
            self.current = target_path.to_string();
            self.window = self.history.clone();
            self.overlay = None;
            self.save_dev_snapshot();
            return true;
        }
        false
    }

    pub fn depth(&self) -> usize {
        self.history.len()
    }

    pub fn is_root(&self) -> bool {
        self.overlay.is_none() && self.window.len() <= 1
    }

    /// Current route path (e.g. "/details")
    pub fn path(&self) -> &str {
        &self.current
    }

    /// Alias for path()
    pub fn location(&self) -> &str {
        &self.current
    }

    /// Array of all paths in the navigation stack (e.g. ["/", "/details"])
    pub fn stack(&self) -> &[String] {
        &self.history
    }
}

pub trait BackTarget {
    fn apply_back(self, router: &mut Router) -> bool;
}

impl BackTarget for usize {
    fn apply_back(self, router: &mut Router) -> bool {
        router.pop_n(self)
    }
}

impl BackTarget for i32 {
    fn apply_back(self, router: &mut Router) -> bool {
        router.pop_n(self.unsigned_abs() as usize)
    }
}

impl BackTarget for &str {
    fn apply_back(self, router: &mut Router) -> bool {
        if self == "/" || self == "root" {
            router.pop_to_root()
        } else {
            router.pop_to(self)
        }
    }
}

impl BackTarget for String {
    fn apply_back(self, router: &mut Router) -> bool {
        BackTarget::apply_back(self.as_str(), router)
    }
}

#[derive(Clone, Debug, Default)]
pub struct RouteParams {
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl RouteParams {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    pub fn query(&self, key: &str) -> Option<&str> {
        self.query.get(key).map(|s| s.as_str())
    }
}

impl std::ops::Index<&str> for RouteParams {
    type Output = String;
    fn index(&self, key: &str) -> &Self::Output {
        self.params.get(key).or_else(|| self.query.get(key)).unwrap_or_else(|| {
            panic!("Route param '{}' not found", key);
        })
    }
}

pub fn match_route(pattern: &str, full_path: &str) -> Option<RouteParams> {
    let (path, query_str) = match full_path.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (full_path, None),
    };

    let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_segments.len() != path_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (pat, act) in pattern_segments.iter().zip(path_segments.iter()) {
        if let Some(param_name) = pat.strip_prefix(':') {
            params.insert(param_name.to_string(), act.to_string());
        } else if pat != act {
            return None;
        }
    }

    let mut query = HashMap::new();
    if let Some(qs) = query_str {
        for pair in qs.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                query.insert(k.to_string(), v.to_string());
            } else if !pair.is_empty() {
                query.insert(pair.to_string(), String::new());
            }
        }
    }

    Some(RouteParams { params, query })
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TabInfo {
    pub title: &'static str,
    pub icon: &'static str,
}

pub struct RouteEntry<'a> {
    pub pattern: &'static str,
    pub presentation: &'static str,
    pub tab_info: Option<TabInfo>,
    pub render: Box<dyn Fn(&RouteParams) -> Node + 'a>,
}

impl<'a> RouteEntry<'a> {
    pub fn tab(mut self, title: &'static str, icon: &'static str) -> Self {
        self.tab_info = Some(TabInfo { title, icon });
        self
    }

    pub fn presentation(mut self, pres: &'static str) -> Self {
        self.presentation = pres;
        self
    }

    pub fn sheet(mut self) -> Self {
        self.presentation = "sheet";
        self
    }

    pub fn form(mut self) -> Self {
        self.presentation = "form";
        self
    }

    pub fn dialog(mut self) -> Self {
        self.presentation = "form";
        self
    }

    pub fn modal(mut self) -> Self {
        self.presentation = "form";
        self
    }

    pub fn fullscreen(mut self) -> Self {
        self.presentation = "fullscreen";
        self
    }

    pub fn popover(mut self) -> Self {
        self.presentation = "popover";
        self
    }
}

#[macro_export]
macro_rules! Route {
    ($pattern:literal, |$params:ident| $render:expr) => {
        $crate::engine::router::RouteEntry {
            pattern: $pattern,
            presentation: "push",
            tab_info: None,
            render: Box::new(|$params: &$crate::engine::router::RouteParams| {
                $crate::engine::builder::IntoNode::into_node($render)
            }),
        }
    };
    ($pattern:literal, $render:expr) => {
        $crate::engine::router::RouteEntry {
            pattern: $pattern,
            presentation: "push",
            tab_info: None,
            render: Box::new(|_params: &$crate::engine::router::RouteParams| {
                $crate::engine::builder::IntoNode::into_node($render)
            }),
        }
    };
}

#[macro_export]
macro_rules! Router {
    ({ $($route:expr),* $(,)? }) => {{
        let router = $crate::engine::router::global_router().lock().unwrap().clone();
        $crate::Router!(__global, router => { $($route),* })
    }};
    ($store:ident => { $($route:expr),* $(,)? }) => {{
        let router = $crate::engine::router::global_router().lock().unwrap().clone();
        $crate::Router!($store, router => { $($route),* })
    }};
    ($router:expr => { $($route:expr),* $(,)? }) => {{
        $crate::Router!(__global, $router => { $($route),* })
    }};
    ($state:ident, $router:expr => { $($route:expr),* $(,)? }) => {{
        let routes = vec![$($route),*];

        for entry in &routes {
            if entry.presentation != "push" {
                $crate::engine::router::register_sheet_pattern(entry.pattern);
            }
        }

        let tabs_meta: Vec<(&'static str, &'static str, &'static str)> = routes
            .iter()
            .filter_map(|r| r.tab_info.as_ref().map(|t| (r.pattern, t.title, t.icon)))
            .collect();

        for (pat, _, _) in &tabs_meta {
            $crate::engine::router::register_tab_root(pat);
        }

        let mut active_tab_idx = 0;
        let mut active_tab_pattern = "/";
        let mut best_match_len = 0;
        let mut matched_tab = false;
        for (idx, (pattern, _, _)) in tabs_meta.iter().enumerate() {
            if *pattern != "/" && ($router.current == *pattern || $router.current.starts_with(&format!("{}/", pattern))) {
                if pattern.len() > best_match_len {
                    best_match_len = pattern.len();
                    active_tab_idx = idx;
                    active_tab_pattern = *pattern;
                    matched_tab = true;
                }
            }
        }
        if !matched_tab {
            if let Some(ref cur_active) = $router.active_tab {
                if let Some(idx) = tabs_meta.iter().position(|(pat, _, _)| pat == cur_active) {
                    active_tab_idx = idx;
                    active_tab_pattern = tabs_meta[idx].0;
                    matched_tab = true;
                }
            }
        }
        if !matched_tab && ($router.current == "/" || $router.current.starts_with("/?")) {
            if let Some(idx) = tabs_meta.iter().position(|(pat, _, _)| *pat == "/") {
                active_tab_idx = idx;
                active_tab_pattern = "/";
            }
        }

        let tab_history: Vec<String> = if !tabs_meta.is_empty() {
            if let Some(pos) = $router.history.iter().rposition(|x| x == active_tab_pattern) {
                $router.history[pos..].to_vec()
            } else if $router.current == active_tab_pattern {
                vec![active_tab_pattern.to_string()]
            } else {
                vec![active_tab_pattern.to_string(), $router.current.clone()]
            }
        } else {
            $router.history.clone()
        };

        let mut layers: Vec<(Vec<$crate::engine::ast::Node>, &'static str, bool)> = Vec::new();
        let mut current_layer_pages = Vec::new();
        let mut current_layer_presentation = "push";
        let mut current_layer_pattern = "";

        for (i, path) in tab_history.iter().enumerate() {
            let mut is_new_layer = false;
            let mut next_presentation = "push";
            let mut next_pattern = "";

            if i > 0 {
                for entry in &routes {
                    if entry.presentation != "push" && $crate::engine::router::match_route(entry.pattern, path).is_some() {
                        is_new_layer = true;
                        next_presentation = entry.presentation;
                        next_pattern = entry.pattern;
                        break;
                    }
                }
            }

            if is_new_layer && !current_layer_pages.is_empty() {
                let has_subroutes = routes.iter().any(|other| {
                    !current_layer_pattern.is_empty()
                        && other.pattern != current_layer_pattern
                        && other.pattern.starts_with(&format!("{}/", current_layer_pattern))
                });
                layers.push((current_layer_pages, current_layer_presentation, has_subroutes));
                current_layer_pages = Vec::new();
                current_layer_presentation = next_presentation;
                current_layer_pattern = next_pattern;
            }

            let mut found_node = None;
            for entry in &routes {
                if let Some(params) = $crate::engine::router::match_route(entry.pattern, path) {
                    let mut node = (entry.render)(&params);
                    if let $crate::engine::ast::Node::Element { ref mut props, .. } = node {
                        if !props.iter().any(|(k, _)| k == "presentation") {
                            props.push(("presentation".to_string(), $crate::engine::ast::PropValue::String(entry.presentation.to_string())));
                        }
                        // Automatically inject on_back for drilled-in pages or form/dialog root pages if not provided
                        let is_form_root = current_layer_pages.is_empty()
                            && matches!(current_layer_presentation, "form" | "dialog" | "modal" | "fullscreen");
                        if (current_layer_pages.len() > 0 || is_form_root) && !props.iter().any(|(k, _)| k == "on_back") {
                            let back_id = if is_form_root {
                                $crate::engine::registry::register_handler(|| {
                                    $crate::engine::router::dismiss();
                                })
                            } else {
                                $crate::engine::registry::register_handler(|| {
                                    $crate::engine::router::navigate(-1);
                                })
                            };
                            props.push(("on_back".to_string(), $crate::engine::ast::PropValue::String(back_id)));
                        }
                    }
                    found_node = Some(node);
                    break;
                }
            }

            if found_node.is_none() {
                if let Some(mut node) = $crate::engine::router::get_ephemeral_view(path) {
                    if let $crate::engine::ast::Node::Element { ref mut props, .. } = node {
                        if !props.iter().any(|(k, _)| k == "presentation") {
                            props.push(("presentation".to_string(), $crate::engine::ast::PropValue::String("push".to_string())));
                        }
                        if current_layer_pages.len() > 0 && !props.iter().any(|(k, _)| k == "on_back") {
                            let back_id = $crate::engine::registry::register_handler(|| {
                                $crate::engine::router::navigate(-1);
                            });
                            props.push(("on_back".to_string(), $crate::engine::ast::PropValue::String(back_id)));
                        }
                    }
                    found_node = Some(node);
                }
            }

            if let Some(node) = found_node {
                current_layer_pages.push(node);
            }
        }
        if !current_layer_pages.is_empty() {
            let has_subroutes = current_layer_pages.len() > 1 || routes.iter().any(|other| {
                !current_layer_pattern.is_empty()
                    && other.pattern != current_layer_pattern
                    && other.pattern.starts_with(&format!("{}/", current_layer_pattern))
            });
            layers.push((current_layer_pages, current_layer_presentation, has_subroutes));
        }

        let mut root_nav_node: Option<($crate::engine::ast::Node, &'static str, bool)> = None;

        while let Some((pages, pres, subroutes)) = layers.pop() {
            let mut children = pages;
            let has_overlay = root_nav_node.is_some();
            let mut child_overlay_kind = String::new();
            let mut child_has_subroutes = false;
            if let Some((child_overlay, child_pres, child_sub)) = root_nav_node {
                children.push(child_overlay);
                child_overlay_kind = child_pres.to_string();
                child_has_subroutes = child_sub;
            }

            let window_count = if has_overlay { children.len().saturating_sub(1) as u32 } else { children.len() as u32 };
            let nav_stack = $crate::engine::primitives::NavigationStack::new()
                .top_style("soft")
                .current_path($router.current.clone())
                .window_count(window_count)
                .has_overlay(has_overlay)
                .has_subroutes(child_has_subroutes)
                .overlay_kind(child_overlay_kind)
                .children(children)
                .on_pop(|| {
                    $crate::engine::router::navigate(-1);
                })
                .on_dismiss(|| {
                    $crate::engine::router::dismiss();
                });

            root_nav_node = Some(($crate::engine::builder::IntoNode::into_node(nav_stack), pres, subroutes));
        }

        let final_node = root_nav_node.map(|(node, _, _)| node).unwrap_or_else(|| {
            $crate::engine::builder::IntoNode::into_node($crate::engine::primitives::NavigationStack::new().top_style("soft"))
        });

        if !tabs_meta.is_empty() {
            let tab_json_list: Vec<serde_json::Value> = tabs_meta.iter().map(|(pat, title, icon)| {
                let pattern = *pat;
                let action_id = $crate::engine::registry::register_handler(move || {
                    $crate::engine::router::switch_tab(pattern);
                });
                let (ios_ic, and_ic, lin_ic, win_ic, web_ic) = $crate::engine::router::resolve_tab_icon_props(icon);
                serde_json::json!({
                    "title": title,
                    "icon": icon,
                    "ios_icon": ios_ic,
                    "android_icon": and_ic,
                    "linux_icon": lin_ic,
                    "windows_icon": win_ic,
                    "web_icon": web_ic,
                    "action": action_id,
                    "pattern": pattern,
                })
            }).collect();

            $crate::engine::ast::Node::Element {
                name: "tabbar".to_string(),
                props: vec![
                    ("active_index".to_string(), $crate::engine::ast::PropValue::Int(active_tab_idx as u32)),
                    ("tabs".to_string(), $crate::engine::ast::PropValue::Json(serde_json::Value::Array(tab_json_list))),
                    ("theme_mode".to_string(), $crate::engine::ast::PropValue::String($crate::engine::runtime::get_theme_mode())),
                    ("accent_mode".to_string(), $crate::engine::ast::PropValue::String($crate::engine::runtime::get_accent_mode())),
                    ("os_accent".to_string(), $crate::engine::ast::PropValue::String($crate::engine::runtime::get_os_accent_hex())),
                ],
                children: vec![final_node],
            }
        } else {
            final_node
        }
    }};
}

#[macro_export]
macro_rules! Navigate {
    // Negative count with mutation and custom state: Navigate!(CustomState, -1, |state| { ... })
    ($state:ident, -$count:literal, |$state_arg:ident| $extra:expr) => {
        move |state: &mut $state| {
            let $state_arg = &mut *state;
            $extra;
            $crate::engine::router::BackTarget::apply_back(-$count, &mut state.router);
        }
    };
    // Custom state negative count: Navigate!(CustomState, -1)
    ($state:ident, -$count:literal) => {
        |state: &mut $state| {
            $crate::engine::router::BackTarget::apply_back(-$count, &mut state.router);
        }
    };
    // Custom state with mutations: Navigate!(CustomState, "/path", |state| { ... })
    ($state:ident, $path:expr, |$state_arg:ident| $extra:expr) => {
        move |state: &mut $state| {
            let $state_arg = &mut *state;
            $extra;
            state.router.navigate($path);
        }
    };
    // Custom state type: Navigate!(CustomState, $path_expr)
    ($state:ident, $path:expr) => {
        |state: &mut $state| {
            state.router.navigate($path);
        }
    };
    // Negative count with mutation: Navigate!(-1, |store| { ... })
    (-$count:literal, |$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::navigate(-$count);
        }
    };
    // Negative count navigation: Navigate!(-1)
    (-$count:literal) => {
        || {
            $crate::engine::router::navigate(-$count);
        }
    };
    // 1. With mutations: Navigate!("/path", |store| { store.cart.checkout(); })
    ($path:expr, |$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::navigate($path);
        }
    };
    // 2. Standard simple navigation: Navigate!("/path")
    ($path:expr) => {
        || {
            $crate::engine::router::navigate($path);
        }
    };
}

#[macro_export]
macro_rules! Pop {
    ($state:ident, $target:literal, |$state_arg:ident| $extra:expr) => {
        move |state: &mut $state| {
            let $state_arg = &mut *state;
            $extra;
            $crate::engine::router::BackTarget::apply_back($target, &mut state.router);
        }
    };
    ($state:ident, -$count:literal, |$state_arg:ident| $extra:expr) => {
        move |state: &mut $state| {
            let $state_arg = &mut *state;
            $extra;
            $crate::engine::router::BackTarget::apply_back(-$count, &mut state.router);
        }
    };
    ($state:ident, -$count:literal) => {
        |state: &mut $state| {
            $crate::engine::router::BackTarget::apply_back(-$count, &mut state.router);
        }
    };
    ($state:ident, $target:literal) => {
        |state: &mut $state| {
            $crate::engine::router::BackTarget::apply_back($target, &mut state.router);
        }
    };
    ($state:ident, $target:expr) => {
        $crate::engine::router::BackTarget::apply_back($target, &mut $state.router)
    };
    ($state:ident) => {
        |state: &mut $state| {
            state.router.pop();
        }
    };
    (|$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::navigate(-1);
        }
    };
    ($target:literal, |$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::navigate($target);
        }
    };
    (-$count:literal, |$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::navigate(-$count);
        }
    };
    (-$count:literal) => {
        || {
            $crate::engine::router::navigate(-$count);
        }
    };
    ($target:literal) => {
        || {
            $crate::engine::router::navigate($target);
        }
    };
    () => {
        || {
            $crate::engine::router::navigate(-1);
        }
    };
}

/// Intuitive React Native / Web style alias for Pop!()
#[macro_export]
macro_rules! Back {
    ($( $args:tt )*) => {
        $crate::Pop!($( $args )*)
    };
}

/// Intuitive modal sheet / overlay dismissal macro
#[macro_export]
macro_rules! Dismiss {
    ($state:ident) => {
        |state: &mut $state| {
            state.router.dismiss();
        }
    };
    (|$state_arg:ident| $extra:expr) => {
        move |$state_arg: &mut _| {
            $extra;
            $crate::engine::router::dismiss();
        }
    };
    () => {
        || {
            $crate::engine::router::dismiss();
        }
    };
}

/// Chain multiple actions into a single handler
#[macro_export]
macro_rules! Actions {
    ( $( $action:expr ),* $(,)? ) => {
        move |state: &mut _| {
            $(
                ($action)(state);
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Text;

    #[test]
    fn test_router_navigation_and_pop() {
        let mut router = Router::default();
        assert_eq!(router.current, "/");
        assert_eq!(router.depth(), 1);
        assert!(router.is_root());

        router.navigate("/details");
        assert_eq!(router.current, "/details");
        assert_eq!(router.depth(), 2);
        assert!(!router.is_root());

        router.navigate("/profile/42");
        assert_eq!(router.current, "/profile/42");
        assert_eq!(router.depth(), 3);

        assert!(router.pop());
        assert_eq!(router.current, "/details");
        assert_eq!(router.depth(), 2);

        assert!(router.pop());
        assert_eq!(router.current, "/");
        assert_eq!(router.depth(), 1);

        // Cannot pop root
        assert!(!router.pop());
        assert_eq!(router.current, "/");
        assert_eq!(router.depth(), 1);
    }

    #[test]
    fn test_router_sheet_and_dismiss() {
        let mut router = Router::default();
        router.sheet_patterns.push("/purchase".to_string());
        router.sheet_patterns.push("/sheet2".to_string());
        assert_eq!(router.current, "/");

        router.navigate("/purchase");
        assert_eq!(router.current, "/purchase");
        assert_eq!(router.history, vec!["/", "/purchase"]);

        // Push inside sheet
        router.navigate("/purchase/shipping");
        assert_eq!(router.current, "/purchase/shipping");
        assert_eq!(router.history, vec!["/", "/purchase", "/purchase/shipping"]);

        // Push sheet 2 on top of sheet 1
        router.navigate("/sheet2");
        assert_eq!(router.current, "/sheet2");
        assert_eq!(router.history, vec!["/", "/purchase", "/purchase/shipping", "/sheet2"]);

        // Drill in to sheet 2 detail
        router.navigate("/sheet2/detail");
        assert_eq!(router.current, "/sheet2/detail");

        // Pop inside sheet 2
        assert!(router.pop());
        assert_eq!(router.current, "/sheet2");

        // Dismiss sheet 2 (pops all of sheet 2 back to sheet 1 detail)
        assert!(router.dismiss());
        assert_eq!(router.current, "/purchase/shipping");

        // Dismiss sheet 1 (pops all of sheet 1 back to root /)
        assert!(router.dismiss());
        assert_eq!(router.current, "/");
        assert_eq!(router.history, vec!["/"]);
    }

    #[test]
    fn test_arbitrary_nested_sheets() {
        let mut router = Router::default();
        for i in 1..=10 {
            router.sheet_patterns.push(format!("/sheet{}", i));
        }

        // Open 10 sheets on top of each other
        for i in 1..=10 {
            router.navigate(format!("/sheet{}", i));
            assert_eq!(router.current, format!("/sheet{}", i));
        }
        assert_eq!(router.history.len(), 11);

        // Dismiss one by one from 10 down to 1
        for i in (1..=10).rev() {
            assert!(router.dismiss());
            let expected = if i > 1 { format!("/sheet{}", i - 1) } else { "/".to_string() };
            assert_eq!(router.current, expected);
        }
        assert_eq!(router.current, "/");
        assert_eq!(router.history, vec!["/"]);
    }

    #[test]
    fn test_match_route_static() {
        assert!(match_route("/", "/").is_some());
        assert!(match_route("/details", "/details").is_some());
        assert!(match_route("/details", "/profile").is_none());
        assert!(match_route("/details", "/details/more").is_none());
    }

    #[test]
    fn test_match_route_dynamic_params() {
        let matched = match_route("/profile/:id", "/profile/123");
        assert!(matched.is_some());
        let params = matched.unwrap();
        assert_eq!(params.get("id"), Some("123"));
        assert_eq!(&params["id"], "123");

        let multi = match_route("/org/:org_id/repo/:repo_id", "/org/rust/repo/nativekit");
        assert!(multi.is_some());
        let p = multi.unwrap();
        assert_eq!(p.get("org_id"), Some("rust"));
        assert_eq!(p.get("repo_id"), Some("nativekit"));
    }

    #[test]
    fn test_match_route_query_params() {
        let matched = match_route("/search", "/search?q=rust&tab=code");
        assert!(matched.is_some());
        let p = matched.unwrap();
        assert_eq!(p.query("q"), Some("rust"));
        assert_eq!(p.query("tab"), Some("code"));
    }

    #[test]
    fn test_router_macro_evaluation() {
        struct TestState {
            pub router: Router,
            pub title: String,
        }

        let mut state = TestState {
            router: Router::default(),
            title: "Home".to_string(),
        };

        // Render at root "/"
        let node = Router!(TestState, state.router => {
            Route!("/", Text!("{}", &state.title)),
            Route!("/details", Text!("Details")),
            Route!("/user/:name", |params| Text!("{}", params["name"])),
        });

        let n = crate::engine::builder::IntoNode::into_node(node);
        if let Node::Element { name, props, children } = n {
            assert_eq!(name, "navigationstack");
            assert_eq!(children.len(), 1);
            let current_prop = props.iter().find(|(k, _)| k == "current_path");
            assert!(current_prop.is_some());
            let win_prop = props.iter().find(|(k, _)| k == "window_count");
            assert!(win_prop.is_some());
        } else {
            panic!("Expected Element");
        }

        // Navigate to /user/alice
        let nav_action = Navigate!(TestState, "/user/alice");
        nav_action(&mut state);
        assert_eq!(state.router.current, "/user/alice");
        assert_eq!(state.router.depth(), 2);

        // Render at "/user/alice"
        let node2 = Router!(TestState, state.router => {
            Route!("/", Text!("{}", &state.title)),
            Route!("/details", Text!("Details")),
            Route!("/user/:name", |params| Text!("{}", params["name"])),
        });

        let n2 = crate::engine::builder::IntoNode::into_node(node2);
        if let Node::Element { name, children, props } = n2 {
            assert_eq!(name, "navigationstack");
            assert_eq!(children.len(), 2); // Root + /user/alice
            let win_prop = props.iter().find(|(k, _)| k == "window_count");
            assert_eq!(win_prop, Some(&("window_count".to_string(), crate::engine::ast::PropValue::Int(2))));
        } else {
            panic!("Expected Element");
        }

        // Pop back
        let pop_action = Pop!(TestState);
        pop_action(&mut state);
        assert_eq!(state.router.current, "/");
        assert_eq!(state.router.depth(), 1);
    }

    #[test]
    fn test_router_nested_form_and_sheet_flow() {
        struct AppState {
            pub router: Router,
        }
        let mut state = AppState {
            router: Router::default(),
        };

        macro_rules! render_demo {
            ($state:expr) => {{
                let node = Router!(AppState, $state.router => {
                    Route!("/", Text!("Home")),
                    Route!("/form1", Text!("Form 1")).form(),
                    Route!("/form1/sheet", Text!("Sheet on Form 1")).sheet(),
                    Route!("/form1/detail", Text!("Form 1 Detail")),
                    Route!("/form2", Text!("Form 2")).form(),
                    Route!("/form2/detail", Text!("Form 2 Detail")),
                    Route!("/form2/sheet", Text!("Sheet on Form 2")).sheet(),
                });
                $crate::engine::builder::IntoNode::into_node(node)
            }};
        }

        // 1. Main page
        assert_eq!(state.router.current, "/");
        if let Node::Element { name, props, children } = render_demo!(state) {
            assert_eq!(name, "navigationstack");
            assert_eq!(children.len(), 1);
            assert_eq!(props.iter().find(|(k, _)| k == "has_overlay"), Some(&("has_overlay".to_string(), crate::engine::ast::PropValue::Bool(false))));
        } else { panic!("Expected Element"); }

        // 2. Open a form
        state.router.navigate("/form1");
        assert_eq!(state.router.current, "/form1");
        if let Node::Element { props, children, .. } = render_demo!(state) {
            assert_eq!(props.iter().find(|(k, _)| k == "has_overlay"), Some(&("has_overlay".to_string(), crate::engine::ast::PropValue::Bool(true))));
            assert_eq!(props.iter().find(|(k, _)| k == "overlay_kind"), Some(&("overlay_kind".to_string(), crate::engine::ast::PropValue::String("form".to_string()))));
            // Child overlay is Form 1 navigationstack
            let overlay = children.last().unwrap();
            if let Node::Element { name: c_name, props: c_props, .. } = overlay {
                assert_eq!(c_name, "navigationstack");
                assert_eq!(c_props.iter().find(|(k, _)| k == "has_overlay"), Some(&("has_overlay".to_string(), crate::engine::ast::PropValue::Bool(false))));
            } else { panic!("Expected overlay element"); }
        } else { panic!("Expected Element"); }

        // 3. Open a sheet (on top of Form 1)
        state.router.navigate("/form1/sheet");
        assert_eq!(state.router.current, "/form1/sheet");
        if let Node::Element { props, children, .. } = render_demo!(state) {
            // Root has overlay Form 1
            assert_eq!(props.iter().find(|(k, _)| k == "overlay_kind"), Some(&("overlay_kind".to_string(), crate::engine::ast::PropValue::String("form".to_string()))));
            // Form 1 has overlay Sheet on Form 1
            let form1_overlay = children.last().unwrap();
            if let Node::Element { props: f_props, children: f_children, .. } = form1_overlay {
                assert_eq!(f_props.iter().find(|(k, _)| k == "has_overlay"), Some(&("has_overlay".to_string(), crate::engine::ast::PropValue::Bool(true))));
                assert_eq!(f_props.iter().find(|(k, _)| k == "overlay_kind"), Some(&("overlay_kind".to_string(), crate::engine::ast::PropValue::String("sheet".to_string()))));
                let sheet_overlay = f_children.last().unwrap();
                if let Node::Element { props: s_props, .. } = sheet_overlay {
                    assert_eq!(s_props.iter().find(|(k, _)| k == "has_overlay"), Some(&("has_overlay".to_string(), crate::engine::ast::PropValue::Bool(false))));
                } else { panic!("Expected sheet overlay element"); }
            } else { panic!("Expected form1 overlay element"); }
        } else { panic!("Expected Element"); }

        // 4. Close the sheet
        assert!(state.router.dismiss());
        assert_eq!(state.router.current, "/form1");

        // 5. Drill in
        state.router.navigate("/form1/detail");
        assert_eq!(state.router.current, "/form1/detail");
        if let Node::Element { children, .. } = render_demo!(state) {
            let form1_overlay = children.last().unwrap();
            if let Node::Element { props: f_props, .. } = form1_overlay {
                // Form 1 now has window_count = 2 (Form 1 + Detail)
                assert_eq!(f_props.iter().find(|(k, _)| k == "window_count"), Some(&("window_count".to_string(), crate::engine::ast::PropValue::Int(2))));
            } else { panic!("Expected form1 overlay element"); }
        } else { panic!("Expected Element"); }

        // 6. Drill out
        assert!(state.router.pop());
        assert_eq!(state.router.current, "/form1");

        // 7. Open another form (on top of existing one)
        state.router.navigate("/form2");
        assert_eq!(state.router.current, "/form2");

        // 8. Drill in to Form 2 detail
        state.router.navigate("/form2/detail");
        assert_eq!(state.router.current, "/form2/detail");

        // 9. Open sheet on top of Form 2
        state.router.navigate("/form2/sheet");
        assert_eq!(state.router.current, "/form2/sheet");

        // Dismiss sheet on Form 2
        assert!(state.router.dismiss());
        assert_eq!(state.router.current, "/form2/detail");

        // Dismiss Form 2
        assert!(state.router.dismiss());
        assert_eq!(state.router.current, "/form1");

        // Dismiss Form 1
        assert!(state.router.dismiss());
        assert_eq!(state.router.current, "/");
    }

    #[test]
    fn test_router_multi_step_pop_and_targets() {
        let mut router = Router::default();
        router.navigate("/settings");
        router.navigate("/settings/profile");
        router.navigate("/settings/profile/address");
        assert_eq!(router.depth(), 4);
        assert_eq!(router.current, "/settings/profile/address");

        // Pop 2 screens (e.g. from address back to settings, skipping profile)
        assert!(router.pop_n(2));
        assert_eq!(router.current, "/settings");
        assert_eq!(router.depth(), 2);

        // Navigate deep again
        router.navigate("/settings/security");
        router.navigate("/settings/security/2fa");
        assert_eq!(router.depth(), 4);

        // Pop to specific target
        assert!(router.pop_to("/settings"));
        assert_eq!(router.current, "/settings");
        assert_eq!(router.depth(), 2);

        // Pop to root
        assert!(router.pop_to_root());
        assert_eq!(router.current, "/");
        assert_eq!(router.depth(), 1);
        assert!(router.is_root());
    }

    #[test]
    fn test_back_target_macro_helpers() {
        let mut router = Router::default();
        router.navigate("/a");
        router.navigate("/b");
        router.navigate("/c");
        assert_eq!(router.depth(), 4);

        struct DummyState {
            pub router: Router,
        }
        let mut s = DummyState { router };

        // Action closure with negative count (React-style navigate(-2))
        let back_two = Back!(DummyState, -2);
        back_two(&mut s);
        assert_eq!(s.router.current, "/a");
        assert_eq!(s.router.depth(), 2);

        s.router.navigate("/x");
        s.router.navigate("/y");
        let back_to_root = Back!(DummyState, "/");
        back_to_root(&mut s);
        assert_eq!(s.router.path(), "/");
        assert_eq!(s.router.depth(), 1);
        assert_eq!(s.router.stack(), &["/"]);

        // Test Navigate!(-1)
        s.router.navigate("/page1");
        s.router.navigate("/page2");
        assert_eq!(s.router.path(), "/page2");
        assert_eq!(s.router.stack(), &["/", "/page1", "/page2"]);

        let nav_back = Navigate!(DummyState, -1);
        nav_back(&mut s);
        assert_eq!(s.router.path(), "/page1");
        assert_eq!(s.router.location(), "/page1");
        assert_eq!(s.router.depth(), 2);
    }

    #[test]
    fn test_global_router_and_action_handlers() {
        {
            let mut r = global_router().lock().unwrap();
            *r = Router::new("/");
        }

        assert_eq!(global_router().lock().unwrap().current, "/");

        // Test navigate string
        navigate("/details");
        assert_eq!(global_router().lock().unwrap().current, "/details");

        // Test navigate negative count
        navigate(-1);
        assert_eq!(global_router().lock().unwrap().current, "/");

        // Test sheet and dismiss
        register_sheet_pattern("/purchase");
        navigate("/purchase");
        assert_eq!(global_router().lock().unwrap().current, "/purchase");

        dismiss();
        assert_eq!(global_router().lock().unwrap().current, "/");

        // Test navigate("/") from deep window pops back to root instead of drilling in
        navigate("/purchase/confirmed");
        assert_eq!(global_router().lock().unwrap().current, "/purchase/confirmed");
        assert_eq!(global_router().lock().unwrap().window.len(), 2);
        navigate("/");
        assert_eq!(global_router().lock().unwrap().current, "/");
        assert_eq!(global_router().lock().unwrap().window.len(), 1);

        // Test button action handler types:
        use crate::engine::primitives::Button;

        // 1. 0-arg closure
        let b1 = Button::new().action(|| {
            navigate("/home");
        });
        assert_eq!(b1.props.len(), 1);

        // 2. 1-arg state closure
        struct MyStore {
            pub count: i32,
        }
        let b2 = Button::new().action(|store: &mut MyStore| {
            store.count += 1;
            navigate("/incremented");
        });
        assert_eq!(b2.props.len(), 1);

        // 3. Standalone function pointer fn()
        fn my_custom_handler() {
            navigate(-1);
        }
        let b3 = Button::new().action(my_custom_handler);
        assert_eq!(b3.props.len(), 1);

        // 4. Directly passing dismiss
        let b4 = Button::new().action(dismiss);
        assert_eq!(b4.props.len(), 1);
    }

    #[test]
    fn test_inline_custom_view_navigation() {
        {
            let mut r = global_router().lock().unwrap();
            *r = Router::new("/");
        }

        struct AppState {
            pub router: Router,
        }
        let mut state = AppState {
            router: Router::default(),
        };

        // Render at root
        let node1 = Router!(AppState, state.router => {
            Route!("/", crate::Text!("Home")),
            Route!("/details", crate::Text!("Details")),
        });
        let n1 = crate::engine::builder::IntoNode::into_node(node1);
        if let Node::Element { name, children, .. } = n1 {
            assert_eq!(name, "navigationstack");
            assert_eq!(children.len(), 1);
        } else { panic!("Expected Element"); }

        // Navigate to an inline custom view
        let custom_page = crate::Page! {
            crate::VStack! {
                crate::Text!("Custom Inline View")
            }
        }.title("Custom");
        state.router.navigate(register_ephemeral_view(crate::engine::builder::IntoNode::into_node(custom_page)));
        assert_eq!(state.router.depth(), 2);
        assert!(state.router.current.starts_with("__ephemeral_"));

        // Render router again - ephemeral view is rendered inside the navigationstack!
        let node2 = Router!(AppState, state.router => {
            Route!("/", crate::Text!("Home")),
            Route!("/details", crate::Text!("Details")),
        });
        let n2 = crate::engine::builder::IntoNode::into_node(node2);
        if let Node::Element { name, children, .. } = n2 {
            assert_eq!(name, "navigationstack");
            assert_eq!(children.len(), 2);
            // Check top child is our custom page
            if let Node::Element { props, .. } = &children[1] {
                assert_eq!(props.iter().find(|(k, _)| k == "title"), Some(&("title".to_string(), crate::engine::ast::PropValue::String("Custom".to_string()))));
                // Automatically has on_back
                assert!(props.iter().any(|(k, _)| k == "on_back"));
            } else { panic!("Expected Element child"); }
        } else { panic!("Expected Element"); }

        // Pop back
        assert!(state.router.pop());
        assert_eq!(state.router.current, "/");
        assert_eq!(state.router.depth(), 1);
    }

    #[test]
    fn test_tab_bar_and_tab_jumping() {
        struct AppState {
            pub router: Router,
        }
        let mut state = AppState {
            router: Router::default(),
        };

        macro_rules! render_tabs {
            ($state:expr) => {{
                let node = Router!(AppState, $state.router => {
                    Route!("/", crate::Text!("Home Screen")).tab("Home", "house"),
                    Route!("/orders", crate::Text!("Orders List")).tab("Orders", "bag"),
                    Route!("/orders/:id", |params| crate::Text!("Order #{}", params["id"])),
                    Route!("/settings", crate::Text!("Settings Screen")).tab("Settings", "gear"),
                });
                crate::engine::builder::IntoNode::into_node(node)
            }};
        }

        // 1. Root / -> Tab 0 active
        let n1 = render_tabs!(state);
        if let Node::Element { name, props, children } = n1 {
            assert_eq!(name, "tabbar");
            assert_eq!(props.iter().find(|(k, _)| k == "active_index"), Some(&("active_index".to_string(), crate::engine::ast::PropValue::Int(0))));
            // Inside tabbar is the navigationstack
            assert_eq!(children.len(), 1);
        } else { panic!("Expected tabbar Element"); }

        // 2. Tab jumping via direct navigation / deep link to "/orders"
        state.router.navigate("/orders");
        assert_eq!(state.router.current, "/orders");
        let n2 = render_tabs!(state);
        if let Node::Element { name, props, children } = n2 {
            assert_eq!(name, "tabbar");
            // Tab 1 (Orders) is now active!
            assert_eq!(props.iter().find(|(k, _)| k == "active_index"), Some(&("active_index".to_string(), crate::engine::ast::PropValue::Int(1))));
            if let Node::Element { name: nav_name, children: nav_children, .. } = &children[0] {
                assert_eq!(nav_name, "navigationstack");
                // Only 1 child: the /orders root! No back button!
                assert_eq!(nav_children.len(), 1);
                if let Node::Element { props: page_props, .. } = &nav_children[0] {
                    assert!(!page_props.iter().any(|(k, _)| k == "on_back"));
                }
            }
        } else { panic!("Expected tabbar Element"); }

        // 3. Tab jumping via deep link to nested route "/orders/42"
        state.router.navigate("/orders/42");
        assert_eq!(state.router.current, "/orders/42");
        let n3 = render_tabs!(state);
        if let Node::Element { name, props, children } = n3 {
            assert_eq!(name, "tabbar");
            // Tab 1 (Orders) is still active because /orders/42 matches /orders prefix!
            assert_eq!(props.iter().find(|(k, _)| k == "active_index"), Some(&("active_index".to_string(), crate::engine::ast::PropValue::Int(1))));
            // Inner navigationstack has the drilled-in Order #42 page on top of /orders root
            if let Node::Element { name: nav_name, children: nav_children, .. } = &children[0] {
                assert_eq!(nav_name, "navigationstack");
                assert_eq!(nav_children.len(), 2); // /orders root, /orders/42
                // Drilled-in page HAS on_back
                if let Node::Element { props: page_props, .. } = &nav_children[1] {
                    assert!(page_props.iter().any(|(k, _)| k == "on_back"));
                }
            }
        } else { panic!("Expected tabbar Element"); }

        // 4. Tab jump to "/settings"
        state.router.navigate("/settings");
        assert_eq!(state.router.current, "/settings");
        let n4 = render_tabs!(state);
        if let Node::Element { name, props, children } = n4 {
            assert_eq!(name, "tabbar");
            // Tab 2 (Settings) is active!
            assert_eq!(props.iter().find(|(k, _)| k == "active_index"), Some(&("active_index".to_string(), crate::engine::ast::PropValue::Int(2))));
            if let Node::Element { name: nav_name, children: nav_children, .. } = &children[0] {
                assert_eq!(nav_name, "navigationstack");
                // Only 1 child: /settings root! No back button!
                assert_eq!(nav_children.len(), 1);
                if let Node::Element { props: page_props, .. } = &nav_children[0] {
                    assert!(!page_props.iter().any(|(k, _)| k == "on_back"));
                }
            }
        } else { panic!("Expected tabbar Element"); }
    }

    #[test]
    fn test_drill_in_picker() {
        use crate::engine::components::picker::Picker;
        let picker = Picker::new(["Small", "Medium", "Large"])
            .title("Size")
            .selected("Medium")
            .style("drill-in");

        let node = crate::engine::builder::IntoNode::into_node(picker);
        if let Node::Element { name, children, .. } = node {
            assert_eq!(name, "button");
            assert_eq!(children.len(), 1);
        } else { panic!("Expected button element for Picker row"); }
    }
}

