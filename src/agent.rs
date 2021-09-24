/*
TODO: The interest vector is initially random. At every time step, the agents increase or decrease the 
interest in a market based on other agents' (close in the network) beliefs, overall profits from that 
asset, and other news (random noise in our case).
TODO: Fundamentalists? From the DGA paper.
*/

use crate::config::Config;

pub type AgentId = u32;

#[derive(Debug, Clone)]
pub struct Agent {
    cash: f32,
    belief_state: u32,
    assets: Vec<u32>, // Vector representing the ammount of assets an agent holds.
    interest_prob_vector: Vec<f32>, // Vector encapsulating each market preference of an agent. Contains probabilities between [0, 1].
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            cash: 0.0,
            belief_state: 0,
            assets: vec![0, 0, 0],
            interest_prob_vector: vec![0.0, 0.0, 0.0],
        }
    }

    pub fn buy(&mut self, asset_quantity: u32, price: f32) {
        todo!();
    }

    pub fn sell(&mut self, asset_quantity: u32, price: f32) {
        todo!();
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

    pub fn assign_states(&mut self, &agent: Agent) {
        todo!()
    }
}
