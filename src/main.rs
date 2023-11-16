#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO_WORLD: &[u8] = b"doDOS is NOT extinct!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    loop { write!(vga_buffer::WRITER.lock(), "AAAAAAAAAAAAAAAAA ").unwrap(); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

