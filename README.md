# Vansch-OS <!-- omit in toc -->
Security-centered, Super-customizable, Open Source, and user freindly OS. 

# Table of Contents
- [Table of Contents](#table-of-contents)
- [Building process](#building-process)
- [TODO](#todo)

# Building process
First, you need to redownload rust into the nightly version. <br>
You can do redownload rust by running:
```
rustc --version
```
After you have installed the nightly version of rust, you need to intall additional rust components with:
```
rustup component add rust-src llvm-tools-preview
```
Then, install the rust crate, and package it with:
```
cargo install
cargo bootloader
```
The packaged bootimage will be found in `.\target\x86_64-vansch_os\bootimage-vansch-os.bin`

# TODO
- [ ] Use recommended instructions for compiling as per [Bootloader Package](https://crates.io/crates/bootloader)
- [ ] Do more than just print "Hello, World!" in cyan color
- [ ] Add tutorial on how to run image using [QEMU](https://www.qemu.org/)