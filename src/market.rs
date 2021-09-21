use crate::agent::AgentId;

#[derive(Debug, Clone)]
pub struct GenoaMarket {
    price_history: Vec<f32>,
    volatility: f32,
    buy_orders: Vec<GenoaBuyOrder>,
    sell_orders: Vec<GenoaSellOrder>,
}

impl GenoaMarket {
    pub fn new(initial_price: f32) -> GenoaMarket {
        GenoaMarket {
            price_history: vec![initial_price],
            volatility: 0.0,
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    /// Call this after all orders have been submitted, this will execute orders, as well as
    /// computing a new price and volatility.
    pub fn step(&mut self) {
        self.buy_orders.sort_by(|a, b| b.limit_price.partial_cmp(&a.limit_price).unwrap());
        self.sell_orders.sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());
    }
}

#[derive(Debug, Clone)]
pub struct GenoaBuyOrder {
    agent: AgentId,
    asset_quantity: u32,
    limit_price: f32,
}

#[derive(Debug, Clone)]
pub struct GenoaSellOrder {
    agent: AgentId,
    asset_quantity: u32,
    limit_price: f32,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_genoa_market_step() {
        let mut market = GenoaMarket::new(1.);
        market.sell_orders.push(GenoaSellOrder { agent: 0, asset_quantity: 2, limit_price: 0.9 });
        market.sell_orders.push(GenoaSellOrder { agent: 1, asset_quantity: 8, limit_price: 0.95 });
        market.sell_orders.push(GenoaSellOrder { agent: 2, asset_quantity: 1, limit_price: 0.8 });
        market.buy_orders.push(GenoaBuyOrder { agent: 3, asset_quantity: 2, limit_price: 1.1 });
        market.buy_orders.push(GenoaBuyOrder { agent: 4, asset_quantity: 2, limit_price: 1.15 });
        market.buy_orders.push(GenoaBuyOrder { agent: 5, asset_quantity: 5, limit_price: 1.05 });
        println!("{:?}", &market);
        market.step();
        println!("{:?}", &market);
        panic!();
    }
}
