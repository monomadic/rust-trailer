use models::*;

#[derive(Debug, Clone)]
pub struct PositionAccumulated {
    pub position:           Position,
    pub size:               f64,
    pub entry_price:        f64,
    pub unrealised_pnl:     f64,
    pub realised_pnl:       f64,
}

// fn add_position(pos_acc: PositionAccumulated, pos: Position) -> PositionAccumulated {
//     let new_size = pos_acc.size + pos.qty;

//     PositionAccumulated {
//         position:           pos,
//         size:               new_size,
//         entry_price:        0.0,
//         unrealised_pnl:     0.0,
//         realised_pnl:       0.0,
//     }
// }

// fn average_positions(positions: Vec<Position>) -> PositionAccumulated {
//     positions.sum
// }

impl PositionAccumulated {
    pub fn calc(orders: Vec<Order>) -> Vec<PositionAccumulated> {
        let mut balance = 0.0;

        // println!("lskdjflkjd");

        for order in orders {

            match order.order_type {
                TradeType::Buy => {
                    balance += order.qty;
                },
                TradeType::Sell => {
                    balance -= order.qty;
                },
            };

            balance += match order.order_type {
                TradeType::Buy => order.qty,
                TradeType::Sell => -order.qty,
            };
            // println!("balance: {}", balance);

        };

        Vec::new()
    }

    pub fn calculate(positions: Vec<Position>) -> Vec<PositionAccumulated> {

        positions.iter().enumerate().map(|(_i, position)| {
            PositionAccumulated {
                position:           position.clone(),
                size:               0.0,
                entry_price:        0.0,
                unrealised_pnl:     0.0,
                realised_pnl:       0.0,
            }
        }).collect()

        // positions.into_iter().map(move |position| {
        //     cur_pos = add_position(*cur_pos, position);
        //     *cur_pos
        // }).collect()



        // let mut iter = positions.into_iter().peekable();
        // let mut acc_positions = Vec::new();

        // if let first_position = acc_positions.first() {
        //     acc_positions.push(PositionAccumulated{
        //         position: 
        //     }

        //     while let Some(position) = iter.next() {
        //         acc_positions.push(
        //             PositionAccumulated{
        //                 position:       position.clone(),
        //                 exit_price_btc: 0.0,
        //                 btc_profit:     calc_btc_profit(position.clone(), iter.peek()),
        //                 usd_profit:     calc_usd_profit(position.clone(), iter.peek()),
        //                 percent_profit: calc_percent_profit(position.clone(), iter.peek()),
        //         });
        //     };
        // }


        // acc_positions

        // iter.map(|p|
        //     PositionAccumulated{
        //         position:       p.clone(),
        //         profit:         calc_profit(p.clone(), iter.peek()),
        //     }
        // ).collect()



        // positions.iter().map(|p|
        //     // println!("{:?}", p.peek());

        //     PositionAccumulated{
        //         position:     p.clone(),
        //         profit:     5.0,
        //     }
        // ).collect()

        // Vec::new()
    }
}

// fn exit_position(position: Option<&Position>) -> f64 {
//     if let Some(exit_pos) = exit_position {
//         exit_position.cost_btc, exit_position.
//     } else {
//         0.0
//     }
// }

fn calc_btc_profit(entry_position: Position, exit_position: Option<&Position>) -> f64 {
    if let Some(exit_pos) = exit_position {
        match entry_position.trade_type {
            TradeType::Buy => (exit_pos.cost_btc - entry_position.cost_btc),
            TradeType::Sell => (entry_position.cost_btc - exit_pos.cost_btc),
        }
    } else {
        0.0
    }
}

// fn calc_usd_profit(entry_position: Position, exit_position: Option<&Position>) -> f64 {
//     if let Some(exit_pos) = exit_position {
//         (entry_position.cost_usd - exit_pos.cost_usd)
//     } else {
//         0.0
//     }
// }

// fn calc_percent_profit(entry_position: Position, exit_position: Option<&Position>) -> f64 {
//     if let Some(exit_pos) = exit_position {
//         99.0
//     } else {
//         0.0
//     }
// }

// #[test]
// fn test_calculate() {
//     let positions = [
//         Position { }
//     ]
//     assert_eq!(15.0, average_cost(vec![])
// }
