#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(rustc_private)]
#![test_runner(lupine_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// use libc;
use lupine_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the Lupine OS kernel\n\n");

    lupine_os::init();

    // let ptr = 0x204f63 as *mut u32;
    //
    // unsafe { let x = *ptr; }
    // println!("Read Worked");
    //
    // unsafe { *ptr = 42; }
    // println!("Write Worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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
