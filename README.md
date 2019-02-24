# efw [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/Richard-W/efw.svg?branch=master
[travis]: https://travis-ci.org/Richard-W/efw
[Latest Version]: https://img.shields.io/crates/v/efw.svg
[crates.io]: https://crates.io/crates/efw

**Framework for writing UEFI application**

---

## State of development

The types in this crate do not yet wrap the entirety of the UEFI spec. Currently
only a subset of UEFI functions are defined. Some types (`SystemTable`) for
example allow you access lower layers though (via the `bits()` method).

## Getting started

This is a hello world application using efw:

```rust
#![no_std]
#![no_main]

#[macro_use] extern crate efw;

#[no_mangle]
fn efw_main() {
    println!("Hello, world!");
}
```

`efw` reexports the contents of the `alloc` crate so you can use dynamic memory allocation:

```rust
#![no_std]
#![no_main]

#[macro_use] extern crate efw;

#[no_mangle]
fn efw_main() {
    let vector = vec![1, 2, 3];
    println!("Allocated vector: {:?}", vector);
}
```

## Protocol support

efw provides a set of predefined protocols that are needed for it to function
properly. You can extend the set of protocols though by implementing the
`Protocol` trait. That trait provides methods for finding handles supporting
the protocol.
