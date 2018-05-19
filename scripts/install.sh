#!/bin/sh
cargo build --package cli && cp ./target/debug/cli ~/.bin/trade
cargo build --package bot && cp ./target/debug/bot ~/.bin/bot
echo "done."
