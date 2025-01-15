.PHONY: all build test clean

export RUST_LOG=debug

all: release

run:
	./easeprobe

build:
	cargo build
	cp ./target/debug/easeprobe .

test:
	cargo test

release:
	cargo build --release
	cp ./target/release/easeprobe .