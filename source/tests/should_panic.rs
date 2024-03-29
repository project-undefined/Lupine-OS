#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lupine_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::fmt;

struct Green(&'static str);

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\x1B[32m")?; // prefix code
        write!(f, "{}", self.0)?;
        write!(f, "\x1B[0m")?; // postfix code
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("should_panic::should_fail...\t");
    should_fail();
    serial_println!("[didn't panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail() {
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
    loop {}
}