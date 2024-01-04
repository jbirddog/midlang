FROM rust:alpine

WORKDIR /app

RUN apk add -U make samurai

COPY qbe/ qbe/

RUN make install -C qbe