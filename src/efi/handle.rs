use super::*;
use late_static::LateStatic;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Handle(bits::Handle);

static SELF_HANDLE: LateStatic<efi::Handle> = LateStatic::new();

impl Handle {
    pub(crate) unsafe fn init_self_handle(handle: bits::Handle) {
        LateStatic::assign(&SELF_HANDLE, Self::new(handle));
    }

    pub fn get_self_handle() -> Self {
        SELF_HANDLE.clone()
    }

    pub fn new(handle: bits::Handle) -> Self {
        Handle(handle)
    }
}

unsafe impl core::marker::Send for Handle {}
