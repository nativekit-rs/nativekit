use native_kit::engine::*;
use native_kit::*;

pub fn navbar() -> impl IntoNode {
  Return! {
    HStack.spacing(16) {
      Text("Settings Header")
      Button("Save").variant(.text)
    }
  }
}
