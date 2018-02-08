#!/bin/sh

# git clone
git clone https://github.com/atsushi130/tabelog-got.git
cd tabelog-bot

# build application
cargo build --release
