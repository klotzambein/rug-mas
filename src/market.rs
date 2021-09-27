use std::{array::IntoIter, cmp::Ordering, collections::VecDeque, ops::Div};

use rand::prelude::*;

use crate::{
    agent::{AgentCollection, AgentId},
    config::Config,
};

pub type MarketId = u32;

#[derive(Debug, Clone)]
pub struct GenoaMarket {
    id: MarketId,
    price_history: VecDeque<f32>,
    price_history_count: usize,
    volatility: f32,
    buy_orders: Vec<GenoaOrder>,
    sell_orders: Vec<GenoaOrder>,
}

impl GenoaMarket {
    pub fn new(config: &Config, id: MarketId) -> GenoaMarket {
        GenoaMarket {
            id,
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
        self.sort_orders();
        // println!("{:?}", (self.buy_orders.len(), self.sell_orders.len()));

        let (price, amount_executed) = self.compute_price().unwrap_or_else(|| (self.price(), 0));

        self.record_price(price);

        self.execute_buy_orders(amount_executed, agents, price);
        self.execute_sell_orders(amount_executed, agents, price);

        self.compute_volatility();

        self.buy_orders.clear();
        self.sell_orders.clear();
    }

    fn compute_volatility(&mut self) {
        if self.price_history.len() < 20 {
            // We need at least thre values to compute the log returns
            return;
        }

        let num_log_returns = 20.0;

        let log_returns = self.price_history.iter().rev().take(20).rev()
            .zip(self.price_history.iter().rev().take(20).rev().skip(1))
            .map(|(n, np1)| np1.ln() - n.ln());

        let log_return_average = log_returns.clone().sum::<f32>() / num_log_returns;
        // println!(
        //     "{:?}",
        //     (
        //         num_log_returns,
        //         log_returns.clone().collect::<Vec<_>>(),
        //         log_return_average
        //     )
        // );

        let volatility = log_returns
            .map(|r| {
                let diff = r - log_return_average;
                diff * diff
            })
            .sum::<f32>()
            .div(num_log_returns - 1.0)
            .sqrt();

        self.volatility = volatility;
        println!("{}", volatility);
    }

    /// This function assumes thath the orders are sortet
    fn execute_sell_orders(
        &mut self,
        mut amount_executed: u32,
        agents: &mut AgentCollection,
        price: f32,
    ) {
        for so in &self.sell_orders {
            let agent = agents.agent_mut(so.agent);
            if amount_executed > so.asset_quantity {
                agent.apply_sell(self.id, so.asset_quantity, price);
                amount_executed -= so.asset_quantity;
            } else {
                agent.apply_sell(self.id, amount_executed, price);
                break;
            }
        }
    }

    /// This function assumes thath the orders are sortet
    fn execute_buy_orders(
        &mut self,
        mut amount_executed: u32,
        agents: &mut AgentCollection,
        price: f32,
    ) {
        for bo in &self.buy_orders {
            let agent = agents.agent_mut(bo.agent);
            if amount_executed > bo.asset_quantity {
                agent.apply_buy(self.id, bo.asset_quantity, price);
                amount_executed -= bo.asset_quantity;
            } else {
                agent.apply_buy(self.id, amount_executed, price);
                break;
            }
        }
    }

    /// This function assumes that the orders are sortet and does not record the price
    fn compute_price(&mut self) -> Option<(f32, u32)> {
        let mut bos = self.buy_orders.iter();
        let bo0 = bos.next()?;
        let mut bos_sum = bo0.asset_quantity;
        let mut bos_price = bo0.limit_price;

        let mut sos = self.buy_orders.iter();
        let so0 = sos.next()?;
        let mut sos_sum = so0.asset_quantity;
        let mut sos_price = so0.limit_price;

        if bos_price < sos_price {
            return None; // No deal
        }

        loop {
            match bos_sum.cmp(&sos_sum) {
                Ordering::Less => {
                    let bo = if let Some(b) = bos.next() { b } else { break };
                    if bo.limit_price < sos_price {
                        break;
                    }
                    bos_sum += bo.asset_quantity;
                    bos_price = bo.limit_price;
                }
                Ordering::Equal | Ordering::Greater => {
                    let so = if let Some(s) = sos.next() { s } else { break };
                    if bos_price < so.limit_price {
                        break;
                    }
                    sos_sum += so.asset_quantity;
                    sos_price = so.limit_price;
                }
            }
        }

        let price = (bos_price + sos_price) / 2.0;
        let amount_executed = sos_sum.min(bos_sum);

        Some((price, amount_executed))
    }

    fn sort_orders(&mut self) {
        self.buy_orders.sort_by(|a, b| {
            b.limit_price
                .partial_cmp(&a.limit_price)
                .expect("found nan in buy limit_price")
        });
        self.sell_orders.sort_by(|a, b| {
            a.limit_price
                .partial_cmp(&b.limit_price)
                .expect("found nan in sell limit_price")
        });
    }

    fn record_price(&mut self, price: f32) {
        self.price_history.push_front(price);
        while self.price_history.len() > self.price_history_count {
            self.price_history.pop_back();
        }
    }

    pub fn sell_order(&mut self, agent: AgentId, asset_quantity: u32) {
        let limit_price = self.price()
            / rand_distr::Normal::new(1.01, 3.5 * self.volatility)
                .unwrap()
                .sample(&mut rand::thread_rng());
        self.sell_orders.push(GenoaOrder {
            agent,
            asset_quantity,
            limit_price,
        })
    }

    pub fn buy_order(&mut self, agent: AgentId, cash_quantity: f32) {
        let limit_price = self.price()
            / rand_distr::Normal::new(1.01, 3.5 * self.volatility)
                .unwrap()
                .sample(&mut rand::thread_rng());
        self.buy_orders.push(GenoaOrder {
            agent,
            limit_price,
            asset_quantity: (cash_quantity / limit_price) as u32,
        })
    }

    pub fn volatility(&self) -> f32 {
        self.volatility
    }

    pub fn price(&self) -> f32 {
        self.price_history[0]
    }

    /// Get the genoa market's id.
    pub fn id(&self) -> MarketId {
        self.id
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
    use core::panic;

    use super::*;

    #[test]
    fn test_genoa_market_step() {
        let mut market = GenoaMarket::new(&Config::default(), 0);
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
        market.sort_orders();
        println!("{:?}", &market);
        panic!()
    }
}
