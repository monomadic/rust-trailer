use trailer::presenters::FundsPresenter;
use views::*;

pub fn text(funds: FundsPresenter) -> String {
    let mut output = String::new();

    if let Some(btc) = funds.clone().btc {
        output.push_str(&format!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}\n", "BTC", btc.asset.amount, btc.asset.amount, display_fiat(btc.value_in_usd)));
    }

    for fiat in funds.clone().fiat {
        output.push_str(&format!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}\n", fiat.asset.symbol, fiat.asset.amount, display_fiat(fiat.asset.amount), "-"));
    }

    for altcoin in funds.clone().alts {
        // let value_in_btc = altcoin.value_in_btc.unwrap_or(0.0);
        output.push_str(&format!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}\n", altcoin.asset.symbol, altcoin.asset.amount, altcoin.value_in_btc, display_fiat(altcoin.value_in_usd)));
    }

    output.push_str(&format!("\nTotal value in BTC: {:.3}\n", funds.total_value_in_btc));
    output.push_str(&format!("Total value in USD: {}\n", display_fiat(funds.total_value_in_usd)));

    output
}
