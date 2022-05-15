FROM rust:1.60.0-slim

COPY . /src

WORKDIR /src

RUN rustup target add aarch64-unknown-linux-musl

RUN cargo build --release