MY_USER := $(shell id -u)
MY_GROUP := $(shell id -g)
ME := $(MY_USER):$(MY_GROUP)

DOCKER_IMG := midlang
IN_DEV := docker run -v .:/app $(DOCKER_IMG)
IN_IDEV := docker run -itv .:/app $(DOCKER_IMG)

all: dev-env

dev-env:
	docker build --progress=plain -t $(DOCKER_IMG) .

take-ownership:
	sudo chown -R $(ME) .

check-ownership:
	find . ! -user $(MY_USER) ! -group $(MY_GROUP)

.PHONY: all \
	dev-env \
	check-ownership take-ownership
