use super::*;
use late_static::LateStatic;

/// Handle of an opaque object
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle(bits::Handle);

static SELF_HANDLE: LateStatic<efi::Handle> = LateStatic::new();

impl Handle {
    /// Save handle for the current image
    pub(crate) unsafe fn init_self_handle(handle: bits::Handle) {
        LateStatic::assign(&SELF_HANDLE, Self::new(handle));
    }

    /// Get handle for the current image
    ///
    /// This is the handle that was originally passed to the entry point.
    pub fn get_self_handle() -> Self {
        *SELF_HANDLE
    }

    /// Wrap a low-level handle
    pub fn new(handle: bits::Handle) -> Self {
        Handle(handle)
    }

    /// Get the low-level handle
    pub fn value(&self) -> bits::Handle {
        self.0 as _
    }
}

unsafe impl core::marker::Send for Handle {}
unsafe impl core::marker::Sync for Handle {}
