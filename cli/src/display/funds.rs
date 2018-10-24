use trailer::presenters::FundsPresenter;
use colored::*;
use display::*;

pub fn show_funds(funds: FundsPresenter) {
    if let Some(btc) = funds.clone().btc {
        println!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}", "BTC".blue(), btc.asset.amount, btc.asset.amount, display_fiat(btc.value_in_usd));
    }

    for fiat in funds.clone().fiat {
        println!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}", fiat.asset.symbol.green(), fiat.asset.amount, fiat.value_in_btc, display_fiat(fiat.value_in_usd));
    }

    for altcoin in funds.clone().alts {
        // let value_in_btc = altcoin.value_in_btc.unwrap_or(0.0);
        println!("{:<8}\t{:<8.3} \t{:<8.3}\t{:<16}", altcoin.asset.symbol.yellow(), altcoin.asset.amount, altcoin.value_in_btc, display_fiat(altcoin.value_in_usd));
    }

    println!("\nTotal value {:.4}B⃦ / {}\n", funds.total_value_in_btc, display_fiat(funds.total_value_in_usd));
}
