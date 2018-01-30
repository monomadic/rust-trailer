#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub cost: f64,
    pub qty: f64,
    pub buy: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

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
