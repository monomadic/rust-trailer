#!/bin/sh

cargo build --release --all
cp target/release/trailer ~/.bin/trl
