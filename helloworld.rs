// Welcome to Rust!
// In order to download all the developers tool:
// curl https://sh.rustup.rs -sSf | sh and then restart you bash

// Rust programs end with .rs

// To compile this: rustc helloworld.rs
// rustc is the rust compiler, it should behave in a similar fashion w.r.t.
// clang or gcc
fn main(){
    println!("Hello, World!");
}


// The question mark denotes a Macro call. TODO


// Cargo
//
// Cargo is the Rust build system. It comes with the developers tools you have
// already downloaded.
// Cargo helps you a lot, handling:
//    -> Downloads the dependency libraries
//    -> Builds the dependency libraries
//    -> Builds your code
