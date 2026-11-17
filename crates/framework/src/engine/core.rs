


pub trait App: 'static {
    type State: Send + Sync + 'static;


    fn init() -> Self::State;
    fn view(state: &Self::State) -> impl crate::engine::builder::IntoNode;
}

pub fn get_server_url() -> &'static str {
    static URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    URL.get_or_init(|| {
        let app_json = option_env!("NK_APP_JSON_CONTENT").unwrap_or("{}");
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(app_json) {
            if let Some(server) = json.get("server") {
                #[cfg(debug_assertions)]
                if let Some(url) = server.get("dev").and_then(|u| u.as_str()) {
                    let url = url.replace("http://native-kit", "https://native-kit");
                    return url;
                }
                #[cfg(not(debug_assertions))]
                if let Some(url) = server.get("prod").and_then(|u| u.as_str()) {
                    let url = url.replace("http://native-kit", "https://native-kit");
                    return url;
                }
            }
        }
        "http://127.0.0.1:5824".to_string()
    })
}

#[macro_export]
macro_rules! server_fn {
    (
        $(#[$attr:meta])*
        $vis:vis async fn $name:ident ( $( $arg:ident : $ty:ty ),* $(,)? ) -> $ret:ty {
            $($body:tt)*
        }
    ) => {
        $(#[$attr])*
        $vis async fn $name( $( $arg: $ty ),* ) -> $ret {
            $($body)*
        }

        pub mod client {
            // use super::*; // REMOVED
            $vis fn $name( $( $arg: $ty ),* ) -> Result<String, String> {
                let url = format!("{}/api/{}", $crate::engine::core::get_server_url(), stringify!($name));
                let body_json = serde_json::json!({
                    $( stringify!($arg): $arg , )*
                });
                
                let req = ureq::post(&url).header("Content-Type", "application/json");
                match req.send(&body_json.to_string()) {
                    Ok(mut resp) => Ok(resp.body_mut().read_to_string().unwrap_or_default()),
                    Err(e) => Err(e.to_string())
                }
            }
        }
    }
}
