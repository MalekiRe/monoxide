extern crate cc;

use std::fs;
use std::process::Command;
use target_helper::{get_out_dir, get_root_dir};

mod target_helper;
mod xrt;

fn main() {
    //panic!("info: {}", target_helper::get_out_dir());
    //panic!("{}", get_root_dir());
    generate_python_bindings();
    xrt::auxiliary::math::build();
    xrt::auxiliary::util::build();
}

fn generate_python_bindings() {
    gen_bindings("b_generated_bindings.h");
    gen_bindings("b_generated_bindings.c");
}
fn gen_bindings(str: &str) {
    let dir = get_root_dir() + "/monado/src/xrt/auxiliary/bindings";
    let out_dir = get_out_dir() + "/monado/src/xrt/auxiliary/bindings";
    fs::create_dir_all(out_dir.clone()).unwrap();
    let command = Command::new("python")
        .arg(dir.clone() + "/bindings.py")
        .arg(dir.clone() + "/bindings.json")
        .arg(format!("{}", out_dir + "/" + str))
        .status().unwrap();
}