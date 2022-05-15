FROM rust:1.60.0-slim

COPY . /src

WORKDIR /src

RUN rustup toolchain add nightly
RUN rustup default nightly

RUN cargo install trunk

RUN cargo build --release