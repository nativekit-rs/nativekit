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
        Page.title("Welcome").large_title(true).top_style(.soft) {
            Section {
                VStack.spacing(12) {
                    Text("NativeKit Starter").font(.title3).emphasized()
                    Text("Edit src/app.rs and save to hot-reload across all 5 native platforms.")
                        .font(.subhead)
                        .color(.text_secondary)
                }
            }

            Section {
                HStack.align(.space_between) {
                    VStack.spacing(4) {
                        Text("Counter").font(.headline)
                        Text("Current value: {}", count).font(.caption1).color(.text_secondary)
                    }
                    HStack.spacing(8) {
                        Button("-").variant(.secondary).action(|s: &mut Store| s.count -= 1)
                        Button("+").variant(.primary).action(|s: &mut Store| s.count += 1)
                    }
                }
            }
        }
    }
}
