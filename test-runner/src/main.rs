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
        println!("Mutable entry of type {:#x}", desc.r#type);
    }

    let mut graphics_output_instances = efw::efi::protocols::GraphicsOutput::find_instances().unwrap();
    println!("Found {} graphics output protocols", graphics_output_instances.len());

    let graphics = &mut graphics_output_instances[0];
    let current_mode = unsafe { graphics.mode().mode };
    let info = unsafe { graphics.query_mode(current_mode).unwrap() };
    println!("Found {} infos for mode {}", info.len(), current_mode);
}
