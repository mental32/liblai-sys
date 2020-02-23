extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let bindings = bindgen::Builder::default()
        // Use core instead of libstd in the generated bindings.
        .use_core()
        // Use libc as the ctypes prefix instead of libstd.
        .ctypes_prefix("libc")
        // The input header we would like to generate
        // bindings for.
        .header("./lai/include/lai/host.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("raw_bindings.rs"))
        .expect("Couldn't write bindings!");

    let lai_build = out_path.join("lai-build");
    let lai_lib = lai_build.join("liblai.a");

    meson::build("lai", lai_build.to_str().unwrap());

    println!("cargo:rustc-link-lib={}", lai_lib.to_str().unwrap());
}
