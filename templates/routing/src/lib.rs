pub mod app;

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
