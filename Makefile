.PHONY: all check clean

all: build

check: fmt test clippy

test:
	cargo test --all

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	cargo clean

build:
	cargo build --release