use std::env;
use std::path::{Path, PathBuf};

fn list_directories<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut directories = Vec::new();
    let mut stack = vec![path.as_ref().to_path_buf()];

    while let Some(current) = stack.pop() {
        for entry in std::fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                directories.push(path.clone());
                stack.push(path);
            }
        }
    }

    directories
}

fn is_cpp<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().extension().map_or(false, |ext| ext == "cpp")
}

fn main() {
    let oodle_dir = PathBuf::from("oodle-src");

    let include_dirs = list_directories(oodle_dir.clone());

    let cpp_files = include_dirs
        .clone()
        .into_iter()
        .map(|dir| std::fs::read_dir(dir).unwrap())
        .flatten()
        .map(|entry| entry.unwrap().path())
        .filter(|path| is_cpp(path));

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .includes(include_dirs.clone())
        .files(cpp_files)
        // Define Oodle usage
        .flag("-DOODLE_BUILDING_LIB")
        // enable AVX2 support
        .flag_if_supported("-mavx2") // GCC & Clang
        .flag_if_supported("/arch:AVX2") // MSVC
        // GCC flags to suppress warnings
        .flag_if_supported("-Wno-class-memaccess")
        .flag_if_supported("-Wno-comment")
        .flag_if_supported("-Wno-deprecated-enum-float-conversion")
        .flag_if_supported("-Wno-empty-body")
        .flag_if_supported("-Wno-extra")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("-Wno-type-limits")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-local-typedefs")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-value")
        .flag_if_supported("-Wno-unused-variable")
        .compile("oodle2");

    let bindings = bindgen::Builder::default()
        .clang_args(
            include_dirs
                .iter()
                .map(|dir| format!("-I{}", dir.display())),
        )
        .header(oodle_dir.join("core/oodlelzpub.h").to_string_lossy())
        // Define Oodle usage
        .clang_arg("-DOODLE_BUILDING_LIB")
        // Force C++ mode
        .clang_args(["-x", "c++"])
        .clang_arg("-std=c++20")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Unable to write bindings");

    println!("cargo:rerun-if-changed=src/lib.rs");

    // Link to the static library
    println!("cargo:rustc-link-lib=static=oodle2");
    println!(
        "cargo:rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );

    // Link to the C++ standard library
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
