#![no_std]
#![no_main]

#[macro_use] extern crate efw;

#[no_mangle]
fn efw_main() {
    println!("Hello, world!");

    let vector = vec![1, 2, 3];
    println!("Allocated vector: {:?}", vector);
}
