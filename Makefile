MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

RUSTFLAGS ?=
DOCKER_IMG := midlang
DOCKER_RUN_COMMON := --env RUSTFLAGS=$(RUSTFLAGS) --env-file ./docker.env -v .:/app $(DOCKER_IMG)
IN_DEV ?= docker run $(DOCKER_RUN_COMMON)
IN_IDEV ?= docker run -it $(DOCKER_RUN_COMMON)


all: dev-env compile tests check

dev-env:
	docker build --progress=plain -t $(DOCKER_IMG) .

compile:
	$(IN_DEV) cargo build --color=never

tests:
	$(IN_DEV) cargo test --color=never

fmt:
	$(IN_DEV) cargo fmt

check:
	$(IN_DEV) cargo fmt --check

start:
	$(IN_DEV) ./target/debug/mlc

sh:
	$(IN_IDEV) /bin/sh

take-ownership:
	sudo chown -R $(ME) .

check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)

.PHONY: all \
	dev-env sh \
	compile test fmt check start \
	check-ownership take-ownership
