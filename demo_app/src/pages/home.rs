#![allow(non_snake_case)]
use crate::app::Store;
use native_kit::*;

pub fn HomeScreen() -> impl IntoNode {
  let store = Store::current();

  Return! {
    Page
      .title("Enjoy a break, James")
      .large_title(true)
      .top_style(.soft)
    {

      Section {
        HStack.background(.blue) {
          VStack {
            Text("Stars").font(.caption1).color(.white).opacity(0.8)
            HStack {
              Text("147").font(.title3).emphasized().color(.white)
              Icon(.star_fill).color(.white)
            }
          }
          Button("View Rewards")
            .color(.white)
        }
      }

      Grid.spacing(14).columns(1) {
        // Image("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=900&q=80")
        //     .height(160)
        //     .corner_radius(14)

        VStack
          .spacing(6)
          .padding(12)
          .corner_radius(12)
          .background(.surface_secondary) {
            Text("Live TextInput Component")
              .font(.caption1)
              .color(.text_secondary)
              .emphasized()
            TextInput(store.search_text.clone())
              .placeholder("Type here to sync across all 5 platforms...")
              .on_change(|store: &mut Store, val: String| {
                store.search_text = val;
              })
            Text("You typed: {}", if store.search_text.is_empty() { "(nothing yet)" } else { &store.search_text })
              .font(.footnote)
              .color(.primary)
          }

        Text("heyyyyo").font(.title2)
        HStack.spacing(8) {
          Icon(.sparkles).color(.yellow).size(26)
          Text("Welcome to NativeKit Router!")
            .font(.title1)
            .emphasized()
        }

        VStack
          .spacing(6)
          .padding(16)
          .corner_radius(14)
          .background(
            Gradient::linear()
              .direction(.to_bottom_right)
              .stop(.primary, 0.0)
              .stop_with_opacity(.secondary, 0.55, 0.95)
          ) {
            HStack.spacing(8) {
              Icon(.brand_spark).color(.white).size(22)
              Text("Unified Colors, Icons & Gradients")
                .font(.headline)
                .color(.white)
            }

            Text("Multi-stop gradient with custom stops, opacity, and direction across all 5 native platforms.")
              .font(.subheadline)
              .color(.white)
              .opacity(0.9)
          }

        HStack
          .spacing(10)
          .padding(12)
          .corner_radius(10)
          .background(.surface_secondary) {
            Icon(.star).color(.orange).size(20)
            Text("Count is: {}", store.count)
              .font(.body)
              .color(.text_primary)
            Icon(.heart).color(.pink).size(20)
            Icon(.check_circle).color(.green).size(20)
          }

        HStack.spacing(8) {
          Button("Increment")
            .variant(.primary)
            .action(|store: &mut Store| store.count += 1)
        }

        Button("Open Sheet")
          .variant(.tertiary)
          .action(|| navigate("/purchase"))
        Text("Localized Navigation (Drill-In Picker)")
          .font(.caption1)
          .color(.text_secondary)
          .emphasized()
        Picker(["Priority Delivery ($15)", "Standard Ground ($5)", "Store Pickup (Free)"])
          .title("Shipping")
          .selected(store.shipping_method.clone())
          .style(.drill_in)
          .on_select(|store: &mut Store, val: &str| {
            store.shipping_method = val.to_string();
          })

        Text("Inline Layout Navigation (No Route Needed)")
          .font(.caption1)
          .color(.text_secondary)
          .emphasized()
        Button("⚡ Open Anonymous Inline View")
          .variant(.secondary)
          .action(|| {
            navigate(Page.title("Custom View") {
              VStack.spacing(12) {
                Text("Inline Custom Screen").font(.title2).emphasized()
                Text("This entire layout was constructed dynamically and passed into navigate(Page)!")
                Button("Close / Go Back")
                  .variant(.primary)
                  .action(|| navigate(-1))
              }
            })
          })

        Text("Tab Jumping & Deep Links")
          .font(.caption1)
          .color(.text_secondary)
          .emphasized()
        HStack.spacing(8) {
          Button("Orders Tab")
            .variant(.secondary)
            .action(|| navigate("/orders"))

          Button("Deep Link Order #2")
            .variant(.primary)
            .action(|| navigate("/orders/2"))

          Button("Settings Tab")
            .variant(.secondary)
            .action(|| navigate("/settings"))
        }

        Button("Demo Flow: Form & Sheet 🚀")
          .variant(.primary)
          .action(|| navigate("/form1"))
      }
    }
  }
}
