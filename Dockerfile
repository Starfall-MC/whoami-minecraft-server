FROM rust:latest AS builder

ENV HOME=/home/root

WORKDIR $HOME/app

RUN rustup default nightly

ADD src src
ADD assets assets
ADD Cargo.lock .
ADD Cargo.toml .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/home/root/app/target \
	cargo build --release && mv target/release/whoami-minecraft .



FROM debian:stable-slim

COPY --from=builder /home/root/app/whoami-minecraft .

CMD ["./whoami-minecraft"]