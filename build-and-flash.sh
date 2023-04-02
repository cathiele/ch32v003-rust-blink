#!/bin/sh

cargo build -Zbuild-std=core --release && \
riscv64-unknown-elf-objcopy -O binary target/riscv32i-unknown-none-elf/release/ch32v203-rust-blink ch32v003-rust-blink.bin && \
minichlink -w ch32v003-rust-blink.bin flash && \
minichlink -b
