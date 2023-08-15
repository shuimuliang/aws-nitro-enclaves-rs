# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# ARCH := $(shell uname -m)
ARCH := aarch64
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

build:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	rustup target install ${ARCH}-unknown-linux-musl
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=${ARCH}-unknown-linux-musl --release

server:
	docker build --build-arg TARGETPLATFORM=$(ARCH)-unknown-linux-musl -t vsock-sample-server -f Dockerfile.server .
	nitro-cli build-enclave --docker-uri vsock-sample-server:latest --output-file vsock_sample_server.eif

client:
	docker build --build-arg TARGETPLATFORM=$(ARCH)-unknown-linux-musl -t vsock-sample-client -f Dockerfile.client .
	nitro-cli build-enclave --docker-uri vsock-sample-client:latest --output-file vsock_sample_client.eif

.PHONY: clean
clean:
	cargo clean
