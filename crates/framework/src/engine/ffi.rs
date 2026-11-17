// FFI generation macro to decouple the framework from the specific app
#[macro_export]
macro_rules! export_app {
    ($app_type:ty) => {
        use std::ffi::{CStr, CString};
        use std::os::raw::c_char;
        use std::os::raw::c_void;

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn init_app() {
            $crate::engine::dev_server::get_dev_state::<$app_type>();
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn get_ui() -> String {
            $crate::engine::dev_server::get_ui_b64::<$app_type>(false)
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn dispatch_event(action: &str, payload_str: Option<String>) -> String {
            let payload = payload_str.and_then(|p| $crate::engine::serde_json::from_str(&p).ok());
            $crate::engine::dev_server::dispatch_event::<$app_type>(action, payload)
        }


        static REFRESH_CALLBACK: std::sync::OnceLock<extern "C" fn()> = std::sync::OnceLock::new();

        struct StatePtr(pub *mut c_void);
        unsafe impl Send for StatePtr {}
        unsafe impl Sync for StatePtr {}

        static APP_PTR: std::sync::OnceLock<std::sync::Mutex<StatePtr>> = std::sync::OnceLock::new();

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn register_refresh_callback(callback: extern "C" fn()) {
            let _ = REFRESH_CALLBACK.set(callback);
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn init_app() -> *mut c_void {
            let state = <$app_type as $crate::engine::core::App>::init();
            let ptr = Box::into_raw(Box::new(state)) as *mut c_void;
            let _ = APP_PTR.set(std::sync::Mutex::new(StatePtr(ptr)));
            
            $crate::engine::runtime::set_dispatcher(Box::new(|update_fn| {
                if let Some(ptr_mutex) = APP_PTR.get() {
                    let ptr = ptr_mutex.lock().unwrap().0;
                    if !ptr.is_null() {
                        let state = unsafe { &mut *(ptr as *mut <$app_type as $crate::engine::core::App>::State) };
                        update_fn(state);
                        
                        if let Some(cb) = REFRESH_CALLBACK.get() {
                            cb(); // tell Swift to refresh
                        }
                    }
                }
            }));
            
            $crate::engine::runtime::set_refresher(Box::new(|| {
                if let Some(cb) = REFRESH_CALLBACK.get() {
                    cb(); // tell Swift to refresh
                }
            }));
            
            ptr
        }


        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn get_ui(app_ptr: *mut c_void) -> *const c_char {
            if app_ptr.is_null() {
                return std::ptr::null();
            }
            let state = unsafe { &*(app_ptr as *mut <$app_type as $crate::engine::core::App>::State) };
            $crate::engine::registry::reset_render_action_seq();
            let ui_node = $crate::engine::builder::IntoNode::into_node(<$app_type as $crate::engine::core::App>::view(state));
            
            thread_local! {
                static LAST_DATA: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
                static LAST_NODE: std::cell::RefCell<Option<$crate::engine::ast::Node>> = std::cell::RefCell::new(None);
            }
            
            let encoded = $crate::engine::binary::node_to_base64(&ui_node);
            
            LAST_DATA.with(|last| {
                *last.borrow_mut() = CString::new(encoded).unwrap();
                last.borrow().as_ptr()
            })
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn dispatch_event(app_ptr: *mut c_void, action_id: *const c_char, payload: *const c_char) -> *const c_char {
            if app_ptr.is_null() || action_id.is_null() {
                return get_ui(app_ptr);
            }
            
            let action = unsafe { CStr::from_ptr(action_id) }.to_string_lossy();
            let clean_action = action.trim_matches('"');
            
            let payload_val: Option<$crate::engine::serde_json::Value> = if !payload.is_null() {
                let payload_str = unsafe { CStr::from_ptr(payload) }.to_string_lossy();
                $crate::engine::serde_json::from_str(&payload_str).ok()
            } else {
                None
            };
            
            if clean_action == "navigate" {
                let path = payload_val.as_ref()
                    .and_then(|p| p.get("path").and_then(|s| s.as_str()).or_else(|| p.as_str()))
                    .unwrap_or("/");
                $crate::engine::router::navigate(path);
            } else if clean_action == "pop" {
                $crate::engine::router::navigate(-1);
            } else if clean_action == "dismiss" {
                $crate::engine::router::dismiss();
            } else if clean_action.starts_with("action_") {
                let state = unsafe { &mut *(app_ptr as *mut <$app_type as $crate::engine::core::App>::State) };
                $crate::engine::registry::execute_action(clean_action, state, payload_val);
            }
            
            get_ui(app_ptr)
        }

        #[no_mangle]
        pub extern "C" fn free_app(app_ptr: *mut c_void) {
            if !app_ptr.is_null() {
                unsafe { let _ = Box::from_raw(app_ptr as *mut <$app_type as $crate::engine::core::App>::State); }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_init(host_notify: extern "C" fn()) {
            let _ = REFRESH_CALLBACK.set(host_notify);
            $crate::engine::router::restore_dev_router();
            let _ = $crate::engine::dev_server::get_dev_state::<$app_type>();
            $crate::engine::runtime::set_refresher(Box::new(move || {
                host_notify();
            }));
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_get_ui(is_diff: bool) -> *const c_char {
            thread_local! {
                static DEV_UI_B64: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
            }
            let encoded = $crate::engine::dev_server::get_ui_b64::<$app_type>(is_diff);
            DEV_UI_B64.with(|cell| {
                *cell.borrow_mut() = CString::new(encoded).unwrap_or_else(|_| CString::new("").unwrap());
                cell.borrow().as_ptr()
            })
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_dispatch(action_id: *const c_char, payload: *const c_char) -> *const c_char {
            thread_local! {
                static DEV_DISPATCH_B64: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
            }
            let action = if action_id.is_null() {
                String::new()
            } else {
                unsafe { CStr::from_ptr(action_id) }.to_string_lossy().to_string()
            };
            let payload_val: Option<$crate::engine::serde_json::Value> = if !payload.is_null() {
                let s = unsafe { CStr::from_ptr(payload) }.to_string_lossy();
                $crate::engine::serde_json::from_str(&s).ok()
            } else {
                None
            };
            let res = $crate::engine::dev_server::dispatch_event::<$app_type>(&action, payload_val);
            DEV_DISPATCH_B64.with(|cell| {
                *cell.borrow_mut() = CString::new(res).unwrap_or_else(|_| CString::new("").unwrap());
                cell.borrow().as_ptr()
            })
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_get_ui_json() -> *const c_char {
            thread_local! {
                static DEV_UI_JSON: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("{}").unwrap());
            }
            let state = $crate::engine::dev_server::get_dev_state::<$app_type>().lock().unwrap();
            $crate::engine::registry::reset_render_action_seq();
            let _scope = $crate::engine::runtime::set_scoped_state(&*state);
            let node = $crate::engine::builder::IntoNode::into_node(<$app_type as $crate::engine::core::App>::view(&state));
            let json_str = node.to_json_value().to_string();
            DEV_UI_JSON.with(|cell| {
                *cell.borrow_mut() = CString::new(json_str).unwrap_or_else(|_| CString::new("{}").unwrap());
                cell.borrow().as_ptr()
            })
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_navigate(path: *const c_char) {
            if !path.is_null() {
                let p = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
                $crate::engine::router::navigate(p);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        #[no_mangle]
        pub extern "C" fn nk_dev_handle_route(url: *const c_char) -> *const c_char {
            thread_local! {
                static DEV_ROUTE_RES: std::cell::RefCell<CString> = std::cell::RefCell::new(CString::new("").unwrap());
            }
            if url.is_null() {
                return std::ptr::null();
            }
            let url_str = unsafe { CStr::from_ptr(url) }.to_string_lossy();
            for route in $crate::engine::dev_server::inventory::iter::<$crate::engine::dev_server::ServerRoute> {
                if url_str == route.path {
                    let data = (route.handler)();
                    return DEV_ROUTE_RES.with(|cell| {
                        *cell.borrow_mut() = CString::new(data).unwrap_or_else(|_| CString::new("").unwrap());
                        cell.borrow().as_ptr()
                    });
                }
            }
            std::ptr::null()
        }
    };
}
