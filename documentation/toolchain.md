# Howto install rv32ec rusttoolchain


Checkout and build custom rv32i-toolchain:

```
git clone --recurse-submodules --remote -b riscv32e-toolchain https://github.com/cathiele/rust.git

cd rust

./x.py build --target riscv32i-unknown-none-elf

```

Add custom toolchain to local rust installation:

```
rustup toolchain link riscv32i-toolchain $PWD/build/x86_64-unknown-linux-gnu/stage2
```

use custom toolchain in ch32v003-projects:

```
cd project-folder
rustup override set riscv32i-toolchain
```
