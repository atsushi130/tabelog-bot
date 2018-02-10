#!/bin/sh

# git clone
git clone https://github.com/atsushi130/tabelog-bot.git
cd tabelog-bot

# build application
cargo build --release
