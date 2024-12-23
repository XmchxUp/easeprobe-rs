.PHONY: all build test clean

all: release

build:
	cargo build
	cp ./target/debug/easeprobe .

release:
	cargo build --release
	cp ./target/release/easeprobe .