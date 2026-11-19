use crate::Store;
use native_kit::*;

pub fn DetailsScreen() -> impl IntoNode {
  let store = Store::current();

  Return! {
    Page.title("Details").on_back(|| navigate(-1)) {
      ToolbarTop {
        Button.placement(.leading).action(|| navigate(-1)) {
          Icon(.navigate_back)
        }
        Button("+1")
          .placement(.trailing)
          .variant(.secondary)
          .action(|s: &mut Store| s.count += 1)
      }
      VStack.spacing(16) {
        Text("Detail Screen").font(.title1)
        Text("Notice the custom Leading (Done) and Trailing (+1) buttons in ToolbarTop!")
          .font(.footnote)
          .color(.text_secondary)
        Text("Current count from Home: {}", store.count)
        HStack.spacing(8) {
          Button("Back")
            .variant(.secondary)
            .action(|| navigate(-1))

          Button("Go to Profile #42")
            .variant(.primary)
            .action(|| navigate("/profile/42"))
        }
      }
    }
  }
}
