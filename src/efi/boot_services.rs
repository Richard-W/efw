use super::*;

pub struct BootServices(*mut bits::BootServices);

impl BootServices {
    pub(crate) fn new(boot_services: *mut bits::BootServices) -> Self {
        BootServices(boot_services)
    }

    /// Allocate `size` bytes of memory.
    pub unsafe fn allocate_pool(&self, pool_type: bits::MemoryType, size: usize) -> Result<*mut u8> {
        let mut buffer: *mut core::ffi::c_void = 0 as _;
        status_to_result(((*self.0).allocate_pool)(pool_type, size, &mut buffer as _))?;
        Ok(buffer as _)
    }

    /// Frees memory allocated by `allocate_pool`.
    pub unsafe fn free_pool(&self, buffer: *mut u8) -> Result<()> {
        status_to_result(((*self.0).free_pool)(buffer as _))
    }

    /// Get an array of handles that support a specific protocol.
    pub unsafe fn locate_handle(&self, search_type: bits::LocateSearchType, protocol: *mut bits::Guid, search_key: *mut core::ffi::c_void) -> Result<Vec<Handle>> {
        // Find out needed buffer size.
        let mut buffer_size = 0;
        let null_status = ((*self.0).locate_handle)(search_type, protocol, search_key, &mut buffer_size as _, 0 as _);
        if null_status == bits::Status::SUCCESS {
            return Ok(vec![]);
        }

        // Create buffer.
        let mut vector: Vec<Handle> = Vec::new();
        vector.resize(buffer_size / core::mem::size_of::<Handle>(), Handle::new(0 as _));
        let buffer = &mut vector.as_mut_slice()[0] as *mut Handle as *mut bits::Handle;

        // Perform the search.
        status_to_result(((*self.0).locate_handle)(search_type, protocol, search_key, &mut buffer_size as _, buffer))?;
        Ok(vector)
    }

    /// Get a pointer to a protocol supported by the handle.
    pub unsafe fn handle_protocol(&self, handle: Handle, protocol: *mut bits::Guid) -> Result<*mut core::ffi::c_void> {
        let mut interface: *mut core::ffi::c_void = 0 as _;
        status_to_result(((*self.0).handle_protocol)(handle.value() as bits::Handle, protocol, &mut interface as _))?;
        Ok(interface)
    }
}
