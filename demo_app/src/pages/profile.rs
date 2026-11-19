
pub fn ProfileScreen(user_id: &str) -> impl IntoNode {
  Return! {
    Page.title(format!("Profile {}", user_id)).on_back(|| navigate(-1)) {
      VStack.spacing(16) {
        Text("User Profile").font(.title1)
        Text("Showing user: {}", user_id)

        Button("Back to Details")
          .variant(.secondary)
          .action(|| navigate(-1))
      }
    }
  }
}
