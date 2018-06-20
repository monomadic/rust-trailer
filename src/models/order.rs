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

// group/average orders into a single buy and sell alternating postition
pub fn group_orders(orders: Vec<Order>) -> Vec<Order> {
    let mut grouped_orders = Vec::new();
    let mut current_order = orders.first().unwrap().clone();

    for order in orders.clone() {
        if order.price == current_order.price && order.order_type == current_order.order_type {
            current_order.qty += order.qty;
        } else {
            grouped_orders.push(current_order.clone());
            current_order = order.clone();
        }
    }
    grouped_orders.push(current_order.clone());
    grouped_orders
}

// remove realised positions - pop off any old orders that have been realised.
