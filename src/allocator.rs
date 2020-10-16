use super::*;
use core::alloc::Layout;
use core::ptr;

pub struct Allocator;

#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

// Largest power of two lower than u16::MAX
const MAX_ALIGN: usize = 32768;

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = align_worst_case_size(layout.size(), align);
        efi::SystemTable::get()
            .boot_services()
            .allocate_pool(efi::bits::MemoryType::LoaderData, size)
            .map(|ptr| core::slice::from_raw_parts_mut(ptr, size))
            .map(|slice| align_slice(slice, align).as_mut_ptr())
            .unwrap_or(ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let aligned_slice = core::slice::from_raw_parts_mut(ptr, layout.size());
        let unaligned_slice = unalign_slice(aligned_slice);
        efi::SystemTable::get()
            .boot_services()
            .free_pool(unaligned_slice.as_mut_ptr())
            .unwrap();
    }
}

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    panic!("Allocation failed");
}

/// Minimum slice of an unaligned slice so that the aligned slice has at least length `length`
fn align_worst_case_size(size: usize, align: usize) -> usize {
    size + 2 + (align - 1)
}

/// Cut a slice so the first element is aligned.
fn align_slice(slice: &mut [u8], align: usize) -> &mut [u8] {
    assert!(align <= MAX_ALIGN);
    let address = slice.as_mut_ptr() as usize;
    let spaced = address + 2;
    let aligned = (spaced + align - 1) / align * align;
    let offset = aligned - address;
    let (_, tail) = slice.split_at_mut(offset - 2);
    let (offset_buffer, aligned_buffer) = tail.split_at_mut(2);
    write_u16(offset_buffer, offset as u16);
    aligned_buffer
}

/// Get the original slice from an aligned slice
///
/// # Safety
///
/// Safe if `slice` was returned by `align_slice`.
unsafe fn unalign_slice(slice: &mut [u8]) -> &mut [u8] {
    let ptr = slice.as_mut_ptr();
    let offset_buffer = core::slice::from_raw_parts(ptr.offset(-2), 2);
    let offset = read_u16(offset_buffer);
    let orig_ptr = ptr.offset(-(offset as isize));
    core::slice::from_raw_parts_mut(orig_ptr, slice.len() + offset as usize)
}

/// Write a `u16` value to a `&[u8]` of length 2
fn write_u16(slice: &mut [u8], value: u16) {
    assert_eq!(slice.len(), core::mem::size_of::<u16>());
    let ptr = slice.as_mut_ptr() as *mut u16;
    unsafe {
        ptr.write(value);
    }
}

/// Read a `u16` value from a `&[u8]` of length 2
fn read_u16(slice: &[u8]) -> u16 {
    assert_eq!(slice.len(), core::mem::size_of::<u16>());
    let ptr = slice.as_ptr() as *const u16;
    unsafe { ptr.read() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_then_unalign() {
        let mut buffer = vec![0; 128];
        let align = 64;

        let aligned = align_slice(&mut buffer, align);
        assert_eq!(aligned.as_mut_ptr() as usize % align, 0);

        let unaligned = unsafe { unalign_slice(aligned) };
        assert_eq!(unaligned.len(), buffer.len());
    }
}
