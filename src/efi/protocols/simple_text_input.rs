use super::*;

/// Simple Text Input protocol
pub struct SimpleTextInput(*mut bits::protocols::simple_text_input::Protocol);

impl Protocol for SimpleTextInput {
    const PROTOCOL_GUID: bits::Guid = bits::protocols::simple_text_input::PROTOCOL_GUID;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        SimpleTextInput(&mut *(ptr as *mut bits::protocols::simple_text_input::Protocol))
    }
}

impl SimpleTextInput {
    /// Pointer to the underlying struct as defined by the UEFI spec
    pub fn bits(&mut self) -> *mut bits::protocols::simple_text_input::Protocol {
        self.0
    }
}
