FROM rust:buster

WORKDIR /usr/src/twitchroulette
COPY . .

WORKDIR /usr/src/twitchroulette/service
RUN rustup component add rustfmt
RUN cargo install --path .

CMD ["twitchroulette", "server", "start"]

