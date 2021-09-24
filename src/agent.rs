/*
TODO: The interest vector is initially random. At every time step, the agents increase or decrease the
interest in a market based on other agents' (close in the network) beliefs, overall profits from that
asset, and other news (random noise in our case).
TODO: Fundamentalists? From the DGA paper.
*/

use rand::random;

use crate::{
    config::Config,
    market::{GenoaMarket, MarketId},
};

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {
    cash: f32,
    belief_state: u32,
    assets: Vec<u32>, // Vector representing the ammount of assets an agent holds.
    interest_prob_vector: Vec<f32>, // Vector encapsulating each market preference of an agent. Contains probabilities between [0, 1].
}

impl Agent {
    pub fn new(config: &Config) -> Agent {
        Agent {
            cash: config.agent.initial_cash,
            belief_state: 0,
            assets: vec![0, 0, 0],
            interest_prob_vector: vec![0.0, 0.0, 0.0],
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
    }

    pub fn assign_states(&mut self) {
        // Every agent updates their beliefs based on other agents' interests,
        // overall profits from a market and other random factors.
        todo!()
    }
}
