FILE_NAME=graphics_out.png
dw:
	cargo run --release
	display $(FILE_NAME)
build:
	cargo build --release
run:
	cargo run --release
clean:
	cargo clean
.PHONY: all run clean dw
