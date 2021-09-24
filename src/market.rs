use std::{array::IntoIter, cmp::Ordering, collections::VecDeque, ops::Div};

use rand::{distributions::Standard, prelude::*};

use crate::{
    agent::{AgentCollection, AgentId},
    config::Config,
};

#[derive(Debug, Clone)]
pub struct GenoaMarket {
    price_history: VecDeque<f32>,
    price_history_count: usize,
    volatility: f32,
    buy_orders: Vec<GenoaOrder>,
    sell_orders: Vec<GenoaOrder>,
}

impl GenoaMarket {
    pub fn new(config: &Config) -> GenoaMarket {
        GenoaMarket {
            price_history: IntoIter::new([config.market.initial_price]).collect(),
            price_history_count: config.market.price_history_count,
            volatility: 0.0,
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    /// Call this after all orders have been submitted, this will execute orders, as well as
    /// computing a new price and volatility.
    pub fn step(&mut self, agents: &mut AgentCollection) {
        // TODO: (FIXME) Agents submit buy orders with a price not an amount.
        self.sort_orders();

        let (price, amount_executed) = self.compute_price();

        self.execute_buy_orders(amount_executed, agents, price);
        self.execute_sell_orders(amount_executed, agents, price);

        self.compute_volatility();
    }

    fn compute_volatility(&mut self) {
        let num_log_returns = (self.price_history.len() - 1) as f32;
        let log_returns = self
            .price_history
            .iter()
            .zip(self.price_history.iter().skip(1))
            .map(|(n, np1)| (np1 / n).log10());
        let log_return_average = log_returns.clone().sum::<f32>() / num_log_returns;
        let volatility = log_returns
            .map(|r| {
                let diff = r - log_return_average;
                diff * diff
            })
            .sum::<f32>()
            .div(num_log_returns - 1.0)
            .sqrt();
        self.volatility = volatility;
    }

    fn execute_sell_orders(
        &mut self,
        mut amount_executed: u32,
        agents: &mut AgentCollection,
        price: f32,
    ) {
        for so in &self.sell_orders {
            let agent = agents.agent_mut(so.agent);
            if amount_executed > so.asset_quantity {
                agent.sell(so.asset_quantity, price);
                amount_executed -= so.asset_quantity;
            } else {
                agent.sell(amount_executed, price);
                break;
            }
        }
    }

    fn execute_buy_orders(
        &mut self,
        mut amount_executed: u32,
        agents: &mut AgentCollection,
        price: f32,
    ) {
        for bo in &self.buy_orders {
            let agent = agents.agent_mut(bo.agent);
            if amount_executed > bo.asset_quantity {
                agent.buy(bo.asset_quantity, price);
                amount_executed -= bo.asset_quantity;
            } else {
                agent.buy(amount_executed, price);
                break;
            }
        }
    }

    /// This function assumes thath the orders are sortet
    fn compute_price(&mut self) -> (f32, u32) {
        let mut bos = self.buy_orders.iter();
        let mut bos_sum = 0;
        let mut bos_price = 0.0;
        let mut sos = self.buy_orders.iter();
        let mut sos_sum = 0;
        let mut sos_price = f32::INFINITY;
        let (price, amount_executed) = loop {
            match bos_sum.cmp(&sos_sum) {
                Ordering::Less => {
                    let bo = bos.next().unwrap();
                    bos_sum += bo.asset_quantity;
                    bos_price = bo.limit_price;
                }
                Ordering::Greater => {
                    let so = sos.next().unwrap();
                    sos_sum += so.asset_quantity;
                    sos_price = so.limit_price;
                }
                Ordering::Equal => {
                    let bo = bos.next().unwrap();
                    let so = sos.next().unwrap();
                    bos_price = bo.limit_price;
                    sos_sum += so.asset_quantity;
                    sos_price = so.limit_price;
                    bos_sum += bo.asset_quantity;
                }
            }

            if bos_price > sos_price {
                break ((bos_price + sos_price) / 2.0, sos_sum.min(bos_sum));
            }
        };
        self.record_price(price);
        (price, amount_executed)
    }

    fn sort_orders(&mut self) {
        self.buy_orders
            .sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());
        self.sell_orders
            .sort_by(|a, b| b.limit_price.partial_cmp(&a.limit_price).unwrap());
    }

    fn record_price(&mut self, price: f32) {
        self.price_history.push_front(price);
        while self.price_history.len() > self.price_history_count {
            self.price_history.pop_back();
        }
    }

    pub fn sell_order(&mut self, agent: AgentId, asset_quantity: u32) {
        self.sell_orders.push(GenoaOrder {
            agent,
            asset_quantity,
            limit_price: self.price()
                / rand_distr::Normal::new(1.01, 3.5 * self.volatility)
                    .unwrap()
                    .sample(&mut rand::thread_rng()),
        })
    }

    fn price(&mut self) -> f32 {
        self.price_history[0]
    }
}

#[derive(Debug, Clone)]
pub struct GenoaOrder {
    agent: AgentId,
    asset_quantity: u32,
    limit_price: f32,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_genoa_market_step() {
        let mut market = GenoaMarket::new(&Config::default());
        market.sell_orders.push(GenoaOrder {
            agent: 0,
            asset_quantity: 2,
            limit_price: 0.9,
        });
        market.sell_orders.push(GenoaOrder {
            agent: 1,
            asset_quantity: 8,
            limit_price: 0.95,
        });
        market.sell_orders.push(GenoaOrder {
            agent: 2,
            asset_quantity: 1,
            limit_price: 0.8,
        });
        market.buy_orders.push(GenoaOrder {
            agent: 3,
            asset_quantity: 2,
            limit_price: 1.1,
        });
        market.buy_orders.push(GenoaOrder {
            agent: 4,
            asset_quantity: 2,
            limit_price: 1.15,
        });
        market.buy_orders.push(GenoaOrder {
            agent: 5,
            asset_quantity: 5,
            limit_price: 1.05,
        });
        println!("{:?}", &market);
        market.step(todo!());
        println!("{:?}", &market);
    }
}
