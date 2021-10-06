use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub market: MarketConfig,
    pub agent: AgentConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
        let config = std::fs::read_to_string(path)?;
        Ok(from_str(&config)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketConfig {
    pub market_count: usize,
    pub initial_price: f32,
    pub initial_volatility: f32,
    pub price_history_count: usize,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            market_count: 3,
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
    pub fundamentalist_count: usize,
    pub agent_count: usize,
    pub order_probability: f32,
    pub influence_probability: f32,
    pub influencers_count: usize,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            initial_cash: 3000.0,
            initial_assets: 30,
            agent_count: 1000,
            fundamentalist_count: 100,
            order_probability: 1.0,
            influence_probability: 0.8,
            influencers_count: 1,
        }
    }
}
