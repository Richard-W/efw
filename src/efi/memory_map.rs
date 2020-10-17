use super::*;

/// UEFI Memory Map container.
pub struct MemoryMap {
    pub(crate) buffer: Vec<u8>,
    pub(crate) map_key: usize,
    pub(crate) desc_size: usize,
    pub(crate) desc_ver: u32,
}

impl MemoryMap {
    pub(crate) fn new(buffer: Vec<u8>, map_key: usize, desc_size: usize, desc_ver: u32) -> Self {
        MemoryMap {
            buffer,
            map_key,
            desc_size,
            desc_ver,
        }
    }

    /// Key of the memory map.
    pub fn key(&self) -> usize {
        self.map_key
    }

    /// Get a constant iterator of memory map entries.
    pub fn iter(&self) -> impl Iterator<Item = &bits::MemoryDescriptor> {
        ConstMemoryMapIterator::new(self)
    }

    /// Get a mutable iterator of memory map entries.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut bits::MemoryDescriptor> {
        MutMemoryMapIterator::new(self)
    }
}

/// Constant iterator type for the `MemoryMap` struct.
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

/// Mutable iterator type for the `MemoryMap` struct.
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
