// ⚡ Built with NativeKit — https://nativekit.rs
#![allow(non_snake_case)]
use native_kit::*;

#[derive(Clone)]
pub struct Store {
    pub count: i32,
    pub notifications_enabled: bool,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            count: 1,
            notifications_enabled: true,
        }
    }
}

pub type AppState = Store;

pub fn App() -> impl IntoNode {
    Router!({
        Route!("/", HomeScreen()).tab("Home", "home"),
        Route!("/explore", ExploreScreen()).tab("Explore", "sparkles"),
        Route!("/settings", SettingsScreen()).tab("Settings", "settings"),
        Route!("/quick-sheet", QuickActionSheet()).sheet(),
    })
}
