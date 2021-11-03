use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    agent::AgentCollection,
    config::Config,
    market::GenoaMarket,
    report::{report, Reporter},
};

pub const AGENT_PER_MARKET_INLINE_THRESHOLD: usize = 5;

#[derive(Debug, Clone)]
pub struct Simulation {
    markets: Vec<GenoaMarket>,
    agents: AgentCollection<AGENT_PER_MARKET_INLINE_THRESHOLD>,
}

impl Simulation {
    pub fn new(config: &Config) -> Simulation {
        Simulation {
            markets: (0..config.market.market_count)
                .map(|i| GenoaMarket::new(config, i))
                .collect(),
            agents: AgentCollection::new(config),
        }
    }

    pub fn agents(&self) -> &AgentCollection<AGENT_PER_MARKET_INLINE_THRESHOLD> {
        &self.agents
    }

    pub fn markets(&self) -> &[GenoaMarket] {
        &self.markets[..]
    }

    pub fn step(&mut self, step: usize, reporter: &mut Reporter) {
        // just runs dga
        self.agents.step(&self.markets[..], step);

        // Runs market logic (we shuffle the market access)
        let mut markets = self.markets.iter_mut().collect::<Vec<_>>();
        markets.shuffle(&mut thread_rng());

        for m in markets {
            self.agents.step_market(m);
            m.step(&mut self.agents);
        }

        // Update friends
        self.agents.update_friends(&self.markets[..]);

        // report values
        for (i, m) in self.markets.iter_mut().enumerate() {
            let i = i as u32;
            report!(reporter, "price"[i], m.price() as f64);
            report!(reporter, "volatility"[i], m.volatility() as f64);
        }

        // for agent in 0..10 {
        //     report!(
        //         reporter,
        //         "agent_cash"[agent as u32],
        //         self.agents.agent(agent).cash as f64
        //     );
        //     report!(
        //         reporter,
        //         "agent_total_assets"[agent as u32],
        //         self.agents.agent(agent).assets.iter().sum::<u32>() as f64
        //     );
        //     report!(
        //         reporter,
        //         "agent_assets_0"[agent as u32],
        //         self.agents.agent(agent).assets[0] as f64
        //     );
        //     report!(
        //         reporter,
        //         "agent_state_0"[agent as u32],
        //         self.agents.agent(agent).state[0] as f64
        //     );
        //     report!(
        //         reporter,
        //         "agent_friend_count"[agent as u32],
        //         self.agents.agent(agent).friends.len() as f64
        //     );
        // }

        report!(reporter, "median_cash", self.agents.cash_median() as f64);
        // report!(reporter, "total cash", self.agents.total_cash());
        // report!(reporter, "total assets", self.agents.total_assets(0) as f64);
    }
}
