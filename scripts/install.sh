#!/bin/sh
cargo build --package cli && cp ./target/debug/cli ~/.bin/trade
# cargo build --package cli && cp ./target/debug/cli ~/.bin/trade-beta
# cargo build --package tgbot && cp ./target/debug/tgbot ~/.bin/trade-tgbot
# cargo build --package bot && cp ./target/debug/bot ~/.bin/bot
# cargo build --package repl && cp ./target/debug/repl ~/.bin/trade-repl
echo "done."
