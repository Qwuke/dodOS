#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO_WORLD: &[u8] = b"dodOS is NOT extinct!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut color_increment: u8 = 0;
    for (i, &byte) in HELLO_WORLD.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb + color_increment;
            color_increment = color_increment.wrapping_add(64);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

