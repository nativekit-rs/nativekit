use std::sync::{OnceLock, Mutex};
use std::any::Any;
use std::cell::RefCell;

thread_local! {
    static CURRENT_STATE: RefCell<Option<*const dyn Any>> = const { RefCell::new(None) };
}

pub struct StateScope<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for StateScope<'a> {
    fn drop(&mut self) {
        CURRENT_STATE.with(|c| *c.borrow_mut() = None);
    }
}

pub fn set_scoped_state<'a, S: 'static>(state: &'a S) -> StateScope<'a> {
    let ptr = state as &dyn Any as *const dyn Any;
    CURRENT_STATE.with(|c| *c.borrow_mut() = Some(ptr));
    StateScope { _marker: std::marker::PhantomData }
}

pub trait CurrentStore: Sized + Clone + 'static {
    fn current() -> Self {
        CURRENT_STATE.with(|c| {
            let borrowed = c.borrow();
            if let Some(ptr) = *borrowed {
                unsafe {
                    let any_ref = &*ptr;
                    if let Some(s) = any_ref.downcast_ref::<Self>() {
                        return s.clone();
                    }
                }
            }
            panic!("{}::current() called outside of an active UI render scope or with mismatched type!", std::any::type_name::<Self>());
        })
    }

    fn with_current<R>(f: impl FnOnce(&Self) -> R) -> R {
        CURRENT_STATE.with(|c| {
            let borrowed = c.borrow();
            if let Some(ptr) = *borrowed {
                unsafe {
                    let any_ref = &*ptr;
                    if let Some(s) = any_ref.downcast_ref::<Self>() {
                        return f(s);
                    }
                }
            }
            panic!("{}::with_current() called outside of an active UI render scope or with mismatched type!", std::any::type_name::<Self>());
        })
    }
}

impl<T: Clone + 'static> CurrentStore for T {}

pub fn use_state<S: Clone + 'static>() -> S {
    CURRENT_STATE.with(|c| {
        let borrowed = c.borrow();
        if let Some(ptr) = *borrowed {
            unsafe {
                let any_ref = &*ptr;
                if let Some(s) = any_ref.downcast_ref::<S>() {
                    return s.clone();
                }
            }
        }
        panic!("use_state<{}> called outside of an active UI render scope or with mismatched type!", std::any::type_name::<S>());
    })
}

pub fn with_state<S: 'static, R>(f: impl FnOnce(&S) -> R) -> R {
    CURRENT_STATE.with(|c| {
        let borrowed = c.borrow();
        if let Some(ptr) = *borrowed {
            unsafe {
                let any_ref = &*ptr;
                if let Some(s) = any_ref.downcast_ref::<S>() {
                    return f(s);
                }
            }
        }
        panic!("with_state called outside of an active UI render scope!");
    })
}

type StateUpdateFn = Box<dyn FnOnce(&mut dyn Any) + Send>;
type DispatchFn = Box<dyn Fn(StateUpdateFn) + Send + Sync>;

static DISPATCHER: OnceLock<DispatchFn> = OnceLock::new();
static REFRESHER: OnceLock<Box<dyn Fn() + Send + Sync>> = OnceLock::new();

pub fn set_dispatcher(d: DispatchFn) {
    let _ = DISPATCHER.set(d);
}

pub fn set_refresher(r: Box<dyn Fn() + Send + Sync>) {
    let _ = REFRESHER.set(r);
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn js_trigger_refresh();
}

pub fn trigger_refresh() {
    #[cfg(target_arch = "wasm32")]
    {
        js_trigger_refresh();
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(r) = REFRESHER.get() {
        r();
    }
}

pub enum ResourceState {
    Loading,
    Loaded(Box<dyn Any + Send + Sync>),
}

fn resource_cache() -> &'static Mutex<std::collections::HashMap<String, ResourceState>> {
    static CACHE: OnceLock<Mutex<std::collections::HashMap<String, ResourceState>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

pub fn clear_resource_cache(key: &str) {
    {
        let mut cache = resource_cache().lock().unwrap();
        cache.remove(key);
    }
    trigger_refresh();
}

pub fn clear_all_resource_caches() {
    {
        let mut cache = resource_cache().lock().unwrap();
        cache.clear();
    }
    trigger_refresh();
}

pub fn use_resource<T, F>(key: &str, fetcher: F) -> Option<T>
where
    T: Clone + Send + Sync + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let mut cache = resource_cache().lock().unwrap();
    if let Some(state) = cache.get(key) {
        match state {
            ResourceState::Loading => return None,
            ResourceState::Loaded(any_val) => {
                if let Some(val) = any_val.downcast_ref::<T>() {
                    return Some(val.clone());
                }
            }
        }
    }
    
    cache.insert(key.to_string(), ResourceState::Loading);
    
    let key_clone = key.to_string();
    std::thread::spawn(move || {
        let result = fetcher();
        let mut cache = resource_cache().lock().unwrap();
        cache.insert(key_clone, ResourceState::Loaded(Box::new(result)));
        trigger_refresh();
    });
    
    None
}

pub fn spawn_background<F, S: 'static>(f: F)
where
    F: FnOnce(Context<S>) + Send + 'static,
{
    std::thread::spawn(move || {
        let cx = Context { _marker: std::marker::PhantomData };
        f(cx);
    });
}


pub struct Context<S> {
    _marker: std::marker::PhantomData<S>,
}

impl<S: 'static> Context<S> {
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        if let Some(dispatch) = DISPATCHER.get() {
            let wrapper: StateUpdateFn = Box::new(move |any_state: &mut dyn Any| {
                if let Some(state) = any_state.downcast_mut::<S>() {
                    f(state);
                }
            });
            dispatch(wrapper);
        }
    }
}

pub fn update_state<S: 'static>(f: impl FnOnce(&mut S) + Send + 'static) {
    if let Some(dispatch) = DISPATCHER.get() {
        let wrapper: StateUpdateFn = Box::new(move |any_state: &mut dyn Any| {
            if let Some(state) = any_state.downcast_mut::<S>() {
                f(state);
            }
        });
        dispatch(wrapper);
    }
}

#[cfg(target_arch = "wasm32")]
pub trait SendBound {}
#[cfg(target_arch = "wasm32")]
impl<T> SendBound for T {}

#[cfg(not(target_arch = "wasm32"))]
pub trait SendBound: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> SendBound for T {}

pub fn use_resource_async<T, F, Fut>(key: &str, fetcher: F) -> Option<T>
where
    T: Clone + Send + Sync + 'static,
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + SendBound + 'static,
{
    let mut cache = resource_cache().lock().unwrap();
    if let Some(state) = cache.get(key) {
        match state {
            ResourceState::Loading => return None,
            ResourceState::Loaded(any_val) => {
                if let Some(val) = any_val.downcast_ref::<T>() {
                    return Some(val.clone());
                }
            }
        }
    }
    
    cache.insert(key.to_string(), ResourceState::Loading);
    
    let key_clone = key.to_string();
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            let result = fetcher().await;
            {
                let mut cache = resource_cache().lock().unwrap();
                cache.insert(key_clone, ResourceState::Loaded(Box::new(result)));
            } // Mutex is dropped here
            trigger_refresh();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(move || {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => {
                rt.block_on(async move {
                    let result = fetcher().await;
                    {
                        let mut cache = resource_cache().lock().unwrap();
                        cache.insert(key_clone, ResourceState::Loaded(Box::new(result)));
                    } // Mutex is dropped here
                    trigger_refresh();
                });
            }
            Err(e) => {
                println!("CRITICAL: Failed to start Tokio runtime: {}", e);
            }
        }
    });
    
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorScheme { System, Light, Dark }

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AccentSource { Brand, System }

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub mode: String,
    pub accent: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self { mode: "system".to_string(), accent: "app".to_string() }
    }
}

pub fn global_theme() -> &'static Mutex<Theme> {
    static THEME: OnceLock<Mutex<Theme>> = OnceLock::new();
    THEME.get_or_init(|| Mutex::new(Theme::default()))
}

pub fn dev_theme_snapshot_path() -> std::path::PathBuf {
    std::env::temp_dir().join("native_kit_dev_theme.json")
}

pub fn restore_dev_theme() {
    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    if let Ok(s) = std::fs::read_to_string(dev_theme_snapshot_path()) {
        if let Ok(r) = serde_json::from_str::<Theme>(&s) {
            if let Ok(mut lock) = global_theme().lock() { *lock = r; }
        }
    }
}

impl Theme {
    pub fn current() -> Self { global_theme().lock().unwrap().clone() }
    pub fn save_dev_snapshot(&self) {
        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        if let Ok(s) = serde_json::to_string(self) { let _ = std::fs::write(dev_theme_snapshot_path(), s); }
    }
    pub fn update(f: impl FnOnce(&mut Theme)) {
        { let mut t = global_theme().lock().unwrap(); f(&mut *t); t.save_dev_snapshot(); }
        crate::engine::dev_server::notify_reload();
        trigger_refresh();
    }
    pub fn set_scheme(scheme: ColorScheme) {
        Self::update(|t| t.mode = match scheme { ColorScheme::Light => "light", ColorScheme::Dark => "dark", ColorScheme::System => "system" }.to_string());
    }
    pub fn set_accent_source(source: AccentSource) {
        Self::update(|t| t.accent = match source { AccentSource::Brand => "app", AccentSource::System => "system" }.to_string());
    }
    pub fn light() { Self::set_scheme(ColorScheme::Light); }
    pub fn dark() { Self::set_scheme(ColorScheme::Dark); }
    pub fn system() { Self::set_scheme(ColorScheme::System); }
    pub fn toggle_dark() { Self::update(|t| t.mode = if t.mode == "dark" { "light".to_string() } else { "dark".to_string() }); }
    pub fn toggle_system_accent() { Self::update(|t| t.accent = if t.accent == "system" { "app".to_string() } else { "system".to_string() }); }
    pub fn is_dark(&self) -> bool { self.mode == "dark" }
    pub fn is_system_accent(&self) -> bool { self.accent == "system" }
    pub fn set_mode(mode: impl Into<String>) {
        let m = mode.into().to_lowercase();
        Self::update(|t| t.mode = (if m.contains("light") { "light" } else if m.contains("dark") { "dark" } else { "system" }).to_string());
    }
    pub fn set_accent(accent: impl Into<String>) {
        let a = accent.into().to_lowercase();
        Self::update(|t| t.accent = (if a.contains("system") || a.contains("os") { "system" } else { "app" }).to_string());
    }
    pub fn mode_label(&self) -> &'static str { match self.mode.as_str() { "light" => "Light", "dark" => "Dark", _ => "System Default" } }
    pub fn accent_label(&self) -> &'static str { match self.accent.as_str() { "system" => "System Accent (OS)", _ => "Brand Theme (Config)" } }
}

pub fn set_theme_mode(mode: impl Into<String>) { Theme::set_mode(mode); }
pub fn get_theme_mode() -> String { Theme::current().mode }
pub fn set_accent_mode(mode: impl Into<String>) { Theme::set_accent(mode); }
pub fn get_accent_mode() -> String { Theme::current().accent }

pub fn get_os_accent_hex() -> String {
    static CACHED: OnceLock<String> = OnceLock::new();
    CACHED.get_or_init(|| {
        #[cfg(target_os = "macos")]
        if let Ok(out) = std::process::Command::new("defaults").args(["read", "-g", "AppleAccentColor"]).output() {
            if out.status.success() {
                return match String::from_utf8_lossy(&out.stdout).trim() {
                    "-1" => "#8E8E93", "0" => "#FF3B30", "1" => "#FF9500", "2" => "#FFCC00",
                    "3" => "#34C759", "5" => "#AF52DE", "6" => "#FF2D55", _ => "#007AFF",
                }.to_string();
            }
        }
        option_env!("NK_HOST_OS_ACCENT").unwrap_or("AccentColor").to_string()
    }).clone()
}
