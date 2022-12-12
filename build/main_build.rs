extern crate cc;

use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;
use target_helper::{get_out_dir, get_root_dir};

mod target_helper;
mod xrt;
macro_rules! cargo_link {
	($feature:expr) => {
		println!("cargo:rustc-link-lib={}", $feature);
	};
}
macro_rules! cargo_link_s {
    ($feature:expr) => {
      println!("cargo:rustc-link-lib=static={}", $feature);
    };
}
fn main() {
    //panic!("info: {}", target_helper::get_out_dir());
    //panic!("{}", get_root_dir());
    generate_python_bindings();
    xrt::auxiliary::math::build();
    //xrt::auxiliary::util::build();
    cargo_link_s!("aux_math");
    cargo_link!("dylib=stdc++");
    generate_raw_bindings();
}

fn generate_raw_bindings() {
    let bindings = bindgen::Builder::default().clang_arg(format!("-I{}", get_root_dir() + "/monado/src/xrt/include")).header("src/static_wrapper.h").generate().unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
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