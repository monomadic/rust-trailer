use trailer;
use trailer::error::TrailerError;

pub fn positions(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;

    let output_buffer = String::new();

    let mut unsorted_positions = Vec::new();

    for pair in pairs {
        let orders = client.past_trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            let price = *(prices.get(&pair).unwrap_or(&0.0));

            let grouped_orders = trailer::models::average_orders(orders.clone());

            trailer::models::PositionAccumulated::calc(grouped_orders.clone());

            let positions = trailer::models::Position::calculate(grouped_orders, price, btc_price, None);
            let acc_positions = trailer::models::PositionAccumulated::calculate(positions);

            if let Some(position) = acc_positions.last() {
                // output_buffer.push_str(&::display::position_accumulated::row(position.clone()));
                // output_buffer.push_str("\n");

                if position.position.cost_btc >= 0.001 {
                    println!("{}", ::display::position_accumulated::row(position.clone()));
                    unsorted_positions.push(position.clone());
                }

            }
        }
    }

    println!("\nWINNING POSITIONS:");

    for ap in unsorted_positions.into_iter().filter(|position| position.position.potential_profit_percent > 0.0) {
        println!("{}", ::display::position_accumulated::row(ap));
    }

    Ok(output_buffer)
}

pub fn position_historic(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, symbol: &str) -> Result<String, TrailerError> {
    return Ok("historic".to_string())
}

// let fn position(client: Box<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {

// }
