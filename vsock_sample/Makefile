# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# ARCH := $(shell uname -m)
ARCH := aarch64
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

build:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	rustup target install ${ARCH}-unknown-linux-musl
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=${ARCH}-unknown-linux-musl --release
	cp ${RUST_DIR}/target/${ARCH}-unknown-linux-musl/release/vsock_sample_server ${RUST_DIR}

server:
	docker build --build-arg TARGETPLATFORM=$(ARCH)-unknown-linux-musl -t vsock-sample-server -f Dockerfile.server .
	#nitro-cli build-enclave --docker-uri vsock-sample-server --output-file vsock_sample_server.eif

.PHONY: clean
clean:
	cargo clean
	rm -rf ${RUST_DIR}/target ${RUST_DIR}/vsock_sample_*.eif ${RUST_DIR}/vsock_sample_server
