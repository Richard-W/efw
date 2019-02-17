//! Ergonomic wrappers around the UEFI API.

use super::*;

pub use r_efi::efi as bits;

mod boot_services;
pub use self::boot_services::*;

mod handle;
pub use self::handle::*;

mod system_table;
pub use self::system_table::*;
