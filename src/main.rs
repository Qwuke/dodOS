#![no_std]
#![no_main]

mod vga_buffer;
mod rand;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..10000000 {
        rand::PRNG.lock().gen_range(0, 1);
        if i == 0 { println!("dodOS is NOT extinct!"); }
    }

    loop { 
        let num_a = rand::PRNG.lock().gen_range(1, 100);
        let num_spaces = rand::PRNG.lock().gen_range(1, 4);
        for _ in 0..num_a {
            print!("A"); 
        }
        for _ in 0..num_spaces {
            print!(" "); 
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("You shouldn't be proud.\n {}", info);
    loop {}
}

