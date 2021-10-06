/*
TODO: The interest vector is initially random. At every time step, the agents increase or decrease the
interest in a market based on other agents' (close in the network) beliefs, overall profits from that
asset, and other news (random noise in our case).
TODO: Fundamentalists? From the DGA paper.
*/

use rand::{distributions::Uniform, prelude::random, prelude::Rng, thread_rng};

use crate::{
    config::Config,
    market::{GenoaMarket, MarketId},
};

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {
    cash: f32,
    /// Vector representing the ammount of assets an agent holds.
    assets: Vec<u32>,

    // /// Value that represents the market in which an agent invests next.
    // market_preference: u32,
    /// Vector encapsulating each market preference of an agent. Contains probabilities between [0, 1].
    state: Vec<f32>,

    // /// Variable describing how likely an agent is to make informed decisions vs following the crowd.
    // fundamentalism_ratio: f32,
    /// Value that describes how likely and agent is to place orders at each timestep.
    order_probability: Vec<f32>,

    /// Value that describes how likely and agent is to be influenced at each timestep.
    influence_probability: f32,

    /// Value that determines how many agents a single agent should be influenced from.
    influencers_count: usize,
    // /// Value that describes how likely an agent is to change its preferences.
    // change_probability: f32,

    // /// Friend list containing trust values for other agents.
    // friends: VecDeque<f32>,
}

impl Agent {
    pub fn new(config: &Config) -> Agent {
        Agent {
            cash: config.agent.initial_cash,
            // market_preference: 0,
            assets: vec![config.agent.initial_assets; config.market.market_count],
            state: std::iter::repeat_with(|| random::<bool>() as usize as f32)
                .take(config.market.market_count)
                .collect(),
            // fundamentalism_ratio: 0.35,
            order_probability: vec![config.agent.order_probability; config.market.market_count],
            influence_probability: config.agent.influence_probability,
            influencers_count: config.agent.influencers_count,
        }
    }

    pub fn apply_buy(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash -= price_per_item * asset_quantity as f32;

        assert!(self.cash >= 0.0, "Agent ran out of cash");

        self.assets[market] += asset_quantity;
    }

    pub fn apply_sell(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash += price_per_item * asset_quantity as f32;

        let a = &mut self.assets[market];
        *a = a
            .checked_sub(asset_quantity)
            .expect("Agent ran out of asset");
    }
}

#[derive(Debug, Clone)]
pub struct AgentCollection {
    agents: Vec<Agent>,
    fundamentalists: Vec<f32>,
}

impl AgentCollection {
    pub fn new(config: &Config) -> AgentCollection {
        AgentCollection {
            agents: std::iter::repeat_with(|| Agent::new(config))
                .take(config.agent.agent_count)
                .collect(),
            fundamentalists: std::iter::repeat_with(|| random::<bool>() as usize as f32)
                .take(config.agent.fundamentalist_count)
                .collect(),
        }
    }

    pub fn agent(&self, id: AgentId) -> &Agent {
        &self.agents[id as usize]
    }

    pub fn agent_mut(&mut self, id: AgentId) -> &mut Agent {
        &mut self.agents[id as usize]
    }

    /// Call this function first, once every step.
    pub fn step(&mut self) {
        self.dga();
    }

    /// Call this function after [`Self::step`], once for every market.
    pub fn step_market(&mut self, market: &mut GenoaMarket) {
        self.trade_on_market(market);
    }

    pub fn total_cash(&self) -> f64 {
        self.agents.iter().map(|a| a.cash as f64).sum()
    }

    pub fn total_assets(&self, market: MarketId) -> u32 {
        self.agents.iter().map(|a| a.assets[market]).sum()
    }

    pub fn mean_state(&self, market: MarketId) -> f32 {
        let states = self
            .agents
            .iter()
            .map(|a| a.state[market] as f32)
            .sum::<f32>();
        states / self.agents.len() as f32
    }

    pub fn cash_median(&self) -> f32 {
        let mut cashs: Vec<_> = self.agents.iter().map(|a| a.cash).collect();
        cashs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        cashs[cashs.len() / 2]
    }

    /// Give the state of either an agent or a fundamentalist. `idx` must be in
    /// range 0 to len(agents) + len(fundamentalists)
    pub fn influence_at(&self, idx: usize, market: MarketId) -> f32 {
        if idx < self.agents.len() {
            self.agents[idx].state[market]
        } else {
            self.fundamentalists[idx - self.agents.len()]
        }
    }

    /// Every agent updates their beliefs based on other agents' preferences
    /// and their own interests. At every time step, the interest for a market
    /// is updated based on performance (overall profits from a market), news
    /// and random noise.
    pub fn dga(&mut self) {
        // let m_id = market;
        // let mut market_noise: f32;

        // More efficient this way.
        let mut rng = thread_rng();

        let range = Uniform::from(0..self.agents.len() + self.fundamentalists.len());
        let market_count = self.agents.first().expect("No agents.").assets.len();

        for idx in 0..self.agents.len() {
            let influencer_count = self.agents[idx].influencers_count;
            // Check if the current agent is to be influenced based on the influence probability.
            if rng.gen::<f32>() < self.agents[idx].influence_probability {
                // Generate our influencers
                let influencers = rng
                    .clone()
                    .sample_iter(&range)
                    // Make sure we do not influence ourselves
                    .filter(|&i: &usize| i != idx)
                    .take(influencer_count)
                    .collect::<Vec<_>>();

                // Influence all markets
                for market in 0..market_count {
                    // Take random agents and sum their influence.
                    let influence_sum = influencers
                        .iter()
                        .map(|&i| self.influence_at(i, market))
                        .sum::<f32>();

                    let influence = influence_sum / influencer_count as f32;

                    self.agents[idx].state[market] = influence.round();
                }
            }
        }
    }

    pub fn trade_on_market(&mut self, market: &mut GenoaMarket) {
        let mut rng = thread_rng();

        for (agent_id, agent) in self.agents.iter_mut().enumerate() {
            let agent_id = agent_id as AgentId;
            let m_id = market.id();

            if rng.gen::<f32>() < agent.order_probability[m_id] {
                if rng.gen::<f32>() < agent.state[m_id] {
                    market.buy_order(agent_id, agent.cash * rng.gen::<f32>());
                } else {
                    let assets = agent.assets[m_id] as f32 * rng.gen::<f32>();
                    market.sell_order(agent_id, assets as u32)
                }
            }
        }
    }
}
