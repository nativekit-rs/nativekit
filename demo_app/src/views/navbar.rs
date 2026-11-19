#[component]
pub fn Navbar() {
    Return! {
        HStack.spacing(16) {
            Text("Settings Header")
            Button("Save").variant(.text)
        }
    }
}
