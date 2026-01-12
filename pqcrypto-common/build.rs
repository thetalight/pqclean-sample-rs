use std::path::Path;

extern crate cc;


fn main() {
   let include_path = Path::new("pqclean/common");
   println!("cargo:includepath={}", include_path.to_str().unwrap());

    let cfiledir = Path::new("pqclean/common");
    // exclude the randombytes.c file
    let common_files = vec![
        cfiledir.join("fips202.c"),
        cfiledir.join("aes.c"),
        cfiledir.join("sha2.c"),
        cfiledir.join("nistseedexpander.c"),
        cfiledir.join("sp800-185.c"),
    ];

    // println!("cargo:rerun-if-changed=cfiles/");
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=src/");

    let mut build = cc::Build::new();
    build.include(&include_path);

    build.files(common_files).compile("pqclean_common");
    println!("cargo:rustc-link-lib=pqclean_common");
}