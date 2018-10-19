use trailer;
use trailer::error::TrailerError;

use trailer::models::*;
use trailer::presenters::*;

pub fn positions(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>, show_all: bool) -> Result<Vec<Result<PositionPresenter, String>>, TrailerError> {
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;
    let mut presenters:Vec<Result<PositionPresenter, String>> = Vec::new();

    for pair in pairs.clone() {
        let orders = client.trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            if pairs.len() > 1 && pair == "BNBBTC".to_string() { continue };

            let price = *(prices.get(&pair).unwrap_or(&0.0));
            // let grouped_orders = trailer::models::group_orders(orders.clone());
            let positions = trailer::models::Position::new(orders);

            if pairs.len() == 1 { // if just one symbol was supplied
                // print the entire history
                for (index, position) in positions.into_iter().enumerate() {
                    if index == 0 || show_all || (position.state() != PositionState::Irreconciled && position.state() != PositionState::Partial) {
                        presenters.push(Ok(PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price }));
                    }
                }
            } else {
                // filter out closed positions as first shown entry (as they are less irrelevant)
                let positions:Vec<Position> = positions.into_iter().filter(|p|
                    p.state() != PositionState::Closed &&
                    p.state() != PositionState::Irreconciled
                ).collect();

                if let Some(position) = positions.last() {
                    presenters.push(Ok(PositionPresenter{ position: position.clone(), current_price: price, btc_price_in_usd: btc_price }));
                } else {
                    // todo push error here.
                    presenters.push(Err(format!("error finding position for: {}", pair)))
                }
            }
        }
    }

    // sort
    if pairs.len() != 1 {
        presenters.sort_by(|a, b| {
            match a {
                Ok(a) => match b {
                    Ok(b) => b.percent_change().partial_cmp(&a.percent_change()).expect("sort failed"),
                    _ => ::std::cmp::Ordering::Less,
                },
                _ => ::std::cmp::Ordering::Less,
            }
        });
    }

    Ok(presenters)
}
