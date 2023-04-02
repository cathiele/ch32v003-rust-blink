# ch32v003 blink demo in rust

Based on https://github.com/ch32-rs/ch32v203-demo/

At the moment this needs a custom rust toolchain that supports R32EC.


```shell

cargo build -Zbuild-std=core --release

riscv64-unknown-elf-objcopy -O binary target/riscv32i-unknown-none-elf/release/ch32v203-rust-blink ch32v003-rust-blink.bin

minichlink -w ch32v003-rust-blink.bin flash

minichlink -b

```
