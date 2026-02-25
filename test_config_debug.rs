use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DefaultsConfig {
    #[serde(default = "default_exchange")]
    pub exchange: String,
    #[serde(default = "default_product")]
    pub product: String,
}

fn default_exchange() -> String { "NSE".to_string() }
fn default_product() -> String { "CNC".to_string() }

fn main() {
    let config = DefaultsConfig::default();
    println!("exchange: {:?}", config.exchange);
    println!("product: {:?}", config.product);
}
