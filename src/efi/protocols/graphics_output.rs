use super::*;

pub struct GraphicsOutput(*mut bits::protocols::graphics_output::Protocol);

impl Protocol for GraphicsOutput {
    const PROTOCOL_GUID: bits::Guid = bits::protocols::graphics_output::PROTOCOL_GUID;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        GraphicsOutput(&mut *(ptr as *mut bits::protocols::graphics_output::Protocol))
    }
}

impl GraphicsOutput {
    pub fn bits(&mut self) -> *mut bits::protocols::graphics_output::Protocol {
        self.0
    }
}

