use models::*;
// use presenters::*;

#[derive(Debug, Clone)]
pub struct PositionPresenter {
    pub position:           Position,
    pub current_price:      f64,
    pub btc_price_in_usd:   f64,
}

impl PositionPresenter {
    // fn new(position: Position, current_price: f64, btc_price_in_usd: f64) -> Self {
    //     Self {
    //         position:               position,
    //         current_price:          current_price,
    //         btc_price_in_usd:       btc_price_in_usd,
    //     }
    // }

    pub fn remaining_quantity(&self) -> f64 {
        if self.position.state() == PositionState::Closed {
            self.position.buy_order.qty
        } else {
            let sold_qty = self.position.clone().sell_order.and_then(|o| Some(o.qty)).unwrap_or(0.0);
            (self.position.buy_order.qty - sold_qty)
        }
    }

    pub fn total_current_value(&self) -> f64 {
        self.remaining_quantity() * self.current_price
    }
    
    pub fn size_in_btc(&self) -> f64 {
        self.position.size() * self.current_price
    }

    pub fn size_in_usd(&self) -> f64 {
        self.size_in_btc() * self.btc_price_in_usd
    }

    pub fn percent_change(&self) -> f64 {
        price_percent(self.position.buy_order.price, self.current_price)
    }

    pub fn total_profit_btc(&self) -> f64 {
        self.size_in_btc() * (self.percent_change() / 100.0)
    }

    pub fn total_profit_usd(&self) -> f64 {
        self.total_profit_btc() * self.btc_price_in_usd
    }

    pub fn unrealised_profit_btc(&self) -> f64 {
        // price of remaining units at the current price - those units at buy price
        (self.remaining_quantity() * self.current_price) - (self.remaining_quantity() * self.position.buy_order.price)
    }

    pub fn unrealised_profit_usd(&self) -> f64 {
        self.unrealised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_btc(&self) -> f64 {
        if let Some(sell_order) = self.clone().position.sell_order {
            (sell_order.price * sell_order.qty) - (self.position.buy_order.price * sell_order.qty)
        } else { 0.0 }
    }

    pub fn realised_profit_usd(&self) -> f64 {
        self.realised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_percent(&self) -> f64 {
        if let Some(sell_order) = self.clone().position.sell_order {
            price_percent(self.position.buy_order.price, sell_order.price)
        } else { 0.0 }
    }
}

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
    if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
    else { -(100. + -100. / entry_price * exit_price) }
}
