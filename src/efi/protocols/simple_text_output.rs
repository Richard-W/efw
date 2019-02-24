use super::*;

pub struct SimpleTextOutput(*mut bits::protocols::simple_text_output::Protocol);

impl Protocol for SimpleTextOutput {
    const PROTOCOL_GUID: bits::Guid = bits::protocols::simple_text_output::PROTOCOL_GUID;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        SimpleTextOutput(&mut *(ptr as *mut bits::protocols::simple_text_output::Protocol))
    }
}

impl SimpleTextOutput {
    pub unsafe fn output_string(&mut self, string: *mut u16) -> Result<()> {
        status_to_result(((*self.0).output_string)(self.0 as _, string))
    }
}
