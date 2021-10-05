/*
TODO: The interest vector is initially random. At every time step, the agents increase or decrease the
interest in a market based on other agents' (close in the network) beliefs, overall profits from that
asset, and other news (random noise in our case).
TODO: Fundamentalists? From the DGA paper.
*/

use rand::prelude::{Rng, random, thread_rng};

use crate::{
    config::Config,
    market::{GenoaMarket, MarketId},
};

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {
    cash: f32,
    assets: Vec<u32>, // Vector representing the ammount of assets an agent holds.
    market_preference: u32, // Value that represents the market in which an agent invests next.
    interest_prob_vector: Vec<f32>, // Vector encapsulating each market preference of an agent. Contains probabilities between [0, 1].
    fundamentalism_ratio: f32, // Variable describing how likely an agent is to make informed decisions vs following the crowd.
    order_probability: f32, // Value that describes how likely and agent is to place orders at each timestep.
}

impl Agent {
    pub fn new(config: &Config) -> Agent {
        Agent {
            cash: config.agent.initial_cash,
            market_preference: 0,
            assets: vec![config.agent.initial_assets; 3],
            interest_prob_vector: vec![0.0, 0.0, 0.0],
            fundamentalism_ratio: 0.35,
            order_probability: 0.5,
        }
    }

    pub fn apply_buy(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash -= price_per_item * asset_quantity as f32;

        assert!(self.cash >= 0.0, "Agent ran out of cash");

        self.assets[market as usize] += asset_quantity;
    }

    pub fn apply_sell(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash += price_per_item * asset_quantity as f32;

        let a = &mut self.assets[market as usize];
        *a = a
            .checked_sub(asset_quantity)
            .expect("Agent ran out of asset");
    }
}

#[derive(Debug, Clone)]
pub struct AgentCollection {
    agents: Vec<Agent>,
}

impl AgentCollection {
    pub fn new(config: &Config) -> AgentCollection {
        AgentCollection {
            agents: std::iter::repeat_with(|| Agent::new(config))
                .take(config.agent_count)
                .collect(),
        }
    }

    pub fn agent(&self, id: AgentId) -> &Agent {
        &self.agents[id as usize]
    }

    pub fn agent_mut(&mut self, id: AgentId) -> &mut Agent {
        &mut self.agents[id as usize]
    }

    pub fn step(&mut self, market: &mut GenoaMarket) {
        for (agent_id, agent) in self.agents.iter_mut().enumerate() {
            let agent_id = agent_id as AgentId;
            if random() {
                market.buy_order(agent_id, agent.cash * random::<f32>());
            } else {
                let assets_owned = agent.assets[market.id() as usize] as f32;

                market.sell_order(agent_id, (assets_owned * random::<f32>()) as u32)
            }
        }

        self.update_market_interest(market);
        self.update_behaviour();
    }

    pub fn total_cash(&self) -> f64 {
        self.agents.iter().map(|a| a.cash as f64).sum()
    }

    pub fn total_assets(&self, market: MarketId) -> u32 {
        self.agents.iter().map(|a| a.assets[market as usize]).sum()
    }

    pub fn cash_median(&self) -> f32 {
        let mut cashs: Vec<_> = self.agents.iter().map(|a| a.cash).collect();
        cashs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        cashs[cashs.len() / 2]
    } // Making decision based on

    /// Update trading behaviour based on how much the agent won/lost.
    /// Gambler's fallacy + noise
    /// Variables to update: fundamentalism_ratio(?), order_probability
    pub fn update_behaviour(&mut self) {
        todo!()
    }

    /// Every agent updates their beliefs based on other agents' preferences
    /// and their own interests. At every time step, the interest for a market
    /// is updated based on performance (overall profits from a market), news
    /// and random noise.
    pub fn update_market_interest(&mut self, market: &GenoaMarket) {
        let mut rng = thread_rng();
        let m_id = market.id() as usize;
        let mut rand_idx: usize;
        let mut market_noise: f32;

        for idx in 0..self.agents.len() {
            rand_idx = rng.gen_range(0..self.agents.len());
            market_noise = rng.gen_range(0.0..1.0);

            self.agents[idx].interest_prob_vector[m_id] = ((1.
                - self.agents[idx].fundamentalism_ratio)
                * self.agents[rand_idx].interest_prob_vector[m_id]  // Making decision based on herd behaviour.
                + self.agents[idx].fundamentalism_ratio
                * (market.get_markup() + market.get_news())         // Making decision based on fundamentals.
                + market_noise) // Adding noise to decision.
                .tanh(); // Keeping the value between 0 and 1.
        }
    }
}
