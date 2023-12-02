fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/sort.cpp")
        .flag_if_supported("-std=c++14")
        .compile("sort_demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/sort.cpp");
    println!("cargo:rerun-if-changed=include/sort.h");
}
