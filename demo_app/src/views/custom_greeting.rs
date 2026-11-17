use native_kit::engine::*;
use native_kit::*;

pub fn custom_greeting(name: &str) -> impl IntoNode {
  Return! {
    VStack.spacing(4) {
      Text("Reusable Component!").font(.title1)
      Text("Hello, {}", name)
    }
  }
}
