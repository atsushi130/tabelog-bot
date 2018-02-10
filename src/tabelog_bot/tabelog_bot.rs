
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use slack::{ Event, EventHandler, RtmClient, Message };
use tabelog_searcher::TabelogClient;
use std::marker::PhantomData;
use super::SearchConditionTokenizer;

pub struct TabelogBot<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> TabelogBot<'a> {

    const NAME: &'a str = "@tabelog-bot";

    pub fn from<'b>() -> TabelogBot<'b> {
        TabelogBot {
            phantom: PhantomData
        }
    }

    fn to_me(&self, text: &str) -> bool {
        text.contains(TabelogBot::NAME)
    }

    fn analyze_event(&self, event: Event) -> (Option<String>, Option<String>) {

        if let Event::DesktopNotification { avatar_image, channel, content, event_ts, image_uri, is_shared, launch_uri, msg, ssb_filename, subtitle, thread_ts, title } = event {
            return (content, channel)
        }

        (None, None)
    }

    fn send(&self, client: &RtmClient, channel: &str, message: &str) {
        let _ = client.sender().send_message(channel, message);
    }
}

impl<'a> EventHandler for TabelogBot<'a> {

    fn on_event(&mut self, cli: &RtmClient, event: Event) {

        let (maybe_message, maybe_channel) = self.analyze_event(event);

        maybe_message.iter()
            .zip(maybe_channel.iter())
            .filter(|&(message, _)| self.to_me(message.as_str()))
            .map(|(message, channel)| (message.replace(format!("{} ", TabelogBot::NAME).as_str(), ""), channel))
            .map(|(message, channel)| (channel, SearchConditionTokenizer.analyze(message.as_str())))
            .map(|(channel, (location, word))| (channel, TabelogClient.search(&location, &word)))
            .for_each(|(channel, results)| {
                results.iter().for_each(|result| self.send(cli, channel, result.as_str()));
            })
    }

    fn on_close(&mut self, cli: &RtmClient) {}
    fn on_connect(&mut self, cli: &RtmClient) {}
}