use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub agent_count: usize,
    pub market: MarketConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
        let config = std::fs::read_to_string(path)?;
        Ok(from_str(&config)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_count: 1000,
            market: MarketConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketConfig {
    pub initial_price: f32,
    pub price_history_count: usize,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            initial_price: 100.0,
            price_history_count: 20,
        }
    }
}