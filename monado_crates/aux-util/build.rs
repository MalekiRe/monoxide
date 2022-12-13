use std::fs;
use std::path::PathBuf;
use std::process::Command;
use build_helper::{BuildHelper, get_out_dir, get_root_dir};
use build_helper::target_helper::TargetOs::Windows;

struct AuxUtilBuild;

impl BuildHelper for AuxUtilBuild {
    fn get_system_libs() -> Vec<String> {
        vec![]
    }

    fn get_source_paths() -> Vec<String> {
        vec!["monado/src/xrt/auxiliary/util"]
            .into_iter().map(|a| a.to_string()).collect()
    }

    fn get_include_dirs() -> Vec<String> {
        vec![
            "fake_headers",
            "monado/src/xrt/auxiliary",
            "monado/src/xrt/auxiliary/math",
            "monado/src/xrt/include",
            "monado/src/external/nanopb",
            "monado/src/external",
            "monado/src/external/cjson"
        ].into_iter().map(|a| { a.to_string() }).collect()
    }

    fn blacklist_source_files() -> Vec<String> {
        let mut list = vec![];
        if build_helper::target_helper::TargetOs::get() == Windows {
            list.push("u_windows.c");
        }
        list.into_iter().map(|a| {a.to_string()}).collect()
    }
    fn blacklist_header_files() -> Vec<String> {
        let mut list = vec![];
        if build_helper::target_helper::TargetOs::get() == Windows {
            list.push("u_windows.h");
        }
        list.into_iter().map(|a| {a.to_string()}).collect()
    }

    fn extra_source_files() -> Vec<String> {
        vec![
            get_out_dir() + "/" + "monado/src/xrt/auxiliary/bindings/b_generated_bindings.c"
        ].into_iter().map(|a| {a.to_string()}).collect()
    }
}
fn main() {
    generate_python_bindings();
    let mut build = AuxUtilBuild::setup();
    build.include(get_out_dir() + "/" + "monado/src/xrt/auxiliary");
    build.include(get_out_dir() + "/" + "monado/src/xrt/auxiliary/bindings");

    AuxUtilBuild::compile_and_link(build, "aux-util");
    AuxUtilBuild::gen_bindings();
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