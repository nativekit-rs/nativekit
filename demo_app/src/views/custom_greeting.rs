#[component(name: string = "Friend")]
pub fn CustomGreeting() {
    Return! {
        VStack.spacing(4) {
            Text("Reusable Component!").font(.title1)
            Text("Hello, {}", name)
        }
    }
}
