use models::*;

#[derive(Debug, Clone)]
pub struct TradeBucket {
    pub trades: Vec<Trade>,
    pub averaged_trade: Trade,
}

pub fn trade_buckets(trades: Vec<Trade>) -> Vec<TradeBucket> {
    let mut trade_buckets:Vec<TradeBucket> = Vec::new();
    let mut current_trade_group:Vec<Trade> = Vec::new();
    let mut trade_iterator = trades.iter().peekable();

    for trade in trade_iterator.clone() {
        current_trade_group.push(*trade);

        if let Some(next_trade) = trade_iterator.peek() {
            if next_trade.buy != trade.buy {
                trade_buckets.push(TradeBucket{
                    trades:             current_trade_group.clone(),
                    averaged_trade:     averaged_trade(current_trade_group),
                });
                current_trade_group = Vec::new();
            }
        }
    }

    trade_buckets
}
