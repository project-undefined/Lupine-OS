#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(rustc_private)]
#![test_runner(lupine_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// use libc;
use lupine_os::{println, print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the Lupine OS kernel!");
    print!("Initalizing...\t");
    
    lupine_os::init();

    println!("[DONE]");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    print!("\n[kernel@SYSNM ~]>> ");
    lupine_os::hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    lupine_os::hlt_loop();
}

// This function is called on panic during a test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lupine_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
