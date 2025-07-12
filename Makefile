# Makefile for Tea Editor

build:
	cargo build

run:
	cargo run

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

dev: lint test
