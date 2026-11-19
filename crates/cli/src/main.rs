mod cloud;
mod dev;
mod project;
mod templates;
mod theme;

use std::env;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::Read;
use std::os::raw::{c_char, c_void};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[cfg(unix)]
extern "C" {
    fn dlopen(filename: *const c_char, flag: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

extern "C" fn host_notify_reload() {
    native_kit::engine::dev_server::notify_reload();
}

struct LoadedDylib {
    _handle: *mut c_void,
    init_fn: extern "C" fn(extern "C" fn()),
    get_ui_fn: extern "C" fn(bool) -> *const c_char,
    dispatch_fn: extern "C" fn(*const c_char, *const c_char) -> *const c_char,
    get_ui_json_fn: extern "C" fn() -> *const c_char,
    navigate_fn: extern "C" fn(*const c_char),
    handle_route_fn: extern "C" fn(*const c_char) -> *const c_char,
}

unsafe impl Send for LoadedDylib {}
unsafe impl Sync for LoadedDylib {}

struct DylibHostBridge {
    lib: Mutex<Option<LoadedDylib>>,
    gen_seq: AtomicUsize,
}

impl DylibHostBridge {
    fn new() -> Self {
        Self {
            lib: Mutex::new(None),
            gen_seq: AtomicUsize::new(0),
        }
    }

    #[cfg(unix)]
    fn reload_from_target(&self) -> bool {
        let ext = if cfg!(target_os = "macos") { "dylib" } else { "so" };
        let lib_name = project::project_lib_name();
        let src_path = format!("target/debug/lib{}.{}", lib_name, ext);
        if !Path::new(&src_path).exists() {
            return false;
        }
        let seq = self.gen_seq.fetch_add(1, Ordering::SeqCst) + 1;
        let temp_dir = env::temp_dir().join("nk_hot_dylibs");
        let _ = fs::create_dir_all(&temp_dir);
        if seq > 3 {
            let old_path = temp_dir.join(format!("lib{}_{}.{}", lib_name, seq - 3, ext));
            let _ = fs::remove_file(old_path);
        }
        let dst_path = temp_dir.join(format!("lib{}_{}.{}", lib_name, seq, ext));
        if fs::copy(&src_path, &dst_path).is_err() {
            return false;
        }
        let Ok(c_path) = CString::new(dst_path.to_string_lossy().as_bytes()) else {
            return false;
        };
        unsafe {
            let handle = dlopen(c_path.as_ptr(), 2);
            if handle.is_null() {
                return false;
            }
            let get_sym = |name: &[u8]| -> *mut c_void {
                dlsym(handle, name.as_ptr() as *const c_char)
            };
            let init_ptr = get_sym(b"nk_dev_init\0");
            let get_ui_ptr = get_sym(b"nk_dev_get_ui\0");
            let dispatch_ptr = get_sym(b"nk_dev_dispatch\0");
            let get_ui_json_ptr = get_sym(b"nk_dev_get_ui_json\0");
            let navigate_ptr = get_sym(b"nk_dev_navigate\0");
            let handle_route_ptr = get_sym(b"nk_dev_handle_route\0");

            if init_ptr.is_null()
                || get_ui_ptr.is_null()
                || dispatch_ptr.is_null()
                || get_ui_json_ptr.is_null()
                || navigate_ptr.is_null()
                || handle_route_ptr.is_null()
            {
                return false;
            }

            let dylib = LoadedDylib {
                _handle: handle,
                init_fn: std::mem::transmute(init_ptr),
                get_ui_fn: std::mem::transmute(get_ui_ptr),
                dispatch_fn: std::mem::transmute(dispatch_ptr),
                get_ui_json_fn: std::mem::transmute(get_ui_json_ptr),
                navigate_fn: std::mem::transmute(navigate_ptr),
                handle_route_fn: std::mem::transmute(handle_route_ptr),
            };
            (dylib.init_fn)(host_notify_reload);
            if let Ok(mut lock) = self.lib.lock() {
                *lock = Some(dylib);
            }
            native_kit::engine::dev_server::notify_reload();
            true
        }
    }

    #[cfg(not(unix))]
    fn reload_from_target(&self) -> bool {
        false
    }
}

impl native_kit::engine::dev_server::DevHostBridge for DylibHostBridge {
    fn get_ui_b64(&self) -> String {
        if let Ok(lock) = self.lib.lock() {
            if let Some(ref dylib) = *lock {
                let ptr = (dylib.get_ui_fn)(false);
                if !ptr.is_null() {
                    return unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string();
                }
            }
        }
        String::new()
    }

    fn dispatch_event(&self, action: &str, payload: Option<serde_json::Value>) -> String {
        if let Ok(lock) = self.lib.lock() {
            if let Some(ref dylib) = *lock {
                let c_act = CString::new(action).unwrap_or_else(|_| CString::new("").unwrap());
                let c_pay = payload.and_then(|p| CString::new(p.to_string()).ok());
                let pay_ptr = c_pay.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null());
                let ptr = (dylib.dispatch_fn)(c_act.as_ptr(), pay_ptr);
                if !ptr.is_null() {
                    return unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string();
                }
            }
        }
        String::new()
    }

    fn get_ui_json(&self) -> String {
        if let Ok(lock) = self.lib.lock() {
            if let Some(ref dylib) = *lock {
                let ptr = (dylib.get_ui_json_fn)();
                if !ptr.is_null() {
                    return unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string();
                }
            }
        }
        "{}".to_string()
    }

    fn navigate(&self, path: &str) {
        if let Ok(lock) = self.lib.lock() {
            if let Some(ref dylib) = *lock {
                if let Ok(c_path) = CString::new(path) {
                    (dylib.navigate_fn)(c_path.as_ptr());
                }
            }
        }
    }

    fn handle_route(&self, url: &str) -> Option<String> {
        if let Ok(lock) = self.lib.lock() {
            if let Some(ref dylib) = *lock {
                if let Ok(c_url) = CString::new(url) {
                    let ptr = (dylib.handle_route_fn)(c_url.as_ptr());
                    if !ptr.is_null() {
                        return Some(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string());
                    }
                }
            }
        }
        None
    }
}

fn get_dir_max_mtime(dir: &Path) -> SystemTime {
    let mut max_time = SystemTime::UNIX_EPOCH;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let t = get_dir_max_mtime(&path);
                if t > max_time {
                    max_time = t;
                }
            } else if let Ok(meta) = path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if modified > max_time {
                        max_time = modified;
                    }
                }
            }
        }
    }
    max_time
}

pub fn generate_all_projects() {
    project::sync_auto_modules();
    generate_ios_project();
    generate_android_project();
    generate_web_project();
    generate_windows_project();
    dev::generate_gtk_dev_shell();
    theme::export_theme(
        &project::project_path("platforms/iOS/cargo_native_app"),
        &project::project_path("platforms/Android/app/src/main/java/com/nativeui/cargonativeapp"),
    );
}

fn interactive_terminal(child_ref: Arc<Mutex<Option<std::process::Child>>>) {
    let _ = Command::new("stty").args(["raw", "-echo"]).status();

    let mut buf = [0u8; 1];
    loop {
        if let Ok(1) = std::io::stdin().read(&mut buf) {
            match buf[0] {
                b'i' if dev::is_platform_enabled("ios") => {
                    dev::IOS_LAUNCHED.store(true, Ordering::SeqCst);
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    dev::clear_and_print_header();
                    std::thread::spawn(dev::build_and_launch_ios);
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'a' if dev::is_platform_enabled("android") => {
                    dev::ANDROID_LAUNCHED.store(true, Ordering::SeqCst);
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    dev::clear_and_print_header();
                    std::thread::spawn(dev::build_and_launch_android);
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'w' | b'd' if dev::is_platform_enabled("windows") => {
                    dev::WINDOWS_LAUNCHED.store(true, Ordering::SeqCst);
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    dev::clear_and_print_header();
                    std::thread::spawn(dev::build_and_launch_windows);
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'b' if dev::is_platform_enabled("web") => {
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    let _ = Command::new("open").arg("http://127.0.0.1:5824").status();
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'l' | b'4' if dev::is_platform_enabled("linux") => {
                    dev::LINUX_LAUNCHED.store(true, Ordering::SeqCst);
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    dev::clear_and_print_header();
                    std::thread::spawn(dev::build_and_launch_linux);
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'c' => {
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    let _ = Command::new("open").arg("https://github.com/nativekit-rs/nativekit").status();
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'r' => {
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    if let Ok(mut lock) = child_ref.lock() {
                        if let Some(mut child) = lock.take() {
                            let _ = child.kill();
                        }
                    }
                    let _ = std::process::Command::new("xcrun")
                        .args(["simctl", "terminate", "booted", "com.nativeui.cargo-native-app"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    let _ = std::process::Command::new("xcrun")
                        .args(["simctl", "terminate", "booted", "com.nativeui.cargonativeapp"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    let _ = std::process::Command::new("adb")
                        .args(["shell", "am", "force-stop", "com.nativeui.cargonativeapp"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    do_production_build();
                    std::process::exit(0);
                }
                b'f' => {
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    let url = "mailto:hello@nativekit.rs?subject=Feedback%20on%20NativeKit&body=What%27s%20wrong%3F%0A%0AFeatures%20I%20want%20to%20request%3A%0A%0AQuestions%3A%0A";
                    let _ = Command::new("open").arg(url).status();
                    let _ = Command::new("stty").args(["raw", "-echo"]).status();
                }
                b'q' | 3 => {
                    let _ = Command::new("stty").args(["-raw", "echo"]).status();
                    if let Ok(mut lock) = child_ref.lock() {
                        if let Some(mut child) = lock.take() {
                            let _ = child.kill();
                        }
                    }

                    let _ = std::process::Command::new("xcrun")
                        .args(["simctl", "terminate", "booted", "com.nativeui.cargo-native-app"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    let _ = std::process::Command::new("xcrun")
                        .args(["simctl", "terminate", "booted", "com.nativeui.cargonativeapp"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();
                    let _ = std::process::Command::new("adb")
                        .args(["shell", "am", "force-stop", "com.nativeui.cargonativeapp"])
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();

                    print!("\x1B[2J\x1B[1;1H");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

pub fn replace_section(content: &str, begin_m: &str, end_m: &str, body: &str) -> String {
    if let (Some(s), Some(e)) = (content.find(begin_m), content.find(end_m)) {
        if s < e {
            return format!("{}\n{}\n{}", &content[..s + begin_m.len()], body.trim(), &content[e..]);
        }
    }
    content.to_string()
}

pub fn gather_primitives(ext: &str) -> String {
    match ext {
        "swift" => vec![
            native_kit::BUTTON_IOS_SNIPPET,
            native_kit::TEXT_IOS_SNIPPET,
            native_kit::ICON_IOS_SNIPPET,
            native_kit::HSTACK_IOS_SNIPPET,
            native_kit::VSTACK_IOS_SNIPPET,
            native_kit::SWITCH_IOS_SNIPPET,
            native_kit::NAVIGATIONSTACK_IOS_SNIPPET,
            native_kit::PAGE_IOS_SNIPPET,
            native_kit::TEXTINPUT_IOS_SNIPPET,
            native_kit::IMAGE_IOS_SNIPPET,
            native_kit::TOOLBARTOP_IOS_SNIPPET,
            native_kit::SECTION_IOS_SNIPPET,
            native_kit::GRID_IOS_SNIPPET,
            native_kit::SPACER_IOS_SNIPPET,
        ]
        .join("\n"),
        "kt" => vec![
            native_kit::BUTTON_ANDROID_SNIPPET,
            native_kit::TEXT_ANDROID_SNIPPET,
            native_kit::ICON_ANDROID_SNIPPET,
            native_kit::HSTACK_ANDROID_SNIPPET,
            native_kit::VSTACK_ANDROID_SNIPPET,
            native_kit::SWITCH_ANDROID_SNIPPET,
            native_kit::NAVIGATIONSTACK_ANDROID_SNIPPET,
            native_kit::PAGE_ANDROID_SNIPPET,
            native_kit::TEXTINPUT_ANDROID_SNIPPET,
            native_kit::IMAGE_ANDROID_SNIPPET,
            native_kit::TOOLBARTOP_ANDROID_SNIPPET,
            native_kit::SECTION_ANDROID_SNIPPET,
            native_kit::GRID_ANDROID_SNIPPET,
            native_kit::SPACER_ANDROID_SNIPPET,
        ]
        .join("\n"),
        "rs" => vec![
            native_kit::BUTTON_GTK_SNIPPET,
            native_kit::TEXT_GTK_SNIPPET,
            native_kit::ICON_GTK_SNIPPET,
            native_kit::HSTACK_GTK_SNIPPET,
            native_kit::VSTACK_GTK_SNIPPET,
            native_kit::SWITCH_GTK_SNIPPET,
            native_kit::NAVIGATIONSTACK_GTK_SNIPPET,
            native_kit::PAGE_GTK_SNIPPET,
            native_kit::TEXTINPUT_GTK_SNIPPET,
            native_kit::IMAGE_GTK_SNIPPET,
            native_kit::TOOLBARTOP_GTK_SNIPPET,
            native_kit::SECTION_GTK_SNIPPET,
            native_kit::GRID_GTK_SNIPPET,
            native_kit::SPACER_GTK_SNIPPET,
        ]
        .join("\n"),
        "js" => vec![
            native_kit::BUTTON_WEB_SNIPPET,
            native_kit::TEXT_WEB_SNIPPET,
            native_kit::ICON_WEB_SNIPPET,
            native_kit::HSTACK_WEB_SNIPPET,
            native_kit::VSTACK_WEB_SNIPPET,
            native_kit::SWITCH_WEB_SNIPPET,
            native_kit::NAVIGATIONSTACK_WEB_SNIPPET,
            native_kit::PAGE_WEB_SNIPPET,
            native_kit::TEXTINPUT_WEB_SNIPPET,
            native_kit::IMAGE_WEB_SNIPPET,
            native_kit::TOOLBARTOP_WEB_SNIPPET,
            native_kit::SECTION_WEB_SNIPPET,
            native_kit::GRID_WEB_SNIPPET,
            native_kit::SPACER_WEB_SNIPPET,
        ]
        .join("\n"),
        "cs" => vec![
            native_kit::BUTTON_WINDOWS_SNIPPET,
            native_kit::TEXT_WINDOWS_SNIPPET,
            native_kit::ICON_WINDOWS_SNIPPET,
            native_kit::HSTACK_WINDOWS_SNIPPET,
            native_kit::VSTACK_WINDOWS_SNIPPET,
            native_kit::SWITCH_WINDOWS_SNIPPET,
            native_kit::NAVIGATIONSTACK_WINDOWS_SNIPPET,
            native_kit::PAGE_WINDOWS_SNIPPET,
            native_kit::TEXTINPUT_WINDOWS_SNIPPET,
            native_kit::IMAGE_WINDOWS_SNIPPET,
            native_kit::TOOLBARTOP_WINDOWS_SNIPPET,
            native_kit::SECTION_WINDOWS_SNIPPET,
            native_kit::GRID_WINDOWS_SNIPPET,
            native_kit::SPACER_WINDOWS_SNIPPET,
        ]
        .join("\n"),
        _ => String::new(),
    }
}

fn print_help() {
    println!("\n⚡ \x1B[1;36mNativeKit CLI (nk)\x1B[0m — https://nativekit.rs");
    println!("   High-Performance 5-Platform Dynamic UI Engine for Rust\n");
    println!("\x1B[1mUSAGE:\x1B[0m");
    println!("  nk <COMMAND> [OPTIONS]\n");
    println!("\x1B[1mCOMMANDS:\x1B[0m");
    println!("  \x1B[1;32mnew\x1B[0m <name> [--template <id>]   Scaffold a new NativeKit project from a starter template");
    println!("  \x1B[1;32mtemplates\x1B[0m                      List all available starter templates (Free & Pro)");
    println!("  \x1B[1;32mdev\x1B[0m [--ios|--android|--linux]  Start the live sub-second hot-reloading dev server (default)");
    println!("  \x1B[1;32mbuild\x1B[0m                          Compile production native binaries across enabled platforms");
    println!("  \x1B[1;32mlink\x1B[0m <org> <project-id>        Link project to <org>.nativekit.rs/<project-id>/config");
    println!("  \x1B[1;32mlogin\x1B[0m <token>                  Authenticate with NativeKit Cloud");
    println!("  \x1B[1;32msync\x1B[0m                           Pull latest theme, colors, and icon mappings from Cloud\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h" || a == "help") {
        print_help();
        return;
    }

    // Subcommand: `nk templates`
    if args.get(1).map(|s| s.as_str()) == Some("templates") {
        templates::print_templates_list();
        return;
    }

    // Subcommand: `nk new [name] [--template <id>]` or `nk init [name]`
    if matches!(args.get(1).map(|s| s.as_str()), Some("new" | "init")) {
        let mut app_name: Option<String> = None;
        let mut template_id: Option<String> = None;
        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "-t" | "--template" => {
                    if let Some(val) = args.get(i + 1) {
                        template_id = Some(val.clone());
                        i += 2;
                        continue;
                    }
                }
                other if !other.starts_with('-') && app_name.is_none() => {
                    app_name = Some(other.to_string());
                }
                _ => {}
            }
            i += 1;
        }
        templates::scaffold_new_project(app_name, template_id);
        return;
    }

    // Subcommand: `nk link <org> <project-id>`
    if args.get(1).map(|s| s.as_str()) == Some("link") {
        let Some(org) = args.get(2) else {
            eprintln!("Usage: nk link <org> <project-id>");
            return;
        };
        let Some(proj) = args.get(3) else {
            eprintln!("Usage: nk link <org> <project-id>");
            return;
        };
        cloud::link_cloud_project(org, proj);
        return;
    }

    // Subcommand: `nk login <token>`
    if args.get(1).map(|s| s.as_str()) == Some("login") {
        let Some(tok) = args.get(2) else {
            eprintln!("Usage: nk login <token>");
            return;
        };
        cloud::save_cloud_token(tok);
        return;
    }

    // Subcommand: `nk sync` / `nk pull`
    if matches!(args.get(1).map(|s| s.as_str()), Some("sync" | "pull")) {
        if cloud::sync_cloud_config(true) {
            generate_all_projects();
        }
        return;
    }

    let is_child = args.iter().any(|a| a == "--child-worker");
    let is_build = args.iter().any(|a| a == "build" || a == "--build");
    let is_prod = args.iter().any(|a| a == "prod" || a == "--prod");
    let is_generate = args.iter().any(|a| a == "--generate");

    if args.iter().any(|a| a == "--random-theme") {
        let _ = std::fs::remove_file(native_kit::engine::runtime::dev_theme_snapshot_path());
        let name = theme::randomize_theme_palette();
        theme::export_theme(
            &project::project_path("platforms/iOS/cargo_native_app"),
            &project::project_path("platforms/Android/app/src/main/java/com/nativeui/cargonativeapp"),
        );
        println!("🎨 Generated random theme: {}", name);
        return;
    }

    let is_dev = !is_child && !is_build && !is_prod && !is_generate;

    if is_generate {
        generate_all_projects();
        return;
    }

    let launch_ios = args.iter().any(|a| a == "--ios");
    let launch_android = args.iter().any(|a| a == "--android");
    let launch_linux = args.iter().any(|a| a == "--linux");
    let launch_windows = args.iter().any(|a| a == "--windows");

    if is_dev && !is_child {
        // Automatically pull latest styles & icon mappings if project is linked to NativeKit Cloud
        let _ = cloud::sync_cloud_config(false);
        generate_all_projects();

        let _ = std::fs::remove_file(native_kit::engine::router::dev_router_snapshot_path());
        let _ = std::fs::remove_dir_all(project::project_path("platforms/Android/app/src/main/jniLibs"));
        let _ = std::fs::remove_file(project::project_path("platforms/Windows/engine.dll"));
        if launch_ios {
            dev::IOS_LAUNCHED.store(true, Ordering::SeqCst);
        }
        if launch_android {
            dev::ANDROID_LAUNCHED.store(true, Ordering::SeqCst);
        }
        if launch_linux {
            dev::LINUX_LAUNCHED.store(true, Ordering::SeqCst);
        }
        if launch_windows {
            dev::WINDOWS_LAUNCHED.store(true, Ordering::SeqCst);
        }

        let bridge = Arc::new(DylibHostBridge::new());
        #[cfg(unix)]
        let use_dylib_host = true;
        #[cfg(not(unix))]
        let use_dylib_host = false;

        let crate_name = project::project_crate_name();
        println!("\r\x1B[33m⏳ Compiling engine for the first time...\x1B[0m");
        if use_dylib_host {
            let _ = Command::new("cargo")
                .args(["rustc", "-p", &crate_name, "--lib", "--crate-type", "cdylib"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
            let _ = bridge.reload_from_target();
            native_kit::engine::dev_server::start_host_server(bridge.clone());
        } else {
            let _ = Command::new("cargo")
                .args(["build", "-p", &crate_name])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
        }

        dev::clear_and_print_header();

        let child_ref: Arc<Mutex<Option<std::process::Child>>> = if use_dylib_host {
            Arc::new(Mutex::new(None))
        } else {
            let bin_path = format!("./target/debug/{}", crate_name);
            let initial_child = Command::new(&bin_path)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to start application binary");
            Arc::new(Mutex::new(Some(initial_child)))
        };

        let mut last_check = SystemTime::now();
        let is_compiling = Arc::new(AtomicBool::new(false));

        let child_ref_clone = child_ref.clone();
        let bridge_clone = bridge.clone();
        std::thread::spawn(move || {
            let src_dir = project::project_path("src");
            let cfg_dir = project::project_path("config");
            let crate_name = project::project_crate_name();
            loop {
                std::thread::sleep(Duration::from_millis(25));

                let cur_src_mtime = get_dir_max_mtime(&src_dir);
                let cur_theme_mtime = get_dir_max_mtime(&cfg_dir);

                if (cur_src_mtime > last_check || cur_theme_mtime > last_check)
                    && !is_compiling.load(Ordering::SeqCst)
                {
                    let theme_changed = cur_theme_mtime > last_check;
                    last_check = SystemTime::now();
                    is_compiling.store(true, Ordering::SeqCst);

                    let is_compiling_clone = is_compiling.clone();
                    let child_ref_clone2 = child_ref_clone.clone();
                    let bridge_clone2 = bridge_clone.clone();
                    let crate_name_clone = crate_name.clone();

                    std::thread::spawn(move || {
                        if theme_changed {
                            generate_all_projects();
                        }
                        let _ = dev::HOT_RELOADS.fetch_add(1, Ordering::SeqCst);

                        let _ = Command::new("stty").args(["-raw", "echo"]).status();
                        println!("\r\x1B[33m⏳ Compiling changes...\x1B[0m");
                        let _ = Command::new("stty").args(["raw", "-echo"]).status();

                        let start_time = SystemTime::now();

                        // Sync auto-discovered modules in src/lib.rs when files in src/{components,pages,views} change
                        project::sync_auto_modules();

                        if !use_dylib_host {
                            if let Ok(mut lock) = child_ref_clone2.lock() {
                                if let Some(mut old_child) = lock.take() {
                                    let _ = old_child.kill();
                                    let _ = old_child.wait();
                                }
                            }
                        }

                        let build_status = if use_dylib_host {
                            Command::new("cargo")
                                .args(["rustc", "-p", &crate_name_clone, "--lib", "--crate-type", "cdylib"])
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .status()
                        } else {
                            Command::new("cargo")
                                .args(["build", "-p", &crate_name_clone])
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .status()
                        };

                        if let Ok(s) = build_status {
                            if s.success() {
                                if use_dylib_host {
                                    let _ = bridge_clone2.reload_from_target();
                                } else if let Ok(mut lock) = child_ref_clone2.lock() {
                                    let bin_path = format!("./target/debug/{}", crate_name_clone);
                                    if let Ok(new_child) = Command::new(&bin_path)
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn()
                                    {
                                        *lock = Some(new_child);
                                    }
                                }
                                let elapsed = start_time.elapsed().unwrap_or_default();
                                let _ = Command::new("stty").args(["-raw", "echo"]).status();
                                dev::HOT_RELOAD_TIME.store(elapsed.as_millis() as u64, Ordering::SeqCst);
                                dev::clear_and_print_header();
                                let _ = Command::new("stty").args(["raw", "-echo"]).status();
                            } else {
                                let _ = Command::new("stty").args(["-raw", "echo"]).status();
                                println!("\r\x1B[31m❌ Compilation failed!\x1B[0m\n");
                                let _ = Command::new("cargo").args(["check", "-p", &crate_name_clone]).status();
                                let _ = Command::new("stty").args(["raw", "-echo"]).status();
                            }
                        }

                        is_compiling_clone.store(false, Ordering::SeqCst);
                    });
                }
            }
        });

        if launch_ios {
            std::thread::spawn(dev::build_and_launch_ios);
        }
        if launch_android {
            std::thread::spawn(dev::build_and_launch_android);
        }
        if launch_linux {
            std::thread::spawn(dev::build_and_launch_linux);
        }
        if launch_windows {
            std::thread::spawn(dev::build_and_launch_windows);
        }

        interactive_terminal(child_ref.clone());
        return;
    }

    if is_build {
        do_production_build();
        return;
    }

    let dim = "\x1B[90m";
    let reset = "\x1B[0m";
    let green = "\x1B[32;1m";
    let cyan = "\x1B[36;1m";

    if is_prod {
        println!("\x1B[2J\x1B[1;1H");
        println!("{dim}────────────────────────────────────────────────────────────{reset}");
        println!("{green}🌐 NativeKit Production Server{reset}");
        println!("{dim}────────────────────────────────────────────────────────────{reset}");
        println!("  ✅ Environment: {cyan}Production{reset}");
        println!("  ⚡ Listening:   {cyan}http://0.0.0.0:5824{reset}");
        println!("  🚀 Status:      {green}Serving API Endpoints{reset}");
        println!("{dim}────────────────────────────────────────────────────────────{reset}\n");
    }

    std::thread::sleep(Duration::from_millis(100));
    std::thread::park();
}

fn set_ios_prod_mode(is_prod: bool) {
    let path = project::project_path("platforms/iOS/cargo_native_app/GeneratedPrimitives.swift");
    if let Ok(content) = std::fs::read_to_string(&path) {
        let updated = if is_prod {
            content.replace("let isProdBuild = false", "let isProdBuild = true")
        } else {
            content.replace("let isProdBuild = true", "let isProdBuild = false")
        };
        if updated != content {
            let _ = std::fs::write(&path, updated);
        }
    }
}

fn generate_ios_project() {
    let mut raw_name = "NativeKit".to_string();
    if let Ok(app_json_str) = std::fs::read_to_string(project::project_path("config/app.json")) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
            if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                raw_name = name.to_string();
            }
        }
    }

    let pbxproj_path = project::project_path("platforms/iOS/cargo_native_app.xcodeproj/project.pbxproj");
    if let Ok(pbxproj) = std::fs::read_to_string(&pbxproj_path) {
        let mut lines = Vec::new();
        for line in pbxproj.lines() {
            if line.contains("INFOPLIST_KEY_CFBundleDisplayName =") {
                lines.push(format!("\t\t\t\tINFOPLIST_KEY_CFBundleDisplayName = \"{}\";", raw_name));
            } else {
                lines.push(line.to_string());
            }
        }
        let _ = std::fs::write(&pbxproj_path, lines.join("\n"));
    }

    let gen_swift = format!(
        "// ⚡ GENERATED BY NATIVEKIT (https://nativekit.rs) - DO NOT EDIT\nimport SwiftUI\n\nlet isProdBuild = false\npublic struct AppTheme {{\n    public static var primary: Color {{ Color(\"AccentColor\") }}\n}}\n\n{}\n\n@ViewBuilder\nfunc renderGeneratedPrimitive(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {{\n    switch node.name {{\n{}\n    default:\n        EmptyView()\n    }}\n}}\n",
        native_kit::COLOR_IOS_HELPERS,
        gather_primitives("swift")
    );
    let _ = std::fs::write(
        project::project_path("platforms/iOS/cargo_native_app/GeneratedPrimitives.swift"),
        gen_swift,
    );
}

fn generate_android_project() {
    let gradlew = project::project_path("platforms/Android/gradlew");
    if gradlew.exists() {
        let _ = std::process::Command::new("chmod").arg("+x").arg(&gradlew).status();
    }

    let mut raw_name = "NativeKit".to_string();
    if let Ok(app_json_str) = std::fs::read_to_string(project::project_path("config/app.json")) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
            if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                raw_name = name.to_string();
            }
        }
    }

    let manifest_path = project::project_path("platforms/Android/app/src/main/AndroidManifest.xml");
    if let Ok(manifest) = std::fs::read_to_string(&manifest_path) {
        let mut lines = Vec::new();
        for line in manifest.lines() {
            if line.contains("android:label=") {
                lines.push(format!("        android:label=\"{}\"", raw_name));
            } else {
                lines.push(line.to_string());
            }
        }
        let _ = std::fs::write(&manifest_path, lines.join("\n"));
    }

    let gen_kt = format!(
        "// ⚡ GENERATED BY NATIVEKIT (https://nativekit.rs) - DO NOT EDIT\npackage com.nativeui.cargonativeapp\n\nimport androidx.compose.foundation.*\nimport androidx.compose.foundation.layout.*\nimport androidx.compose.material.icons.Icons\nimport androidx.compose.material.icons.filled.*\nimport androidx.compose.material.icons.rounded.*\nimport androidx.compose.material.icons.outlined.*\nimport androidx.compose.material.icons.sharp.*\nimport androidx.compose.material.icons.twotone.*\nimport androidx.compose.material3.*\nimport androidx.compose.runtime.*\nimport androidx.compose.ui.Alignment\nimport androidx.compose.ui.Modifier\nimport androidx.compose.ui.draw.clip\nimport androidx.compose.ui.focus.onFocusChanged\nimport androidx.compose.ui.graphics.Color\nimport androidx.compose.ui.graphics.graphicsLayer\nimport androidx.compose.ui.layout.layout\nimport androidx.compose.ui.input.nestedscroll.nestedScroll\nimport androidx.compose.ui.input.pointer.pointerInput\nimport androidx.compose.foundation.gestures.detectTapGestures\nimport androidx.compose.ui.unit.dp\n\nval AppIcons = Icons.Rounded\n\n{}\n\n@OptIn(androidx.compose.foundation.ExperimentalFoundationApi::class, androidx.compose.foundation.layout.ExperimentalLayoutApi::class, androidx.compose.material3.ExperimentalMaterial3Api::class)\n@Composable\nfun RenderGeneratedPrimitive(node: DynamicUINode, dispatch: (String, org.json.JSONObject?) -> Unit, modifier: Modifier = Modifier) {{\n    when (node.name) {{\n{}\n        else -> {{}}\n    }}\n}}\n",
        native_kit::COLOR_ANDROID_HELPERS,
        gather_primitives("kt")
    );
    let _ = std::fs::write(
        project::project_path("platforms/Android/app/src/main/java/com/nativeui/cargonativeapp/GeneratedPrimitives.kt"),
        gen_kt,
    );
}

fn generate_web_project() {
    let web_dir = project::project_path("platforms/Web");
    let html_path = web_dir.join("index.html");
    if let Ok(html_template) = std::fs::read_to_string(&html_path) {
        let html = replace_section(&html_template, "/* BEGIN_COLOR_HELPERS */", "/* END_COLOR_HELPERS */", native_kit::COLOR_WEB_HELPERS);
        let mut html = replace_section(&html, "/* BEGIN_PRIMITIVES */", "/* END_PRIMITIVES */", &gather_primitives("js"));
        if !html.contains("name=\"generator\"") {
            html = html.replace(
                "<head>",
                "<head>\n    <meta name=\"generator\" content=\"NativeKit (https://nativekit.rs)\">",
            );
        }
        let _ = std::fs::create_dir_all(web_dir.join("src"));
        let _ = std::fs::write(&html_path, html);
    }
    let build_sh = web_dir.join("build-web-app.sh");
    if build_sh.exists() {
        let _ = std::process::Command::new("chmod").arg("+x").arg(&build_sh).status();
    }
}

pub fn generate_windows_project() {
    let win_dir = project::project_path("platforms/Windows");
    let _ = std::fs::create_dir_all(&win_dir);

    let mut app_title = "NativeKit".to_string();
    if let Ok(app_json_str) = std::fs::read_to_string(project::project_path("config/app.json")) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
            if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                app_title = name.to_string();
            }
        }
    }

    let xaml_path = win_dir.join("MainWindow.xaml");
    if let Ok(content) = std::fs::read_to_string(&xaml_path) {
        let updated = content.replace("/* APP_NAME */", &app_title);
        if updated != content {
            let _ = std::fs::write(&xaml_path, updated);
        }
    }

    let mut ips = vec![crate::dev::get_local_ip()];
    ips.push("10.211.55.2".to_string());
    ips.push("192.168.64.1".to_string());
    ips.push("10.0.2.2".to_string());
    ips.push("127.0.0.1".to_string());

    let mut ip_array = String::new();
    for ip in ips {
        ip_array.push_str(&format!("\"http://{}:5824\", ", ip));
    }

    let inject_url = format!("serverUrls = new string[] {{ {} }};", ip_array);
    let cs_path = win_dir.join("MainWindow.xaml.cs");
    if let Ok(existing_cs) = std::fs::read_to_string(&cs_path) {
        let mut main_cs = existing_cs.replace("/* APP_NAME */", &app_title);
        main_cs = replace_section(&main_cs, "/* BEGIN_DEV_URL */", "/* END_DEV_URL */", &inject_url);
        if main_cs != existing_cs {
            let _ = std::fs::write(&cs_path, main_cs);
        }
    }

    let gen_cs = format!(
        "// ⚡ GENERATED BY NATIVEKIT (https://nativekit.rs) - DO NOT EDIT\nusing System;\nusing System.Collections.Generic;\nusing Microsoft.UI.Xaml;\nusing Microsoft.UI.Xaml.Controls;\n\nnamespace CargoNativeApp {{\n    public partial class MainWindow : Window {{\n{}\n\n        private UIElement RenderGeneratedPrimitive(DynamicUINode node) {{\n            string name = node.Name;\n            switch (name) {{\n{}\n                default:\n                    return new StackPanel();\n            }}\n        }}\n    }}\n}}\n",
        native_kit::COLOR_WINDOWS_HELPERS,
        gather_primitives("cs")
    );
    let gen_cs_path = win_dir.join("GeneratedPrimitives.cs");
    if std::fs::read_to_string(&gen_cs_path).unwrap_or_default() != gen_cs {
        let _ = std::fs::write(&gen_cs_path, gen_cs);
    }
}

fn generate_gtk_prod_project() {
    crate::dev::generate_gtk_dev_shell();
    let crate_name = project::project_crate_name();
    let toml_path = project::project_path("platforms/Linux/Cargo.toml");
    if let Ok(mut toml) = std::fs::read_to_string(&toml_path) {
        if !toml.contains("[features]") {
            toml.push_str(&format!(
                "\n[features]\nprod = [\"dep:{0}\"]\n\n[dependencies.{0}]\npath = \"../../\"\noptional = true\nfeatures = [\"client\"]\n",
                crate_name
            ));
            let _ = std::fs::write(&toml_path, toml);
        }
    }
}

fn do_production_build() {
    println!("\n🚀 NativeKit Production Build");
    println!("Checking enabled platforms in config/app.json...\n");

    let ios_on = dev::is_platform_enabled("ios");
    let android_on = dev::is_platform_enabled("android");
    let linux_on = dev::is_platform_enabled("linux");
    let windows_on = dev::is_platform_enabled("windows");
    let web_on = dev::is_platform_enabled("web");

    let crate_name = project::project_crate_name();
    let lib_name = project::project_lib_name();
    let mut app_name = "MyNativeKitApp".to_string();
    if let Ok(app_json_str) = std::fs::read_to_string(project::project_path("config/app.json")) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
            if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                app_name = name.replace(" ", "");
            }
        }
    }

    if ios_on {
        generate_ios_project();
        set_ios_prod_mode(true);
    }
    if android_on {
        generate_android_project();
    }
    if linux_on {
        generate_gtk_prod_project();
    }
    if web_on {
        generate_web_project();
    }
    if windows_on {
        generate_windows_project();
    }

    theme::export_theme(
        &project::project_path("platforms/iOS/cargo_native_app"),
        &project::project_path("platforms/Android/app/src/main/java/com/nativeui/cargonativeapp"),
    );
    if ios_on {
        set_ios_prod_mode(true);
    }

    if ios_on {
        println!("🍎 [iOS] Compiling static library (aarch64-apple-ios-sim & aarch64-apple-ios)...");
        let _ = Command::new("cargo")
            .args(["build", "-p", &crate_name, "--release", "--target", "aarch64-apple-ios-sim"])
            .status();
        let src_a = format!("target/aarch64-apple-ios-sim/release/lib{}.a", lib_name);
        let dst_a = project::project_path("platforms/iOS/libdemo_app.a");
        let _ = std::fs::copy(&src_a, &dst_a);
    }

    if android_on {
        println!("🤖 [Android] Compiling JNI shared libraries (arm64-v8a & x86_64)...");
        let jni_dir = project::project_path_str("platforms/Android/app/src/main/jniLibs");
        let _ = Command::new("cargo")
            .args(["ndk", "-t", "arm64-v8a", "-t", "x86_64", "-o", &jni_dir, "build", "-p", &crate_name, "--release"])
            .status();
    }

    if windows_on {
        println!("🪟 [Windows] Compiling x86_64 engine.dll...");
        let _ = Command::new("cargo")
            .args(["build", "--release", "--target", "x86_64-pc-windows-gnu", "-p", &crate_name, "--features", "client"])
            .status();
        let src_dll = format!("target/x86_64-pc-windows-gnu/release/{}.dll", lib_name);
        let dst_dll = project::project_path("platforms/Windows/engine.dll");
        let _ = std::fs::copy(&src_dll, &dst_dll);
    }

    if linux_on {
        println!("🐧 [Linux] Compiling standalone GTK4 release binary...");
        let _ = Command::new("cargo")
            .args(["build", "--release", "--features", "prod"])
            .env("CARGO_TARGET_DIR", std::env::current_dir().unwrap().join("target/linux-build"))
            .current_dir(project::project_path("platforms/Linux"))
            .status();
    }

    if web_on {
        println!("🌐 [Web] Compiling WebAssembly bundle...");
        let web_dir = project::project_path("platforms/Web");
        let _ = Command::new("sh")
            .current_dir(&web_dir)
            .arg("./build-web-app.sh")
            .status();
        let _ = std::fs::write(web_dir.join("Dockerfile"), "FROM nginx:alpine\nCOPY . /usr/share/nginx/html\nEXPOSE 80\n");
    }

    println!("\n\x1B[90m────────────────────────────────────────────────────────────\x1B[0m");
    println!("✨ \x1B[1mProduction Build Complete! How to Run Each Enabled Platform:\x1B[0m\n");
    if ios_on {
        println!("  🍎 iOS:      Open platforms/iOS/cargo_native_app.xcodeproj in Xcode and press Run (Cmd + R)");
    }
    if android_on {
        println!("  🤖 Android:  Open platforms/Android in Android Studio and press Run (▶)");
    }
    if windows_on {
        println!("  🪟 Windows:  Open platforms/Windows/CargoNativeApp.csproj in Visual Studio");
    }
    if linux_on {
        println!("  🐧 Linux:    Run ./platforms/Linux/{}", app_name);
    }
    if web_on {
        println!("  🌐 Web:      Run: npx serve platforms/Web");
    }
    println!("\x1B[90m────────────────────────────────────────────────────────────\x1B[0m\n");
}
