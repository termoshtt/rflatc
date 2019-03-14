extern crate proc_macro;

use log::warn;
use proc_macro::TokenStream;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use syn::parse_macro_input;

#[proc_macro]
pub fn flatc_gen(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);

    // Validate input file path
    let path = PathBuf::from(input.value());
    if !path.exists() {
        panic!("Flatbuffer file '{}' does not exist.", path.display());
    }
    let stem = path
        .file_stem()
        .expect("Cannot get the stem portion of filename")
        .to_str()
        .expect("Cannot convert filename into UTF-8");

    let work_dir = dirs::cache_dir()
        .expect("Cannot get global cache directory")
        .join("flatc-gen");
    fs::create_dir_all(&work_dir).expect("Failed to create cache directory");

    // Download flatbuffers
    //
    // FIXME use release version instead of HEAD
    let fbs_repo = work_dir.join("flatbuffers");
    if !fbs_repo.exists() {
        let st = Command::new("git")
            .args(&["clone", "http://github.com/google/flatbuffers"])
            .current_dir(&work_dir)
            .status()
            .expect("Git is not installed");
        if !st.success() {
            panic!("Git clone of google/flatbuffers failed");
        }
    }

    // Build flatbuffers
    let st = Command::new("cmake")
        .args(&["-Bbuild", "-H."])
        .current_dir(&fbs_repo)
        .status()
        .expect("cmake not found");
    if !st.success() {
        panic!("cmake failed with error code: {}", st.code().unwrap());
    }
    let st = Command::new("cmake")
        .args(&["--build", "build", "--target", "flatc"])
        .current_dir(&fbs_repo)
        .status()
        .expect("cmake not found");
    if !st.success() {
        panic!("cmake failed with error code: {}", st.code().unwrap());
    }

    let flatc = fbs_repo.join("build/flatc");

    // Generate Rust code from FlatBuffer definitions
    let st = Command::new(flatc)
        .args(&["-r", "-o"])
        .arg(&work_dir)
        .arg("-b")
        .arg(&path)
        .status()
        .expect("flatc command failed");
    if !st.success() {
        panic!("flatc failed: {}", st.code().expect("No error code"));
    }

    let generated = work_dir.join(format!("{}_generated.rs", stem));
    if !generated.exists() {
        panic!(
            "Generated Rust file '{}' does not found.",
            generated.display()
        );
    }

    // Optional: Format generated code
    match Command::new("rustfmt").arg(&generated).status() {
        Ok(st) => {
            if !st.success() {
                panic!("rustfmt failed: {}", st.code().expect("No error code"));
            }
        }
        Err(_) => warn!("rustfmt is not installed"),
    }

    let mut f = File::open(&generated).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code)
        .expect("Failed to read generated file");
    let ts: proc_macro2::TokenStream = syn::parse_str(&code).unwrap();
    ts.into()
}
