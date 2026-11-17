#![allow(unexpected_cfgs)]
use native_kit::engine::*;
#[allow(unused_imports)]
use native_kit::*;

#[server]
pub async fn marketing_banner() -> Result<Node, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(VStack! {
        Text!("🚀 Finally stable!!!").font("title1").emphasized()
        Text!("Get 500% off all NativeKit components!")
        Text!("Fetched live from Railway at UNIX: {}", timestamp)
    }
    .spacing(8)
    .into_node())
}

#[server]
pub async fn health_check() -> Result<Node, String> {
    Ok(native_kit::engine::ast::Node::RawJson(serde_json::json!({
        "status": "ok",
        "uptime": "100%"
    })))
}

#[server]
pub async fn hello_world() -> Result<Node, String> {
    Ok(native_kit::engine::ast::Node::RawJson(serde_json::json!(
        "hello world"
    )))
}
