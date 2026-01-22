#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub fn kernel_main() {
    loop {}
}

#[panic_handler]
pub fn kernel_panic(panic_info: &PanicInfo) -> ! {
    loop {}
}
