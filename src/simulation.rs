use crate::{agent::Agent, config::Config};

#[derive(Debug, Clone)]
pub struct Simulation {
    agents: Vec<Agent>,
}

impl Simulation {
    pub fn new(config: &Config) -> Simulation {
        Simulation {
            agents: std::iter::repeat(Agent::new())
                .take(config.agent_count)
                .collect(),
        }
    }

    pub fn step(&mut self) {}
}
