#![allow(non_snake_case)]
use native_kit::*;

pub fn OrdersScreen() -> impl IntoNode {
  let firstOrders = [
    (1, "Mechanical Keyboard", 120),
    (2, "4K UltraWide Monitor", 450),
    (3, "USB-C Thunderbolt 4 Dock", 80),
    (4, "Ergonomic Mesh Chair", 350),
    (5, "Wireless Studio Headphones", 220),
  ];

  let secondOrders = [
    (6, "Aluminum Monitor Arm", 95),
    (7, "Precision Trackpad", 130),
    (8, "USB Condenser Microphone", 110),
    (9, "Walnut Desk Shelf", 85),
    (10, "Braided Cable Organizer", 25),
    (11, "Keycap Custom Set", 45),
    (12, "Desk Mat Leather", 35),
    (13, "Magnetic MagSafe Stand", 40),
    (14, "100W GaN Travel Charger", 65),
    (15, "Smart LED Light Bar", 55),
  ];

  Return! {
    Page.title("Orders").top_style(.hard).section_spacing(24) {
      Text("Scroll down to see the hard navbar remain solid with content passing beneath:")
        .font(.subhead)
        .color(.text_secondary)

      Section
        .header(
          HStack{
            Text("Header")
            Text("subtext").font(.caption1)
          }
        ).prominent()
      {
        HStack{
          Text("Row")
          Button("Buy").variant("primary")
        }
        Text("Regular row")
      }

      Section.header("Your Orders") {
        firstOrders.into_iter().map(|(id, item, price)| {
          Button("📦 Order #{} - {} (${})", id, item, price)
            .action(move || navigate(format!("/orders/{}", id)))
        }).collect::<Vec<_>>()
      }

      Section.header("Past Orders") {
        secondOrders.into_iter().map(|(id, item, price)| {
          Button("📦 Order #{} - {} (${})", id, item, price)
            .variant(.secondary)
            .action(move || navigate(format!("/orders/{}", id)))
        }).collect::<Vec<_>>()
      }
    }
  }
}

pub fn OrderDetailScreen(order_id: &str) -> impl IntoNode {
  let id_str = order_id.to_string();
  Return! {
    Page.title(format!("Order #{}", id_str)) {
      VStack.spacing(16) {
        Text("Order #{}", id_str).font(.title1).emphasized()
        Text("Status: Shipped via Express").color(.text_secondary)

        Text("Tracking: NK-892348-US").font(.footnote)

        Picker(["Standard ($5)", "Express Overnight ($15)", "Same Day Courier ($25)"])
          .title("Change Shipping")
          .selected("Express Overnight ($15)")
          .style(.drill_in)

        Button("Back to Orders")
          .variant(.primary)
          .action(|| navigate(-1))
      }
    }
  }
}
