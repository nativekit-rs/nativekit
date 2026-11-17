// ⚡ Built with NativeKit — https://nativekit.rs
#![allow(non_snake_case)]
use native_kit::*;

#[derive(Clone, Default)]
pub struct Store {
    pub count: i32,
}

pub type AppState = Store;

pub fn App() -> impl IntoNode {
    Router!({
        Route!("/", HomeScreen()),
    })
}

pub fn HomeScreen() -> impl IntoNode {
    let state = use_state::<Store>();
    let count = state.count;

    Return! {
        Page.title("Fullstack OTA").large_title(true).top_style(.soft) {
            Section.header("Local Native UI") {
                HStack.align(.space_between) {
                    Text("Local Counter: {}", count).font(.headline)
                    Button("Increment")
                        .variant(.primary)
                        .action(|s: &mut Store| s.count += 1)
                }
            }

            Section.header("ServerFunctions (src/server.rs)") {
                VStack.spacing(8) {
                    Text("Functions marked with #[server] in src/server.rs are automatically exposed at /api/<fn_name>.")
                        .font(.subhead)
                        .color(.text_secondary)
                }
            }
        }
    }
}
