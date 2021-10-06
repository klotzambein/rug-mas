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

    pub fn step(&mut self, _step: u32, reporter: &mut Reporter) {
        self.agents.step(&mut self.market);
        // if step % 1000 == 0{
        //     self.market.plot_depth(&format!("depth_{}.png", step));
        // }
        self.market.step(&mut self.agents);
        println!("{}", self.agents.mean_state());
        report!(reporter, "price", self.market.price() as f64);
        report!(reporter, "volatility", self.market.volatility() as f64);
        report!(reporter, "median_cash", self.agents.cash_median() as f64);
        // report!(reporter, "total cash", self.agents.total_cash());
        // report!(reporter, "total assets", self.agents.total_assets(0) as f64);
    }
}
