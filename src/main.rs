#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(r"./bindings.rs");
mod bindings;

fn main() {
    println!("Hello, world!");

    // Test bindings
    unsafe {
      bindings::audio_mixer_done();
    }

    println!("Hello, world!");
}
