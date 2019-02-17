use super::*;

#[repr(transparent)]
pub struct SystemTable(&'static mut refi::SystemTable);

unsafe impl core::marker::Send for SystemTable {}

impl SystemTable {
    pub fn new(system_table: &'static mut refi::SystemTable) -> Self {
        SystemTable(system_table)
    }

    pub fn boot_services(&self) -> BootServices {
        BootServices::new(self.0.boot_services)
    }

    pub fn native(&mut self) -> *mut refi::SystemTable {
        self.0 as _
    }
}
