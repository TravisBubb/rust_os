#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

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

    rust_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

// This function is called on panic when not testing
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
