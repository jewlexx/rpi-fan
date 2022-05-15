FROM --platform=arm64 rust:1.60.0-slim 

COPY ./src-build /src

WORKDIR /src

RUN rustup toolchain add nightly
RUN rustup default nightly

WORKDIR /src

RUN cargo build --release

RUN mkdir /www

RUN cp /src/dist /www -r

RUN cp /src/target/release/rpi-fan /www/rpi-fan

RUN rm -rf /src

CMD [ "/www/rpi-fan" ]