# Lupine OS <!-- omit in toc -->

Security-centered, Super-customizable, Open Source, and user freindly OS.

Currently based off of Philipp Oppermann's, [Blog OS](https://github.com/phil-opp/blog_os).

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Installation and Running](#installation-and-running)
  - [Building process](#building-process)
  - [Running Process](#running-process)
- [TODO](#todo)
- [Developers](#developers)
  - [Lead Developers](#lead-developers)
  - [All Developers](#all-developers)

## Installation and Running

### Building process

First, you need to redownload rust into the nightly version.

You can do redownload rust by running:

```bash
rustup update nightly --force
```

After you have installed the nightly version of rust, you need to intall additional rust components with:

```bash
rustup component add rust-src llvm-tools-preview
```

Then, install the rust crate, and package it into a debug binary with:

```bash
cargo install --path .
cargo install bootimage
cargo bootimage
```

The packaged bootimage will be found in `./target/x86_64-lupine_os/debug/bootimage-lupine-os.bin`

### Running Process

You can run a the debug binary in QEMU with:

```bash
cargo run
```

## TODO

- [x] Get user input to kernel (sorta done)
  - [ ] Feed keyboard input into a command interpreter
  - [ ] Handle keys: `escape, delete, backspace, tab`
- [ ] Create functions, that can be used in other rust files, to create executable scripts (kinda like python, just for now)
- [ ] Implement a proper text coloring system (for vga and serial)
- [ ] Create better tutorial on how to run bootimage (inluding how to download and install) using [QEMU](https://www.qemu.org/)
- [ ] Implement interfacing Cosmopolitan Libc through kernel api
- [ ] Create file system structure mockup

## Developers

### Lead Developers

- [PitchBlackNights](https://github.com/PitchBlackNights)

### All Developers

- [PitchBlackNights](https://github.com/PitchBlackNights)
- [wsdevv](https://github.com/wsdevv)
