use crate::{
    agent::Agent,
    config::Config,
    market::GenoaMarket,
    report::{report, Reporter},
};

#[derive(Debug, Clone)]
pub struct Simulation {
    market: GenoaMarket,
    agents: Vec<Agent>,
}

impl Simulation {
    pub fn new(config: &Config) -> Simulation {
        Simulation {
            market: GenoaMarket::new(100.0),
            agents: std::iter::repeat(Agent::new())
                .take(config.agent_count)
                .collect(),
        }
    }

    pub fn step(&mut self, step: u32, reporter: &mut Reporter) {
        report!(reporter, "step", step.into());
    }
}
