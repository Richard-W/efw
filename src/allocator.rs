use super::*;
use core::alloc::Layout;

pub struct Allocator;

const PTR_SIZE: usize = core::mem::size_of::<usize>();

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match SYSTEM_TABLE.boot_services().allocate_pool(efi::bits::MemoryType::LoaderData, layout.size() + layout.align() + PTR_SIZE - 1) {
            Err(_) => 0 as _,
            Ok(raw) => {
                let unaligned = raw as usize;

                // Align pointer and leave space for original pointer.
                let aligned = (unaligned + PTR_SIZE + layout.align() - 1) / layout.align() * layout.align();

                // Save copy of the original pointer.
                ((aligned - PTR_SIZE) as *mut usize).write(unaligned);

                aligned as _
            },
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let aligned = ptr as usize;
        let unaligned = ((aligned - PTR_SIZE) as *mut usize).read();
        SYSTEM_TABLE.boot_services().free_pool(unaligned as _).unwrap();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    panic!("Allocation failed");
}

