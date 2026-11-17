use serde_json::json;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use crate::engine::core::App;

static RELOAD_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
static RELOAD_CONDVAR: (Mutex<u64>, std::sync::Condvar) = (Mutex::new(0), std::sync::Condvar::new());
static DEV_APP_STATE: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
static DEV_LAST_NODE: std::sync::Mutex<Option<crate::engine::ast::Node>> = std::sync::Mutex::new(None);

pub trait IntoRouteResponse {
    fn into_response(self) -> String;
}

pub use inventory;

impl IntoRouteResponse for String {
    fn into_response(self) -> String { self }
}

impl IntoRouteResponse for Result<crate::engine::Node, String> {
    fn into_response(self) -> String {
        match self {
            Ok(node) => crate::engine::binary::node_to_base64(&node),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }
}

pub struct ServerRoute {
    pub path: &'static str,
    pub handler: fn() -> String,
}

inventory::collect!(ServerRoute);

pub trait DevHostBridge: Send + Sync + 'static {
    fn get_ui_b64(&self) -> String;
    fn dispatch_event(&self, action: &str, payload: Option<serde_json::Value>) -> String;
    fn get_ui_json(&self) -> String;
    fn navigate(&self, path: &str);
    fn handle_route(&self, url: &str) -> Option<String>;
}

fn read_web_shell_file(rel: &str) -> Result<String, std::io::Error> {
    let p1 = std::path::PathBuf::from(format!("platforms/Web/{}", rel));
    if p1.exists() {
        return std::fs::read_to_string(p1);
    }
    std::fs::read_to_string(format!("demo_app/platforms/Web/{}", rel))
}

fn read_web_shell_bytes(rel: &str) -> Result<Vec<u8>, std::io::Error> {
    let p1 = std::path::PathBuf::from(format!("platforms/Web/{}", rel));
    if p1.exists() {
        return std::fs::read(p1);
    }
    std::fs::read(format!("demo_app/platforms/Web/{}", rel))
}

pub fn start_host_server(bridge: std::sync::Arc<dyn DevHostBridge>) {
    kill_stale_port();
    notify_reload();

    std::thread::spawn(move || {
        let mut server = None;
        let port = std::env::var("PORT").unwrap_or_else(|_| "5824".to_string());
        let bind_addr = format!("0.0.0.0:{}", port);
        for _ in 0..10 {
            match tiny_http::Server::http(&bind_addr) {
                Ok(s) => {
                    server = Some(s);
                    break;
                }
                Err(_) => {
                    kill_stale_port();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        }
        let Some(server) = server else {
            return;
        };

        for mut request in server.incoming_requests() {
            let b = bridge.clone();
            std::thread::spawn(move || {
                let raw_url = request.url().to_string();
                let url = raw_url.split('?').next().unwrap_or(&raw_url).to_string();

                if request.method().as_str() == "OPTIONS" {
                    let response = tiny_http::Response::empty(204)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap());
                    let _ = request.respond(response);
                    return;
                }

                if url == "/api/live_reload" {
                    let wait_for = raw_url
                        .split_once('?')
                        .and_then(|(_, q)| q.split('&').find_map(|p| p.strip_prefix("wait_for=")))
                        .and_then(|v| v.parse::<u64>().ok());
                    if let Some(client_ts) = wait_for {
                        if get_reload_timestamp() == client_ts {
                            if let Ok(guard) = RELOAD_CONDVAR.0.lock() {
                                let _ = RELOAD_CONDVAR.1.wait_timeout_while(
                                    guard,
                                    std::time::Duration::from_secs(15),
                                    |_| get_reload_timestamp() == client_ts,
                                );
                            }
                        }
                    }
                    let ts = get_reload_timestamp();
                    let data = json!({ "timestamp": ts }).to_string();
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    let _ = request.respond(response);
                } else if url == "/api/ui_layout" {
                    let data = b.get_ui_b64();
                    let ts_str = get_reload_timestamp().to_string();
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Expose-Headers"[..], &b"X-Reload-Timestamp"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"X-Reload-Timestamp"[..], ts_str.as_bytes()).unwrap());
                    let _ = request.respond(response);
                } else if url == "/api/dispatch" {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);
                    let json_val: Option<serde_json::Value> = serde_json::from_str(&content).ok();
                    let action = json_val
                        .as_ref()
                        .and_then(|j| j.get("action"))
                        .and_then(|a| a.as_str())
                        .unwrap_or("")
                        .to_string();
                    let payload = json_val.and_then(|j| j.get("payload").cloned());

                    let data = b.dispatch_event(&action, payload);
                    let ts_str = get_reload_timestamp().to_string();
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Expose-Headers"[..], &b"X-Reload-Timestamp"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"X-Reload-Timestamp"[..], ts_str.as_bytes()).unwrap());
                    let _ = request.respond(response);
                } else if url == "/ui" || url == "/api/ui" {
                    let data = b.get_ui_json();
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    let _ = request.respond(response);
                } else if url == "/action" {
                    let mut content = String::new();
                    let _ = request.as_reader().read_to_string(&mut content);
                    let json_val: Option<serde_json::Value> = serde_json::from_str(&content).ok();
                    let action = json_val
                        .as_ref()
                        .and_then(|j| j.get("action"))
                        .and_then(|a| a.as_str())
                        .unwrap_or("")
                        .to_string();
                    let payload = json_val.and_then(|j| j.get("payload").cloned());
                    let _ = b.dispatch_event(&action, payload);
                    let data = b.get_ui_json();
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    let _ = request.respond(response);
                } else if let Some(data) = b.handle_route(&url) {
                    let response = tiny_http::Response::from_string(data)
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    let _ = request.respond(response);
                } else if url == "/theme.css" {
                    if let Ok(css) = read_web_shell_file("theme.css") {
                        let response = tiny_http::Response::from_string(css)
                            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/css"[..]).unwrap());
                        let _ = request.respond(response);
                    } else {
                        let _ = request.respond(tiny_http::Response::empty(404));
                    }
                } else if url == "/wasm_app.js" {
                    if let Ok(js) = read_web_shell_file("wasm_app.js") {
                        let response = tiny_http::Response::from_string(js)
                            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/javascript"[..]).unwrap());
                        let _ = request.respond(response);
                    } else {
                        let _ = request.respond(tiny_http::Response::empty(404));
                    }
                } else if url == "/wasm_app_bg.wasm" {
                    if let Ok(wasm) = read_web_shell_bytes("wasm_app_bg.wasm") {
                        let response = tiny_http::Response::from_data(wasm)
                            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/wasm"[..]).unwrap());
                        let _ = request.respond(response);
                    } else {
                        let _ = request.respond(tiny_http::Response::empty(404));
                    }
                } else if url == "/health" {
                    let response = tiny_http::Response::from_string("{\"status\":\"ok\"}")
                        .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                    let _ = request.respond(response);
                } else if !url.starts_with("/api/") {
                    if let Ok(html) = read_web_shell_file("index.html") {
                        let response = tiny_http::Response::from_string(html)
                            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                        let _ = request.respond(response);
                    } else {
                        let _ = request.respond(tiny_http::Response::empty(404));
                    }
                } else {
                    let _ = request.respond(tiny_http::Response::empty(404));
                }
            });
        }
    });
}

pub fn get_dev_state<T: App>() -> &'static Mutex<T::State> {
    let boxed_any = DEV_APP_STATE.get_or_init(|| {
        crate::engine::runtime::set_dispatcher(Box::new(|update_fn| {
            if let Some(boxed) = DEV_APP_STATE.get() {
                if let Some(state_mutex) = boxed.downcast_ref::<Mutex<T::State>>() {
                    let mut state = state_mutex.lock().unwrap();
                    update_fn(&mut *state);
                    notify_reload();
                }
            }
        }));
        
        crate::engine::runtime::set_refresher(Box::new(|| {
            notify_reload();
        }));
        
        let state = T::init();
        Box::new(Mutex::new(state)) as Box<dyn std::any::Any + Send + Sync>
    });
    boxed_any.downcast_ref::<Mutex<T::State>>().unwrap()
}

pub fn get_ui_b64<T: App>(is_diff: bool) -> String {
    let state = get_dev_state::<T>().lock().unwrap();
    crate::engine::registry::reset_render_action_seq();
    let _scope = crate::engine::runtime::set_scoped_state(&*state);
    let node = crate::engine::builder::IntoNode::into_node(T::view(&state));
    
    let mut last_node_lock = DEV_LAST_NODE.lock().unwrap();
    
    if is_diff {
        if let Some(old_node) = last_node_lock.as_ref() {
            let patches = crate::engine::diff::diff(old_node, &node);
            *last_node_lock = Some(node);
            return crate::engine::binary::patches_to_base64(&patches);
        }
    }
    
    *last_node_lock = Some(node.clone());
    crate::engine::binary::node_to_base64(&node)
}

pub fn dispatch_event<T: App>(action: &str, payload: Option<serde_json::Value>) -> String {
    let mut handled = false;
    
    let clean_action = action.trim_matches('"');
    
    println!("Received dispatch: {}", clean_action);
    if clean_action == "navigate" {
        let path = payload.as_ref()
            .and_then(|p| p.get("path").and_then(|s| s.as_str()).or_else(|| p.as_str()))
            .unwrap_or("/");
        crate::engine::router::navigate(path);
        handled = true;
    } else if clean_action == "pop" {
        crate::engine::router::navigate(-1);
        handled = true;
    } else if clean_action == "dismiss" {
        crate::engine::router::dismiss();
        handled = true;
    } else if clean_action.starts_with("action_") {
        let mut state = get_dev_state::<T>().lock().unwrap();
        handled = crate::engine::registry::execute_action(clean_action, &mut *state, payload);
    }
    
    let prev_node = DEV_LAST_NODE.lock().unwrap().clone();
    let b64 = get_ui_b64::<T>(false);
    let cur_node = DEV_LAST_NODE.lock().unwrap().clone();
    if handled && prev_node != cur_node {
        notify_reload();
    }
    b64
}

pub fn notify_reload() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        RELOAD_TIMESTAMP.store(now, Ordering::SeqCst);
        if let Ok(mut g) = RELOAD_CONDVAR.0.lock() { *g = now; RELOAD_CONDVAR.1.notify_all(); }
    }
    #[cfg(target_arch = "wasm32")]
    {
        RELOAD_TIMESTAMP.fetch_add(1, Ordering::SeqCst);
    }
}

pub fn get_reload_timestamp() -> u64 {
    RELOAD_TIMESTAMP.load(Ordering::SeqCst)
}

pub fn kill_stale_port() {
    let output = Command::new("lsof").args(["-t", "-i:5824", "-sTCP:LISTEN", "-n", "-P"]).output();
    if let Ok(out) = output {
        let pid_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let my_pid = std::process::id();
        let mut killed = false;
        for pid in pid_str.lines() {
            if let Ok(p) = pid.parse::<u32>() {
                if p != my_pid {
                    let _ = Command::new("kill").args(["-9", &p.to_string()]).status();
                    killed = true;
                }
            }
        }
        if killed {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}




pub fn start<T: App>() {
    start_with_routes::<T, _>(|_url| None)
}

pub fn start_with_routes<T: App, F>(router: F)
where
    F: Fn(&str) -> Option<String> + Send + Sync + 'static,
{
    kill_stale_port();
    crate::engine::router::restore_dev_router();
    notify_reload();
    
    // Initialize state and trigger any initial side effects
    let _ = get_dev_state::<T>();

    std::thread::spawn(move || {
        let mut server = None;
        let port = std::env::var("PORT").unwrap_or_else(|_| "5824".to_string());
        let host = "0.0.0.0"; // Always bind to 0.0.0.0 to support cloud deployment
        let bind_addr = format!("{}:{}", host, port);
        for _ in 0..10 {
            match tiny_http::Server::http(&bind_addr) {
                Ok(s) => {
                    server = Some(s);
                    break;
                }
                Err(_) => {
                    kill_stale_port();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        }
        let server = match server {
            Some(s) => {
                println!("🚀 Server listening on http://{}", bind_addr);
                s
            },
            None => {
                eprintln!("❌ [Server] Failed to bind to http://{} after retries.", bind_addr);
                return;
            }
        };

        let router = std::sync::Arc::new(router);
        for request in server.incoming_requests() {
            let r = std::sync::Arc::clone(&router);
            std::thread::spawn(move || {
                handle_client_request::<T, _>(request, move |url| r(url));
            });
        }
    });
}

fn handle_client_request<T: App, F>(
    mut request: tiny_http::Request,
    router: F,
) where
    F: Fn(&str) -> Option<String>,
{
    let raw_url = request.url().to_string();
    let url = raw_url.split('?').next().unwrap_or(&raw_url).to_string();

    if request.method().as_str() == "OPTIONS" {
        let response = tiny_http::Response::empty(204)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"GET, POST, OPTIONS"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap());
        let _ = request.respond(response);
        return;
    }

    if url == "/api/live_reload" {
        let wait_for = raw_url.split_once('?')
            .and_then(|(_, q)| q.split('&').find_map(|p| p.strip_prefix("wait_for=")))
            .and_then(|v| v.parse::<u64>().ok());
        if let Some(client_ts) = wait_for {
            if get_reload_timestamp() == client_ts {
                if let Ok(guard) = RELOAD_CONDVAR.0.lock() {
                    let _ = RELOAD_CONDVAR.1.wait_timeout_while(
                        guard,
                        std::time::Duration::from_secs(15),
                        |_| get_reload_timestamp() == client_ts,
                    );
                }
            }
        }
        let ts = get_reload_timestamp();
        let data = json!({ "timestamp": ts }).to_string();
        let response = tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else if url == "/api/ui_layout" {
        if let Some((_, query)) = raw_url.split_once('?') {
            for pair in query.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    if k == "path" && !v.is_empty() {
                        let mut decoded = String::new();
                        let mut chars = v.chars().peekable();
                        while let Some(c) = chars.next() {
                            if c == '%' {
                                if let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
                                    if let Ok(byte) = u8::from_str_radix(&format!("{}{}", c1, c2), 16) {
                                        decoded.push(byte as char);
                                        continue;
                                    }
                                }
                            } else if c == '+' {
                                decoded.push(' ');
                            } else {
                                decoded.push(c);
                            }
                        }
                        if !decoded.is_empty() && decoded != "/" {
                            let curr = crate::engine::router::global_router().lock().unwrap().current.clone();
                            if curr != decoded {
                                crate::engine::router::navigate(decoded);
                            }
                        }
                    }
                }
            }
        }
        let data = get_ui_b64::<T>(false);
        let ts_str = get_reload_timestamp().to_string();
        let response = tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Expose-Headers"[..], &b"X-Reload-Timestamp"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"X-Reload-Timestamp"[..], ts_str.as_bytes()).unwrap());
        let _ = request.respond(response);
    } else if url == "/api/dispatch" {
        let mut content = String::new();
        let _ = request.as_reader().read_to_string(&mut content);
        let json: Option<serde_json::Value> = serde_json::from_str(&content).ok();
        let action = json.as_ref().and_then(|j| j.get("action")).and_then(|a| a.as_str()).unwrap_or("").to_string();
        let payload = json.and_then(|j| j.get("payload").cloned());
        
        let data = dispatch_event::<T>(&action, payload);
        let ts_str = get_reload_timestamp().to_string();
        let response = tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Expose-Headers"[..], &b"X-Reload-Timestamp"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"X-Reload-Timestamp"[..], ts_str.as_bytes()).unwrap());
        let _ = request.respond(response);
    } else if url == "/ui" || url == "/api/ui" {
        let state = get_dev_state::<T>().lock().unwrap();
        crate::engine::registry::reset_render_action_seq();
    let _scope = crate::engine::runtime::set_scoped_state(&*state);
        let node = crate::engine::builder::IntoNode::into_node(T::view(&state));
        let data = node.to_json_value().to_string();
        let response = tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else if url == "/action" {
        let mut content = String::new();
        let _ = request.as_reader().read_to_string(&mut content);
        let json: Option<serde_json::Value> = serde_json::from_str(&content).ok();
        let action = json.as_ref().and_then(|j| j.get("action")).and_then(|a| a.as_str()).unwrap_or("").to_string();
        let payload = json.and_then(|j| j.get("payload").cloned());
        
        let _ = dispatch_event::<T>(&action, payload);
        let state = get_dev_state::<T>().lock().unwrap();
        crate::engine::registry::reset_render_action_seq();
    let _scope = crate::engine::runtime::set_scoped_state(&*state);
        let node = crate::engine::builder::IntoNode::into_node(T::view(&state));
        let data = node.to_json_value().to_string();
        let response = tiny_http::Response::from_string(data)
            .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
        let _ = request.respond(response);
    } else {
        let mut custom_data = None;
        for route in inventory::iter::<ServerRoute> {
            if url == route.path {
                custom_data = Some((route.handler)());
                break;
            }
        }
        
        if let Some(data) = custom_data {
            let response = tiny_http::Response::from_string(data)
                .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            let _ = request.respond(response);
        } else if let Some(data) = router(&url) {
            let response = tiny_http::Response::from_string(data)
                .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            let _ = request.respond(response);
        } else if url == "/theme.css" {
            if let Ok(css) = std::fs::read_to_string("demo_app/platforms/Web/theme.css") {
                let response = tiny_http::Response::from_string(css)
                    .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/css"[..]).unwrap());
                let _ = request.respond(response);
            } else {
                let response = tiny_http::Response::empty(404);
                let _ = request.respond(response);
            }
        } else if url == "/wasm_app.js" {
            if let Ok(js) = std::fs::read_to_string("demo_app/platforms/Web/wasm_app.js") {
                let response = tiny_http::Response::from_string(js)
                    .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                    .with_header(tiny_http::Header::from_bytes(&b"application/javascript"[..], &b"application/javascript"[..]).unwrap());
                let _ = request.respond(response);
            } else {
                let response = tiny_http::Response::empty(404);
                let _ = request.respond(response);
            }
        } else if url == "/wasm_app_bg.wasm" {
            if let Ok(wasm) = std::fs::read("demo_app/platforms/Web/wasm_app_bg.wasm") {
                let response = tiny_http::Response::from_data(wasm)
                    .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/wasm"[..]).unwrap());
                let _ = request.respond(response);
            } else {
                let response = tiny_http::Response::empty(404);
                let _ = request.respond(response);
            }
        } else if url == "/health" {
            let response = tiny_http::Response::from_string("{\"status\":\"ok\"}")
                .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
            let _ = request.respond(response);
        } else if !url.starts_with("/api/") {
            // SPA route fallback: serve index.html for root and any deep route
            if let Ok(html) = std::fs::read_to_string("demo_app/platforms/Web/index.html") {
                let response = tiny_http::Response::from_string(html)
                    .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                let _ = request.respond(response);
            } else {
                let html = "<html><body><h1>NativeKit Server is Live!</h1><p>Run `cargo run -p cli` to generate the Web Shell.</p></body></html>";
                let response = tiny_http::Response::from_string(html)
                    .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                let _ = request.respond(response);
            }
        } else {
            let response = tiny_http::Response::from_string("Page Not Found")
                .with_header(tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
