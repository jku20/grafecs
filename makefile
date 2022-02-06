all:
	cargo build --release
run:
	cargo run --release
clean:
	cargo clean
.PHONY: all run clean
