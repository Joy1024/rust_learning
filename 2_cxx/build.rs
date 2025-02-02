// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cpp/blobstore.cpp")
        .std("c++14")
        .compile("cpp");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cpp/src/blobstore.cpp");
    println!("cargo:rerun-if-changed=src/cpp/include/blobstore.h");
}