prog := interrupt
OPENOCD ?= openocd

.PHONY: write

debug-write:
	cargo build
	$(OPENOCD) -f interface/cmsis-dap.cfg -f target/rp2040.cfg -s tcl -c "program ../../target/thumbv6m-none-eabi/debug/$(prog) verify reset exit"

write:
	cargo build --release
	elf2uf2-rs -d ../../target/thumbv6m-none-eabi/release/$(prog)
