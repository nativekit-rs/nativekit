#![allow(non_snake_case)]
use crate::app::Store;
use native_kit::*;

pub fn SettingsScreen() -> impl IntoNode {
    let state = use_state::<Store>();
    let enabled = state.notifications_enabled;

    Return! {
        Page.title("Settings").large_title(true).top_style(.soft) {
            Section.header("Preferences") {
                HStack.align(.space_between) {
                    Text("Push Notifications").font(.body)
                    Switch(enabled).on_change(|s: &mut Store, val: bool| {
                        s.notifications_enabled = val;
                    })
                }
            }
        }
    }
}
