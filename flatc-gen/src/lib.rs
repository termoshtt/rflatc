extern crate proc_macro;

use log::warn;
use proc_macro::TokenStream;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::Duration;
use syn::parse_macro_input;

fn check_output(output: &Output, command_name: &str) {
    if !output.status.success() {
        let out = String::from_utf8(output.stdout.clone()).expect("Failed to parse output");
        let err = String::from_utf8(output.stderr.clone()).expect("Failed to parse error output");
        eprintln!("=== {} output ===", command_name);
        eprintln!("{}", out);
        eprintln!("{}", err);
        panic!(
            "{} failed with error code: {}",
            command_name,
            output.status.code().unwrap()
        );
    }
}

#[proc_macro]
pub fn flatc_gen(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);

    // Validate input file path
    let path = PathBuf::from(input.value());
    let path = if path.is_relative() {
        let src = input.span().source_file(); // XXX This needs `RUSTFLAG=--cfg procmacro2_semver_exempt`
                                              // see https://docs.rs/proc-macro2/*/proc_macro2/#unstable-features
        if !src.is_real() {
            panic!("flatc_gen! with relative path is supported only from real file and nightly compiler");
        }
        let src = src.path();
        let basedir = src.parent().unwrap();
        basedir.join(path)
    } else {
        path
    };

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
    let lock_file = work_dir.join("flatc-gen.lock");
    {
        fs::File::create(&lock_file).expect("Cannot create lock file");
    }

    // inter-process exclusion (parallel cmake will cause problems)
    let mut count = 0;
    let _lock = loop {
        match file_lock::FileLock::lock(lock_file.to_str().unwrap(), true, true) {
            Ok(lock) => break lock,
            Err(err) => {
                count += 1;
                eprintln!("Waiting lock of {}, {:?}", lock_file.display(), err);
            }
        };
        // Try 30s to get lock
        if count > 30 {
            panic!("Cannot get lock of {} in 30s", lock_file.display());
        }
        sleep(Duration::from_secs(1));
    };

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
    let output = Command::new("cmake")
        .args(&["-Bbuild", "-H."])
        .current_dir(&fbs_repo)
        .output()
        .expect("cmake not found");
    check_output(&output, "cmake");

    let output = Command::new("cmake")
        .args(&["--build", "build", "--target", "flatc"])
        .current_dir(&fbs_repo)
        .output()
        .expect("cmake not found");
    check_output(&output, "cmake");

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
