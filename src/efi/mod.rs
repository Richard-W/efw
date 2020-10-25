//! Ergonomic wrappers around the UEFI API
use super::*;

pub use r_efi::efi as bits;

mod boot_services;
pub use self::boot_services::*;

mod handle;
pub use self::handle::*;

mod memory_map;
pub use self::memory_map::*;

mod memory_type;
pub use self::memory_type::*;

mod protocol;
pub use self::protocol::*;

pub mod protocols;

mod result;
pub use self::result::*;

mod runtime_services;
pub use self::runtime_services::*;

mod system_table;
pub use self::system_table::*;
