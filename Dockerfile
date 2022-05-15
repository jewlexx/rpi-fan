FROM rust:1.60.0-slim

COPY . /src

WORKDIR /src

RUN cargo install trunk

RUN cargo build --release