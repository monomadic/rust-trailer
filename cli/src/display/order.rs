use colored::*;
use colored::ColoredString;
use trailer::models::*;
use trailer::presenters::*;

pub fn row(order: Order) -> String {
    format!("{:20}\t{:20}\t{:20.8}\t{:20.2}\n",
        order.symbol, order.order_type, order.price, order.qty)
}

pub fn sub_row(presenter: OrderPresenter) -> String {
    format!("{:<9}\t{:20}\t{:20.8}\t{:20.2}\n",
        trade_type(presenter.order.order_type),
        // format!("{:.2} btc (${:.0})", presenter.current_value_in_btc(), presenter.current_value_in_usd()),
        ::display::btc_and_usd_price(presenter.current_value_in_btc(), presenter.current_value_in_usd()),
        presenter.order.price,
        presenter.order.qty
    )
}

pub fn trade_type(state: TradeType) -> ColoredString {
    match state {
        TradeType::Buy         => "BUY".green(),
        TradeType::Sell      => "SELL".red(),
    }
}
