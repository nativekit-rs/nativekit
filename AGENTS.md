# NativeKit Project Rules & Architecture

## Core Principles & Philosophy

1. **Developer Joy & Empowerment ("The React-to-JS Leap")**:
   - **Make coding fun, exciting, and fast**: Eliminate tedious boilerplate so developers feel genuine joy and control.
   - **Massive Leverage**: Empower both Junior and Senior developers to generate massive, production-grade cross-platform outputs with simple, logical keystrokes.
   - **Declarative Superpower**: Just as React revolutionized JavaScript with hooks (`useState`, `useContext`) and component hierarchies, this project gives developers a high-leverage declarative layer over Rust while preserving type safety and raw performance.
2. **Lightweight & Zero Redundancy**: 
   - ZERO redundant logic.
   - Any unused functions, structs, or imports must be eliminated immediately.
3. **Clean, Idiomatic Rust (No Spaghetti Code)**:
   - Zero tolerance for convoluted, boilerplate-heavy, or unmaintainable code.
   - Code must be clean, structured, and ready for open-sourcing.
4. **Low Floor, Absurdly High Ceiling (Full Escape Hatches)**:
   - **Low Floor (Easy to Start)**: Keep the entry point (`app.rs`) welcoming, minimal, and instantly productive so anyone can jump in and build without fighting the language.
   - **Absurdly High Ceiling (Unlimited Power)**: Zero artificial limitations. When an advanced developer wants to do fancy, deep-level engineering, they have full access to drop into raw, high-performance Rust, custom macros, low-level concurrency, or platform-native interop. You get the speed of a framework with the limitless power of native Rust.

---

## Architecture & Workflow

### 1. Dynamic UI Engine (Server-Driven UI & Local Engine)

- **The Paradigm Shift**: NativeKit is *not* a source-to-source transpiler that spits out static Swift/Kotlin files. It is a **High-Performance Dynamic UI Engine**.
- **How it Works**: 
  - The developer writes UI in Rust.
  - Rust evaluates this UI into an AST (Abstract Syntax Tree).
  - The native iOS (SwiftUI), Android (Jetpack Compose), and Linux (GTK4) shells are generic, reusable host applications that interpret this AST at runtime and render it using native components.
- **Over-The-Air (OTA) native**: Because the native shells evaluate the UI dynamically, developers can push instant UI updates, marketing banners, or entire screens from their server without requiring an App Store review.

### 2. Holistic Configuration, Theming & Cloud Vision

- **Configuration Scope**: Manages holistic project settings beyond just colors, including metadata, assets, branding, and server endpoints.
- **Theming Evolution**: Currently, themes are managed via local `config/` files. **Future Vision**: Theming will not live exclusively in local config files. It will integrate with an authenticated, Vercel-like cloud GUI where designers can tweak colors, spacing, and styles visually. Upon hitting "Publish", `cargo run` (or the production app) will fetch the latest styling payload instantly.

### 3. Output & Templates (`dist/`)

- **`dist/` Directory**: Contains the reusable native host apps (SwiftUI, Jetpack Compose, GTK4). 
- **Code Generation**: Instead of transpiling Rust UI to static native code, the build process injects the *parsers* for primitive components into the `DynamicNodeView` engine in the native shells.

---

## Component System & Blueprint Architecture

### 1. SwiftUI-Style Macro Syntax
All declarative UI components follow a SwiftUI-inspired syntax:
```rust
ComponentName(properties, strings, etc) {
    // children components
}
```

### 2. YAML Blueprints & Primitives
- **Scaleable Blueprints**: Primitives (`Button`, `Text`, `HStack`, etc.) are defined using simple YAML blueprints.
- **Community Extensibility**: This architecture allows developers to drop in new YAML files for third-party native SDKs. The Rust compiler automatically generates the Rust struct API and injects the Swift/Kotlin/GTK parser snippets into the dynamic engine.
- Use EXTREME caution before editing the core blueprints. The engine heavily relies on them.

### 3. `app.rs` (UI Entry Point)
- **Role**: The main user-facing entry point for building UI layouts.
- **Philosophy**: Kept clean, minimal, and welcoming by default. Developers compose primitives and custom components.

### 4. `server.rs` (Server Functions & Routing)
- **Role**: Provides Next.js-style fullstack backend integration.
- **Server Functions**: Functions marked as server functions automatically generate type-checked route tunnels, allowing seamless fetching of OTA components via some sort of bit transfer, and NOT JSON.
- **Configuration**:
  - **Development (`rn dev`)**: Route tunnels default to hitting `localhost:5824`.
  - **Production (`rn build`)**: Automatically swaps to the production server endpoint.

---

## Development & Hot Reloading

- **Hot Reload Performance**: Uses HTTP polling in dev mode with bit encoding rather than JSON tunneling to fetch layout changes in **under 2 seconds**.
- **Production Compilation**: In production builds, the dev HTTP server is stripped. The Rust app compiles to a native static library (`.a` / `.so`), and the native shells communicate with Rust directly via C-ABI/FFI boundaries for instant UI evaluation.

---

## AI Agent Directives (CRITICAL)

- **Architecture Awareness**: NEVER assume the framework compiles Rust to static SwiftUI/Kotlin source code. It is a Dynamic UI Engine. Do not generate code expecting AOT translation of Rust closures to Swift closures.
- **UI Syntax**: ALWAYS use the macro syntax `ComponentName(props) { ... }` for declarative UI components. 
- **Strict Cleanup**: NEVER leave unused imports, structs, or functions.
- **Server Functions**: When adding a backend function to `server.rs`, ensure it is explicitly routed according to the Next.js-style routing.
