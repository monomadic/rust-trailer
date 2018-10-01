use trailer;
use trailer::error::TrailerError;

use trailer::models::*;
use trailer::presenters::*;

pub fn positions(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, show_all: bool) -> Result<Vec<Result<PositionPresenter, String>>, TrailerError> {
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;
    let mut presenters:Vec<Result<PositionPresenter, String>> = Vec::new();

    for pair in pairs.clone() {
        let orders = client.trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            if pair == "BNBBTC".to_string() { continue };

            let price = *(prices.get(&pair).unwrap_or(&0.0));
            let grouped_orders = trailer::models::average_orders(orders.clone());
            let positions = trailer::models::Position::new(grouped_orders);

            if pairs.len() == 1 { // if just one symbol was supplied
                // print the entire history
                for (index, position) in positions.into_iter().enumerate() {
                    if index == 0 || show_all || (position.state() != PositionState::Irreconciled && position.state() != PositionState::Partial) {
                        presenters.push(Ok(PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price }));
                    }
                }
            } else {
                // filter out closed positions as first shown entry (as they are less irrelevant)
                let positions:Vec<Position> = positions.into_iter().filter(|p|
                    // p.symbol != "BNBBTC".to_string() &&
                    p.state() != PositionState::Closed &&
                    p.state() != PositionState::Irreconciled
                ).collect();

                if let Some(position) = positions.first() {
                    presenters.push(Ok(PositionPresenter{ position: position.clone(), current_price: price, btc_price_in_usd: btc_price }));
                } else {
                    // todo push error here.
                    presenters.push(Err(format!("error finding position for: {}", pair)))
                }
            }
        }
    }

    Ok(presenters)
}

// pub fn positions(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, show_all: bool) -> Result<String, TrailerError> {
//     let prices = client.prices()?;
//     let btc_price = client.btc_price()?;
//     let mut output_buffer = ::display::position::row_title();

//     for pair in pairs.clone() {
//         let orders = client.past_trades_for(&pair);

//         if let Ok(orders) = orders {  // ok to swallow error here. not critical.
//             let price = *(prices.get(&pair).unwrap_or(&0.0));

//             let grouped_orders = trailer::models::average_orders(orders.clone());

//             let positions = trailer::models::Position::new(grouped_orders);

//             if pairs.len() == 1 { // if just one symbol was supplied
//                 // print the entire history
//                 for (index, position) in positions.into_iter().enumerate() {
//                     if index == 0 || show_all || (position.state() != PositionState::Irreconciled && position.state() != PositionState::Partial) {
//                         let presenter = PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price };
//                         output_buffer.push_str(&::display::position::row(presenter));
//                     }
//                 }
//             } else {
//                 // filter out closed positions as first shown entry (as they are less irrelevant)
//                 let positions:Vec<Position> = positions.into_iter().filter(|p| p.state() != PositionState::Closed).collect();
//                 if let Some(position) = positions.first() {
//                     let presenter = PositionPresenter{ position: position.clone(), current_price: price, btc_price_in_usd: btc_price };
//                     output_buffer.push_str(&::display::position::row(presenter));
//                 } else {
//                     output_buffer.push_str(&format!("error finding position for: {}", pair));
//                 }
//                 // let position = &*positions.first().expect("last position error");
//                 // else just print the last position
//             }


//             // if let Some(position) = position {
//             //     let presenter = PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price };
//             //     output_buffer.push_str(&::display::position::row(presenter));
//             // }

//             // let acc_positions = trailer::models::PositionAccumulated::calculate(positions);

//             // use ::trailer::models::PositionAccumulated;
//             // let limited_positions:Vec<PositionAccumulated> = acc_positions.into_iter().rev().take(2).collect::<Vec<PositionAccumulated>>().into_iter().rev().collect();

//             // // if let Some(position) = acc_positions.last() {
//             // for position in limited_positions {
//             //     // output_buffer.push_str(&::display::position_accumulated::row(position.clone()));
//             //     // output_buffer.push_str("\n");

//             //     if position.position.cost_btc >= 0.001 {
//             //         println!("{}", ::display::position_accumulated::row(position.clone()));
//             //         unsorted_positions.push(position.clone());
//             //     }

//             // }
//         }
//     }

//     // println!("\nSORTED POSITIONS:");

//     // // move this sort to the model
//     // // let mut open_positions:Vec<trailer::models::PositionAccumulated> = unsorted_positions.into_iter().filter(|position| position.position.potential_profit_percent > 0.0).collect();
//     // unsorted_positions.sort_by(|a, b| a.position.change_as_percent().partial_cmp(&b.position.change_as_percent()).expect("sort failed"));

//     // for ap in unsorted_positions.clone() {
//     //     println!("{}", ::display::position_accumulated::row(ap));
//     // }

//     // use ::trailer::models::*;
//     // println!("\nTotal Profit: ${:.2}", PositionAccumulated::total_potential_profit_usd(unsorted_positions));

//     Ok(output_buffer)
// }

// pub fn position_historic(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, symbol: &str) -> Result<String, TrailerError> {
//     return Ok("historic".to_string())
// }

// let fn position(client: Box<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {

// }
