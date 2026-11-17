# NativeKit ⚡

**High-Performance 5-Platform Dynamic UI Engine for Rust**  
Write declarative, SwiftUI-style UI in pure Rust and render genuine native widgets across **iOS (SwiftUI)**, **Android (Jetpack Compose)**, **Linux (GTK4 / Libadwaita)**, **Windows (WinUI 3)**, and **Web (DOM / WASM)** with sub-second hot reloading and Over-The-Air (OTA) server components.

🌐 **Website:** [https://nativekit.rs](https://nativekit.rs)  
📦 **Crates.io:** [`native-kit`](https://crates.io/crates/native-kit) • [`nativekit-cli`](https://crates.io/crates/nativekit-cli)

---

## ✨ Why NativeKit?

- **Not a Static Transpiler:** NativeKit is a **Dynamic UI Engine**. Your Rust UI compiles to a compact binary AST evaluated at runtime by lightweight native host shells (`SwiftUI`, `Jetpack Compose`, `GTK4`, `WinUI 3`, and `Web`).
- **SwiftUI-Style Ergonomics in Rust:** Clean leading-dot modifiers (`.color(.primary)`, `.font(.title3)`, `.variant(.primary)`) with full IDE autocomplete generated directly from your `config/` files.
- **Sub-Second `cdylib` Hot Reloading:** Edit `src/` and watch changes hot-swap in memory across simulators, desktop shells, and browsers in milliseconds.
- **Next.js-Style `#[server]` Functions & OTA UI:** Annotate async Rust functions with `#[server]` to stream live Over-The-Air native UI updates without waiting for App Store review.

---

## 🚀 Quick Start

### 1. Install the NativeKit CLI
```bash
cargo install nativekit-cli
```

### 2. Create a New App
Choose from `basic`, `routing` (Tabs & Sheets), or `fullstack` (`#[server]` OTA):
```bash
nk new my-app
cd my-app
```

### 3. Launch the Live Dev Server
```bash
nk dev
```
Inside the interactive terminal:
- Press **`i`** to launch the **iOS Simulator**
- Press **`a`** to launch the **Android Emulator**
- Press **`l`** to launch the **Linux GTK4 Shell**
- Press **`w`** to launch the **Windows WinUI 3 Shell**
- Press **`b`** to open the **Web Shell**

---

## 🧩 Declarative UI Example (`src/app.rs`)

```rust
use native_kit::*;

#[derive(Clone, Default)]
pub struct Store {
    pub count: i32,
}

pub fn HomeScreen() -> impl IntoNode {
    let state = use_state::<Store>();

    Return! {
        Page.title("Dashboard").large_title(true).top_style(.soft) {
            Section {
                HStack.align(.space_between) {
                    VStack.spacing(4) {
                        Text("Revenue Counter").font(.headline)
                        Text("Total: {}", state.count).font(.caption1).color(.text_secondary)
                    }
                    Button("Increment")
                        .variant(.primary)
                        .action(|s: &mut Store| s.count += 1)
                }
            }
        }
    }
}
```

---

## 📄 License & Trademarks

- **Code License:** Distributed under the [MIT License](./LICENSE).
- **Trademarks:** The **NativeKit** name, logos, and **nativekit.rs** brand are protected under our [Trademark Policy](./TRADEMARK.md).
