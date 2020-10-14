#![no_std]
#![feature(alloc_error_handler)]
#![feature(alloc_prelude)]
#![feature(panic_info_message)]

extern crate alloc;
extern crate late_static;
extern crate r_efi;
extern crate ucs2;

mod allocator;
#[macro_use]
mod console;

use alloc::prelude::v1::*;

pub use self::console::*;
pub use alloc::*;

pub mod efi;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

extern "C" {
    fn efw_main();
}

#[no_mangle]
unsafe extern "C" fn efi_main(
    handle: efi::bits::Handle,
    system_table: *mut efi::bits::SystemTable,
) -> efi::bits::Status {
    efi::Handle::init_self_handle(handle);
    efi::SystemTable::init(system_table);

    efw_main();

    efi::bits::Status::SUCCESS
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let message = match panic_info.message() {
        Some(s) => *s,
        None => format_args!("no message"),
    };
    println!("Panic occured: \"{}\"", message);
    if let Some(l) = panic_info.location() {
        println!("  File: {}", l.file());
        println!("  Line: {}", l.line());
        println!("  Column: {}", l.column());
    }
    println!("Halting...");
    loop {}
}
