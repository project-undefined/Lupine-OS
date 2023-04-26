cargo build --release -Zbuild-std=panic_abort,std -Zbuild-std-features=""  --target=./x86_64-unknown-linux-cosmo.json
objcopy -SO binary ./target/x86_64-unknown-linux-cosmo/release/lupine-os.com.dbg ./lupine-os.com
qemu-system-x86_64 -m 16 -drive file=lupine-os.com,format=raw,index=0,media=disk