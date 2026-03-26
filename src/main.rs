#![no_std]
#![no_main]

use core::arch::asm;

pub fn kernel_main() {
    unsafe {
        asm!("hlt");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
