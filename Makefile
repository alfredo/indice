.PHONY: build
build:
	@echo "Building indice"
	cargo build

.PHONY: run
run:
	@echo "Running indice"
	cargo run

.PHONY: completions
completions:
	@echo "Generating completions"
	mkdir -p ~/.zsh/completion
	indice --completion zsh > ~/.zsh/completion/_indice

.PHONY: release
release:
	@echo "Building release"
	cargo build --release
	cp target/release/indice $(HOME)/.cargo/bin

.PHONY: clean
clean:
	cargo clean

