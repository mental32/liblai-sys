extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        // Use core instead of libstd in the generated bindings.
        .use_core()
        // Detect include paths
        .detect_include_paths(true)
        // Use libc as the ctypes prefix instead of libstd.
        .ctypes_prefix("libc")
        // Include path
        .clang_arg("-I./lai/include")
        // Avoid deriving debug since a lot of the structs are packed
        .derive_debug(false)
        // Header files.
        .header("./lai/include/lai/host.h")
        .header("./lai/include/lai/helpers/resource.h")
        .header("./lai/include/acpispec/resources.h")
        .header("./lai/include/acpispec/hw.h")
        .header("./lai/include/acpispec/tables.h")
        .header("./lai/include/lai/core.h")
        .header("./lai/include/lai/drivers/ec.h")
        .header("./lai/include/lai/drivers/timer.h")
        .header("./lai/include/lai/helpers/pc-bios.h")
        .header("./lai/include/lai/helpers/pci.h")
        .header("./lai/include/lai/helpers/pm.h")
        .header("./lai/include/lai/helpers/sci.h")
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
