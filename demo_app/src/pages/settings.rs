
pub fn SettingsScreen() -> impl IntoNode {
  let store = Store::current();
  let theme = Theme::current();

  Return! {
    Page.title("Settings").large_title(true) {
      Section
        .header("Appearance & Demo")
        .footer("On iOS, open any Sheet to see this List automatically turn translucent gray!") {
        Picker(["NovaPay (FinTech)", "Kona Craft (Commerce)", "Pulse AI (SaaS)", "Kitchen Sink (Original)"])
          .title("Active White-Label App")
          .selected(store.active_showcase.clone())
          .on_select(|s: &mut Store, val: &str| {
            s.active_showcase = val.to_string();
            navigate("/");
          })
        Switch()
          .title("Dark Mode")
          .value(theme.is_dark())
          .on_change(Theme::toggle_dark)
        Switch()
          .title("Use OS System Accent")
          .value(theme.is_system_accent())
          .on_change(Theme::toggle_system_accent)
      }

      Grid
        .columns(2)
        .spacing(12)
        .header("Engine Telemetry") {
        VStack.spacing(4).padding(14).corner_radius(12).background(.surface_secondary) {
          Icon(.bolt).color(.primary).size(22)
          Text("99.98%").font(.title2).emphasized()
          Text("Uptime").font(.caption1).color(.text_secondary)
        }

        VStack.spacing(4).padding(14).corner_radius(12).background(.surface_secondary) {
          Icon(.sparkles).color(.orange).size(22)
          Text("0.4ms").font(.title2).emphasized()
          Text("AST Diff").font(.caption1).color(.text_secondary)
        }
      }

      Section
        .header("System Preferences")
        .footer("Android renders this as a Material 3 Segmented List; Linux uses AdwPreferencesGroup.") {
        Picker(["System Default", "Light", "Dark"])
          .title("Theme Mode")
          .selected(theme.mode_label())
          .on_change(|val: &str| Theme::set_mode(val))

        Picker(["English (US)", "Spanish", "French", "German"])
          .title("Language")
          .selected("English (US)")

        Button("Open Sheet (Test iOS Translucent List)")
          .action(|| navigate("/send"))
      }
    }
  }
}
