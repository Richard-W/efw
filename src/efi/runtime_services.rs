use super::*;

/// Runtime services function table
pub struct RuntimeServices(*mut bits::RuntimeServices);

impl RuntimeServices {
    pub(crate) fn new(runtime_services: *mut bits::RuntimeServices) -> Self {
        RuntimeServices(runtime_services)
    }

    /// Announces virtual address mappings to EFI components so they can continue to work
    /// after page tables have been modified
    ///
    /// Dont use this function. Use `MemoryMap::set_virtual_address_map` instead.
    ///
    /// # Safety
    ///
    /// Safe if boot services were terminated and `memory_map` is valid.
    pub unsafe fn set_virtual_address_map(
        &self,
        size: usize,
        desc_size: usize,
        desc_version: u32,
        buffer: *mut bits::MemoryDescriptor,
    ) -> Result<()> {
        status_to_result(((*self.0).set_virtual_address_map)(
            size,
            desc_size,
            desc_version,
            buffer,
        ))
    }
}
