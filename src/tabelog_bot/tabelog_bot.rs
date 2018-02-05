
use slack::{ Event, EventHandler, RtmClient, Message };
use std::marker::PhantomData;

pub struct TabelogBot<'a> {
    phantom: PhantomData<&'a str>
}

type SlackEvent = (String, String);
impl<'a> TabelogBot<'a> {

    const USER_ID: &'a str = "U93L3AP6H";
    const NAME: &'a str    = "@tabelog-bot";

    pub fn from<'b>() -> TabelogBot<'b> {
        TabelogBot {
            phantom: PhantomData
        }
    }

    fn to_me(&self, text: &str) -> bool {
        text.contains(TabelogBot::USER_ID)
    }

    fn analyze_event(&self, event: Event) -> (Option<String>, Option<String>) {

        if let Event::Message(content) = event {
            if let Message::Standard(message) = *content {
                return (message.text, message.channel)
            }
        }

        (None, None)
    }

    fn get_message(&self, event: Event) -> Option<String> {

        if let Event::Message(content) = event {
            if let Message::Standard(message) = *content {
                return message.text
            }
        }

        return None
    }

    fn get_channel(&self, event: Event) -> Option<String> {

        if let Event::Message(content) = event {
            if let Message::Standard(message) = *content {
                return message.channel
            }
        }

        return None
    }

    fn send(&self, client: &RtmClient, channel: &str, message: &str) {
        let _ = client.sender().send_message(channel, message);
    }
}

impl<'a> EventHandler for TabelogBot<'a> {

    fn on_event(&mut self, cli: &RtmClient, event: Event) {

        let (maybeMessage, maybeChannel) = self.analyze_event(event);

        maybeMessage.iter()
            .zip(maybeChannel.iter())
            .filter(|&(message, _)| self.to_me(message.as_str()))
            .map( |(message, channel)| (message.replace(format!("<@{}> ", TabelogBot::USER_ID).as_str(), ""), channel))
            .for_each(|(message, channel)| println!("{}: {} {}", channel, TabelogBot::NAME, message));
    }

    fn on_close(&mut self, cli: &RtmClient) {}
    fn on_connect(&mut self, cli: &RtmClient) {}
}