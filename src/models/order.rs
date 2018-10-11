use models::*;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub order_type: TradeType,
    pub qty: f64,
    pub price: f64,
}

impl Order {
    pub fn to_trade(&self) -> Trade {
        Trade {
            cost: self.price,
            qty: self.qty,
            buy: self.order_type.buy(),
            // order_type: TradeType,
        }
    }

    pub fn compact(orders: Vec<Order>) -> Vec<Order> {
        // println!("{:?}", group_orders(orders.clone()));
        // group_orders(orders).first().unwrap_or(&Vec::new()).clone()
        compact_orders(orders)
    }
}

// reduce all orders with same price together (technically different orders from the order book)
pub fn compact_orders(orders: Vec<Order>) -> Vec<Order> {
    let mut grouped_orders = Vec::new();
    let mut orders = orders;

    if let Some(first_order) = orders.pop() {
        let mut current_order = first_order.clone();

        for order in orders.clone() {
            if order.price == current_order.price {
                current_order.qty += order.qty;
            } else {
                grouped_orders.push(current_order.clone());
                current_order = order.clone();
            }
        }
        grouped_orders.push(current_order.clone());
    }
    grouped_orders
}

#[test]
fn test_compact_orders_1() {
    fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
        Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
    }

    let orders = compact_orders(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);
    assert_eq!(orders.first().unwrap().qty, 1.0);
}

#[test]
fn test_compact_orders_2() {
    fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
        Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
    }

    let orders = compact_orders(vec![
        order_fixture(TradeType::Buy, 1.0, 100.0),
        order_fixture(TradeType::Buy, 1.0, 100.0),
    ]);

    assert_eq!(orders.len(), 1);
    assert_eq!(orders.first().unwrap().qty, 2.0);
}

// group/average orders into a grouped vector by buy/sell type
// used by average orders - not really too useful directly.
pub fn group_orders(orders: Vec<Order>) -> Vec<Vec<Order>> {
    let mut grouped_orders = Vec::new();
    let mut current_order_group = Vec::new();

    if let Some(first_order) = orders.first() {
        let mut current_order_type = first_order.order_type;

        for order in orders.clone() {
            if order.order_type != current_order_type {
                grouped_orders.push(current_order_group.clone());
                current_order_group = Vec::new();
                current_order_type = order.order_type;
            }
            current_order_group.push(order);
        }
        grouped_orders.push(current_order_group);
    }
    grouped_orders
}

pub fn average_orders(orders: Vec<Order>) -> Vec<Order> {
    group_orders(orders).iter().map(|order_group| {
        average_order(order_group.to_vec())
    }).collect()
}

pub fn average_order(orders: Vec<Order>) -> Order {
    let mut first_order = orders.first().unwrap().clone();
    let total_qty = orders.iter().map(|o| o.qty).sum();
    let total_price = orders.iter().map(|o| o.qty * o.price).sum::<f64>();
    let average_price = total_price / total_qty as f64;

    // println!("total qty: {}, total price: {}, average_price: {}", total_qty, total_price, average_price);

    first_order.price   = average_price;
    first_order.qty     = total_qty;

    first_order
}
