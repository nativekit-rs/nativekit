// BEGIN_NK_AUTO_MODULES
#[macro_use]
extern crate native_kit;
#[allow(non_snake_case, dead_code, unused_imports)]
pub mod views {
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod custom_greeting;
    pub use custom_greeting::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod navbar;
    pub use navbar::*;
}
#[allow(non_snake_case, dead_code, unused_imports)]
pub mod __nk_auto {
    pub use super::app::Store;
    pub use super::views::*;
}
#[allow(unused_imports)]
pub use __nk_auto::*;
// END_NK_AUTO_MODULES

pub mod showcases;
pub mod app;
pub mod pages;
pub mod server;

pub struct NativeApp;

impl native_kit::App for NativeApp {
    type State = app::Store;

    fn init() -> Self::State {
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();
        app::Store::default()
    }

    fn view(state: &Self::State) -> impl native_kit::IntoNode {
        let _scope = native_kit::set_scoped_state(state);
        app::App()
    }
}

native_kit::export_app!(NativeApp);


