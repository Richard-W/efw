use super::*;

use bits::protocols::graphics_output::{BltOperation, BltPixel, Mode, ModeInformation};

/// Configures the video hardware and exposes the framebuffer.
pub struct GraphicsOutput(*mut bits::protocols::graphics_output::Protocol);

impl Protocol for GraphicsOutput {
    const PROTOCOL_GUID: bits::Guid = bits::protocols::graphics_output::PROTOCOL_GUID;

    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self {
        GraphicsOutput(&mut *(ptr as *mut bits::protocols::graphics_output::Protocol))
    }
}

impl GraphicsOutput {
    /// Get a pointer to the underlying struct as defined by the UEFI spec.
    pub fn bits(&mut self) -> *mut bits::protocols::graphics_output::Protocol {
        self.0
    }

    /// Get the mode information struct.
    ///
    /// Contains information about the dimensions of the framebuffer.
    ///
    /// # Safety
    ///
    /// Safe if boot services are still running.
    pub unsafe fn query_mode(&mut self, mode_number: u32) -> Result<&'static [ModeInformation]> {
        let mut size_of_info: usize = 0;
        let mut info: *mut ModeInformation = 0x0 as _;
        status_to_result(((*self.0).query_mode)(
            self.0,
            mode_number,
            &mut size_of_info as _,
            &mut info as _,
        ))?;
        Ok(core::slice::from_raw_parts(info, size_of_info))
    }

    /// Switch to a different mode.
    ///
    /// # Safety
    ///
    /// Safe if boot services are still running.
    pub unsafe fn set_mode(&mut self, mode_number: u32) -> Result<()> {
        status_to_result(((*self.0).set_mode)(self.0, mode_number))
    }

    /// Draw pixels to framebuffer.
    ///
    /// # Safety
    ///
    /// Safe if boot services are still running.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn blt(
        &mut self,
        blt_buffer: *mut BltPixel,
        blt_operation: BltOperation,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
        delta: usize,
    ) -> Result<()> {
        status_to_result(((*self.0).blt)(
            self.0,
            blt_buffer,
            blt_operation,
            source_x,
            source_y,
            destination_x,
            destination_y,
            width,
            height,
            delta,
        ))
    }

    /// Get the current mode.
    ///
    /// # Safety
    ///
    /// Safe if boot services are still running.
    pub unsafe fn mode(&self) -> &'static Mode {
        &mut *(*self.0).mode
    }
}
