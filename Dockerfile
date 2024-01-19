FROM rust:slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y \
       make \
       ninja-build \
       jq \
    && rm -rf /var/lib/apt/lists/*

COPY ./ ./

RUN make install -C qbe

RUN rustup component add rustfmt clippy

RUN \
    --mount=type=cache,target=/var/cache/cargo \
    cargo build

RUN cargo install --path mlc