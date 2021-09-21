use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub agent_count: usize,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
        let config = std::fs::read_to_string(path)?;
        Ok(from_str(&config)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { agent_count: 1000 }
    }
}
