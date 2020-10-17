//! Collection of UEFI protocols

use super::*;

mod graphics_output;
pub use graphics_output::*;

mod simple_text_input;
pub use simple_text_input::*;

mod simple_text_output;
pub use simple_text_output::*;
