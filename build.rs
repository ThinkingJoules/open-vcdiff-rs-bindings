extern crate cc;

fn main() {
    // Compile a single C file from the zlib part of the project.
    cc::Build::new()
        .file("open-vcdiff/src/zlib/adler32.c")
        .compile("libzlib-adler32.a");

    // Initialize a new build configuration for C++ files
    let mut build = cc::Build::new();
    build.cpp(true); // Enable C++ compilation
    build.include("open-vcdiff/src");
    build.include("src");
    build.include("open-vcdiff/src/zlib");
    build.flag("-Wno-deprecated-declarations");

    // Conditionally include directories or definitions based on the presence of certain headers
    if include_exists("ext/rope") {
        build.define("HAVE_EXT_ROPE", Some("1"));
    }

    if include_exists("malloc.h") {
        build.define("HAVE_MALLOC_H", Some("1"));
    }

    if include_exists("sys/mman.h") {
        build.define("HAVE_SYS_MMAN_H", Some("1"));
    }

    if include_exists("sys/stat.h") {
        build.define("HAVE_SYS_STAT_H", Some("1"));
    }

    if include_exists("sys/time.h") {
        build.define("HAVE_SYS_TIME_H", Some("1"));
    }

    if include_exists("unistd.h") {
        build.define("HAVE_UNISTD_H", Some("1"));
    }

    // if include_exists("windows.h") {
    //     build.define("HAVE_WINDOWS_H", Some("1"));
    // }

    // Adding all C++ files to be compiled
    build.file("open-vcdiff/src/addrcache.cc");
    build.file("open-vcdiff/src/blockhash.cc");
    build.file("open-vcdiff/src/codetable.cc");
    build.file("open-vcdiff/src/decodetable.cc");
    build.file("open-vcdiff/src/encodetable.cc");
    build.file("open-vcdiff/src/headerparser.cc");
    build.file("open-vcdiff/src/instruction_map.cc");
    build.file("open-vcdiff/src/jsonwriter.cc");
    build.file("open-vcdiff/src/logging.cc");
    build.file("open-vcdiff/src/varint_bigendian.cc");
    build.file("open-vcdiff/src/vcdecoder.cc");
    build.file("open-vcdiff/src/vcdiffengine.cc");
    build.file("open-vcdiff/src/vcencoder.cc");

    // Compile all C++ files into a single static library
    build.compile("libopen-vcdiff.a");

    cc::Build::new()
        .cpp(true)
        .file("src/glue.cc") // Path to your glue/wrapper file
        .include("open-vcdiff/src") // Include directory
        .compile("libglue.a");


    println!("cargo:rustc-link-lib=static=zlib-adler32");
    println!("cargo:rustc-link-lib=static=open-vcdiff");
    println!("cargo:rustc-link-lib=stdc++"); // Use `c++` instead of `stdc++` if you're on a platform/compiler that requires it


    let bindings = bindgen::Builder::default()
        // Specify the C++ header file(s)
        .header("src/glue.h")
        // Specify any clang arguments, such as include paths
        .clang_arg("-Iopen-vcdiff/src")
        // Specify other bindgen options as needed
        // For example, to handle C++ specifically:
        .clang_args(&["-x", "c++", "-std=c++11"])
        // Generate the bindings
        .generate()
        // Handle potential errors
        .expect("Unable to generate bindings");

    // Write the bindings to a file in the OUT_DIR
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:warning=Bindings are written to: {:?}", out_path);



}

use std::fs;
use std::io::Write;
use tempfile::tempdir;

fn include_exists(path: &str) -> bool {
    let dir = tempdir().unwrap();
    let test_c_file_path = dir.path().join("test.c");

    // Create a temporary C file that includes the specified header
    {
        let mut test_c_file = fs::File::create(&test_c_file_path).unwrap();
        writeln!(test_c_file, "#include <{}>", path).unwrap();
    }

    // Attempt to compile the temporary C file using the cc crate configuration
    let mut build = cc::Build::new();
    build.file(test_c_file_path);
    build.cpp(true); // Set to false if you're only working with C headers
    build.cargo_metadata(false); // Do not print cargo metadata linking flags
    let compile_result = build.try_compile("test");

    // Check if the compilation succeeded
    compile_result.is_ok()
}
