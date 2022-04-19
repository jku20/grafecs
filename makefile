PNG_FILE_NAME=graphics_out.png
PPM_FILE_NAME=graphics_out.ppm
dw:
	cargo run --release data/script.dw
build:
	cargo build --release
run:
	cargo run --release data/script.dw
test:
	cargo run data/script.dw
clean:
	cargo clean
.PHONY: all run clean dw test
