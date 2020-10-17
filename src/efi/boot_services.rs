use super::*;

/// Boot services function table
pub struct BootServices(*mut bits::BootServices);

impl BootServices {
    pub(crate) fn new(boot_services: *mut bits::BootServices) -> Self {
        BootServices(boot_services)
    }

    /// Allocate `size` bytes of memory
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called.
    pub unsafe fn allocate_pool(
        &self,
        pool_type: bits::MemoryType,
        size: usize,
    ) -> Result<*mut u8> {
        let mut buffer: *mut core::ffi::c_void = 0 as _;
        status_to_result(((*self.0).allocate_pool)(pool_type, size, &mut buffer as _))?;
        Ok(buffer as _)
    }

    /// Frees memory allocated by `allocate_pool`
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called and `buffer` was allocated by `allocate_pool`.
    pub unsafe fn free_pool(&self, buffer: *mut u8) -> Result<()> {
        status_to_result(((*self.0).free_pool)(buffer as _))
    }

    /// Allocate `num` consecutive pages of physical memory
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called.
    pub unsafe fn allocate_pages(
        &self,
        allocate_type: bits::AllocateType,
        memory_type: bits::MemoryType,
        num: usize,
    ) -> Result<*mut u8> {
        let mut result: bits::PhysicalAddress = 0;
        status_to_result(((*self.0).allocate_pages)(
            allocate_type,
            memory_type,
            num,
            &mut result as _,
        ))?;
        Ok(result as _)
    }

    /// Free `num` consecutive pages of physical memory
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called and `memory` was allocated by `allocate_pages`.
    pub unsafe fn free_pages(&self, memory: *mut u8, num: usize) -> Result<()> {
        status_to_result(((*self.0).free_pages)(memory as _, num))
    }

    /// Get the current memory map
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called.
    pub unsafe fn get_memory_map(&self) -> Result<MemoryMap> {
        let mut buffer_size: usize = 0;
        let mut map_key: usize = 0;
        let mut desc_size: usize = 0;
        let mut desc_ver: u32 = 0;
        ((*self.0).get_memory_map)(
            &mut buffer_size as _,
            0 as _,
            &mut map_key as _,
            &mut desc_size as _,
            &mut desc_ver as _,
        );

        // Account for the additional allocation
        buffer_size += desc_size;

        let mut buffer = Vec::new();
        buffer.resize(buffer_size, 0);

        status_to_result(((*self.0).get_memory_map)(
            &mut buffer_size as _,
            &mut buffer.as_mut_slice()[0] as *mut u8 as *mut _,
            &mut map_key as _,
            &mut desc_size as _,
            &mut desc_ver as _,
        ))?;

        Ok(MemoryMap::new(buffer, map_key, desc_size, desc_ver))
    }

    /// Get an array of handles that support a specific protocol
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called and passed pointer point to valid memory.
    pub unsafe fn locate_handle(
        &self,
        search_type: bits::LocateSearchType,
        protocol: *mut bits::Guid,
        search_key: *mut core::ffi::c_void,
    ) -> Result<Vec<Handle>> {
        // Find out needed buffer size
        let mut buffer_size = 0;
        let null_status = ((*self.0).locate_handle)(
            search_type,
            protocol,
            search_key,
            &mut buffer_size as _,
            0 as _,
        );
        if null_status == bits::Status::SUCCESS {
            return Ok(vec![]);
        }

        // Create buffer
        let mut vector: Vec<Handle> = Vec::new();
        vector.resize(
            buffer_size / core::mem::size_of::<Handle>(),
            Handle::new(0 as _),
        );
        let buffer = &mut vector.as_mut_slice()[0] as *mut Handle as *mut bits::Handle;

        // Perform the search
        status_to_result(((*self.0).locate_handle)(
            search_type,
            protocol,
            search_key,
            &mut buffer_size as _,
            buffer,
        ))?;
        Ok(vector)
    }

    /// Get a pointer to a protocol supported by the handle
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called and passed pointer point to valid memory.
    pub unsafe fn handle_protocol(
        &self,
        handle: Handle,
        protocol: *mut bits::Guid,
    ) -> Result<*mut core::ffi::c_void> {
        let mut interface: *mut core::ffi::c_void = 0 as _;
        status_to_result(((*self.0).handle_protocol)(
            handle.value() as bits::Handle,
            protocol,
            &mut interface as _,
        ))?;
        Ok(interface)
    }

    /// Exit the boot services and take control of the machine
    ///
    /// # Safety
    ///
    /// Safe if `exit_boot_services` was not called before.
    pub unsafe fn exit_boot_services(&self, map_key: usize) -> Result<()> {
        status_to_result(((*self.0).exit_boot_services)(
            Handle::get_self_handle().value() as _,
            map_key,
        ))
    }
}
