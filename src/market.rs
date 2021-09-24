use std::{cmp::Ordering, ops::Div};

use crate::{agent::AgentId, config::Config};

#[derive(Debug, Clone)]
pub struct GenoaMarket {
    price_history: Vec<f32>,
    volatility: f32,
    buy_orders: Vec<GenoaOrder>,
    sell_orders: Vec<GenoaOrder>,
}

impl GenoaMarket {
    pub fn new(config: &Config) -> GenoaMarket {
        GenoaMarket {
            price_history: vec![config.market.initial_price],
            volatility: 0.0,
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    /// Call this after all orders have been submitted, this will execute orders, as well as
    /// computing a new price and volatility.
    pub fn step(&mut self) {
        self.buy_orders
            .sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());
        self.sell_orders
            .sort_by(|a, b| b.limit_price.partial_cmp(&a.limit_price).unwrap());

        let mut bos = self.buy_orders.iter().peekable();
        let mut bos_sum = 0;
        let mut bos_price = 0.0;

        let mut sos = self.buy_orders.iter().peekable();
        let mut sos_sum = 0;
        let mut sos_price = f32::INFINITY;

        let price = loop {
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
                break (bos_price + sos_price) / 2.0;
            }
        };

        //TODO: Execute orders

        self.price_history.push(price);

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
        market.step();
        println!("{:?}", &market);
        panic!();
    }
}
