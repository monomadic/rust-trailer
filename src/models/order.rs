use models::TradeType;
use models::trades::Trade;

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
}

// reduce all orders with same price together (technically different orders from the order book)
pub fn compact_orders(orders: Vec<Order>) -> Vec<Order> {
    let mut grouped_orders = Vec::new();
    let mut current_order = orders.first().unwrap().clone();

    for order in orders.clone() {
        if order.price == current_order.price {
            current_order.qty += order.qty;
        } else {
            grouped_orders.push(current_order.clone());
            current_order = order.clone();
        }
    }
    grouped_orders.push(current_order.clone());
    grouped_orders
}

// group/average orders into a grouped vector
pub fn group_orders(orders: Vec<Order>) -> Vec<Vec<Order>> {
    let mut grouped_orders = Vec::new();
    let mut current_order_group = Vec::new();
    let mut current_order_type = orders.first().unwrap().order_type;

    for order in orders.clone() {
        if order.order_type != current_order_type {
            grouped_orders.push(current_order_group.clone());
            current_order_group = Vec::new();
            current_order_type = order.order_type;
        }
        current_order_group.push(order);
    }
    grouped_orders.push(current_order_group);
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
    let average_price = orders.iter().map(|o| o.price).sum::<f64>() / orders.len() as f64;

    first_order.price   = average_price;
    first_order.qty     = total_qty;

    first_order
}
