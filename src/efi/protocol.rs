use super::*;

/// EFI component that can be located via its GUID
pub trait Protocol: core::marker::Sized {
    const PROTOCOL_GUID: bits::Guid;

    /// Wrap a raw pointer as the protocol
    ///
    /// # Safety
    ///
    /// Safe if `ptr` points to the expected protocol.
    unsafe fn new(ptr: *mut core::ffi::c_void) -> Self;

    fn find_supporting_handles() -> Result<Vec<Handle>> {
        unsafe {
            SystemTable::get().boot_services().locate_handle(
                bits::LocateSearchType::ByProtocol,
                &Self::PROTOCOL_GUID as *const _ as *mut _,
                0 as _,
            )
        }
    }

    fn find_instances() -> Result<Vec<Self>> {
        let handles = Self::find_supporting_handles()?;
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
