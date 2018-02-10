# tabelog-bot
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit)  

## Usage
First, tabelog-bot install:
```
❯ curl -sL --proto-redir -all, https https://raw.githubusercontent.com/atsushi130/tabelog-bot/master/install.sh| sh
```

and configure Slack token to `./src/main.rs`:
```
let key = "configure Slack token".to_string();
```

tabelog-bot run!
```
❯ ./target/release/tabelog_bot
```

## How to tabelog search
```
@tabelog-bot {location} {search word}

// example
@tabelog-bot 恵比寿 ラーメン
```

## Join bot to your slack team
Please let Bot join as a your team member, And set `tabelog-bot` to bot's name.  
Bot setting page: `https://{your team}.slack.com/apps/manage/custom-integrations`

## License
tabelog-bot is available under the MIT and Apache 2.0 license. See the [LICENSE file](https://github.com/atsushi130/tabelog-bot/blob/master/license-mit).
