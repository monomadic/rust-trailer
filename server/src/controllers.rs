use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use serde_json;

use ::error::*;

pub fn handle_request(result: Result<String, ServerError>) -> IronResult<Response> {
    match result {
        Ok(body) => Ok(Response::with((
            ContentType::html().0,
            status::Ok,
            body,
        ))),
        Err(error) => Err(::iron::error::IronError {
            error: Box::new(error),
            response: Response::with((
                ContentType::html().0,
                status::Ok,
                "iron error".to_string(),
            )),
        })
    }
}

pub fn chart(req: &mut Request) -> Result<String, ServerError> {
    let symbol = req.extensions.get::<::router::Router>().unwrap().find("symbol").unwrap();

    Ok(format!("
        <div class='tradingview-widget-container'>
          <div id='tradingview_33b5f'></div>
          <script type='text/javascript' src='https://s3.tradingview.com/tv.js'></script>
          <script type='text/javascript'>
          new TradingView.widget(
            {{
                'autosize': true,
                'symbol': 'BINANCE:{}',
                'interval': '15',
                'timezone': 'Etc/UTC',
                'theme': 'Dark',
                'style': '1',
                'locale': 'en',
                'toolbar_bg': 'rgba(101, 101, 101, 1)',
                'enable_publishing': false,
                'hide_legend': true,
                'studies': [
                'MACD@tv-basicstudies',
                'RSI@tv-basicstudies'
                ],
                'container_id': 'tradingview_33b5f'
            }}
          );
          </script>
        </div>
    ", symbol))
}

pub fn prices(_req: &mut Request, conn: &::rusqlite::Connection) -> Result<String, ServerError> {
    let pairs = ::cache::get_all_pairs(&conn)?;
    let mut output_json = Vec::new();
    for (symbol, price, rsi_15m, rsi_1h, rsi_1d) in pairs.clone() {
        output_json.push(json!({
            "pair": symbol,
            "price": price,
            "rsi_15m": rsi_15m,
            "rsi_1h": rsi_1h,
            "rsi_1d": rsi_1d,
        }))
    }
    Ok(format!("{}", serde_json::to_string(&output_json)?))
}

pub fn rsi(_req: &mut Request, conn: &::rusqlite::Connection) -> Result<String, ServerError> {
    let candles = ::cache::get_all_candles(&conn);
    let rsi = ::trailer::indicators::rsi_from_clean_chart_data(14, candles);

    let page = rsi.into_iter().map(|(_s, r)|
        format!("{{{}}}", r.into_iter().map(|v|v.round().to_string()).collect::<Vec<String>>().join(","))
    ).collect::<Vec<String>>().join("");

    Ok(format!("{:#?}", page))
}

pub fn funds(_req: &mut Request) -> Result<String, ServerError> {
    use trailer::exchanges::ExchangeAPI;
    let client = ::trailer::exchanges::binance::connect("9N5duztMdrYfYg2ErhSDV837s8xfBIqF8D7mxpJTKiujvSwoIDI52UguhhkyRQBg", "OG6avXJGOeDt5Phbp150zeEgwjQZpgkXdrp8z2vwPv5bWlHuNFLrK4uAGidnpAIU");

    use trailer::presenters::*;

    let prices = client.prices()?;
    let btc_price = *prices.get("BTCUSDT").expect("btc price not found."); // fix this with exchange agnostic value
    let funds = FundsPresenter::new(client.funds()?, prices, btc_price);

    ::views::funds(funds)
}

pub fn positions(_req: &mut Request) -> Result<String, ServerError> {
    use trailer::exchanges::ExchangeAPI;
    use trailer::presenters::*;

    let client = ::trailer::exchanges::binance::connect("9N5duztMdrYfYg2ErhSDV837s8xfBIqF8D7mxpJTKiujvSwoIDI52UguhhkyRQBg", "OG6avXJGOeDt5Phbp150zeEgwjQZpgkXdrp8z2vwPv5bWlHuNFLrK4uAGidnpAIU");
    let prices = client.prices()?;
    let btc_price = client.btc_price()?;
    let funds = client.funds()?;

    let mut output_buffer = ::views::position::row_title();
    let pairs:Vec<String> = funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();

    for pair in pairs {
        let orders = client.trades_for(&pair);

        if let Ok(orders) = orders {  // ok to swallow error here. not critical.
            let price = *(prices.get(&pair).unwrap_or(&0.0));

            let grouped_orders = ::trailer::models::average_orders(orders.clone());
            // let positions = trailer::models::Position::calculate(grouped_orders, price, btc_price, None);
            let positions = ::trailer::models::Position::new(grouped_orders);

            for position in positions {
                let presenter = PositionPresenter{ position: position, current_price: price, btc_price_in_usd: btc_price };
                // if presenter.into_iter().filter(|p| p.position.state() == PositionState::Open) {
                    output_buffer.push_str(&::views::position::row(presenter));
                // }
                // positions.push(::views::position::row(presenter));
            }
        }
    };

    ::views::layout("positions", format!("<pre>{}</pre>", output_buffer))
}

