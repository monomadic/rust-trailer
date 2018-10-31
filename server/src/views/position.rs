use trailer::models::*;
use trailer::presenters::*;
use views::*;

pub fn row_title() -> String {
    format!("{symbol:12}{state:<9}{size:<32}{change:<40}{profit_loss:<40}{entry_price:<16}{exit_price:<16}\n",
        symbol = "symbol",
        state = "state",
        size = "size",
        change = "change",
        profit_loss = "p/l",
        entry_price = "entry",
        exit_price = "exit",
    )
}

pub fn row(presenter: PositionPresenter) -> String {
    let position = presenter.position.clone();
    format!("{:#?}", position)
}

// pub fn profit_loss(presenter: PositionPresenter) -> String {
//     match presenter.position.state() {
//         PositionState::Closed | PositionState::Irreconciled => "".to_string(),
//         _ => 
//             colored_number(
//                 presenter.percent_change(),
//                 format!("{:.2}% (${:.2}, {:.8} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
//     }
// }

pub fn print_price(percent: f64, usd: f64, btc: f64) -> String {
    colored_number(
        percent,
        format!("{:.2}% (${:.2}, {:.4} btc)", percent, usd, btc)
    )
}

// pub fn unrealised_profit(presenter: PositionPresenter) -> String {
//     match presenter.position.state() {
//         PositionState::Closed | PositionState::Irreconciled => "".to_string(),
//         _ => 
//             colored_number(
//                 presenter.percent_change(),
//                 format!("{:.2}% (${:.2}, {:.4} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
//     }
// }

pub fn position_state(state: PositionState) -> String {
    match state {
        PositionState::Open         => "OPEN".to_string(),
        PositionState::Partial      => "PART".to_string(),
        PositionState::Closed       => "CLOSED".to_string(),
        PositionState::Irreconciled => "IRREC".to_string(),
        PositionState::Invalid      => "INVALID".to_string(),
    }
}

pub fn position_size_compact(presenter: PositionPresenter) -> String {
    match presenter.position.state()  {
        PositionState::Closed | PositionState::Irreconciled => format!("{:.2} btc, ${:.2}", presenter.position.buy_cost(), presenter.current_value_in_usd()),
        _ => format!("{:.2} btc, ${:.2}", presenter.current_value_in_btc(), presenter.current_value_in_usd()),
    }
}

