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
            market: GenoaMarket::new(config),
            agents: AgentCollection::new(config),
        }
    }

    pub fn step(&mut self, step: u32, reporter: &mut Reporter) {
        report!(reporter, "step", step.into());
        self.market.step();
    }
}
