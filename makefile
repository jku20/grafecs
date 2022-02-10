PNG_FILE_NAME=graphics_out.png
PPM_FILE_NAME=graphics_out.ppm
dw:
	cargo run --release
	convert $(PPM_FILE_NAME) $(PNG_FILE_NAME)
	rm $(PPM_FILE_NAME)
	display $(PNG_FILE_NAME)
build:
	cargo build --release
run:
	cargo run --release
clean:
	cargo clean
.PHONY: all run clean dw
