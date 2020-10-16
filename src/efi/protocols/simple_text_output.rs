use super::*;

/// Simple Text Output protocol
pub struct SimpleTextOutput(*mut bits::protocols::simple_text_output::Protocol);

impl Protocol for SimpleTextOutput {
    const PROTOCOL_GUID: bits::Guid = bits::protocols::simple_text_output::PROTOCOL_GUID;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        SimpleTextOutput(&mut *(ptr as *mut bits::protocols::simple_text_output::Protocol))
    }
}

impl SimpleTextOutput {
    /// Pointer to the underlying struct as defined by the UEFI spec.
    pub fn bits(&mut self) -> *mut bits::protocols::simple_text_output::Protocol {
        self.0
    }

    /// Print the UCS2 string in `string`.
    ///
    /// # Safety
    ///
    /// Safe if `string` points to a valid UCS2 string.
    pub unsafe fn output_string(&mut self, string: *mut u16) -> Result<()> {
        status_to_result(((*self.0).output_string)(self.0 as _, string))
    }
}
