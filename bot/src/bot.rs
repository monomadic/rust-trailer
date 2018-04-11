use trailer;

pub struct Bot {
    pub symbol: String,
    // entries: Vec<(f64)>, // positions at which to enter the market
}

impl Bot {
    pub fn run(&self) {
        println!("bot started.");
        let current_position = 0.0_f64;

        trailer::exchanges::binance::ws(self.symbol.clone());
    }

    pub fn backtest(&self, prices: Vec<f64>) {
        let price = 10.0;
        let buy_price = prices.first().unwrap();

        for price in prices.clone() {
            let stop = price - (price * 0.1);
            println!("price: {}, stop: {}", price, stop);
        }
    }

    // pub fn backtest(&self, prices: Vec<f64>) {
    //     println!("backtesting bot on coin: {}", self.symbol);

    //     let mut ratelimit = ratelimit::Builder::new()
    //         .capacity(1) //number of tokens the bucket will hold
    //         .quantum(1) //add one token per interval
    //         .interval(Duration::new(1, 0)) //add quantum tokens every 1 second
    //         .build();

    //     let handle = ratelimit.make_handle();
    //     thread::spawn(move || { ratelimit.run() });

    //     // launch threads
    //     let mut threads = Vec::new();
    //     for _ in 0..10 {
    //         let mut handle = handle.clone();
    //         threads.push(thread::spawn(move || {
    //             handle.wait();
    //             // println!("current time: {:?}", Instant::now());
    //             println!(" - heartbeat");
    //         }));
    //     }
    //     for thread in threads {
    //         thread.join().unwrap();
    //     }
    //     println!("time's up!");
    // }

    pub fn load_config(_config_file: String) -> Self {
        Self {
            symbol: "icxbtc".to_string(),
            // entries
        }
    }
}