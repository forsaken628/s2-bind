use bindgen::Builder;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from("./sys/src/s2.rs");
    write_bindings(&out_path)
}

fn write_bindings(out_path: &Path) {
    Builder::default()
        .size_t_is_usize(true)
        .header("./sys/install/include/s2/s2latlng.h")
        .clang_args(&[
            "-x",
            "c++",
            "-std=c++14",
            "-I./sys/install/include",
            "-fretain-comments-from-system-headers",
        ])
        .ctypes_prefix("libc")
        .opaque_type("std::string")
        .opaque_type("Vector2")
        .opaque_type("Vector3")
        .allowlist_type("S2LatLng")
        .no_convert_floats()
        //.generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Unable to write bindings to file");

    println!("Bindings generated successfully; please review the results");
}
