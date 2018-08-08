
// use colored::*;
use trailer::models::*;
// use trailer::error::*;

use super::colored_number;

pub fn row(position: Position) -> String {
    format!("{symbol:12}{trade_type:<12}{pos_size:<22}{percent_change:<40}{sale_price:<16}",
        symbol                      = position.symbol,
        trade_type                  = position.trade_type.colored_string(),
        pos_size                    = format!("{:.2} ({:.2} btc)", position.qty, position.cost_btc),
        percent_change              = colored_number(
                                        position.potential_profit_percent,
                                        format!("{:.2}% (${:.2}, {:.8} btc)", position.potential_profit_percent, position.potential_profit_usd, position.potential_profit_btc)),
        sale_price                  = format!("{:.8}",  position.sale_price),
    )
}

pub fn row_compact(position: Position) -> String {
    format!("{symbol:12}{trade_type:<8}{pos_size:<20}{percent_change:<20}",
        symbol                      = position.symbol,
        trade_type                  = position.trade_type.colored_string(),
        pos_size                    = format!("{:.2} ({:.2} btc)", position.qty, position.cost_btc),
        percent_change              = colored_number(
                                        position.potential_profit_percent,
                                        format!("{:.2}% (${:.2})", position.potential_profit_percent, position.potential_profit_usd))
    )
}
