#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dodos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use dodos::{print, println};
use dodos::rand::PRNG;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    dodos::init();

    for i in 0..10000000 {  
        PRNG.lock().gen_range(0, 1);
        if i == 0 { println!("dodOS is NOT extinct!"); }
    }
    
    dodos::hlt_loop();

    loop { 
        let num_a = PRNG.lock().gen_range(1, 100);
        let num_spaces = PRNG.lock().gen_range(1, 4);
        for _ in 0..num_a {
            print!("A"); 
        }
        for _ in 0..num_spaces {
            print!(" "); 
        }
    }
}

#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("You shouldn't be proud.\n {}", info);
    dodos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dodos::test_panic_handler(info)
}