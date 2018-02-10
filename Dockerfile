FROM rustlang/rust:nightly

RUN curl -sL --proto-redir -all, https https://raw.githubusercontent.com/atsushi130/tabelog-bot/master/install.sh| sh
WORKDIR tabelog-bot

CMD cargo run --release $TOKEN
