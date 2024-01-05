MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

DOCKER_IMG := midlang
DOCKER_RUN_COMMON := --env-file ./docker.env -v .:/app $(DOCKER_IMG)
IN_DEV ?= docker run $(DOCKER_RUN_COMMON)
IN_IDEV ?= docker run -it $(DOCKER_RUN_COMMON)


all: dev-env

dev-env:
	docker build --progress=plain -t $(DOCKER_IMG) .

sh:
	$(IN_IDEV) /bin/sh

take-ownership:
	sudo chown -R $(ME) .

check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)

.PHONY: all \
	dev-env sh \
	check-ownership take-ownership
