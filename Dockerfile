FROM rust:1 as builder

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build --release

RUN rm ./target/release/deps/discord_bot*

COPY . .

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder ./target/release/discord-bot ./target/release/discord-bot
COPY --from=builder ./data/games.json ./data/games.json

CMD ["./target/release/discord-bot"]
