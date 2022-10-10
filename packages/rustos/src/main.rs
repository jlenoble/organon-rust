#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use rustos::println;
use bootloader::{ BootInfo, entry_point };

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");
    rustos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rustos::hlt_loop();
}