use models::*;

#[derive(Debug, Clone)]
pub struct Position {
	pub orders: Vec<Order>,
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
	pub fn symbol(&self) -> String { self.orders.first().unwrap().symbol.clone() }

	pub fn entry_price(&self) -> f64 {
		self.buy_orders().into_iter().map(|o|o.price).sum::<f64>() / self.buy_orders().len() as f64
	}

	pub fn exit_price(&self) -> Option<f64> {
		if self.sell_orders().len() > 0 {
			Some(self.sell_orders().into_iter().map(|o|o.price).sum())
		} else { None }
	}

	pub fn buy_qty(&self) -> f64 { self.buy_orders().into_iter().map(|o|o.qty).sum() }
	pub fn sell_qty(&self) -> f64 { self.sell_orders().into_iter().map(|o|o.qty).sum() }

	pub fn buy_cost(&self) -> f64 { self.entry_price() * self.buy_qty() }
	pub fn sell_cost(&self) -> f64 { self.exit_price().unwrap_or(0.0) * self.sell_qty() }

	// todo: memoize
	pub fn compact_orders(&self) -> Vec<Order> {
		Order::compact(self.orders.clone())
	}

	// todo: memoize
	pub fn buy_orders(&self) -> Vec<Order> {
		self.orders.clone().into_iter().filter(|o| o.order_type == TradeType::Buy).collect()
	}

	// todo: memoize
	pub fn sell_orders(&self) -> Vec<Order> {
		self.orders.clone().into_iter().filter(|o| o.order_type == TradeType::Sell).collect()
	}
	
	pub fn remaining_quantity(&self) -> f64 {
		if self.state() == PositionState::Closed {
			self.buy_qty()
		} else {
			self.buy_qty() - self.sell_qty()
		}
	}

	pub fn state(&self) -> PositionState {
		derive_state(self.buy_qty(), self.sell_qty())
	}

	pub fn new(orders: Vec<Order>) -> Vec<Position> {
		group_orders_by_positions(orders).into_iter().map(|order_group| {
			Position { orders: order_group }
		}).collect()
	}
}

pub fn group_orders_by_positions(orders: Vec<Order>) -> Vec<(Vec<Order>)> {
	let mut positions = Vec::new();
	let mut current_orders:Vec<Order> = Vec::new();
	let mut orders:Vec<Order> = orders.into_iter().rev().collect();

	while let Some(last_order) = orders.pop() {
		match last_order.order_type {
			TradeType::Buy => {
				// if the list contains sells, and we've encountered a buy, lets toss it
				if current_orders.clone().into_iter().filter(|o|o.order_type == TradeType::Sell).collect::<Vec<Order>>().len() > 0 {
					positions.push(current_orders.clone());
					current_orders = Vec::new();
				}
			},
			TradeType::Sell => {
			},
		}
		current_orders.push(last_order.clone());
	};

	positions.push(current_orders.clone());
	positions
}

pub fn derive_state(buy_qty: f64, sell_qty: f64) -> PositionState {
	if sell_qty == 0.0 { return PositionState::Open };
	if buy_qty == sell_qty { return PositionState::Closed };
	if sell_qty < buy_qty { return PositionState::Partial };
	PositionState::Irreconciled
}

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
	if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
	else { -(100. + -100. / entry_price * exit_price) }
}

fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
    Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
}

#[test]
fn test_group_orders_by_positions_1() {
    let orders = group_orders_by_positions(vec![
        order_fixture(TradeType::Buy, 10.0, 100.0)
    ]);

    assert_eq!(orders.len(), 1);
    assert_eq!(orders.first().unwrap().len(), 1);
}

#[test]
fn test_group_orders_by_positions_2() {
    let orders = group_orders_by_positions(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 2.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);
    assert_eq!(orders.first().unwrap().len(), 2);
}

#[test]
fn test_group_orders_by_positions_3() {
    let orders = group_orders_by_positions(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 2.0, 100.0),
        order_fixture(TradeType::Sell, 3.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);
    assert_eq!(orders.first().unwrap().len(), 3);
}

#[test]
fn test_group_orders_by_positions_4() {
    let orders = group_orders_by_positions(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Sell, 2.0, 100.0),
        order_fixture(TradeType::Buy, 3.0, 100.0),
    ]);

    assert_eq!(orders.len(), 2);
    assert_eq!(orders.first().unwrap().len(), 2);
    assert_eq!(orders.last().unwrap().len(), 1);
}

#[test]
fn test_group_orders_by_positions_5() {
    let orders = group_orders_by_positions(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Sell, 2.0, 100.0),
        order_fixture(TradeType::Buy, 3.0, 100.0),
        order_fixture(TradeType::Sell, 4.0, 100.0),
        order_fixture(TradeType::Buy, 5.0, 100.0),
    ]);

    assert_eq!(orders.len(), 3);

    let first_order = orders.first().unwrap();
    let last_order = orders.last().unwrap();

    assert_eq!(first_order.len(), 2);
    assert_eq!(last_order.len(), 1);
}

#[test]
fn test_positions_1() {
    let positions = Position::new(vec![
        order_fixture(TradeType::Buy, 10.0, 100.0)
    ]);

    assert_eq!(positions.len(), 1);


    let first_position = positions.first().unwrap();
    println!("{:?}", first_position.buy_orders());
    assert_eq!(first_position.orders.len(), 1);
    assert_eq!(first_position.buy_orders().len(), 1);
    assert_eq!(first_position.buy_qty(), 10.0);
    assert_eq!(first_position.entry_price(), 100.0);
    assert_eq!(first_position.exit_price(), None);
    assert_eq!(first_position.buy_qty(), 10.0);
}

#[test]
fn test_positions_2() {
    let positions = Position::new(vec![
        order_fixture(TradeType::Buy, 10.0, 100.0),
        order_fixture(TradeType::Buy, 10.0, 200.0),
    ]);

    assert_eq!(positions.len(), 1);

    let first_position = positions.first().unwrap();
    assert_eq!(first_position.orders.len(), 2);
    assert_eq!(first_position.buy_orders().len(), 2);
    assert_eq!(first_position.sell_orders().len(), 0);
    assert_eq!(first_position.buy_qty(), 20.0);
    assert_eq!(first_position.entry_price(), 150.0);
    assert_eq!(first_position.exit_price(), None);
    assert_eq!(first_position.buy_qty(), 20.0);
    assert_eq!(first_position.sell_qty(), 0.0);
}

#[test]
fn test_positions_3() {
    let positions = Position::new(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Sell, 2.0, 100.0),
        order_fixture(TradeType::Buy, 3.0, 100.0),
        order_fixture(TradeType::Sell, 4.0, 100.0),
        order_fixture(TradeType::Buy, 5.0, 100.0),
        order_fixture(TradeType::Buy, 6.0, 200.0),
    ]);

    assert_eq!(positions.len(), 3);

    let first_position = positions.first().unwrap();
    assert_eq!(first_position.buy_orders().len(), 1);
    assert_eq!(first_position.buy_qty(), 1.0);

    let last_position = positions.last().unwrap();
    assert_eq!(last_position.orders.len(), 2);
    assert_eq!(last_position.buy_orders().len(), 2);
    assert_eq!(last_position.buy_qty(), 11.0);
}