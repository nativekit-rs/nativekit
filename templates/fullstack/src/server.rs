#![allow(unexpected_cfgs)]
use native_kit::engine::*;
#[allow(unused_imports)]
use native_kit::*;

#[server]
pub async fn live_banner() -> Result<Node, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(VStack! {
        Text!("Live OTA Component").font("title3").emphasized()
        Text!("Rendered from #[server] at UNIX: {}", timestamp)
    }
    .spacing(6)
    .into_node())
}
