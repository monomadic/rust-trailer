extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

mod bot;

fn main() {
    let args : Vec<_> = std::env::args().collect();
    if let Some(key) = args.get(1) {
        bot::build(key);
    }
}
