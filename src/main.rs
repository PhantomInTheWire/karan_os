#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    print!("{}", "hello world");
    println!("This is Karan {}", "I need time....");
    loop {}
}