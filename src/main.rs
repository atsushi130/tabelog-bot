
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
