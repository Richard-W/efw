use super::*;

#[repr(transparent)]
pub struct Handle(refi::Handle);

impl Handle {
    pub fn new(handle: refi::Handle) -> Self {
        Handle(handle)
    }
}

unsafe impl core::marker::Send for Handle {}
