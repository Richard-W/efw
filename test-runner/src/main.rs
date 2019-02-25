#![no_std]
#![no_main]

#[macro_use] extern crate efw;
use efw::efi::Protocol;

#[no_mangle]
fn efw_main() {
    println!("Hello, world!");

    let vector = vec![1, 2, 3];
    println!("Allocated vector: {:?}", vector);

    let text_output_instances = efw::efi::protocols::SimpleTextOutput::find_instances().unwrap();
    println!("Text Output Instances found: {:?}", text_output_instances.len());

    let mut memory_map = unsafe {
        efw::efi::SystemTable::get()
            .boot_services()
            .get_memory_map()
            .unwrap()
    };


    println!("Memory map");
    for desc in memory_map.iter_mut() {
        println!("Mutable entry of type {:#x}", desc.type_);
    }
    for desc in memory_map.iter() {
        println!("  Type:       {:#x}", desc.type_);
        println!("  Phys Start: {:#x}", desc.physical_start);
        println!("  Virt Start: {:#x}", desc.virtual_start);
        println!("  Num Pages:  {:#x}", desc.number_of_pages);
        println!();
    }
}
