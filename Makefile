.PHONY: all build test clean

all: release

run:
	./easeprobe

build:
	RUST_LOG=debug cargo build
	cp ./target/debug/easeprobe .

release:
	RUST_LOG=info cargo build --release
	cp ./target/release/easeprobe .