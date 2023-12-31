# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# ARCH := $(shell uname -m)
ARCH := aarch64
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

build:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	rustup target install ${ARCH}-unknown-linux-gnu
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=${ARCH}-unknown-linux-gnu --release

server:
	docker build --build-arg TARGETPLATFORM=$(ARCH)-unknown-linux-gnu -t vsock-sample-server -f Dockerfile.server .
	nitro-cli build-enclave --docker-uri vsock-sample-server:latest --output-file vsock-sample-server.eif

client:
	docker build --build-arg TARGETPLATFORM=$(ARCH)-unknown-linux-gnu -t vsock-sample-client -f Dockerfile.client .
	nitro-cli build-enclave --docker-uri vsock-sample-client:latest --output-file vsock-sample-client.eif

.PHONY: clean
clean:
	cargo clean
