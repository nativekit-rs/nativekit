#![allow(non_snake_case)]
use crate::showcases::*;
use native_kit::*;

#[derive(Clone)]
pub struct Store {
    pub count: i32,
    pub shipping_method: String,
    pub search_text: String,
    pub active_showcase: String,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            count: 2,
            shipping_method: "USDC (Solana) • $12,400".to_string(),
            search_text: String::new(),
            active_showcase: "NovaPay (FinTech)".to_string(),
        }
    }
}

pub type AppState = Store;

pub fn App() -> impl IntoNode {
    Router!({
        Route!("/", HomeScreen()).tab("Home", "house"),
        Route!("/orders", OrdersScreen()).tab("Order", "coffee"),
        Route!("/orders/:id", |params| OrderDetailScreen(&params["id"])),
        Route!("/settings", SettingsScreen()).tab("Scan", "qrcode"),
        Route!("/send", NovaPaySendSheet()).sheet(),
        Route!("/purchase", PurchaseModal()).sheet(),
        Route!("/purchase/confirmed", PurchaseConfirmedScreen()),
        Route!("/form1", Form1Screen()).form(),
        Route!("/form1/sheet", Form1SheetScreen()).sheet(),
        Route!("/form1/detail", Form1DetailScreen()),
        Route!("/form2", Form2Screen()).form(),
        Route!("/form2/detail", Form2DetailScreen()),
        Route!("/form2/sheet", Form2SheetScreen()).sheet(),
    })
}
