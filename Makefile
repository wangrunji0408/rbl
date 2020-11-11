arch := riscv32
target := riscv32i-unknown-none-elf
kernel := target/$(target)/release/rbl
qemu-opts := \
		-smp cores=1 \
		-machine virt \
		-kernel $(kernel) \
		-nographic

.PHONY: build run debug $(kernel)

$(kernel):
	cargo build --release

build: $(kernel)

run: build
	qemu-system-$(arch) $(qemu-opts)
