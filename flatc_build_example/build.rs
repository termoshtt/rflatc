use std::env;
use std::path::*;
use std::process::Command;

fn main() {
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

    let st = Command::new(flatc)
        .args(&["-r", "-o"])
        .arg(&out_dir)
        .args(&["-b", "fbs/addressbook.fbs"])
        .status()
        .expect("flatc command failed");
    if !st.success() {
        panic!("flatc failed: {}", st.code().unwrap());
    }

    let st = Command::new("rustfmt")
        .arg("addressbook_generated.rs")
        .current_dir(&out_dir)
        .status()
        .expect("rustfmt cannot start");
    if !st.success() {
        panic!("rustfmt failed: {}", st.code().unwrap());
    }
}
