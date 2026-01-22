#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn syscall(
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    fid: usize,
    eid: usize,
) {
    unsafe {
        let mut _result: usize;
        let mut _value: usize;

        asm! {
            "ecall",
            in("a0") a0,
            in("a1") a1,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") _result,
            lateout("a1") _value,
        }
    }
}

fn putchar(c: u8) {
    unsafe { syscall(c as usize, 0, 0, 0, 0, 0, 0, 1) };
}

fn putstr(s: &str) {
    s.bytes().for_each(putchar);
}

#[unsafe(no_mangle)]
pub fn kernel_main() -> ! {
    loop {
            putstr("Hello, World!\n");
    }
}

#[panic_handler]
pub fn kernel_panic(_: &PanicInfo) -> ! {
    loop {}
}
