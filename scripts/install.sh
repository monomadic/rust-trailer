#!/bin/sh
cargo build --package cli && cp ./target/debug/cli ~/.bin/trade
echo "done."
