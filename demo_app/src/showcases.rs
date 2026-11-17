#![allow(non_snake_case)]
use crate::app::Store;
use native_kit::*;

pub fn app_switcher_card() -> impl IntoNode {
  let store = Store::current();
  let theme = Theme::current();
  Return! {
    VStack
      .spacing(10)
      .padding(14)
      .corner_radius(12)
      .background(.surface_secondary) {
      Text("White-Label Demo Switcher").font(.headline).emphasized()
      Picker(["NovaPay (FinTech)", "Kona Craft (Commerce)", "Pulse AI (SaaS)", "Kitchen Sink (Original)"])
        .title("Active App")
        .selected(store.active_showcase.clone())
        .on_select(|s: &mut Store, val: &str| {
          s.active_showcase = val.to_string();
          navigate("/");
        })
      HStack.spacing(12) {
        Text("Dark Mode").font(.body)
        Switch().value(theme.is_dark()).on_change(Theme::toggle_dark)
      }
      HStack.spacing(12) {
        Text("Use OS System Accent").font(.body)
        Switch().value(theme.is_system_accent()).on_change(Theme::toggle_system_accent)
      }
    }
  }
}

pub fn NovaPayHome() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("NovaPay").large_title(true) {
      ToolbarTop {
        Button.placement(.leading).action(|| navigate("/send")) {
          Icon(.bell).size(18)
        }
        Button("+ Send").placement(.trailing).variant(.primary).action(|| navigate("/send"))
      }
      VStack.spacing(14) {
        VStack
          .spacing(8)
          .padding(20)
          .corner_radius(16)
          .background(Gradient::linear().direction(.to_bottom_right).stop(.primary, 0.0).stop(.secondary, 1.0)) {
          HStack.spacing(6) {
            Icon(.sparkles).color(.white).size(20)
            Text("NOVAPAY PLATINUM").font(.caption1).color(.white).emphasized()
          }
          Text("$42,890.50").font(.large_title).color(.white).emphasized()
          Text("+ $1,420.00 (3.4%) this month  •  •••• 8492").font(.footnote).color(.white).opacity(0.9)
        }

        HStack.spacing(10) {
          Button.variant(.primary).width(.fill).action(|| navigate("/send")) {
            Icon(.bolt).size(18)
            Text("Send")
          }
          Button.variant(.secondary).width(.fill).action(|s: &mut Store| s.count += 100) {
            Icon(.plus).size(18)
            Text("Deposit")
          }
        }

        VStack
          .spacing(8)
          .padding(14)
          .corner_radius(12)
          .background(.surface_secondary) {
          Text("Quick Transfer").font(.subhead).emphasized()
          TextInput(store.search_text.clone())
            .placeholder("@username, ENS, or IBAN...")
            .on_change(|s: &mut Store, v: String| s.search_text = v)
          Picker(["USDC (Solana) • $12,400", "USD Vault (4.8% APY)", "Bitcoin (BTC) • 0.42 BTC"])
            .title("Pay From")
            .selected(store.shipping_method.clone())
            .on_select(|s: &mut Store, v: &str| s.shipping_method = v.to_string())
        }

        Text("Recent Activity").font(.headline).emphasized()
        Button.variant(.secondary).width(.fill).action(|| navigate("/orders/8841")) {
          HStack.spacing(12) {
            Icon(.check_circle).color(.green).size(22)
            VStack.spacing(2) {
              Text("Stripe Payout").font(.body).emphasized()
              Text("Today, 2:14 PM  •  + $3,250.00").font(.caption1).color(.text_secondary)
            }
          }
        }

        Button.variant(.secondary).width(.fill).action(|| navigate("/orders/8842")) {
          HStack.spacing(12) {
            Icon(.bolt).color(.orange).size(22)
            VStack.spacing(2) {
              Text("AWS Cloud Services").font(.body).emphasized()
              Text("Yesterday  •  - $284.50").font(.caption1).color(.text_secondary)
            }
          }
        }
      }
    }
  }
}

pub fn NovaPaySendSheet() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("Send Money") {
      VStack.spacing(16) {
        Text("Instant Transfer").font(.title2).emphasized()
        Text("Sending to: {}", if store.search_text.is_empty() { "@alex.eth" } else { &store.search_text })
          .font(.subhead).color(.primary)
        Text("Zero network fees • Arrives in < 1 second").font(.footnote).color(.text_secondary)
        HStack.spacing(10) {
          Button("Cancel").variant(.secondary).width(.fill).action(dismiss)
          Button("Confirm $250.00").variant(.primary).width(.fill).action(|s: &mut Store| {
            s.count += 250;
            dismiss();
          })
        }
      }
    }
  }
}

pub fn KonaCraftHome() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("Kona Craft").large_title(true) {
      ToolbarTop {
        Button
          .placement(.trailing)
          .variant(.primary)
          .action(|| navigate("/purchase")) {
          Icon(.cart).size(18)
          Text("Bag ({})", store.count)
        }
      }
      VStack.spacing(14) {
        Image("https://images.unsplash.com/photo-1495474472287-4d71bcdd2085?w=900&q=80")
          .height(175)
          .corner_radius(16)

        TextInput(store.search_text.clone())
          .placeholder("Search single-origin pour overs, matcha, pastries...")
          .on_change(|s: &mut Store, v: String| s.search_text = v)

        VStack
          .spacing(4)
          .padding(14)
          .corner_radius(14)
          .background(Gradient::linear().direction(.to_right).stop(.primary, 0.0).stop(.secondary, 1.0)) {
          HStack.spacing(6) {
            Icon(.sparkles).color(.white).size(20)
            Text("ROASTER'S RESERVE").font(.caption1).color(.white).emphasized()
          }
          Text("2x Loyalty Stars on Honey Process Geisha").font(.headline).color(.white)
          Text("Small-batch roasted in Kona, Hawaii • Free local delivery").font(.caption1).color(.white).opacity(0.9)
        }

        Text("Featured Menu").font(.headline).emphasized()

        VStack
          .spacing(10)
          .padding(14)
          .corner_radius(14)
          .background(.surface_secondary) {
          Image("https://images.unsplash.com/photo-1509042239860-f550ce710b93?w=800&q=80")
            .height(130)
            .corner_radius(10)
          HStack {
            VStack.spacing(2) {
              Text("Kona Peaberry Cortado").font(.headline)
              Text("Velvety micro-foam, dark cacao notes • $6.50").font(.footnote).color(.text_secondary)
            }
          }
          Picker(["Oat Milk (+ $0.75)", "Whole Organic Milk", "Almond Milk"])
            .title("Milk Choice")
            .selected(store.shipping_method.clone())
            .on_select(|s: &mut Store, v: &str| s.shipping_method = v.to_string())
          HStack.spacing(8) {
            Button("+ Add to Bag").variant(.primary).width(.fill).action(|s: &mut Store| s.count += 1)
            Button("Checkout").variant(.secondary).width(.fill).action(|| navigate("/purchase"))
          }
        }
      }
    }
  }
}

pub fn PulseAIHome() -> impl IntoNode {
  let store = Store::current();
  Return! {
    Page.title("Pulse AI").large_title(true) {
      ToolbarTop {
        Button("Deploy Agent").placement(.trailing).variant(.primary).action(|| navigate("/form1"))
      }
      VStack.spacing(14) {
        VStack
          .spacing(6)
          .padding(16)
          .corner_radius(14)
          .background(Gradient::linear().direction(.to_bottom_right).stop(.primary, 0.0).stop(.secondary, 1.0)) {
          HStack.spacing(6) {
            Icon(.bolt).color(.white).size(20)
            Text("PULSE AI CLUSTER • 99.99% ONLINE").font(.caption1).color(.white).emphasized()
          }
          Text("Autonomous Agent Studio").font(.title2).color(.white).emphasized()
          Text("328 tokens/sec  •  12 active workflows  •  Region: us-east").font(.footnote).color(.white).opacity(0.9)
        }

        VStack
          .spacing(10)
          .padding(14)
          .corner_radius(12)
          .background(.surface_secondary) {
          Text("Prompt Playground").font(.subhead).emphasized()
          Picker(["Claude 3.7 Sonnet (Reasoning)", "GPT-4o Realtime", "Llama 3.3 70B Instruct"])
            .title("Model")
            .selected(store.shipping_method.clone())
            .on_select(|s: &mut Store, v: &str| s.shipping_method = v.to_string())
          TextInput(store.search_text.clone())
            .placeholder("Instruct your agent to synthesize data or generate UI...")
            .on_change(|s: &mut Store, v: String| s.search_text = v)
          HStack.spacing(8) {
            Button
              .variant(.primary)
              .width(.fill)
              .action(|s: &mut Store| s.count += 1) {
              Icon(.sparkles).size(16)
              Text("Run Agent")
            }
            Button("Logs").variant(.secondary).action(|| navigate("/details"))
          }
        }

        Text("Latest Generated Artifact").font(.headline).emphasized()
        Image("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=900&q=80")
          .height(145)
          .corner_radius(12)
      }
    }
  }
}

pub fn ShowcaseSettingsScreen() -> impl IntoNode {
  Return! {
    Page.title("Settings").large_title(true) {
      VStack.spacing(14) {
        app_switcher_card()
      }
    }
  }
}
