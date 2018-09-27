use std::fs::File;
use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    File::create(out_dir.join("build-date.txt"))
        .unwrap()
        .write_all(build_date().unwrap().as_bytes())
        .unwrap();
}

fn build_date() -> Option<String> {
    let mut result = Command::new("date")
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
        .unwrap();

    let len = result.len() - 1;
    result.truncate(len);
    Some(result)
}
