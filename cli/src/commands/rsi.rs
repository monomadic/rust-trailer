// use trailer;
// use trailer::error::*;
// use trailer::exchanges::*;
// use trailer::indicators;

// use std::thread;
// use std::sync::Arc;

// type RSI_TRIPLE = (Vec<f64>, Vec<f64>, Vec<f64>);

// pub fn top_rsi_15m(client: Arc<ExchangeAPI>, pairs: Vec<String>) -> String {
//     // let mut threads = Vec::new();
//     // let mut rsi_results:Arc<Vec<(String, f64)>> = Arc::new(Vec::new());

//     // for pair in pairs {
//     //     let client = Arc::clone(&client);
//     //     let mut rsi_results = Arc::clone(&rsi_results);

//     //     threads.push(thread::spawn(move || {
//     //         let rsi_15m:Vec<f64> = client.chart_data(&pair, "15m").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
//     //         let rsi_15m = indicators::rsi(14, &rsi_15m);
//     //         let rsi_15m = rsi_15m.last().unwrap();

//     //         rsi_results.push((pair.clone(), *rsi_15m));

//     //         // println!("{symbol:12} {rsi_15m:<2} | {rsi_1h:<2} | {rsi_1d:<2}",
//     //         //     symbol      = pair.yellow(),
//     //         //     rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
//     //         //     rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
//     //         //     rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));
//     //     }));
//     // }

//     // for thread in threads { thread.join().expect("threading failed"); }

//     // format!("{:?}", rsi_results)

//     // rsi_results
// }

// pub fn rsi_triples(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, pairs: Vec<String>) -> String {
//     let mut threads = Vec::new();
//     // let mut rsi_results:RSI_TRIPLE = Vec::new();

//     for pair in pairs {
//         let client = Arc::clone(&client);
//         // let mut output = Arc::clone(&output);

//         threads.push(thread::spawn(move || {
//             // use colored::*;

//             let rsi_15m:Vec<f64> = client.chart_data(&pair, "15m").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
//             let rsi_1h:Vec<f64>  = client.chart_data(&pair, "1h").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
//             let rsi_1d:Vec<f64> = client.chart_data(&pair, "1d").expect("rsi to work").into_iter().map(|price| price.close_price).collect();

//             let rsi_15m = indicators::rsi(14, &rsi_15m);
//             let rsi_1h   = indicators::rsi(14, &rsi_1h);
//             let rsi_1d   = indicators::rsi(14, &rsi_1d);

//             // rsi_results.push((
//             //     indicators::rsi(14, &rsi_15m),
//             //     indicators::rsi(14, &rsi_1h),
//             //     indicators::rsi(14, &rsi_1d)
//             // ));

//             println!("{symbol:12} {rsi_15m:<2} | {rsi_1h:<2} | {rsi_1d:<2}",
//                 symbol      = pair.yellow(),
//                 rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
//                 rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
//                 rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));
//         }));
//     }

//     for thread in threads { thread.join().expect("threading failed"); }

//     // rsi_results
// }