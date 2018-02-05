
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate slack;
use slack::RtmClient;

mod tabelog_bot;
use tabelog_bot::TabelogBot;

fn main() {
    connect();
}

fn connect() {
    let key = "xoxb-309683363221-SdQm1iU6UR9bwLSCNqmSsirY".to_string();

    let mut handler = TabelogBot::from();
    let r = RtmClient::login_and_run(&key, &mut handler);
    match r {
        Ok(_) => {}
        Err(_) => connect(),
    }
}
