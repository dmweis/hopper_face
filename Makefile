TARGET_HOST ?= dweis@hopper.local

TARGET_PATH := ~/face_demo
# TARGET_ARCH := armv7-unknown-linux-musleabihf
TARGET_ARCH := aarch64-unknown-linux-musl
SOURCE_PATH := ./target/${TARGET_ARCH}/release/examples/demo


.PHONY: build-cross
build-cross:
	cargo build --release --target=${TARGET_ARCH} --example demo


.PHONY: deploy-cross
deploy-cross: build-cross
	rsync -c ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

.PHONY: remote-run
remote-run: deploy-cross
	ssh -t $(TARGET_HOST) ./face_demo
