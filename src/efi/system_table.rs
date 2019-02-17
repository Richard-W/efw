use super::*;

#[repr(transparent)]
pub struct SystemTable(&'static mut bits::SystemTable);

unsafe impl core::marker::Send for SystemTable {}

static mut SYSTEM_TABLE: LateStatic<efi::SystemTable> = LateStatic::new();

impl SystemTable {
    pub(crate) unsafe fn init(system_table: &'static mut bits::SystemTable) {
        LateStatic::assign(&SYSTEM_TABLE, SystemTable(system_table));
    }

    pub fn get() -> &'static mut SystemTable {
        unsafe {
            &mut SYSTEM_TABLE
        }
    }

    pub fn boot_services(&self) -> BootServices {
        BootServices::new(self.0.boot_services)
    }

    pub fn bits(&mut self) -> *mut bits::SystemTable {
        self.0 as _
    }
}
