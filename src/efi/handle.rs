use super::*;
use late_static::LateStatic;

/// Handle of an opaque object
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle(bits::Handle);

static SELF_HANDLE: LateStatic<efi::Handle> = LateStatic::new();

impl Handle {
    pub(crate) unsafe fn init_self_handle(handle: bits::Handle) {
        LateStatic::assign(&SELF_HANDLE, Self::new(handle));
    }

    pub fn get_self_handle() -> Self {
        *SELF_HANDLE
    }

    pub fn new(handle: bits::Handle) -> Self {
        Handle(handle)
    }

    pub fn value(&self) -> usize {
        self.0 as _
    }
}

unsafe impl core::marker::Send for Handle {}
