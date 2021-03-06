use super::*;
use core::ptr;

/// UEFI Memory Map container
pub struct MemoryMap {
    buffer: Vec<u8>,
    map_key: usize,
    desc_size: usize,
    desc_version: u32,
}

impl MemoryMap {
    /// Get the current memory map
    pub fn get_current() -> Result<Self> {
        let boot_services = SystemTable::get().boot_services();

        let mut buffer_size: usize = 0;
        let mut map_key: usize = 0;
        let mut desc_size: usize = 0;
        let mut desc_version: u32 = 0;

        // Get required size of memory map buffer
        unsafe {
            boot_services
                .get_memory_map(
                    &mut buffer_size,
                    ptr::null_mut(),
                    &mut map_key,
                    &mut desc_size,
                    &mut desc_version,
                )
                .ok();
        }
        // Account for an additional allocation
        buffer_size += desc_size;
        // Allocate buffer
        let mut buffer = vec![0u8; buffer_size];

        // Actually get the memory map
        unsafe {
            boot_services.get_memory_map(
                &mut buffer_size,
                buffer.as_mut_ptr() as *mut _,
                &mut map_key,
                &mut desc_size,
                &mut desc_version,
            )?;
        }
        // We could shrink `buffer` now but that would invalidate the memory map we just got
        Ok(MemoryMap {
            buffer,
            map_key,
            desc_size,
            desc_version,
        })
    }

    /// Set the memory map as the virtual address map
    ///
    /// # Safety
    ///
    /// Safe when boot services have been terminated.
    pub unsafe fn set_virtual_address_map(&self) -> Result<()> {
        let runtime_services = SystemTable::get().runtime_services();
        let bytes = self.bytes();
        runtime_services.set_virtual_address_map(
            bytes.len(),
            self.desc_size(),
            self.desc_version(),
            bytes.as_ptr() as *mut _,
        )
    }

    /// Key of the memory map
    pub fn key(&self) -> usize {
        self.map_key
    }

    /// Size of a memory descriptor
    pub fn desc_size(&self) -> usize {
        self.desc_size
    }

    /// Memory descriptor version
    pub fn desc_version(&self) -> u32 {
        self.desc_version
    }

    /// Get the raw bytes of the memory map
    pub fn bytes(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Get the raw bytes of the memory map mutably
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    /// Get a constant iterator of memory map entries
    pub fn iter(&self) -> impl Iterator<Item = &bits::MemoryDescriptor> {
        ConstMemoryMapIterator::new(self)
    }

    /// Get a mutable iterator of memory map entries
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut bits::MemoryDescriptor> {
        MutMemoryMapIterator::new(self)
    }
}

/// Constant iterator type for the `MemoryMap` struct
struct ConstMemoryMapIterator<'a> {
    mmap: &'a MemoryMap,
    position: usize,
}

impl<'a> ConstMemoryMapIterator<'a> {
    pub(crate) fn new(mmap: &'a MemoryMap) -> Self {
        ConstMemoryMapIterator { mmap, position: 0 }
    }
}

impl<'a> core::iter::Iterator for ConstMemoryMapIterator<'a> {
    type Item = &'a bits::MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let item_offset = self.mmap.desc_size * self.position;
        let item_end = item_offset + self.mmap.desc_size;
        if item_end >= self.mmap.buffer.len() {
            None
        } else {
            let ptr: *const u8 = &self.mmap.buffer.as_slice()[0] as _;
            unsafe {
                let desc_ptr = ptr.add(item_offset) as *const bits::MemoryDescriptor;
                self.position += 1;
                Some(&*desc_ptr)
            }
        }
    }
}

/// Mutable iterator type for the `MemoryMap` struct
struct MutMemoryMapIterator<'a> {
    mmap: &'a mut MemoryMap,
    position: usize,
}

impl<'a> MutMemoryMapIterator<'a> {
    pub(crate) fn new(mmap: &'a mut MemoryMap) -> Self {
        MutMemoryMapIterator { mmap, position: 0 }
    }
}

impl<'a> core::iter::Iterator for MutMemoryMapIterator<'a> {
    type Item = &'a mut bits::MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let item_offset = self.mmap.desc_size * self.position;
        let item_end = item_offset + self.mmap.desc_size;
        if item_end >= self.mmap.buffer.len() {
            None
        } else {
            let ptr: *mut u8 = &mut self.mmap.buffer.as_mut_slice()[0] as _;
            unsafe {
                let desc_ptr = ptr.add(item_offset) as *mut bits::MemoryDescriptor;
                self.position += 1;
                Some(&mut *desc_ptr)
            }
        }
    }
}
