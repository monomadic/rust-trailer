use trailer::models::Position;
use colored::*;
use trailer::models::*;
// use trailer::error::*;
use trailer::presenters::*;

use super::colored_number;

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

pub fn row_compact(position: Position) -> String {
    // format!("{symbol:12}{trade_type:<8}{pos_size:<20}{percent_change:<20}",
    //     symbol                      = position.symbol,
    //     trade_type                  = position.trade_type.colored_string(),
    //     pos_size                    = format!("{:.2} ({:.2} btc)", position.qty, position.cost_btc),
    //     percent_change              = colored_number(
    //                                     position.potential_profit_percent,
    //                                     format!("{:.2}% (${:.2})", position.potential_profit_percent, position.potential_profit_usd))
    // )
    "nothing again".to_string()
}

pub fn profit_loss(presenter: PositionPresenter) -> ColoredString {
    match presenter.position.state() {
        PositionState::Closed | PositionState::Irreconciled => "".normal(),
        _ => 
            colored_number(
                presenter.percent_change(),
                format!("{:.2}% (${:.2}, {:.8} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
    }
}

pub fn print_price(percent: f64, usd: f64, btc: f64) -> ColoredString {
    colored_number(
        percent,
        format!("{:.2}% (${:.2}, {:.4} btc)", percent, usd, btc)
    )
}

pub fn unrealised_profit(presenter: PositionPresenter) -> ColoredString {
    match presenter.position.state() {
        PositionState::Closed | PositionState::Irreconciled => "".normal(),
        _ => 
            colored_number(
                presenter.percent_change(),
                format!("{:.2}% (${:.2}, {:.4} btc)", presenter.percent_change(), presenter.total_profit_usd(), presenter.total_profit_btc())),
    }
}

pub fn position_state(state: PositionState) -> ColoredString {
    match state {
        PositionState::Open         => "OPEN".green(),
        PositionState::Partial      => "PART".yellow(),
        PositionState::Closed       => "CLOSED".normal(),
        PositionState::Irreconciled => "IRREC".red(),
    }
}
