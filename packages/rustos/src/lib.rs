#![no_std]
#![cfg_attr(test, no_main)]

#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(const_mut_refs)]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{ entry_point, BootInfo };

#[cfg(test)]
entry_point!(test_kernel_main);

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let several = tests.len() > 1;
    serial_println!("Running {} test{}", tests.len(), if several { "s" } else { "" });

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}