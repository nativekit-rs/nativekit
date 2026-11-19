// BEGIN_NK_AUTO_MODULES
#[allow(non_snake_case, dead_code, unused_imports)]
pub mod pages {
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod demo_sheets {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/demo_sheets.rs"));
    }
    pub use demo_sheets::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod details {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/details.rs"));
    }
    pub use details::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod home {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/home.rs"));
    }
    pub use home::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod orders {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/orders.rs"));
    }
    pub use orders::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod profile {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/profile.rs"));
    }
    pub use profile::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod purchase {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/purchase.rs"));
    }
    pub use purchase::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod settings {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pages/settings.rs"));
    }
    pub use settings::*;
}
#[allow(non_snake_case, dead_code, unused_imports)]
pub mod views {
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod avatar {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/avatar.rs"));
    }
    pub use avatar::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod custom_greeting {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/custom_greeting.rs"));
    }
    pub use custom_greeting::*;
    #[allow(non_snake_case, dead_code, unused_imports)]
    pub mod navbar {
        #[allow(unused_imports)]
        use ::native_kit::*;
        #[allow(unused_imports)]
        use crate::__nk_auto::*;
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/navbar.rs"));
    }
    pub use navbar::*;
}
#[allow(non_snake_case, dead_code, unused_imports)]
pub mod __nk_auto {
    pub use super::app::Store;
    pub use super::pages::*;
    pub use super::views::*;
}
#[allow(unused_imports)]
pub use __nk_auto::*;
// END_NK_AUTO_MODULES

pub mod showcases;
pub mod app;
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


