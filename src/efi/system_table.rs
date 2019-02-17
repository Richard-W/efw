use super::*;
use late_static::LateStatic;

/// Contains pointers to the runtime and boot services provided by UEFI.
#[repr(transparent)]
pub struct SystemTable(&'static mut bits::SystemTable);

unsafe impl core::marker::Send for SystemTable {}

static mut SYSTEM_TABLE: LateStatic<efi::SystemTable> = LateStatic::new();

impl SystemTable {
    pub(crate) unsafe fn init(system_table: &'static mut bits::SystemTable) {
        LateStatic::assign(&SYSTEM_TABLE, SystemTable(system_table));
    }

    /// Get a reference to the system table.
    pub fn get() -> &'static mut SystemTable {
        unsafe {
            &mut SYSTEM_TABLE
        }
    }

    /// Handle for the active console input device.
    pub fn console_in_handle(&self) -> Handle {
        Handle::new(self.0.console_in_handle)
    }

    /// SimpleTextInput protocol associated with `console_in_handle()`.
    pub fn con_in(&self) -> protocols::SimpleTextInput {
        unsafe {
            protocols::SimpleTextInput::new(self.0.con_out as _)
        }
    }

    /// Handle for the active console output device.
    pub fn console_out_handle(&self) -> Handle {
        Handle::new(self.0.console_out_handle)
    }

    /// SimpleTextOutput protocol associated with `console_out_handle()`.
    pub fn con_out(&self) ->  protocols::SimpleTextOutput {
        unsafe {
            protocols::SimpleTextOutput::new(self.0.con_out as _)
        }
    }

    /// Handle for the active standard error console device.
    pub fn standard_error_handle(&self) -> Handle {
        Handle::new(self.0.standard_error_handle)
    }

    /// SimpleTextOutput protocol associated with `standard_error_handle()`.
    pub fn std_err(&self) -> protocols::SimpleTextOutput {
        unsafe {
            protocols::SimpleTextOutput::new(self.0.std_err as _)
        }
    }

    /// Boot Services table.
    pub fn boot_services(&self) -> BootServices {
        BootServices::new(self.0.boot_services)
    }

    /// Get a pointer to the underlying system table implementation.
    pub fn bits(&mut self) -> *mut bits::SystemTable {
        self.0 as _
    }
}
