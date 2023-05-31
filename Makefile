DOCKER_REGISTRY=
PACKAGE_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq '.packages[0].version' | tr -d '"')
IMAGE_NAME := $(shell cargo metadata --no-deps --format-version 1 | jq '.packages[0].name' | tr -d '"')

IMAGE_TAG := $(DOCKER_REGISTRY)/$(IMAGE_NAME)
IMAGE_TAG_VERSION := $(IMAGE_TAG):$(PACKAGE_VERSION)
IMAGE_TAG_LATEST  := $(IMAGE_TAG):latest

info:
	@echo IMAGE NAME IS $(IMAGE_NAME)
	@echo VERSION IS $(PACKAGE_VERSION)
	@echo IMAGE TAGS ARE $(IMAGE_TAG_VERSION) $(IMAGE_TAG_LATEST)
clean:
	rm -rf build/
build:
	docker build -t $(IMAGE_TAG_VERSION) -t $(IMAGE_TAG_LATEST) .
publish:
	docker push $(IMAGE_TAG) --all-tags
run-local:
	docker run --rm --env-file=.env --network=host $(IMAGE_TAG_VERSION)
doc:
	GENERATE_DOCS=true cargo doc

all: info build publish clean