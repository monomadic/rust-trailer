// use trailer::models::Position;
use colored::*;
use trailer::models::*;
// use trailer::error::*;
use trailer::presenters::*;

use super::colored_number;

pub fn row_title() -> String {
    format!("{symbol:12}{state:<9}{size:<32}{upnl:<50}{rpnl:<50}{entry_price:<16}{exit_price:<16}\n",
        symbol = "symbol",
        state = "state",
        size = "size",
        upnl = "unrealised p/l",
        rpnl = "realised p/l",
        entry_price = "entry",
        exit_price = "exit",
    )
}

pub fn row(presenter: PositionPresenter) -> String {
    let position = presenter.position.clone();

    format!("{symbol:12}{state:<9}{size:<32}{upnl:50}{rpnl:<50}{entry_price:<16.8}{exit_price:<16}\n",
        symbol                      = position.symbol(),
        state                       = position_state(position.state()),
        size                        = position_size(presenter.position.remaining_quantity(), presenter.current_value_in_btc(), presenter.current_value_in_usd()),
        upnl                        = unrealised_profit_loss(presenter.clone()),
        rpnl                        = realised_profit_loss(presenter.clone()),
        entry_price                 = position.entry_price(),
        exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
    )
}

pub fn row_compact(presenter: PositionPresenter) -> String {
    let position = presenter.position.clone();

    format!("{symbol:12}{state:<9}{size:<20}{upnl:20}{rpnl:<20}{entry_price:<16.8}{exit_price:<16}\n",
        symbol                      = position.symbol(),
        state                       = position_state(position.state()),
        size                        = format!("{:.2} btc (${:.0})", presenter.current_value_in_btc(), presenter.current_value_in_usd()),
        upnl                        = print_price_usd(presenter.percent_change(), presenter.unrealised_profit_usd()),
        rpnl                        = price_or_nothing(presenter),
        entry_price                 = position.entry_price(),
        exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
    )
}

pub fn total(presenters: Vec<PositionPresenter>) -> String {


    let upnl:f64 = presenters.into_iter().map(|p| p.unrealised_profit_btc()
    ).sum();
    // .as_ref().map_or::<Vec<f64>>(0.0, |p| &p.unrealised_profit).sum();
    format!("\ntotal pnl: {} ({} unrealised, {} realised)", upnl, upnl, upnl)
}

pub fn position_size(size: f64, btc: f64, usd: f64) -> String {
    format!("{:.2} ({:.2} btc, ${:.2})", size, btc, usd)
}

pub fn unrealised_profit_loss(presenter: PositionPresenter) -> ColoredString {
    match presenter.position.state() {
        PositionState::Closed | PositionState::Irreconciled => "-".normal(),
        _ => 
            colored_number(
                presenter.percent_change(),
                print_price(presenter.percent_change(), presenter.unrealised_profit_usd(), presenter.unrealised_profit_btc()).to_string()),
    }
}

pub fn realised_profit_loss(presenter: PositionPresenter) -> ColoredString {
    match presenter.position.state() {
        PositionState::Open => "-".normal(),
        _ => colored_number(
            presenter.percent_change(),
            print_price(presenter.percent_change(), presenter.realised_profit_usd(), presenter.realised_profit_btc()).to_string())
    }
}

pub fn price_or_nothing(presenter: PositionPresenter) -> ColoredString {
    match presenter.position.state() {
        PositionState::Open => "-".normal(),
        _ => colored_number(
            presenter.realised_profit_usd(),
            format!("${:.2}", presenter.realised_profit_usd())
        ),
    }
}

pub fn print_price(percent: f64, usd: f64, btc: f64) -> ColoredString {
    colored_number(
        percent,
        format!("{:.2}% (${:.2}, {:.4} btc)", percent, usd, btc)
    )
}

pub fn print_price_usd(percent: f64, usd: f64) -> ColoredString {
    colored_number(
        percent,
        format!("{:.2}% (${:.2})", percent, usd)
    )
}

pub fn position_state(state: PositionState) -> ColoredString {
    match state {
        PositionState::Open         => "OPEN".green(),
        PositionState::Partial      => "PART".yellow(),
        PositionState::Closed       => "CLOSED".normal(),
        PositionState::Irreconciled => "IRREC".red(),
    }
}
