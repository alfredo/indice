.PHONY: build run release clean

build:
	cargo build

run:
	cargo run

release:
	cargo build --release
	cp target/release/indice /Users/alfre/.cargo/bin

clean:
	cargo clean

