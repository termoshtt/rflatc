use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let output = Command::new("flatc")
        .args(&["-r", "-o", &out_dir])
        .args(&["-b", "fbs/addressbook.fbs"])
        .output()
        .unwrap();
    println!("{:?}", output);
}
