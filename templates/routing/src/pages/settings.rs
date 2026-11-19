use native_kit::*;

#[component]
pub fn SettingsScreen() {
    let state = use_state::<Store>();
    let enabled = state.notifications_enabled;

    Return! {
        Page.title("Settings").large_title(true).top_style(.soft) {
            Section.header("Preferences") {
                Switch(enabled)
                    .title("Push Notifications")
                    .on_change(|s: &mut Store| {
                        s.notifications_enabled = !s.notifications_enabled;
                    })
            }
        }
    }
}
