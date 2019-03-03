//! Ergonomic wrappers around the UEFI API.

use super::*;

pub use r_efi::efi as bits;

mod boot_services;
pub use self::boot_services::*;

mod handle;
pub use self::handle::*;

mod memory_map;
pub use self::memory_map::*;

mod protocol;
pub use self::protocol::*;

pub mod protocols;

mod result;
pub use self::result::*;

mod system_table;
pub use self::system_table::*;
