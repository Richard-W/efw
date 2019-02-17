#![no_std]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

extern crate late_static;
extern crate r_efi;
extern crate ucs2;

mod allocator;
#[macro_use] mod console;
mod efi;
mod result;

use late_static::LateStatic;
use r_efi::efi as refi;

pub use self::result::*;
pub use self::console::*;

static HANDLE: LateStatic<efi::Handle> = LateStatic::new();
static mut SYSTEM_TABLE: LateStatic<efi::SystemTable> = LateStatic::new();

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

extern {
    #[no_mangle]
    fn efw_main();
}

#[no_mangle]
unsafe extern fn efi_main(handle: refi::Handle, system_table: &'static mut refi::SystemTable) -> refi::Status {
    LateStatic::assign(&HANDLE, efi::Handle::new(handle));
    LateStatic::assign(&SYSTEM_TABLE, efi::SystemTable::new(system_table));

    efw_main();

    refi::Status::SUCCESS
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let message = match panic_info.message() {
        Some(s) => s.clone(),
        None => format_args!("no message"),
    };
    println!("Panic occured: \"{}\"", message);
    match panic_info.location() {
        Some(l) => {
            println!("  File: {}", l.file());
            println!("  Line: {}", l.line());
            println!("  Column: {}", l.column());
        },
        None => {},
    }
    println!("Halting...");
    loop {}
}


