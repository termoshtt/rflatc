extern crate proc_macro;

use log::warn;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::*;
use std::process::Command;

#[proc_macro]
pub fn flatc_gen(path: TokenStream) -> TokenStream {
    // Validate input file path
    let path = PathBuf::from(path.to_string());
    if !path.exists() {
        panic!("Flatbuffer file '{}' does not exist.");
    }
    let stem = path
        .file_stem()
        .expect("Cannot get the stem portion of filename")
        .to_str()
        .expect("Cannot convert filename into UTF-8");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Download flatbuffers
    //
    // FIXME use release version instead of HEAD
    let fbs_repo = out_dir.join("flatbuffers");
    if !fbs_repo.exists() {
        let st = Command::new("git")
            .args(&["clone", "http://github.com/google/flatbuffers"])
            .current_dir(&out_dir)
            .status()
            .expect("Git is not installed");
        if !st.success() {
            panic!("Git clone of google/flatbuffers failed");
        }
    }

    let dst = cmake::build(fbs_repo);
    let flatc = dst.join("bin/flatc");

    // Generate Rust code from FlatBuffer definitions
    let st = Command::new(flatc)
        .args(&["-r", "-o"])
        .arg(&out_dir)
        .arg("-b")
        .arg(&path)
        .status()
        .expect("flatc command failed");
    if !st.success() {
        panic!("flatc failed: {}", st.code().unwrap());
    }

    let generated = out_dir.join(format!("{}_generated.rs", stem));
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
                panic!("rustfmt failed: {}", st.code().unwrap());
            }
        }
        Err(_) => warn!("rustfmt is not installed"),
    }

    let mut f = File::open(&generated).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code)
        .expect("Failed to read generated file");
    quote!(code).into()
}
