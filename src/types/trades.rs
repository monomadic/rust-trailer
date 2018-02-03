#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub cost: f64,
    pub qty: f64,
    pub buy: bool,
}

#[derive(Debug, Clone)]
pub struct TradeBucket {
    pub trades: Vec<Trade>,
    pub closed: bool,
    pub average_buy: f64,
    pub average_sell: f64,
    pub profit: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_qty() {
        assert_eq!(20.0, sum_qty(vec![
            ::types::Trade{ cost: 10., qty: 10.0, buy: true },
            ::types::Trade{ cost: 20., qty: 10.0, buy: true },
        ]));
        assert_eq!(10.0, sum_qty(vec![
            ::types::Trade{ cost: 10., qty: 10.0, buy: true },
            ::types::Trade{ cost: 20., qty: 10.0, buy: true },
            ::types::Trade{ cost: 10., qty: 10.0, buy: false },
        ]));
    }

    #[test]
    fn test_average_cost() {
        assert_eq!(15.0, average_cost(vec![
            ::types::Trade{ cost: 10., qty: 1.0, buy: true },
            ::types::Trade{ cost: 20., qty: 1.0, buy: true },
        ]));
        assert_eq!(17.5, average_cost(vec![
            ::types::Trade{ cost: 10., qty: 10.0, buy: true },
            ::types::Trade{ cost: 20., qty: 30.0, buy: true },
        ]));
    }

    #[test]
    fn test_average_trades() {
        let result = average_trades(vec![
            ::types::Trade{ cost: 10., qty: 10.0, buy: true },
            ::types::Trade{ cost: 20., qty: 10.0, buy: true },
        ]);
        assert_eq!(1, result.len());

        let test_value = result.first().unwrap();
        assert_eq!(15.0, test_value.cost);
        assert_eq!(20.0, test_value.qty);
        assert_eq!(true, test_value.buy);
    }
}
pub fn sum_qty(trades: Vec<Trade>) -> f64 {
    trades.into_iter().map(|trade| {
        if trade.buy { trade.qty }
        else { -trade.qty }
    }).sum()
}

pub fn sum_cost(trades: Vec<Trade>) -> f64 {
    trades.into_iter().map(|trade| {
        if trade.buy { trade.cost * trade.qty }
        else { -(trade.cost * trade.qty) }
    }).sum()
}

pub fn average_cost(trades: Vec<Trade>) -> f64 {
    let average:f64 = trades.clone().into_iter().map(|trade| {
        trade.qty * trade.cost
    }).sum();
    average / sum_qty(trades)
}

pub fn trade_buckets(trades: Vec<Trade>) -> Vec<TradeBucket> {
    let mut current_trade_bucket:Vec<Trade> = Vec::new();
    let mut buckets = Vec::new();

    for trade in group_trades(trades) {
        current_trade_bucket.push(trade);
        // println!("{:?}", sum_qty(current_trade_bucket.clone()));
        let is_closed = sum_qty(current_trade_bucket.clone()) <= 0.05;

        let buys:Vec<Trade> = current_trade_bucket.clone().into_iter().filter(|t| t.buy ).collect();
        let sells:Vec<Trade> = current_trade_bucket.clone().into_iter().filter(|t| !t.buy ).collect();
        let profit = sum_qty(sells.clone()) - sum_qty(buys.clone());

        if is_closed {
            buckets.push(TradeBucket {
                trades: current_trade_bucket.clone(),
                closed: true,
                average_buy: average_cost(buys),
                average_sell: average_cost(sells),
                profit: profit,
            });
        }

    };

    buckets

    // let mut buckets = Vec::new();
    // let mut current_trade = Trade{ cost: trades.first().unwrap().cost, qty: 0.0, buy: trades.first().unwrap().buy };

    // for trade in trades.clone() {
    //     if trade.buy == current_trade.buy {
    //         current_trade.cost = ((trade.cost * trade.qty) + (current_trade.cost * current_trade.qty)) / (trade.qty + current_trade.qty);
    //         current_trade.qty += trade.qty;
    //     } else {
    //         buckets.push(current_trade.clone());
    //         current_trade = trade.clone();
    //     }
    // }
    // buckets.push(current_trade.clone());
    // buckets
}

/// average together buys and sells into 1 reduction each
pub fn average_trades(trades: Vec<Trade>) -> Vec<Trade> {
    let mut grouped_trades = Vec::new();
    let mut current_trade = Trade{ cost: trades.first().unwrap().cost, qty: 0.0, buy: trades.first().unwrap().buy };

    for trade in trades.clone() {
        if trade.buy == current_trade.buy {
            current_trade.cost = ((trade.cost * trade.qty) + (current_trade.cost * current_trade.qty)) / (trade.qty + current_trade.qty);
            current_trade.qty += trade.qty;
        } else {
            grouped_trades.push(current_trade.clone());
            current_trade = trade.clone();
        }
    }
    grouped_trades.push(current_trade.clone());
    grouped_trades
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_group_trades() {
//         assert_eq!(group_trades(vec![
//                 ::types::Trade{ cost: 10., qty: 10.0, buy: true },
//                 ::types::Trade{ cost: 10., qty: 10.0, buy: true },
//             ]),
//             vec![::types::Trade{ cost: 10., qty: 20.0, buy: true }]
//         );
//     }
// }

pub fn group_trades(trades: Vec<Trade>) -> Vec<Trade> {
    let mut grouped_trades = Vec::new();
    let mut current_trade = Trade{ cost: trades.first().unwrap().cost, qty: 0.0, buy: trades.first().unwrap().buy };

    for trade in trades.clone() {
        if trade.cost == current_trade.cost && trade.buy == current_trade.buy {
            current_trade.qty += trade.qty;
        } else {
            grouped_trades.push(current_trade.clone());
            current_trade = trade.clone();
        }
    }
    grouped_trades.push(current_trade.clone());

    // println!("{:?}", trades);
    // println!("{:?}", grouped_trades);

    grouped_trades
}
