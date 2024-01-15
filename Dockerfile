FROM rust:alpine

WORKDIR /app

RUN apk add -U gcc musl-dev make samurai jq

COPY ./ ./

RUN make install -C qbe

RUN rustup component add rustfmt clippy

RUN \
    --mount=type=cache,target=/var/cache/cargo \
    cargo build

RUN cargo install --path mlc