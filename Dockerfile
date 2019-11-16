FROM ekidd/rust-musl-builder:stable as builder

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build --release

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/discord_bot*

COPY . .

RUN cargo build --release
RUN strip ./target/x86_64-unknown-linux-musl/release/discord-bot

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/discord-bot ./discord-bot

CMD ["./discord-bot"]
