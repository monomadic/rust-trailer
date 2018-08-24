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

    format!("{symbol:12}{state:<9}{size:<32}{change:<40}{profit_loss:<40}{entry_price:<16.8}{exit_price:<16}\n",
        symbol                      = position.symbol,
        state                       = position_state(position.state()),
        size                        = format!("{:.2} ({:.2} btc, ${:.2})", position.size(), presenter.size_in_btc(), presenter.size_in_usd()),
        change                      = profit_loss(presenter.clone()),
        profit_loss                 = print_price(presenter.realised_profit_percent(), presenter.realised_profit_usd(), presenter.realised_profit_btc()),
        // percent_change              = colored_number(
        //                                 position.potential_profit_percent,
        //                                 format!("{:.2}% (${:.2}, {:.8} btc)", position.potential_profit_percent, position.potential_profit_usd, position.potential_profit_btc)),
        // sale_price                  = format!("{:.8} (%{:.2})",  position.sale_price, position.change_as_percent()),
        entry_price                 = position.entry_price(),
        exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
    )
}

pub fn profit_loss(presenter: PositionPresenter) -> String {
    match presenter.position.state() {
        PositionState::Closed | PositionState::Irreconciled => "".to_string(),
        _ => 
            colored_number(
                presenter.percent_change(),
                format!("{:.2}% (${:.2}, {:.8} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
    }
}

pub fn print_price(percent: f64, usd: f64, btc: f64) -> String {
    colored_number(
        percent,
        format!("{:.2}% (${:.2}, {:.4} btc)", percent, usd, btc)
    )
}

pub fn unrealised_profit(presenter: PositionPresenter) -> String {
    match presenter.position.state() {
        PositionState::Closed | PositionState::Irreconciled => "".to_string(),
        _ => 
            colored_number(
                presenter.percent_change(),
                format!("{:.2}% (${:.2}, {:.4} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
    }
}

pub fn position_state(state: PositionState) -> String {
    match state {
        PositionState::Open         => "OPEN".to_string(),
        PositionState::Partial      => "PART".to_string(),
        PositionState::Closed       => "CLOSED".to_string(),
        PositionState::Irreconciled => "IRREC".to_string(),
    }
}

