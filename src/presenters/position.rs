use models::*;

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

    pub fn current_value_in_btc(&self) -> f64 {
        self.position.remaining_quantity() * self.current_price
    }
    
    // pub fn size_in_btc(&self) -> f64 {
    //     self.position.size() * self.current_price
    // }

    pub fn current_value_in_usd(&self) -> f64 {
        self.current_value_in_btc() * self.btc_price_in_usd
    }

    pub fn percent_change(&self) -> f64 {
        price_percent(self.position.entry_price(), self.current_price)
    }

    pub fn total_profit_btc(&self) -> f64 {
        self.current_value_in_btc() * (self.percent_change() / 100.0)
    }

    pub fn total_profit_usd(&self) -> f64 {
        self.total_profit_btc() * self.btc_price_in_usd
    }

    pub fn unrealised_profit_btc(&self) -> f64 {
        // price of remaining units at the current price - those units at buy price
        // (self.position.remaining_quantity() * self.current_price) - (self.position.remaining_quantity() * self.position.entry_price())

        self.position.remaining_quantity() * (self.current_price - self.position.entry_price())
        // self.position.remaining_quantity() * self.current_price
    }

    pub fn unrealised_profit_usd(&self) -> f64 {
        self.unrealised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_btc(&self) -> f64 {
        if let Some(exit_price) = self.position.exit_price() {
            (self.position.sell_qty() * exit_price) - (self.position.sell_qty() * self.position.entry_price())
        } else { 0.0 }
    }

    pub fn realised_profit_usd(&self) -> f64 {
        self.realised_profit_btc() * self.btc_price_in_usd
    }

    pub fn realised_profit_percent(&self) -> f64 {
        if let Some(exit_price) = self.position.exit_price() {
            price_percent(self.position.entry_price(), exit_price)
        } else { 0.0 }
    }
}

pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
    if entry_price < exit_price { (100. / entry_price * exit_price) - 100. }
    else { -(100. + -100. / entry_price * exit_price) }
}

pub fn total_btc_staked(presenters: Vec<PositionPresenter>) -> f64 {
    presenters.into_iter().map(|a| a.current_value_in_btc()).sum()
}

// ------ tests

fn order_fixture(order_type: TradeType, qty: f64, price: f64) -> Order {
    Order{ id: "".to_string(), symbol: "".to_string(), order_type: order_type, qty: qty, price: price }
}

#[test]
fn test_position_presenter_state_partial() {
    let position = Position::new(vec![
        order_fixture(TradeType::Buy, 2.0, 100.0), // value: 200
        order_fixture(TradeType::Sell, 1.0, 100.0), // sold: 100 worth, remaining: 100, profit: 0
    ]);

    let presenter = PositionPresenter {
        position:           position.first().unwrap().clone(),
        current_price:      110.0,
        btc_price_in_usd:   2.0,
    };

    assert_eq!(presenter.current_value_in_btc(), 110.0);
    assert_eq!(presenter.current_value_in_usd(), 220.0);
    assert_eq!(presenter.percent_change(), 10.0);
    assert_eq!(presenter.total_profit_btc(), 11.0); // current btc value of profit
    assert_eq!(presenter.total_profit_usd(), 22.0);
    // assert_eq!(presenter.unrealised_profit_btc(), 20.0);
    assert_eq!(presenter.realised_profit_btc(), 0.0);
}
