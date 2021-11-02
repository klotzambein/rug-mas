use std::{error::Error, path::Path};

use rand::{prelude::ThreadRng, Rng};
use rand_distr::{Distribution as RDist, Normal, Uniform};
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
    /// Total amount of fundamentalists in the simulation.
    pub fundamentalist_count: usize,

    /// Total amount of agents in the simulation.
    pub agent_count: usize,

    /// The amount of influencers that influence the agent every step, they will
    /// be influenced.
    pub influencers_count: Distribution,

    /// The probability that the agent will place any order in a market.
    pub order_probability: Distribution,

    /// The probability that an agent will be influenced by anyone this step.
    pub influence_probability: Distribution,

    /// How long, until the agent reflects on trades made, and potentially adds
    /// friends. Needs to be smaller then `config.market.price_history_count`.
    pub reflection_delay: Distribution,

    /// The threshold above witch an influence becomes a friend. After a person
    /// has been influenced and `reflection_delay` time has passed, the pearson
    /// correlation between the market change and the influence will be
    /// computed. Should this correlation be greater than the threshold, the
    /// person will become a friend.
    ///
    /// To disable friends, set this to anything above 1.
    pub friend_threshold: Distribution,

    /// The maximum number of friends.
    pub max_friends: Distribution,

    /// Chance of being influenced by a particular friend. This only matters
    /// when the agent is being influenced at all.
    pub friend_influence_probability: Distribution,

    /// Initial amount of cash an agent holds, should be balanced with the value
    /// of stocks.
    pub initial_cash: Distribution,

    /// Initial amount of assets/stocks the agent holds in every market.
    pub initial_assets: Distribution,

    /// The initial belief the agent has about each market. Zero is bad, one is
    /// good.
    pub initial_state: Distribution,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            agent_count: 1000,
            fundamentalist_count: 100,
            initial_assets: Distribution::static_value(30.0),
            order_probability: Distribution::static_value(1.0),
            influence_probability: Distribution::static_value(0.8),
            influencers_count: Distribution::static_value(1.0),
            initial_cash: Distribution::static_value(3000.0),
            initial_state: Distribution::Bernoulli { p: 0.5 },
            reflection_delay: Distribution::static_value(10.0),
            friend_threshold: Distribution::static_value(0.6),
            max_friends: Distribution::static_value(10.0),
            friend_influence_probability: Distribution::static_value(0.4),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "distribution")]
pub enum Distribution {
    Uniform {
        start: f32,
        end: f32,
    },
    Normal {
        mean: f32,
        sd: f32,
    },
    Bernoulli {
        p: f32,
    },
    Clamp {
        min: f32,
        max: f32,
        inner: Box<Distribution>,
    },
    Round {
        inner: Box<Distribution>,
    },
}

impl Distribution {
    pub fn static_value(val: f32) -> Distribution {
        Distribution::Normal { mean: val, sd: 0.0 }
    }

    pub fn sample_f32(&self, rng: &mut ThreadRng) -> f32 {
        match self {
            Distribution::Uniform { start, end } => Uniform::new(*start, *end).sample(rng),
            Distribution::Normal { mean, sd } => Normal::new(*mean, *sd).unwrap().sample(rng),
            Distribution::Bernoulli { p } => rng.gen_bool(*p as f64) as usize as f32,
            Distribution::Clamp { min, max, inner } => inner.sample_f32(rng).clamp(*min, *max),
            Distribution::Round { inner } => inner.sample_f32(rng).round(),
        }
    }

    pub fn sample_usize(&self, rng: &mut ThreadRng) -> usize {
        self.sample_f32(rng).round() as usize
    }

    pub fn sample_isize(&self, rng: &mut ThreadRng) -> isize {
        self.sample_f32(rng).round() as isize
    }

    pub fn sample_bool(&self, rng: &mut ThreadRng) -> bool {
        if let Self::Bernoulli { p } = self {
            rng.gen_bool(*p as f64)
        } else {
            self.sample_usize(rng) != 0
        }
    }
}
