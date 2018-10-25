use trailer::error::*;
use trailer::presenters::*;

pub fn funds(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, sort_by_name: bool, log: bool) -> Result<String, TrailerError> {
    let prices = client.prices()?;
    // let btc_price = *prices.get(&client.btc_symbol()).unwrap_or(&0.0);
    let btc_price = *prices.get("BTCUSDT").expect("btc price not found."); // fix this with exchange agnostic value

    let mut funds = FundsPresenter::new(client.funds()?, prices, btc_price);

    if sort_by_name {
        funds.alts.sort_by(|a, b| b.value_in_btc.partial_cmp(&a.value_in_btc).expect("order failed"));
    } else {
        funds.alts.sort_by(|a, b| b.value_in_btc.partial_cmp(&a.value_in_btc).expect("order failed"));
    }

    ::display::title_bar(&format!("{} Balance", client.display()));
    ::display::funds::show_funds(funds.clone());

    if log { ::log::log_funds(funds)? };

    Ok("done".to_string())
}
