FROM rust:1.60.0-slim

COPY . /src

WORKDIR /src

RUN rustup toolchain add nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN cargo update

RUN cargo install trunk

WORKDIR /src/frontend

RUN trunk build --release

WORKDIR /src

RUN cargo build --release

RUN mkdir /www

RUN cp ./dist /www -r

ENTRYPOINT [ "bash" ]