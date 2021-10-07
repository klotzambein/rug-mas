use crate::{
    agent::AgentCollection,
    config::Config,
    market::GenoaMarket,
    report::{report, Reporter},
};

#[derive(Debug, Clone)]
pub struct Simulation {
    markets: Vec<GenoaMarket>,
    agents: AgentCollection,
}

impl Simulation {
    pub fn new(config: &Config) -> Simulation {
        Simulation {
            markets: std::iter::repeat_with(|| GenoaMarket::new(config, 0))
                .take(config.market.market_count)
                .collect(),
            agents: AgentCollection::new(config),
        }
    }

    pub fn step(&mut self, step: usize, reporter: &mut Reporter) {
        self.agents.step(&self.markets[..], step);
        for m in &mut self.markets {
            self.agents.step_market(m);
            m.step(&mut self.agents);
        }

        for (i, m) in self.markets.iter_mut().enumerate() {
            let i = i as u32;
            report!(reporter, "price"[i], m.price() as f64);
            report!(reporter, "volatility"[i], m.volatility() as f64);
        }

        report!(reporter, "median_cash", self.agents.cash_median() as f64);
        // report!(reporter, "total cash", self.agents.total_cash());
        // report!(reporter, "total assets", self.agents.total_assets(0) as f64);
    }
}
