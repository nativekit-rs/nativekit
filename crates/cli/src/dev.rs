pub fn is_platform_enabled(platform: &str) -> bool {
    let path = crate::project::project_path("config/app.json");
    if let Ok(txt) = std::fs::read_to_string(path) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
            if let Some(arr) = val.get("platforms").and_then(|v| v.as_array()) {
                return arr.iter().any(|item| {
                    item.as_str().map(|s| s.eq_ignore_ascii_case(platform)).unwrap_or(false)
                });
            }
        }
    }
    true
}

use std::path::PathBuf;

pub fn get_local_ip() -> String {
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                return addr.ip().to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

pub static IOS_LAUNCHED: AtomicBool = AtomicBool::new(false);
pub static ANDROID_LAUNCHED: AtomicBool = AtomicBool::new(false);
pub static LINUX_LAUNCHED: AtomicBool = AtomicBool::new(false);
pub static WINDOWS_LAUNCHED: AtomicBool = AtomicBool::new(false);
pub static WINDOWS_COMPILE_TIME: AtomicU64 = AtomicU64::new(0);

pub static HOT_RELOADS: AtomicUsize = AtomicUsize::new(0);
pub static IOS_COMPILE_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static ANDROID_COMPILE_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LINUX_COMPILE_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static HOT_RELOAD_TIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static LINUX_CHILD: std::sync::Mutex<Option<std::process::Child>> = std::sync::Mutex::new(None);

pub fn clear_and_print_header() {
    print!("\x1B[2J\x1B[1;1H");
    
    // NativeKit Multi-Color Logo
    println!("\x1B[1;37m  _   _       _   _           \x1B[38;5;33m_  ___\x1B[38;5;46m _   \x1B[0m");
    println!("\x1B[1;37m | \\ | |     | | (_)         \x1B[38;5;33m| |/ (_)\x1B[38;5;46m |  \x1B[0m");
    println!("\x1B[1;37m |  \\| | __ _| |_ ___   _____\x1B[38;5;33m| ' / _|\x1B[38;5;46m |_ \x1B[0m");
    println!("\x1B[1;37m | . ` |/ _` | __| \\ \\ / / _ \\\x1B[38;5;33m  < | |\x1B[38;5;46m __|\x1B[0m");
    println!("\x1B[1;37m | |\\  | (_| | |_| |\\ V /  __/\x1B[38;5;46m . \\| | |_ \x1B[0m");
    println!("\x1B[1;37m \\_| \\_/\\__,_|\\__|_| \\_/ \\___\x1B[38;5;46m|_|\\_\\_|\\__|\x1B[0m\n");
    
    let dim = "\x1B[90m";
    let reset = "\x1B[0m";
    let divider = format!("{dim}────────────────────────────────────────────────────────────{reset}");
    
    println!("⚙️  To edit settings, modify \x1B[1mconfig/app.json\x1B[0m");
    println!("{divider}");
    println!("📱 \x1B[1mDevices & Simulators\x1B[0m\n");
    
    let ios = IOS_LAUNCHED.load(Ordering::SeqCst);
    let android = ANDROID_LAUNCHED.load(Ordering::SeqCst);
    
    if is_platform_enabled("ios") {
        if ios {
            let status = if IOS_COMPILE_TIME.load(Ordering::SeqCst) > 0 {
                "\x1B[32m🚀 Connected\x1B[0m"
            } else {
                "\x1B[33m⏳ booting & compiling...\x1B[0m"
            };
            println!("  🍎 iOS Simulator     | {}", status);
        } else {
            println!("  🍎 Press 'i' to launch iOS Simulator");
        }
    }

    if is_platform_enabled("android") {
        if android {
            let status = if ANDROID_COMPILE_TIME.load(Ordering::SeqCst) > 0 {
                "\x1B[32m🚀 Connected\x1B[0m"
            } else {
                "\x1B[33m⏳ booting & compiling...\x1B[0m"
            };
            println!("  🤖 Android Emulator  | {}", status);
        } else {
            println!("  🤖 Press 'a' to launch Android Emulator");
        }
    }
    
    if is_platform_enabled("web") {
        println!("  🌐 Press 'b' to open the Web Shell in your browser");
    }

    let linux = LINUX_LAUNCHED.load(Ordering::SeqCst);
    if is_platform_enabled("linux") {
        if linux {
            let status = if LINUX_COMPILE_TIME.load(Ordering::SeqCst) > 0 {
                "\x1B[32m🚀 Connected\x1B[0m"
            } else {
                "\x1B[33m⏳ compiling native shell...\x1B[0m"
            };
            println!("  🐧 Linux GTK4 Shell  | {}", status);
        } else {
            println!("  🐧 Press 'l' to generate Linux GTK4 shell");
        }
    }
    
    let windows = WINDOWS_LAUNCHED.load(Ordering::SeqCst);
    if is_platform_enabled("windows") {
        if windows {
            let status = if WINDOWS_COMPILE_TIME.load(Ordering::SeqCst) > 0 {
                "\x1B[32m🚀 Connected\x1B[0m"
            } else if std::env::consts::OS == "windows" {
                "\x1B[33m⏳ compiling native shell...\x1B[0m"
            } else {
                "\x1B[33m⏳ Waiting for VM run...\x1B[0m"
            };
            println!("  🪟 Windows Shell     | {}", status);
        } else {
            println!("  🪟 Press 'w' to generate Windows dev shell");
        }
    }
    
    let t = HOT_RELOAD_TIME.load(Ordering::SeqCst);
    if t > 0 {
        let time_str = format!("{:.2}s", t as f64 / 1000.0);
        println!("\n  ✨ UI Hot Reloaded in \x1B[32;1m{}\x1B[0m", time_str);
    }
    
    println!("{divider}");
    println!("› Press 'c' to contribute to the open-source repo");
    println!("› Press 'f' to share feedback");
    println!("› Press 'r' to compile a production release");
    println!("› Press 'q' to quit\n");
    
    println!("\x1B[35m👀 Watching src/ & config/ for changes...\x1B[0m");
}

fn find_valid_java_home() -> Option<PathBuf> {
    if let Ok(env_java) = std::env::var("JAVA_HOME") {
        let p = PathBuf::from(&env_java);
        if p.exists() && p.join("bin/java").exists() {
            return Some(p);
        }
    }

    let as_jbr_mac = PathBuf::from("/Applications/Android Studio.app/Contents/jbr/Contents/Home");
    if as_jbr_mac.exists() {
        return Some(as_jbr_mac);
    }

    if let Ok(out) = Command::new("/usr/libexec/java_home").output() {
        let path_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !path_str.is_empty() {
            let p = PathBuf::from(path_str);
            if p.exists() {
                return Some(p);
            }
        }
    }

    let linux_jdk = PathBuf::from("/usr/lib/jvm/default-java");
    if linux_jdk.exists() {
        return Some(linux_jdk);
    }

    None
}

fn find_valid_android_home() -> Option<PathBuf> {
    if let Ok(env_sdk) = std::env::var("ANDROID_HOME") {
        let p = PathBuf::from(&env_sdk);
        if p.exists() {
            return Some(p);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let mac_sdk = PathBuf::from(&home).join("Library/Android/sdk");
        if mac_sdk.exists() {
            return Some(mac_sdk);
        }

        let linux_sdk = PathBuf::from(&home).join("Android/Sdk");
        if linux_sdk.exists() {
            return Some(linux_sdk);
        }
    }

    None
}

fn get_adb_path() -> String {
    if let Some(sdk_path) = find_valid_android_home() {
        let adb = sdk_path.join("platform-tools/adb");
        if adb.exists() {
            return adb.to_string_lossy().to_string();
        }
    }
    "adb".to_string()
}

/// Returns the serial ID of an authorized, booted Android device/emulator.
/// Boots one if none is connected. Returns None if unavailable.
fn get_ready_device() -> Option<String> {
    let adb = get_adb_path();

    // Parse `adb devices` output to find the first authorized device
    let find_authorized = |adb: &str| -> Option<String> {
        let out = Command::new(adb).args(["devices"]).output().ok()?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.splitn(2, '\t').collect();
            if parts.len() == 2 && parts[1].trim() == "device" {
                return Some(parts[0].trim().to_string());
            }
        }
        None
    };

    // Fast path: already have a device
    if let Some(serial) = find_authorized(&adb) {
        // Verify it's fully booted (sys.boot_completed = 1)
        if let Ok(out) = Command::new(&adb)
            .args(["-s", &serial, "shell", "getprop", "sys.boot_completed"])
            .output()
        {
            if String::from_utf8_lossy(&out.stdout).trim() == "1" {
                return Some(serial);
            }
        }
        wait_for_boot(&adb, &serial);
        return Some(serial);
    }

    // No device — boot the first AVD we have
    if let Some(sdk_path) = find_valid_android_home() {
        let emulator_bin = sdk_path.join("emulator/emulator");
        if emulator_bin.exists() {
            if let Ok(avd_out) = Command::new(&emulator_bin).args(["-list-avds"]).output() {
                let avds = String::from_utf8_lossy(&avd_out.stdout);
                let avd_name = avds.lines()
                    .map(|s| s.trim())
                    .find(|s| !s.is_empty() && !s.contains("INFO") && !s.contains("ERROR") && !s.contains(' '));
                    
                if let Some(avd_name) = avd_name {
                    let _ = Command::new(&emulator_bin).args(["-avd", avd_name]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn();

                    // Wait for an authorized device to appear (up to 60s)
                    for _ in 0..120 {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            if let Some(serial) = find_authorized(&adb) {
                                wait_for_boot(&adb, &serial);
                                return Some(serial);
                            }
                        }
                }
            }
        }
    }

    None
}

/// Polls sys.boot_completed on a specific device until it returns "1".
fn wait_for_boot(adb: &str, serial: &str) {
    for _ in 0..120 {
        if let Ok(out) = Command::new(adb)
            .args(["-s", serial, "shell", "getprop", "sys.boot_completed"])
            .output()
        {
            if String::from_utf8_lossy(&out.stdout).trim() == "1" {
                return;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

pub fn build_and_launch_ios() {

    let start_time = std::time::Instant::now();

    let sim_check = Command::new("xcrun")
        .args(["simctl", "list", "devices", "booted"])
        .output();

    let mut is_booted = false;
    if let Ok(out) = sim_check {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.contains("Booted") {
            is_booted = true;
        }
    }

    if !is_booted {
        let _ = Command::new("open").args(["-a", "Simulator"]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
    }

    let is_building = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let is_building_clone = is_building.clone();
    std::thread::spawn(move || {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut i = 0;
        while is_building_clone.load(std::sync::atomic::Ordering::SeqCst) {
            print!("\r{} [iOS] Compiling native app (xcodebuild)...", frames[i % frames.len()]);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            i += 1;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
        print!("\r\x1B[K");
    });

    let project_arg = crate::project::project_path_str("platforms/iOS/cargo_native_app.xcodeproj");
    let app_path = "target/ios-build/Build/Products/Debug-iphonesimulator/cargo_native_app.app";
    let bundle_id = "com.nativeui.cargo-native-app";

    crate::generate_all_projects();

    let status = Command::new("xcodebuild")
        .args([
            "-project", &project_arg,
            "-scheme", "cargo_native_app",
            "-sdk", "iphonesimulator",
            "-configuration", "Debug",
            "-derivedDataPath", "target/ios-build",
            "COMPILER_INDEX_STORE_ENABLE=NO",
            "ONLY_ACTIVE_ARCH=YES",
            "ARCHS=arm64",
            "EXCLUDED_ARCHS=x86_64",
            "SWIFT_COMPILATION_MODE=singlefile",
            "build"
        ])
        .output();
        
    is_building.store(false, std::sync::atomic::Ordering::SeqCst);
    std::thread::sleep(std::time::Duration::from_millis(100));

    if let Ok(out) = status {
        if out.status.success() {
            let _ = Command::new("xcrun").args(["simctl", "terminate", "booted", &bundle_id]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
            let _ = Command::new("xcrun").args(["simctl", "install", "booted", &app_path]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
            let _ = Command::new("xcrun").args(["simctl", "launch", "booted", &bundle_id]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();
            
            let _ = Command::new("stty").args(["-raw", "echo"]).status();
            IOS_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
            clear_and_print_header();
            
            let _ = Command::new("stty").args(["raw", "-echo"]).status();
        } else {
            let _ = Command::new("stty").args(["-raw", "echo"]).status();
            let combined = format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
            let _ = std::fs::write("ios_xcodebuild.log", &combined);
            eprintln!("\r❌ [iOS] xcodebuild failed! Error details:");
            for line in combined.lines().filter(|l| l.contains("error:") || l.contains("fatal") || l.contains("ld:")) {
                eprintln!("   {}", line);
            }
            eprintln!("--- Last 10 lines of xcodebuild (full log in ios_xcodebuild.log) ---");
            for line in combined.lines().rev().take(10).collect::<Vec<_>>().into_iter().rev() {
                eprintln!("   {}", line);
            }
            let _ = Command::new("stty").args(["raw", "-echo"]).status();
        }
    }
}

pub fn build_and_launch_android() {

    let start_time = std::time::Instant::now();

    crate::generate_all_projects();

    // Get the authorized, booted device serial — boot one if needed
    let serial = match get_ready_device() {
        Some(s) => s,
        None => {
            eprintln!("❌ [Android] No authorized device found. Please open an emulator and try again.");
            return;
        }
    };

    let android_dir = crate::project::project_path("platforms/Android");
    let gradlew_path = android_dir.join("gradlew");
    if gradlew_path.exists() {
        let _ = Command::new("chmod").arg("+x").arg(&gradlew_path).status();
    }

    let mut cmd = Command::new("sh");
    // Target the specific authorized device to avoid "more than one device" failures
    cmd.args([
        "./gradlew", "installDebug",
        "--parallel", "--build-cache", "--configure-on-demand",
        &format!("-Pandroid.serialNumber={serial}"),
    ])
    .current_dir(&android_dir);

    if let Some(java_home) = find_valid_java_home() {
        cmd.env("JAVA_HOME", java_home);
    }
    if let Some(android_home) = find_valid_android_home() {
        cmd.env("ANDROID_HOME", android_home);
    }

    let is_building = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let is_building_clone = is_building.clone();
    std::thread::spawn(move || {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut i = 0;
        while is_building_clone.load(std::sync::atomic::Ordering::SeqCst) {
            print!("\r{} [Android] Compiling native app (gradlew)...", frames[i % frames.len()]);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            i += 1;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
        print!("\r\x1B[K");
    });

    cmd.stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    let cmd_status = cmd.status();
    
    is_building.store(false, std::sync::atomic::Ordering::SeqCst);
    std::thread::sleep(std::time::Duration::from_millis(100));

    if let Ok(s) = cmd_status {
        if s.success() {
            let adb = get_adb_path();
            // Reset and configure port forwarding so the emulator can reach the dev server
            let _ = Command::new(&adb).args(["-s", &serial, "reverse", "--remove-all"]).status();
            let _ = Command::new(&adb).args(["-s", &serial, "reverse", "tcp:5824", "tcp:5824"]).status();
            // Force-stop and launch the app fresh
            let _ = Command::new(&adb)
                .args(["-s", &serial, "shell", "am", "start", "-S", "-n", "com.nativeui.cargonativeapp/.MainActivity"])
                .status();
                
            let _ = Command::new("stty").args(["-raw", "echo"]).status();
            ANDROID_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
            clear_and_print_header();
            
            let _ = Command::new("stty").args(["raw", "-echo"]).status();
        } else {
            let _ = Command::new("stty").args(["-raw", "echo"]).status();
            eprintln!("\r❌ [Android] gradlew installDebug failed. Try running it manually to see errors.\n");
            
            let _ = Command::new("stty").args(["raw", "-echo"]).status();
        }
    } else {
        let _ = Command::new("stty").args(["-raw", "echo"]).status();
        eprintln!("\r❌ [Android] Failed to execute ./gradlew.\n");
        
        let _ = Command::new("stty").args(["raw", "-echo"]).status();
    }
}

pub fn build_and_launch_linux() {
    let start_time = std::time::Instant::now();
    let is_building = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let is_building_clone = is_building.clone();
    
    std::thread::spawn(move || {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut i = 0;
        while is_building_clone.load(std::sync::atomic::Ordering::SeqCst) {
            print!("\r{} [Linux] Generating GTK4 Dev Shell...", frames[i % frames.len()]);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            i += 1;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
        print!("\r\x1B[K");
    });

    generate_gtk_dev_shell();
    crate::generate_all_projects();

    is_building.store(false, std::sync::atomic::Ordering::SeqCst);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let _ = Command::new("stty").args(["-raw", "echo"]).status();
    
    let pkg_check = Command::new("pkg-config")
        .args(["--exists", "libadwaita-1"])
        .status();
    let has_adw = pkg_check.map_or(false, |s| s.success());
    let linux_dir = crate::project::project_path("platforms/Linux");

    if has_adw {
        clear_and_print_header();

        let mut app_name = "CargoNativeApp".to_string();
        if let Ok(app_json_str) = std::fs::read_to_string(crate::project::project_path("config/app.json")) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
                if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                    app_name = name.replace(" ", "");
                }
            }
        }

        if let Ok(mut lock) = LINUX_CHILD.lock() {
            if let Some(mut old) = lock.take() {
                let _ = old.kill();
                let _ = old.wait();
            }
        }
        let _ = Command::new("killall").args(["-9", &app_name]).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).status();

        let _ = Command::new("cargo")
            .args(["build"])
            .env("CARGO_TARGET_DIR", std::env::current_dir().unwrap().join("target/linux-build"))
            .current_dir(&linux_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
            
        LINUX_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
        clear_and_print_header();
            
        let bin_rel = std::env::current_dir().unwrap().join("target/linux-build/debug").join(&app_name);
        let child = Command::new(&bin_rel)
            .current_dir(&linux_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();

        if let Ok(c) = child {
            if let Ok(mut lock) = LINUX_CHILD.lock() {
                *lock = Some(c);
            }
        }
    } else if std::env::consts::OS == "linux" {
        clear_and_print_header();
        eprintln!("\r\x1B[31m❌ Missing GTK4 / Libadwaita development libraries!\x1B[0m\n");
        eprintln!("To compile the Linux GTK4 shell natively, install the required C packages:\n");
        eprintln!("  • Ubuntu / Debian:  sudo apt install libadwaita-1-dev libgtk-4-dev pkg-config");
        eprintln!("  • Fedora / RHEL:    sudo dnf install libadwaita-devel gtk4-devel pkgconf-pkg-config");
        eprintln!("  • Arch Linux:       sudo pacman -S libadwaita gtk4 pkgconf\n");
    } else {
        LINUX_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
        clear_and_print_header();
        eprintln!("\r✨ [Linux] Generated Dev Shell! Run `cargo run` inside `{}` in your Linux VM to connect to Server 1 on your Mac!\n", linux_dir.display());
    }
    
    let _ = Command::new("stty").args(["raw", "-echo"]).status();
}

pub fn generate_gtk_dev_shell() {
    let linux_dir = crate::project::project_path("platforms/Linux");
    let gtk_dev = std::fs::read_to_string(linux_dir.join("base_main.rs")).unwrap_or_default();

    let mac_ip = get_local_ip();
    let candidates_str = format!(
        r#""http://{}:5824", "http://192.168.64.1:5824", "http://10.0.2.2:5824", "http://10.211.55.2:5824", "http://127.0.0.1:5824""#,
        mac_ip
    );

    let mut app_title = "NativeKit".to_string();
    let mut app_name = "CargoNativeApp".to_string();
    let mut app_name_lower = "cargo_native_app".to_string();
    if let Ok(app_json_str) = std::fs::read_to_string(crate::project::project_path("config/app.json")) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&app_json_str) {
            if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                app_title = name.to_string();
                app_name = name.replace(" ", "");
                app_name_lower = app_name.to_lowercase();
            }
        }
    }

    let crate_name = crate::project::project_crate_name();
    let gtk_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{}"
path = "src/main.rs"

[workspace]

[dependencies]
libadwaita = {{ version = "0.6", features = ["v1_5"] }}
gtk4 = {{ version = "0.8", package = "gtk4", features = ["v4_8"] }}
serde_json = "1.0"
base64 = "0.22"
reqwest = {{ version = "0.12", default-features = false, features = ["json", "blocking", "rustls-tls-webpki-roots"] }}
{crate_name} = {{ path = "../../", optional = true, features = ["client"] }}

[features]
prod = ["dep:{crate_name}"]
"#, app_name_lower, app_name);

    let mut gtk_dev = gtk_dev.to_string();
    gtk_dev = gtk_dev.replace("/* CANDIDATE_URLS_HERE */", &candidates_str);
    gtk_dev = gtk_dev.replace("/* APP_ID_HERE */", &format!("com.nativeui.{}", app_name_lower));
    gtk_dev = gtk_dev.replace("/* APP_TITLE_HERE */", &format!("{} (DEV)", app_title));

    let gen_rs = format!(
        "// ⚠️ GENERATED BY NATIVEKIT - DO NOT EDIT\nuse gtk4::prelude::*;
use libadwaita::prelude::*;\nuse gtk4::Widget;\nuse crate::{{DynamicUINode, parse_dynamic_widget}};\n\n{}\n\npub fn parse_generated_primitive(node: &DynamicUINode, dispatch: &std::rc::Rc<dyn Fn(String, Option<serde_json::Value>)>) -> Widget {{\n    match node.name.as_str() {{\n{}\n        _ => gtk4::Box::new(gtk4::Orientation::Vertical, 0).upcast::<Widget>(),\n    }}\n}}\n",
        native_kit::COLOR_GTK_HELPERS,
        crate::gather_primitives("rs")
    );

    let _ = std::fs::create_dir_all(linux_dir.join("src"));
    let _ = std::fs::write(linux_dir.join("src/generated_primitives.rs"), gen_rs);
    let _ = std::fs::write(linux_dir.join("src/main.rs"), gtk_dev);
    let _ = std::fs::write(linux_dir.join("Cargo.toml"), gtk_toml);
    if !linux_dir.join("theme.css").exists() { let _ = std::fs::write(linux_dir.join("theme.css"), ""); }

    let icon_candidates = [
        crate::project::project_path("assets/logo/icon.png"),
        crate::project::project_path("assets/logo/logo.png"),
        crate::project::project_path("config/icon.png"),
    ];
    if let Some(src) = icon_candidates.iter().find(|p| p.exists()) {
        let _ = std::fs::copy(src, linux_dir.join("icon.png"));
    }

    let launcher_sh = format!(r#"#!/bin/bash
set -e
cd "$(dirname "$0")"
export GSK_RENDERER=cairo
export CARGO_TARGET_DIR="../../../target/linux-build"
export G_ENABLE_DIAGNOSTIC=0
if [ -f "$CARGO_TARGET_DIR/release/{0}" ]; then
    exec "$CARGO_TARGET_DIR/release/{0}" "$@"
else
    cargo build --release --features prod
    exec "$CARGO_TARGET_DIR/release/{0}" "$@"
fi
"#, app_name);

    let launcher_path = linux_dir.join(&app_name);
    let _ = std::fs::write(&launcher_path, &launcher_sh);
    let _ = std::process::Command::new("chmod").arg("+x").arg(&launcher_path).status();

    // Clean up deprecated wrapper scripts
    let _ = std::fs::remove_file(linux_dir.join("run-linux-dev.sh"));
    let _ = std::fs::remove_file(linux_dir.join("build-linux-app.sh"));
    let _ = std::fs::remove_file(linux_dir.join("run-linux.sh"));
    let _ = std::fs::remove_file(linux_dir.join("Build Linux App.sh"));
}


pub fn build_and_launch_windows() {
    let start_time = std::time::Instant::now();
    let is_building = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let is_building_clone = is_building.clone();
    
    std::thread::spawn(move || {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut i = 0;
        while is_building_clone.load(std::sync::atomic::Ordering::SeqCst) {
            print!("\r{} [Windows] Generating WPF Shell...", frames[i % frames.len()]);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            i += 1;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
        print!("\r\x1B[K");
    });

    let mac_ip = get_local_ip();
    let _server_url = if std::env::consts::OS == "windows" {
        "http://127.0.0.1:5824".to_string()
    } else {
        format!("http://{}:5824", mac_ip)
    };

    crate::generate_windows_project();
    let win_dir = crate::project::project_path("platforms/Windows");
    
    is_building.store(false, std::sync::atomic::Ordering::SeqCst);
    let _ = Command::new("stty").args(["-raw", "echo"]).status();
    
    if std::env::consts::OS == "windows" {
        clear_and_print_header();
        
        let _ = Command::new("dotnet")
            .args(["build"])
            .current_dir(&win_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
            
        WINDOWS_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
        clear_and_print_header();
            
        let _ = Command::new("dotnet")
            .args(["run"])
            .current_dir(&win_dir)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
    } else {
        WINDOWS_COMPILE_TIME.store(start_time.elapsed().as_millis() as u64, std::sync::atomic::Ordering::SeqCst);
        clear_and_print_header();
        eprintln!("\r✨ [Windows] Generated Dev Shell! Because you are on Mac, please open your Windows VM, navigate to `{}`, and double-click `Run Windows Application.bat`!\n", win_dir.display());
    }
    
    let _ = Command::new("stty").args(["raw", "-echo"]).status();
}
