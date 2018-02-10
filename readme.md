# tabelog-bot
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit)  

## Usage
```
❯ docker-compose build
❯ TOKEN=${TOKEN} docker-compose up
```

## How to tabelog search
```
@tabelog-bot {location} {search word}

// example
@tabelog-bot 恵比寿 ラーメン
```

## Heroku
```
❯ heroku login 
❯ heroku container:login 
❯ heroku create tabelog-bot 
❯ heroku config:get TOKEN=${TOKEN}
❯ heroku container:push app
```

## Join bot to your slack team
Please let Bot join as a your team member, And set `tabelog-bot` to bot's name.  
Bot setting page: `https://{your team}.slack.com/apps/manage/custom-integrations`

## License
tabelog-bot is available under the MIT and Apache 2.0 license. See the [LICENSE file](https://github.com/atsushi130/tabelog-bot/blob/master/license-mit).
