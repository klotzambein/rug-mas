use crate::config::Config;

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {}

impl Agent {
    pub fn new() -> Agent {
        Agent {}
    }
}

#[derive(Debug, Clone)]
pub struct AgentCollection {
    agents: Vec<Agent>,
}

impl AgentCollection {
    pub fn new(config: &Config) -> AgentCollection {
        AgentCollection {
            agents: std::iter::repeat(Agent::new())
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
}
