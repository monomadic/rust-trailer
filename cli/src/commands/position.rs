use trailer;
use trailer::error::TrailerError;

pub fn positions(client: Box<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;

    let mut output_buffer = String::new();

    for pair in pairs {
        let orders = client.past_trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            let price = *(prices.get(&pair).unwrap_or(&0.0));

            let grouped_orders = trailer::models::average_orders(orders.clone());
            let positions = trailer::models::Position::calculate(grouped_orders, price, btc_price, None);
            let acc_positions = trailer::models::PositionAccumulated::calculate(positions);

            if let Some(position) = acc_positions.last() {
                output_buffer.push_str(&::display::position_accumulated::row(position.clone()));
                output_buffer.push_str("\n");
            }
        }
    }

    Ok(output_buffer)
}

// let fn position(client: Box<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, is_compact: bool) -> Result<String, TrailerError> {

// }
