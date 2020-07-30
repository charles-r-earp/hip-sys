

fn main() {
    // link to hip
    let hip_lib = "/opt/rocm/hip/lib";
    println!("cargo:rustc-link-search=native={}", hip_lib);
    println!("cargo:rustc-link-lib=dylib=hip_hcc");
    
    #[cfg(feature = "bindgen")] {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
        println!("cargo:rerun-if-changed=wrapper.h");
        let bindings = bindgen::Builder::default()
            .raw_line("#![allow(warnings)]")
            .raw_line("use std::fmt::Debug;")
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            .clang_arg("-I/opt/rocm/hip/include")
            .rustified_non_exhaustive_enum("hip.*")
            .generate_block(false)
            .size_t_is_usize(true)
            .ctypes_prefix("::libc")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        bindings
            .write_to_file("src/lib.rs")
            .expect("Couldn't write bindings!");
    }
}
