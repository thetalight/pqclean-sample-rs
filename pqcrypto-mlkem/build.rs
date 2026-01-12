extern crate cc;
extern crate glob;

use std::path::{Path, PathBuf};

macro_rules! build_clean {
    ($variant:expr) => {
        let common_dir = Path::new("pqclean/common");

        let target_dir: PathBuf = ["pqclean", "crypto_kem", $variant, "clean"]
            .iter()
            .collect();

        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();

        let mut builder = cc::Build::new();
        builder
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
