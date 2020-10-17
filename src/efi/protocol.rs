use super::*;
use core::ptr;

/// EFI component that can be located via its GUID
pub trait Protocol: core::marker::Sized {
    const PROTOCOL_GUID: bits::Guid;

    /// Wrap a raw pointer as the protocol
    ///
    /// # Safety
    ///
    /// Safe if `ptr` points to the expected protocol.
    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self;

    fn locate_handles() -> Result<Vec<Handle>> {
        let boot_services = SystemTable::get().boot_services();
        let mut guid = Self::PROTOCOL_GUID;

        // Find out required buffer size
        let mut buffer_size: usize = 0;
        unsafe {
            boot_services
                .locate_handle(
                    bits::LocateSearchType::ByProtocol,
                    &mut guid,
                    ptr::null_mut(),
                    &mut buffer_size,
                    ptr::null_mut(),
                )
                .ok();
        }

        // Actually get handles
        let mut buffer = vec![Handle::new(0 as _); buffer_size / core::mem::size_of::<Handle>()];
        unsafe {
            boot_services
                .locate_handle(
                    bits::LocateSearchType::ByProtocol,
                    &mut guid,
                    ptr::null_mut(),
                    &mut buffer_size,
                    buffer.as_mut_ptr() as *mut _,
                )
                .ok();
        }
        Ok(buffer)
    }

    fn find_instances() -> Result<Vec<Self>> {
        let handles = Self::locate_handles()?;
        let instances = handles
            .iter()
            .map(|handle| unsafe {
                Self::new(
                    SystemTable::get()
                        .boot_services()
                        .handle_protocol(*handle, &Self::PROTOCOL_GUID as *const _ as *mut _)
                        .unwrap(),
                )
            })
            .collect();
        Ok(instances)
    }
}
