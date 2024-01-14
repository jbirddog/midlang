MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

RUSTFLAGS ?=
TEST_CASES_DIR ?= test_cases
BUILD_DIR ?= build
NINJA ?= samu
MLC ?= ./target/debug/mlc
DOCKER_IMG := midlang
DOCKER_RUN_COMMON := --env RUSTFLAGS="$(RUSTFLAGS)" --env-file ./docker.env -v .:/app $(DOCKER_IMG)
IN_DEV ?= docker run $(DOCKER_RUN_COMMON)
IN_IDEV ?= docker run -it $(DOCKER_RUN_COMMON)

all: dev-env compile tests hello-world usage

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

start: hello-world-cond
	@/bin/true

hello-world:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world.json \
		--build-dir $(BUILD_DIR)/hello_world \
		--ninja samu \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world/a.out

hello-world2:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world2.json \
		--build-dir $(BUILD_DIR)/hello_world2 \
		--ninja samu \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world2/a.out

hello-world-cond:
	$(IN_DEV) $(MLC) \
		--json-file $(TEST_CASES_DIR)/json/hello_world_cond.json \
		--build-dir $(BUILD_DIR)/hello_world_cond \
		--ninja samu \
	&& $(IN_DEV) $(BUILD_DIR)/hello_world_cond/a.out

usage:
	$(IN_DEV) $(MLC) --help

sh:
	$(IN_IDEV) /bin/sh

take-ownership:
	sudo chown -R $(ME) .

check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)

.PHONY: all \
	dev-env sh \
	compile test start clean \
	fmt fmt-check fmt-json clippy clippy-check check \
	check-ownership take-ownership \
	hello-world hello-world2 usage
