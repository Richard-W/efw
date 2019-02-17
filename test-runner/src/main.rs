#![no_std]
#![no_main]
#![feature(alloc)]

#[macro_use] extern crate efw;

#[macro_use] extern crate alloc;

#[no_mangle]
fn efw_main() {
    println!("Hello, world!");

    let vector = vec![1, 2, 3];
    println!("Allocated vector: {:?}", vector);
}
