MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

RUSTFLAGS ?=
TEST_CASES_DIR ?= test_cases
TEST_CASE ?= cond
BUILD_DIR ?= build
NINJA ?= ninja
MLC ?= ./target/debug/mlc
DOCKER_IMG := midlang
DOCKER_RUN_COMMON := --env RUSTFLAGS="$(RUSTFLAGS)" --env-file ./docker.env -v .:/app $(DOCKER_IMG)
IN_DEV ?= docker run $(DOCKER_RUN_COMMON)
IN_IDEV ?= docker run -it $(DOCKER_RUN_COMMON)

all: dev-env compile hello_world

dev-env:
	docker build --progress=plain -t $(DOCKER_IMG) .

compile:
	$(IN_DEV) cargo build --color=never

tests:
	$(IN_DEV) cargo test --color=never

fmt:
	$(IN_DEV) cargo fmt

fmt-check:
	$(IN_DEV) cargo fmt --check

fmt-json:
	$(IN_DEV) find $(TEST_CASES_DIR)/json -type f \
		-exec sh -c 'jq . "{}" > /tmp/out.json && mv /tmp/out.json "{}"' \;

clippy:
	$(IN_DEV) cargo clippy

clippy-check:
	$(IN_DEV) cargo clippy -- -D warnings

check: fmt-check clippy-check
	@/bin/true

clean:
	@rm -rf $(BUILD_DIR)

start: $(TEST_CASE)
	@/bin/true

test-compile:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/$(TEST_CASE).json \
		--build-dir $(BUILD_DIR)/$(TEST_CASE) \
		--ninja $(NINJA) \
		-lm \
		-o $(TEST_CASE)

test-run:
	$(IN_DEV) $(BUILD_DIR)/$(TEST_CASE)/$(TEST_CASE)

usage:
	$(IN_DEV) $(MLC) --help

sh:
	$(IN_IDEV) /bin/bash

take-ownership:
	sudo chown -R $(ME) .

check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)

.PHONY: all \
	dev-env sh \
	compile test start clean \
	fmt fmt-check fmt-json clippy clippy-check check \
	check-ownership take-ownership usage \
	test-compile test-run

include integration.mk
