# Bulding on Windows

- Enable VSL2
- Run in Ubuntu 18.04
- Install Rust
- Run following commands
  ```
  rustup toolchain add nightly-2020-06-30
  rustup default nightly-2020-06-30
  rustup component add llvm-tools-preview
  rustup target add aarch64-unknown-none-softfloat
  cargo install cargo-binutil

  Or in one go:

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain nightly-2020-06-30                           \
    --component llvm-tools-preview

  source $HOME/.cargo/env
  rustup target add aarch64-unknown-none-softfloat
  cargo install cargo-binutils
  ```
- VS Code terminal does not read `.profile`. Add: `export PATH="$HOME/.cargo/bin:$PATH"`
to `~/.bashrc`
- Install QEMU
- Install Docker
- Enable Docker for VSL2

## Building

- `Makefile` targets:
    - `doc`: Generate documentation.
    - `qemu`: Run the `kernel` in QEMU
    - `clippy`
    - `clean`
    - `readelf`: Inspect the `ELF` output.
    - `objdump`: Inspect the assembly.
    - `nm`: Inspect the symbols.

## Code to look at

- Custom `link.ld` linker script.
    - Load address at `0x80_000`
    - Only `.text` section.
- `main.rs`: Important [inner attributes]:
    - `#![no_std]`, `#![no_main]`
- `cpu.S`: Assembly `_start()` function that executes `wfe` (Wait For Event), halting all cores that
  are executing `_start()`.
- We (have to) define a `#[panic_handler]` function.
    - Just waits infinitely for a cpu event.

[inner attributes]: https://doc.rust-lang.org/reference/attributes.html

### Test it

In the project folder, invoke QEMU and observe the CPU core spinning on `wfe`:
```console

```
