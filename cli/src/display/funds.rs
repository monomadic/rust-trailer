use trailer::presenters::FundsPresenter;
use colored::*;

pub fn show_funds(funds: FundsPresenter) {
    if let Some(btc) = funds.clone().btc {
        // let value_in_usd = btc.value_in_usd; // (value_in_usd * 1.0 / btc.amount)
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", "BTC".blue(), btc.asset.amount, btc.asset.amount, btc.value_in_usd);
    }

    for fiat in funds.clone().fiat {
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", fiat.asset.symbol.green(), fiat.asset.amount, fiat.asset.amount, "-");
    }

    for altcoin in funds.clone().alts {
        // let value_in_btc = altcoin.value_in_btc.unwrap_or(0.0);
        println!("{:<8}\t{:<8.2} \t{:<8.3}\t{:<16.8}", altcoin.asset.symbol.yellow(), altcoin.asset.amount, altcoin.value_in_btc, altcoin.value_in_usd);
    }

    println!("\nTotal value in BTC: {:.3}", funds.total_value_in_btc);
    println!("Total value in USD: {:.3}\n", funds.total_value_in_usd);
}
