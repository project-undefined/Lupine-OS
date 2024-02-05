# Lupine-OS

From [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

## Setup

1. [Install Docker Engine][install docker].
1. (**Linux only**) Ensure your user account is in the [docker group].
2. Install the needed Crates
   1. If you already have a version of Rust installed:

      ```bash
      cargo install cargo-binutils rustfilt
      ```

   2. If you need to install Rust from scratch:

      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

      source $HOME/.cargo/env
      cargo install cargo-binutils rustfilt
      ```

3. In case you use `Visual Studio Code`, I strongly recommend installing the [Rust Analyzer extension].
4. Install a few `Ruby` gems.

    This was last tested with Ruby version `3.0.2` on `Debian 11`. If you are using
    `rbenv`, the respective `.ruby-version` file is already in place. If you never heard of `rbenv`,
    try using [this little guide](https://stackoverflow.com/a/68118750).

    Run this in the repository root folder:

    ```bash
    bundle config set --local path '.vendor/bundle'
    bundle config set --local without 'development'
    bundle install
    ```

[install docker]: <https://docs.docker.com/engine/install/#server>
[docker group]: <https://docs.docker.com/engine/install/linux-postinstall/>
[Rust Analyzer extension]: <https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer>

## Building and Running

Run all commands in `./chainloader`

`make` commands:

- `all` - Compiles, and generates the stripped kernel binary
- `doc` - Generates documentation
- `qemu` - Complies, and generates the stripped kernel binary and runs it in QEMU
- `minipush` - Runs minipush to connect to a RPI via UART and push the newest kernel
- `clippy` - Runs `clippy`
- `clean` - Cleans directory of kernel image and target dir
- `readelf` - Runs `readelf`
- `objdump` - Runs `objdump`
- `nm` - Runs `nm`
- `jtagboot` - Runs the JTAG Debugger **(Not tested)**
- `openocd` - Runs OpenOCD **(Not tested)**
- `gdb`, `gdb-opt0` - Runs AArch64 capable version of `gdb` **(Not tested)**

## Debugging

This has currently not been tested, as it requires a JTAG connector (which I don't have).
If you do have a JTAG connector, follow [this tutorial](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/08_hw_debug_JTAG) for information on how to use it
