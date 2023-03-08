# Lupine OS <!-- omit in toc -->

Security-centered, Super-customizable, Open Source, and user freindly OS.

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Installation and Running](#installation-and-running)
  - [Building process](#building-process)
- [Running Process](#running-process)
- [TODO](#todo)
- [Developers](#developers)

## Installation and Running

### Building process

First, you need to redownload rust into the nightly version.

You can do redownload rust by running:

```bash
rustc --version
```

After you have installed the nightly version of rust, you need to intall additional rust components with:

```bash
rustup component add rust-src llvm-tools-preview
```

Then, install the rust crate, and package it with:

```bash
cargo install --path .
cargo bootloader
```

The packaged bootimage will be found in `.\target\x86_64-vansch_os\bootimage-lupine-os.bin`

## Running Process

You can run it in QEMU with:

```bash
cargo run
```

## TODO

- [ ] Get user input to kernel
- [ ] Implement a text coloring system (to vga and serial)
- [ ] Add better tutorial on how to run image (inluding how to download and install) using [QEMU](https://www.qemu.org/)
- [ ] Implement cosmopolitan libc
- [ ] Create file system structure mockup

## Developers

Lead Developers:

- [PitchBlackNights](https://github.com/PitchBlackNights)

Developers:

- [PitchBlackNights](https://github.com/PitchBlackNights)
- [wsDev0](https://github.com/wsDev0)
