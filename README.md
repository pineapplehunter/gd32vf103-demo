# gd32vf103-demo

I foked this project for `Wio Lite RISC-V`.

A small example for running Rust code on RISC-V.

The specific hardware used is:
* GD32VF103CBT6 on a custom board
* USB-C cable
* dfu-util

## Installation

First of all, you need to install the `rust-std` components (pre-compiled
`core` crate) for RISC-V.
```console
$ rustup target add riscv32imac-unknown-none-elf
```

To run, just do this.

```console
$ cargo run
```

This would execute `./run.sh` on the compiled binary.

## License
Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
