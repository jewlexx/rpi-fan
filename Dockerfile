FROM rust:1.60.0-slim

COPY . /src

WORKDIR /src

RUN rustup toolchain add nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN cargo update


WORKDIR /src/frontend

RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.15.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN ./trunk build --release

WORKDIR /src

RUN cargo build --release

RUN mkdir /www

RUN cp ./dist /www -r

RUN cp ./target/release/rpi-fan /src/rpi-fan

CMD [ "/src/rpi-fan" ]