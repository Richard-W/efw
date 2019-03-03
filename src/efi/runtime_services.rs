use super::*;

pub struct RuntimeServices(*mut bits::RuntimeServices);

impl RuntimeServices {
    pub(crate) fn new(runtime_services: *mut bits::RuntimeServices) -> Self {
        RuntimeServices(runtime_services)
    }

    pub unsafe fn set_virtual_address_map(&self, memory_map: &mut MemoryMap) -> Result<()> {
        let length = memory_map.buffer.len() * memory_map.desc_size;
        status_to_result(((*self.0).set_virtual_address_map)(length, memory_map.desc_size, memory_map.desc_ver, &mut memory_map.buffer[0] as *mut _ as _))
    }
}
