FROM rust:alpine

WORKDIR /app

RUN apk add -U gcc musl-dev make samurai

COPY qbe/ qbe/

RUN make install -C qbe

RUN rustup component add rustfmt