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

## License
tabelog-bot is available under the MIT and Apache 2.0 license. See the [LICENSE file](https://github.com/atsushi130/tabelog-bot/blob/master/license-mit).
