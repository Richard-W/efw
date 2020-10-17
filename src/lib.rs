#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

extern crate alloc as alloc_crate;
extern crate late_static;
extern crate r_efi;
extern crate ucs2;

// Private modules
mod allocator;
mod console;

// Exported modules
pub use alloc_crate::alloc;
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::collections;
pub mod efi;
pub use alloc_crate::fmt;
pub use alloc_crate::rc;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::sync;
pub use alloc_crate::vec;

/// Commonly used types, traits, and macros
pub mod prelude {
    pub use crate::console::*;
    // Based on experimental feature alloc_prelude
    pub use crate::borrow::ToOwned;
    pub use crate::boxed::Box;
    pub use crate::string::{String, ToString};
    pub use crate::vec::Vec;
}
use prelude::*;

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
