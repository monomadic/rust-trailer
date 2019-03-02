Note: this library has been rewritten from the ground up as [cryptotrader-core](cryptotrader-core). It is a much better library, please check that instead.

# Trailer

Cryptocurrency exchange library for interacting with various exchanges, currently supporting:

- Binance
- Bittrex
- Kucoin

There is also a small toolchain for interacting with apis on the command line and the initial stages of bot trading.

## Build

```bash
cargo build --all
```

To build the most useful tool, the CLI, run:
```bash
cargo run --package cli
```

You should copy the cli to a runnable location and run from there. Something like:
```bash
cargo build --package cli & cp target/debug/cli ~/.bin/trade-cli
```

## Trade

Interact with exchange apis.
