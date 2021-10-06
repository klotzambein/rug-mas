/*
TODO: The interest vector is initially random. At every time step, the agents increase or decrease the
interest in a market based on other agents' (close in the network) beliefs, overall profits from that
asset, and other news (random noise in our case).
TODO: Fundamentalists? From the DGA paper.
*/

use rand::{distributions::Uniform, prelude::random, prelude::thread_rng, prelude::Rng};

use crate::{
    config::Config,
    market::{GenoaMarket, MarketId},
};

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {
    cash: f32,
    /// Vector representing the ammount of assets an agent holds.
    assets: u32,

    // Value that represents the market in which an agent invests next.
    // market_preference: u32,
    /// Vector encapsulating each market preference of an agent. Contains probabilities between [0, 1].
    state: u32,

    /// Variable describing how likely an agent is to make informed decisions vs following the crowd.
    fundamentalism_ratio: f32,

    /// Value that describes how likely and agent is to place orders at each timestep.
    order_probability: f32,

    /// Value that describes how likely and agent is to be influenced at each timestep.
    influence_probability: f32,

    /// Value that determines how many agents a single agent should be influenced from.
    influencers_count: usize,
    // Value that describes how likely an agent is to change its preferences.
    // change_probability: f32,

    // Friend list containing trust values for other agents.
    // friends: VecDeque<f32>,
}

impl Agent {
    pub fn new(config: &Config) -> Agent {
        Agent {
            cash: config.agent.initial_cash,
            // market_preference: 0,
            assets: config.agent.initial_assets,
            state: rand::thread_rng().gen_range(0..1),
            fundamentalism_ratio: 0.35,
            order_probability: config.agent.order_probability,
            influence_probability: config.agent.influence_probability,
            influencers_count: config.agent.influencers_count,
        }
    }

    pub fn apply_buy(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash -= price_per_item * asset_quantity as f32;

        assert!(self.cash >= 0.0, "Agent ran out of cash");

        self.assets += asset_quantity;
    }

    pub fn apply_sell(&mut self, market: MarketId, asset_quantity: u32, price_per_item: f32) {
        self.cash += price_per_item * asset_quantity as f32;

        let a = &mut self.assets;
        *a = a
            .checked_sub(asset_quantity)
            .expect("Agent ran out of asset");
    }
}

#[derive(Debug, Clone)]
pub struct AgentCollection {
    agents: Vec<Agent>,
    fundamentalists: Vec<u32>,
}

impl AgentCollection {
    pub fn new(config: &Config) -> AgentCollection {
        AgentCollection {
            agents: std::iter::repeat_with(|| Agent::new(config))
                .take(config.agent.agent_count)
                .collect(),
            fundamentalists: std::iter::repeat_with(|| random::<bool>() as usize as u32)
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

    pub fn step(&mut self, market: &mut GenoaMarket) {
        self.dga(market.id());
        self.trade_on_market(market);
        // self.update_behaviour();
    }

    pub fn total_cash(&self) -> f64 {
        self.agents.iter().map(|a| a.cash as f64).sum()
    }

    pub fn total_assets(&self, market: MarketId) -> u32 {
        self.agents.iter().map(|a| a.assets).sum()
    }

    pub fn mean_state(&self) -> f32 {
        let states: f32 = self.agents.iter().map(|a| a.state as f32).sum() as f32;
        let mean_state: f32 = states / self.agents.len() as f32;
        mean_state
    }

    pub fn cash_median(&self) -> f32 {
        let mut cashs: Vec<_> = self.agents.iter().map(|a| a.cash).collect();
        cashs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        cashs[cashs.len() / 2]
    }

    // /// Update trading behaviour based on how much the agent won/lost.
    // /// Gambler's fallacy + noise
    // /// Variables to update: fundamentalism_ratio(?), order_probability
    // pub fn update_behaviour(&mut self) {
    //     todo!()
    // }

    /// Every agent updates their beliefs based on other agents' preferences
    /// and their own interests. At every time step, the interest for a market
    /// is updated based on performance (overall profits from a market), news
    /// and random noise.
    pub fn dga(&mut self, market: MarketId) {
        // let m_id = market as usize;
        // let mut market_noise: f32;

        // More efficient this way.
        let range = Uniform::from(0..self.agents.len() + self.fundamentalists.len());

        for idx in 0..self.agents.len() {
            // Check if the current agent is to be influenced based on the influence probablity.
            if random::<f32>() < self.agents[idx].influence_probability {
                let influencer_count = self.agents[idx].influencers_count as f32;
                // Get some random id's for influencer agents.
                let rand_idx: Vec<usize> = rand::thread_rng().sample_iter(&range).take(influencer_count as usize).collect();

                // Agents can't influence themselves.
                if !rand_idx.contains(&idx) {
                    let mut states: f32 = 0.0;

                    // Add the states of the influencer agents together.
                    for current_idx in 0..rand_idx.len() {
                        if rand_idx[current_idx] < self.agents.len() {
                            states += self.agents[rand_idx[current_idx]].state as f32
                        } else {
                            states += self.fundamentalists[rand_idx[current_idx] - self.agents.len()] as f32
                        };
                    }

                    let influencer_count_div = influencer_count / 2.0;
                    let mut influence: u32 = 0;

                    // If more than half of the influencer agents have a certain state, choose that state. Random if it's equal.
                    if states < influencer_count_div {
                        influence = 0;
                    } else if states > influencer_count_div {
                        influence = 1;
                    } else {
                        influence = rand::thread_rng().gen_range(0..1)
                    }

                    self.agents[idx].state = influence;
                }
            }

            // market_noise = rng.gen_range(0.0..1.0); // Adding noise to decision.
            // Making decision based on herd behaviour.
            // let herd_val =
            //     ((1. - self.agents[idx].fundamentalism_ratio)) * self.agents[rand_idx].confidence_probability[m_id];
            // Making decision based on fundamentals.
            // let fund_val = self.agents[idx].fundamentalism_ratio * (market.get_markup() + market.get_news());

            // Keeping the value between 0 and 1 using the Sigmoid function.

            // self.agents[idx].confidence_probability[m_id] = 1. / (1. + (-herd_val - fund_val - market_noise).exp());
        }
    }

    pub fn trade_on_market(&mut self, market: &mut GenoaMarket) {
        for (agent_id, agent) in self.agents.iter_mut().enumerate() {
            let agent_id = agent_id as AgentId;
            let m_id = market.id() as usize;
            
            if random::<f32>() < agent.order_probability {
                if agent.state == 0 {
                    market.buy_order(agent_id, agent.cash * random::<f32>());
                } else {
                    market.sell_order(agent_id, (agent.assets as f32 * random::<f32>()) as u32)
                }
            }
        }
    }
}
