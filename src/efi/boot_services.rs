use super::*;

pub struct BootServices(*mut bits::BootServices);

impl BootServices {
    pub fn new(boot_services: *mut bits::BootServices) -> Self {
        BootServices(boot_services)
    }

    pub unsafe fn allocate_pool(&self, pool_type: bits::MemoryType, size: usize) -> Result<*mut u8> {
        let mut buffer: *mut core::ffi::c_void = 0 as _;
        status_to_result(((*self.0).allocate_pool)(pool_type, size, &mut buffer as _))?;
        Ok(buffer as _)
    }

    pub unsafe fn free_pool(&self, buffer: *mut u8) -> Result<()> {
        status_to_result(((*self.0).free_pool)(buffer as _))
    }
}
