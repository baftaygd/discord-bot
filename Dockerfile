FROM rust:1-slim as builder

RUN apt update
RUN apt install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/discord_bot*

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder ./target/x86_64-unknown-linux-musl/release/discord-bot ./discord-bot

CMD ["./discord-bot"]
