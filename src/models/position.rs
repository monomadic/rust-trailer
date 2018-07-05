use models::*;

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol:                     String,
    pub trade_type:                 TradeType,
    pub qty:                        f64,
    pub cost_btc:                   f64,
    pub cost_usd:                   f64,
    pub sale_price:                 f64,
    pub current_price:              f64,
    pub potential_profit_btc:       f64,
    pub potential_profit_percent:   f64,
    pub potential_profit_usd:       f64,

    // pub realised_profit:    f64,
}

impl Position {
    pub fn calculate(orders: Vec<Order>, symbol: &str, price: f64, btc_price: f64) -> Vec<Position> {
        orders.iter().map(|order| {
            let cost_btc = order.qty * order.price;
            let potential_profit_btc = match order.order_type {
                TradeType::Buy => (order.qty * price) - cost_btc,
                TradeType::Sell => cost_btc - (order.qty * price),
            };

            Position {
                symbol:                     symbol.to_string(),
                trade_type:                 order.order_type,
                cost_btc:                   cost_btc,
                cost_usd:                   order.price * order.qty * btc_price,
                qty:                        order.qty,
                sale_price:                 order.price,
                current_price:              price,
                potential_profit_btc:       potential_profit_btc,
                potential_profit_percent:   -(100. - 100. / order.price * price),
                potential_profit_usd:       potential_profit_btc * btc_price
            }
        }).collect()
    }

    // pub fn unrealised_profit_usd(&self, usd_price: f64) -> f64 { }
    // pub fn cost_usd(&self, usd_price: f64) -> f64 { (price * order.qty) * btc_price }
}
