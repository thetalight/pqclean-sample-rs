extern crate cc;
extern crate dunce;
extern crate glob;

use std::path::{Path, PathBuf};

macro_rules! build_clean {
    ($variant:expr) => {
        let include_path = dunce::canonicalize(Path::new("pqclean/common")).unwrap();

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

        //////////////////////////////////////////////////////

        let common_dir = Path::new("pqclean/common");

        let mut builder = cc::Build::new();
        let target_dir: PathBuf = ["pqclean", "crypto_kem", $variant, "clean"]
            .iter()
            .collect();

        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();

        builder
            .include(include_path)
            .include(&common_dir)
            .include(target_dir);

        builder.files(
            scheme_files
                .into_iter()
                .map(|p| p.unwrap().to_string_lossy().into_owned()),
        );
        builder.compile(format!("{}_clean", $variant).as_str());
    };
}

fn main() {
    build_clean!("ml-kem-512");
    // build_clean!("ml-kem-768");
    // build_clean!("ml-kem-1024");
}
