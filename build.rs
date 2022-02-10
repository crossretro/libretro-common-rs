extern crate bindgen;
use std::path::PathBuf;
use std::env;

const EM_OS: &str = "emscripten";

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

		// Grab this value because #[cfg(all(target_arch = "wasm32", target_os = "emscripten"))] does not work in build.rs
    // because it assumes that the target is the default OS target
    // when you specify wasm32-unknown-emscripten.
		let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap().to_string();
    if target_os == EM_OS {
      // Export as JS file as ES6 Module by adding emscripten flag
      println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
      println!("cargo:rustc-link-arg=-sMODULARIZE=1");
    }
  
    
    // Use the `cc` crate to build a C file and statically link it.
    let mut builder = cc::Build::new();
		builder.include("libretro-common/include")
		.file("libretro-common/audio/audio_mix.c")
    .file("libretro-common/audio/audio_mixer.c")
    .file("libretro-common/audio/dsp_filter.c");
		if target_os == EM_OS {	
			builder.archiver("emar");
		}
    builder.compile("libretro_common");
    
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I./libretro-common/include")
        // .clang_arg("-fvisibility=default")
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