#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lupine_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lupine_os::println;
extern crate libc;
extern crate bootloader_api;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    lupine_os::init();
    println!("hi");
    lupine_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    lupine_os::hlt_loop();
}

/// This function is called on panic during a test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lupine_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
