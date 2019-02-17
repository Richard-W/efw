use super::*;

#[repr(transparent)]
pub struct Handle(bits::Handle);

impl Handle {
    pub fn new(handle: bits::Handle) -> Self {
        Handle(handle)
    }
}

unsafe impl core::marker::Send for Handle {}
