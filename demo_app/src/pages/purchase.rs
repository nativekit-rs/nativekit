
pub fn PurchaseModal() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("Purchase Sheet") {
      VStack.spacing(16) {
        Text("Confirm Purchase").font(.title1)
        Text("Complete your order for item #42?")
        Text("Current count: {}", store.count)

        HStack.spacing(8) {
          Button("Cancel")
            .variant(.secondary)
            .action(|| {
              dismiss();
            })

          Button("Purchase Now")
            .variant(.primary)
            .action(|store: &mut Store| {
              store.count += 10;
              dismiss();
              navigate("/purchase/confirmed");
            })
        }
      }
    }
  }
}

pub fn PurchaseConfirmedScreen() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("Order Confirmed").on_back(|| navigate("/")) {
      VStack.spacing(16) {
        Text("Order Confirmed!").font(.title1)
        Text("Thank you for your purchase of item #42.")
        Text("Current count: {}", store.count)

        Button("Back to Home")
          .variant(.primary)
          .action(|| navigate("/"))
      }
    }
  }
}
