use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub agent_count: usize,
    pub market: MarketConfig,
    pub agent: AgentConfig,
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
            agent_count: 100,
            market: MarketConfig::default(),
            agent: AgentConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketConfig {
    pub initial_price: f32,
    pub initial_volatility: f32,
    pub price_history_count: usize,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            initial_price: 100.0,
            initial_volatility: 0.003,
            price_history_count: 20,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfig {
    pub initial_cash: f32,
    pub initial_assets: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            initial_cash: 3000.0,
            initial_assets: 30,
        }
    }
}