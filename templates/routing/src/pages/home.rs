use native_kit::*;

#[component]
pub fn HomeScreen() {
    let state = use_state::<Store>();
    let count = state.count;

    Return! {
        Page.title("Home").large_title(true).top_style(.soft) {
            Section {
                VStack.spacing(10) {
                    Text("Welcome to NativeKit").font(.title3).emphasized()
                    Text("Multi-tab navigation with native sheets across iOS, Android, Linux, Windows, and Web.")
                        .font(.subhead)
                        .color(.text_secondary)
                    Button("Open Modal Sheet")
                        .variant(.primary)
                        .action(|| navigate("/quick-sheet"))
                }
            }

            Section.header("State Demo") {
                HStack.align(.space_between) {
                    Text("Count: {}", count).font(.headline)
                    HStack.spacing(8) {
                        Button("-").variant(.secondary).action(|s: &mut Store| s.count -= 1)
                        Button("+").variant(.primary).action(|s: &mut Store| s.count += 1)
                    }
                }
            }
        }
    }
}
