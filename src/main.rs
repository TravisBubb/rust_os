#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Our entry point into the program
// - no_mangle ensures the compiler outputs a function with the name _start rather than something
// cryptic or "mangled"
// - extern "C" tells the compiler to use the C calling convention for this function instead of the
// unspecified Rust calling convention
// - "_start" is the default entry point name for most systems
// - the ! return type means the function is diverging, i.e., not allowed to ever return
//      -> this is required because the entry point is not called by any function, but rather
//      invoked directly by the operating system or bootloader
//      -> instead of returning the entry point should invoke the "exit" system call
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message...");

    loop {}
}
