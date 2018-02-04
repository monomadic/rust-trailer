#!/bin/sh

cargo build --release --all
mv target/release/trade ~/.bin/trade
