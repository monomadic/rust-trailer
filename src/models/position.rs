use models::*;

#[derive(Debug, Clone)]
pub struct Position {
	pub symbol:                 String,
	pub buy_order:				Order,
	pub sell_order:				Option<Order>,

	// pub percent_change:			f64,

	// pub change_percent:				f64,


	// pub cost_btc:                   f64,
	// pub cost_usd:                   f64,

	// pub sale_price:                 f64,
	// pub current_price:              f64,

	// pub potential_profit_btc:       f64,
	// pub potential_profit_percent:   f64,
	// pub potential_profit_usd:       f64,

	// pub balance:                    f64,
	// pub held_btc:                f64,
	// pub held_percent:            f64,
	// pub realised_profit:         f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionState {
    Open,
    Partial,
    Closed,
    Irreconciled, // when things don't make sense
}

impl ::std::fmt::Display for PositionState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            PositionState::Open => write!(f, "OPEN"),
            PositionState::Partial => write!(f, "PART"),
            PositionState::Closed => write!(f, "CLOSED"),
            PositionState::Irreconciled => write!(f, "IRREC"),
        }
    }
}

impl Position {
	// pub fn change_as_percent(&self) -> f64 {
	// 	match self.trade_type {
	// 		TradeType::Buy => price_percent(self.sale_price, self.current_price),
	// 		TradeType::Sell => -price_percent(self.sale_price, self.current_price),
	// 	}
	// }

	pub fn entry_price(&self) -> f64 { self.buy_order.price }
	pub fn exit_price(&self) -> Option<f64> { if let Some(ref order) = self.sell_order { Some(order.price) } else { None } }
	pub fn size(&self) -> f64 { self.buy_order.qty }

	// pub fn percent_change(&self, current_price: f64) -> f64 { price_percent(self.buy_order.price, current_price) }

	pub fn state(&self) -> PositionState {
		derive_state(self.buy_order.qty, self.clone().sell_order.and_then(|o| Some(o.qty)).unwrap_or(0.0))
	}

	pub fn new(orders: Vec<Order>) -> Option<Position> {
		// reverse the array cause we want to work backwards.
		// let orders = orders.into_iter().rev().collect();
		let mut orders = Order::compact(orders);
		// let mut orders = orders;
		// println!("{:#?}", orders);

		if let Some(last_order) = orders.pop() {
			let (buy_order, sell_order) = match last_order.order_type {
				TradeType::Buy => (last_order, None),
				TradeType::Sell => {
					if let Some(second_last_order) = orders.pop() {
						(second_last_order, Some(last_order))
					} else {
						return None
					}
				},
			};

			return Some(Position {
				symbol:				buy_order.symbol.clone(),
				// size:				buy_order.qty,
				// entry_price:		buy_order.price,
				// percent_change:		price_percent(buy_order.price, current_price),
				buy_order:			buy_order,
				sell_order:			sell_order,
			})
		};

		None




		// println!("{:#?}", Order::group(orders));

		// let (state, unrealised_profit) = match last_order.order_type {
		// 	TradeType::Buy => { return None },
		// 	// 	(
		// 	// 	PositionState::Open,
		// 	// 	(order.qty * order.price) - (order.qty * current_price),
		// 	// ),
		// 	TradeType::Sell => (
		// 		PositionState::Closed,
		// 		(order.qty * current_price) - (order.qty * order.price),
		// 	),
		// };

		// let unrealised_profit = match last_order.order_type {
		// 	TradeType::Buy => (order.qty * order.price) - (order.qty * current_price),
		// 	TradeType::Sell => (order.qty * current_price) - (order.qty * order.price),
		// };




		// while order in orders.next() {
		// 	match order.trade_type {
		// 		TradeType::Sell => 
		// 	}
		// }
	}

	// pub fn calculate(orders: Vec<Order>, current_price: f64, btc_price: f64, current_balance: Option<f64>) -> Vec<Position> {
	// 	let balance = current_balance.unwrap_or(0.0);

	// 	orders.iter().map(|order| {
	// 		let cost_btc = order.qty * order.price;
			
	// 		let potential_profit_btc = match order.order_type {
	// 			TradeType::Buy => (order.qty * current_price) - cost_btc,
	// 			TradeType::Sell => cost_btc - (order.qty * current_price),
	// 		};

	// 		let potential_profit_percent = match order.order_type {
	// 			TradeType::Buy => price_percent(order.price, current_price),
	// 			TradeType::Sell => -price_percent(order.price, current_price),
	// 		};

	// 		Position {
	// 			symbol:                     order.symbol.to_string(),
	// 			trade_type:                 order.order_type,
	// 			cost_btc:                   cost_btc,
	// 			cost_usd:                   order.price * order.qty * btc_price,
	// 			qty:                        order.qty,
	// 			sale_price:                 order.price,
	// 			current_price:              current_price,
	// 			potential_profit_btc:       potential_profit_btc,
	// 			potential_profit_percent:   potential_profit_percent,
	// 			potential_profit_usd:       potential_profit_btc * btc_price,
	// 			balance:                   	balance,
	// 			// held_btc:                   balance * order.price,
	// 		}
	// 	}).collect()
	// }

	// pub fn unrealised_profit_usd(&self, usd_price: f64) -> f64 { }
	// pub fn cost_usd(&self, usd_price: f64) -> f64 { (price * order.qty) * btc_price }
}

pub fn derive_state(buy_qty: f64, sell_qty: f64) -> PositionState {
	if sell_qty == 0.0 { return PositionState::Open };
	if buy_qty == sell_qty { return PositionState::Closed };
	if sell_qty < buy_qty { return PositionState::Partial };
	PositionState::Irreconciled
}

// #[test]
// fn test_price_percent() {
// 	assert_eq!(-90.0, derive_state(100., 10.));
// 	assert_eq!(100.0, derive_state(100., 200.));
// 	assert_eq!(100.0, derive_state(20., 40.));
// 	assert_eq!(-50.0, derive_state(40., 20.));
// }

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
	if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
	else { -(100. + -100. / entry_price * exit_price) }
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_price_percent() {
// 		assert_eq!(-90.0, price_percent(100., 10.));
// 		assert_eq!(100.0, price_percent(100., 200.));
// 		assert_eq!(100.0, price_percent(20., 40.));
// 		assert_eq!(-50.0, price_percent(40., 20.));
// 	}

// 	#[test]
// 	fn test_position_summing() {
// 		let mut orders = Vec::new();
// 		orders.push(Order{
// 			id:             "test_id".to_string(),
// 			symbol:         "TESTCOIN".to_string(),
// 			order_type:     TradeType::Buy,
// 			qty:            10.0,
// 			price:          1000.0,
// 		});
// 		orders.push(Order{
// 			id:             "test_id".to_string(),
// 			symbol:         "TESTCOIN".to_string(),
// 			order_type:     TradeType::Sell,
// 			qty:            10.0,
// 			price:          1000.0,
// 		});

// 		let mut positions:Vec<Position> = Position::calculate(orders, 1100.0, 7000.0, None).into_iter().rev().collect();

// 		assert_eq!(2, positions.len());

// 		let first_position = positions.pop().unwrap();

// 		assert_eq!(1000.0, first_position.potential_profit_btc);
// 		assert_eq!(10.0, first_position.potential_profit_percent);

// 		let second_position = positions.pop().unwrap();
// 		assert_eq!(-1000.0, second_position.potential_profit_btc);
// 	}
// }

// fn reverse_vec<T>(vec: Vec<T>) -> Vec<T> {
// 	vec.into_iter().rev().collect()
// }