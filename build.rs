extern crate bindgen;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    #[cfg(target_arch = "wasm32")] 
    {
      println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
      println!("cargo:rustc-link-arg=-sMODULARIZE=1");
    }
    
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
    .include("libretro-common/include")
    .file("libretro-common/audio/audio_mix.c")
    .file("libretro-common/audio/audio_mixer.c")
    .file("libretro-common/audio/dsp_filter.c")
    .archiver("emar")
    .compile("libretro_common");
    
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I./libretro-common/include")
        .clang_arg("-fvisibility=default")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(r"./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}