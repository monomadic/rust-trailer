use trailer;
use trailer::error::TrailerError;

use trailer::presenters::*;

pub fn positions(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;
    let mut output_buffer = ::display::position::row_title();

    for pair in pairs {
        let orders = client.past_trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            let price = *(prices.get(&pair).unwrap_or(&0.0));

            let grouped_orders = trailer::models::average_orders(orders.clone());
            // let positions = trailer::models::Position::calculate(grouped_orders, price, btc_price, None);
            let position = trailer::models::Position::new(grouped_orders);

            if let Some(position) = position {
                let presenter = PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price };
                output_buffer.push_str(&::display::position::row(presenter));
            }

            // let acc_positions = trailer::models::PositionAccumulated::calculate(positions);

            // use ::trailer::models::PositionAccumulated;
            // let limited_positions:Vec<PositionAccumulated> = acc_positions.into_iter().rev().take(2).collect::<Vec<PositionAccumulated>>().into_iter().rev().collect();

            // // if let Some(position) = acc_positions.last() {
            // for position in limited_positions {
            //     // output_buffer.push_str(&::display::position_accumulated::row(position.clone()));
            //     // output_buffer.push_str("\n");

            //     if position.position.cost_btc >= 0.001 {
            //         println!("{}", ::display::position_accumulated::row(position.clone()));
            //         unsorted_positions.push(position.clone());
            //     }

            // }
        }
    }

    // println!("\nSORTED POSITIONS:");

    // // move this sort to the model
    // // let mut open_positions:Vec<trailer::models::PositionAccumulated> = unsorted_positions.into_iter().filter(|position| position.position.potential_profit_percent > 0.0).collect();
    // unsorted_positions.sort_by(|a, b| a.position.change_as_percent().partial_cmp(&b.position.change_as_percent()).expect("sort failed"));

    // for ap in unsorted_positions.clone() {
    //     println!("{}", ::display::position_accumulated::row(ap));
    // }

    // use ::trailer::models::*;
    // println!("\nTotal Profit: ${:.2}", PositionAccumulated::total_potential_profit_usd(unsorted_positions));

    Ok(output_buffer)
}

pub fn position_historic(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, symbol: &str) -> Result<String, TrailerError> {
    return Ok("historic".to_string())
}

// let fn position(client: Box<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {

// }
