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
}
