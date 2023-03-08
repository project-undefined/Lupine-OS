#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lupine_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lupine_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Welcome to my very small and boring 'kernel'");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic during a test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lupine_os::test_panic_handler(info)
}