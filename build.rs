use std::fs::File;
use std::process::{Command, Stdio};
use std::os::unix::prelude::*;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    assert!(Command::new("/usr/bin/env")
            .arg("cp")
            .args(&[ "./.libcore/libcore.rlib", &*format!("{}", out_dir)])
            .status().unwrap().success());
}

