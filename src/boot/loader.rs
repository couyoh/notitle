#![no_std]
#![no_main]

use core::arch::asm;
use core::ffi::c_void;
use core::fmt::Write;
use core::ptr::null_mut;

use crate::uefi::EfiLoadedImageProtocol;

mod uefi;

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(
    image_handle: uefi::EfiHandle,
    system_table: &uefi::EfiSystemTable,
) -> uefi::EfiStatus {
    let protocol = &system_table.con_out;
    protocol.reset(false);
    let mut writer = uefi::Writer { protocol };
    let _ = writeln!(writer, "Loading Kernel...\r\n");

    let mut image: *mut EfiLoadedImageProtocol = null_mut();
    let status = system_table.boot_services.handle_protocol(
        image_handle,
        &uefi::EFI_LOADED_IMAGE_PROTOCOL_GUID,
        &mut image as *mut *mut _ as *mut *mut c_void,
    );
    let _ = writeln!(writer, "OK",);
    if status != uefi::EFI_SUCCESS {
        let _ = writeln!(
            writer,
            "[CRITICAL] Error: Failed to get LoadedImageProtocol. Status: {:#X}\r\n",
            status
        );
        loop {
            unsafe {
                asm!("hlt");
            }
        }
    }

    unsafe {
        let _ = writeln!(
            writer,
            "[INFO] Got LoadedImageProtocol. Revision: {:#X} system_table: {:#p}",
            (*image).revision,
            (*image).system_table
        );
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
