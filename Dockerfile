FROM rust:alpine

WORKDIR /app

RUN apk add -U gcc musl-dev make samurai

COPY ./ ./

RUN make install -C qbe

RUN rustup component add rustfmt

RUN \
    --mount=type=cache,target=/var/cache/cargo \
    cargo build