use crate::{
    agent::AgentCollection,
    config::Config,
    market::GenoaMarket,
    report::{report, Reporter},
};

#[derive(Debug, Clone)]
pub struct Simulation {
    market: GenoaMarket,
    agents: AgentCollection,
}

impl Simulation {
    pub fn new(config: &Config) -> Simulation {
        Simulation {
            market: GenoaMarket::new(config, 0),
            agents: AgentCollection::new(config),
        }
    }

    pub fn step(&mut self, step: u32, reporter: &mut Reporter) {
        self.agents.step(&mut self.market);
        self.market.step(&mut self.agents);
        report!(reporter, "price", self.market.price() as f64);
        report!(reporter, "volatility", self.market.volatility() as f64);
    }
}
