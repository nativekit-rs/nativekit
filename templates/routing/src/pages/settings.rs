#![allow(non_snake_case)]
use crate::app::Store;
use native_kit::*;

pub fn SettingsScreen() -> impl IntoNode {
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
