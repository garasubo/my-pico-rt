prog := hello-world

.PHONY: write

write:
	cargo build --release
	elf2uf2-rs -d ../../target/thumbv6m-none-eabi/release/$(prog)
