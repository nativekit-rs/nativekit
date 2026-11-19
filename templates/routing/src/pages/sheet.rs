use native_kit::*;

#[component]
pub fn QuickActionSheet() {
    Return! {
        Page.title("Quick Action") {
            Section {
                VStack.spacing(12) {
                    Text("Native Modal Sheet").font(.title3).emphasized()
                    Text("This route is presented as a native bottom sheet or modal dialog.")
                        .font(.subhead)
                        .color(.text_secondary)
                    Button("Done")
                        .variant(.primary)
                        .width(.fill)
                        .action(|| navigate(-1))
                }
            }
        }
    }
}
