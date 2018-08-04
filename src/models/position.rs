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

	pub balance:                    f64,
	// pub held_btc:                f64,
	// pub held_percent:            f64,
	// pub realised_profit:         f64,
}

impl Position {
	pub fn calculate(orders: Vec<Order>, current_price: f64, btc_price: f64, current_balance: Option<f64>) -> Vec<Position> {
		let balance = current_balance.unwrap_or(0.0);

		orders.iter().map(|order| {
			let cost_btc = order.qty * order.price;
			
			let potential_profit_btc = match order.order_type {
				TradeType::Buy => (order.qty * current_price) - cost_btc,
				TradeType::Sell => cost_btc - (order.qty * current_price),
			};

			let potential_profit_percent = match order.order_type {
				TradeType::Buy => price_percent(order.price, current_price),
				TradeType::Sell => -price_percent(order.price, current_price),
			};

			Position {
				symbol:                     order.symbol.to_string(),
				trade_type:                 order.order_type,
				cost_btc:                   cost_btc,
				cost_usd:                   order.price * order.qty * btc_price,
				qty:                        order.qty,
				sale_price:                 order.price,
				current_price:              current_price,
				potential_profit_btc:       potential_profit_btc,
				potential_profit_percent:   potential_profit_percent,
				potential_profit_usd:       potential_profit_btc * btc_price,
				balance:                   	balance,
				// held_btc:                   balance * order.price,
			}
		}).collect()
	}

	// pub fn unrealised_profit_usd(&self, usd_price: f64) -> f64 { }
	// pub fn cost_usd(&self, usd_price: f64) -> f64 { (price * order.qty) * btc_price }
}

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
	if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
	else { -(100. + -100. / entry_price * exit_price) }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_price_percent() {
		assert_eq!(-90.0, price_percent(100., 10.));
		assert_eq!(100.0, price_percent(100., 200.));
		assert_eq!(100.0, price_percent(20., 40.));
		assert_eq!(-50.0, price_percent(40., 20.));
	}

	#[test]
	fn test_position_summing() {
		let mut orders = Vec::new();
		orders.push(Order{
			id:             "test_id".to_string(),
			symbol:         "TESTCOIN".to_string(),
			order_type:     TradeType::Buy,
			qty:            10.0,
			price:          1000.0,
		});
		orders.push(Order{
			id:             "test_id".to_string(),
			symbol:         "TESTCOIN".to_string(),
			order_type:     TradeType::Sell,
			qty:            10.0,
			price:          1000.0,
		});

		let mut positions:Vec<Position> = Position::calculate(orders, 1100.0, 7000.0, None).into_iter().rev().collect();

		assert_eq!(2, positions.len());

		let first_position = positions.pop().unwrap();

		assert_eq!(1000.0, first_position.potential_profit_btc);
		assert_eq!(10.0, first_position.potential_profit_percent);

		let second_position = positions.pop().unwrap();
		assert_eq!(-1000.0, second_position.potential_profit_btc);
	}
}

fn reverse_vec<T>(vec: Vec<T>) -> Vec<T> {
	vec.into_iter().rev().collect()
}