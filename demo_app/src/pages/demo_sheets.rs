
pub fn Form1Screen() -> impl IntoNode {
  Return! {
    Page.title("Form 1") {
      VStack.spacing(16) {
        Text("Form 1 (Dialog)").font(.title2).emphasized()
        Text("This is a Form (Full-screen dialog on Android, Large sheet on iOS).")

        Button("1. Open Sheet (Bottom Sheet) 📄")
          .variant(.primary)
          .action(|| navigate("/form1/sheet"))

        Button("2. Drill In to Form 1 Detail ➡️")
          .variant(.secondary)
          .action(|| navigate("/form1/detail"))

        Button("3. Open Form 2 (On Top of Form 1) 🚀")
          .variant(.primary)
          .action(|| navigate("/form2"))

        Button("Dismiss Form 1 ✖️")
          .variant(.secondary)
          .action(|| dismiss())
      }
    }
  }
}

pub fn Form1SheetScreen() -> impl IntoNode {
  Return! {
    Page.title("Sheet on Form 1") {
      VStack.spacing(16) {
        Text("Sheet on Form 1 (Bottom Sheet)").font(.title2).emphasized()
        Text("This is a resizable bottom sheet presented on top of Form 1!")

        Button("Close Sheet ✖️")
          .variant(.primary)
          .action(|| dismiss())
      }
    }
  }
}

pub fn Form1DetailScreen() -> impl IntoNode {
  Return! {
    Page.title("Form 1 Detail") {
      VStack.spacing(16) {
        Text("Form 1 — Detail").font(.title3).emphasized()
        Text("You have drilled deeper inside Form 1's navigation stack.")

        Button("Drill Out (Back to Form 1 Root) ⬅️")
          .variant(.secondary)
          .action(|| navigate(-1))

        Button("Open Form 2 (On Top of Form 1) 🚀")
          .variant(.primary)
          .action(|| navigate("/form2"))

        Button("Dismiss All ✖️")
          .variant(.secondary)
          .action(|| dismiss())
      }
    }
  }
}

pub fn Form2Screen() -> impl IntoNode {
  Return! {
    Page.title("Form 2") {
      VStack.spacing(16) {
        Text("Form 2 (Nested Dialog)").font(.title2).emphasized()
        Text("This is Form 2, opened on top of Form 1.")

        Button("1. Drill In to Form 2 Detail ➡️")
          .variant(.primary)
          .action(|| navigate("/form2/detail"))

        Button("2. Open Sheet (On Top of Form 2) 📄")
          .variant(.secondary)
          .action(|| navigate("/form2/sheet"))

        Button("Dismiss Form 2 ✖️")
          .variant(.secondary)
          .action(|| dismiss())

        Button("Pop All to Home 🏠")
          .variant(.secondary)
          .action(|| navigate("/"))
      }
    }
  }
}

pub fn Form2DetailScreen() -> impl IntoNode {
  Return! {
    Page.title("Form 2 Detail") {
      VStack.spacing(16) {
        Text("Form 2 — Detail").font(.title2).emphasized()
        Text("You drilled inside Form 2's navigation stack!")

        Button("Open Sheet (On Top of Form 2) 📄")
          .variant(.primary)
          .action(|| navigate("/form2/sheet"))

        Button("Back to Form 2 Root ⬅️")
          .variant(.secondary)
          .action(|| navigate(-1))

        Button("Dismiss Form 2 ✖️")
          .variant(.secondary)
          .action(|| dismiss())

        Button("Pop All to Home 🏠")
          .variant(.secondary)
          .action(|| navigate("/"))
      }
    }
  }
}

pub fn Form2SheetScreen() -> impl IntoNode {
  Return! {
    Page.title("Sheet on Form 2") {
      VStack.spacing(16) {
        Text("Sheet on Form 2 (Bottom Sheet)").font(.title2).emphasized()
        Text("This is a resizable bottom sheet presented on top of Form 2!")

        Button("Close Sheet ✖️")
          .variant(.primary)
          .action(|| dismiss())

        Button("Pop All to Home 🏠")
          .variant(.secondary)
          .action(|| navigate("/"))
      }
    }
  }
}
