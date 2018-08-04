// use colored::*;
// use trailer::models::*;

// pub fn row(asset: Asset) -> String {
//     if let Some(btc) = funds.clone().btc {
//         let value_in_usd = btc.value_in_usd.unwrap_or(0.0); // (value_in_usd * 1.0 / btc.amount)
//         println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", "BTC".blue(), btc.amount, (value_in_usd * btc.amount), value_in_usd);
//     }

//     for fiat in funds.clone().fiat {
//         println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", fiat.symbol.green(), fiat.amount, fiat.amount, "-");
//     }

//     for altcoin in funds.clone().alts {
//         let value_in_btc = altcoin.value_in_btc.unwrap_or(0.0);
//         println!("{:<8}\t{:<8.2} \t{:<8.3}\t{:<16.8}", altcoin.symbol.yellow(), altcoin.amount, (value_in_btc * altcoin.amount), value_in_btc);
//     }

//     println!("\nTotal value in BTC: {:.3}", funds.total_value_in_btc);
//     println!("Total value in USD: {:.3}\n", funds.total_value_in_usd);
// }
