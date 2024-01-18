FROM rust:latest

WORKDIR /app

RUN apt-get update -q \
    && apt-get install -y -q \
       ninja-build \
       jq

COPY ./ ./

RUN make install -C qbe

RUN rustup component add rustfmt clippy

RUN \
    --mount=type=cache,target=/var/cache/cargo \
    cargo build

RUN cargo install --path mlc