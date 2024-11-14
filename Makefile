run:
	cargo run "test-query.txt"

test:
	cargo test

build:
	cargo build

fmt:
	cargo fmt

clippy:
	cargo clippy

all: test build fmt clippy