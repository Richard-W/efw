use super::*;

pub trait Protocol {
    const PROTOCOL_GUID: bits::Guid;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self;
}
