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

There are two interfaces to use the tool. An interactive TUI-based (text ui) client and a simple cli version. To compile the TUI version for example, run:
```bash
cargo run --package tui
```

You should copy the cli and tui versions to a runnable location and run from there. Something like:
```bash
cargo build --all & cp target/debug/cli ~/.bin/trade-cli & cp target/debug/tui cp ~/.bin/trade
```

## Trade

Interact with exchange apis.
