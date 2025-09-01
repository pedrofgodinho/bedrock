# Bedrock OS

This is a simple operating system kernel written in Rust. It runs on the x86_64 architecture.

This is meant as a learning project to understand low level kernel development, such as booting, memory management, and CPU features. Huge thanks to [Phil Opp](https://os.phil-opp.com/) for his blog series, which I'm following closely. At the moment, this repository is mostly a copy of his code, but it may diverge in the future.

## Building and Running

To build and run the kernel, you need to have Rust nightly installed, as well as `cargo bootimage` and `qemu`. You can install the necessary tools with the following commands:

```sh
rustup default nightly
cargo install bootimage
```

QEMU can be installed via your package manager or downloaded from [the official website](https://www.qemu.org/download/).

To build and run the kernel, use the following command:

```sh
cargo run
```

To run unit and integration tests, use:

```sh
cargo test
```
