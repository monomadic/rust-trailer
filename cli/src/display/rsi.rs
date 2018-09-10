use colored::*;
// use display::*;

// use trailer::indicators;

// type RSI_TRIPLE = (Vec<f64>, Vec<f64>, Vec<f64>);

// pub fn rsi_triple(pair: &str, rsi_triple: RSI_TRIPLE) -> String {
//     let mut output_buffer = String::new();

//     let (rsi_15m, rsi_1h, rsi_1d) = rsi_triple {
//         output_buffer.push_str(format!("{pair:12} {rsi_15m:<2} | {rsi_1h:<2} | {rsi_1d:<2}",
//             pair        = pair.yellow(),
//             rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
//             rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
//             rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())))
//         );
//     };

//     output_buffer
// }

pub fn rsi(values: Vec<(String, Vec<f64>)>) -> String {
    for rsi_set in values {
        let (pair, rsi_values) = rsi_set;
        println!("{symbol:12} {rsi_15m:<2}",
            symbol      = pair.yellow(),
            rsi_15m     = ::display::colored_rsi(*rsi_values.last().unwrap(), format!("{:.0}", rsi_values.last().unwrap())),
        )
    }

    "".to_string()
}