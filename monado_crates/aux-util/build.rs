use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use build_helper::MonadoBuilder;
use build_helper::target_helper::TargetOs;

use build_helper::target_helper::TargetOs::Windows;

struct AuxUtilBuild;

impl MonadoBuilder for AuxUtilBuild {
    fn source_file_dirs() -> Vec<PathBuf> {
        AuxUtilBuild::append_parent_root_vec(vec!["monado/src/xrt/auxiliary/util"])
    }

    fn include_dirs() -> Vec<PathBuf> {
        let mut dirs = AuxUtilBuild::append_parent_root_vec(vec![
            "fake_headers",
            "monado/src/xrt/auxiliary",
            "monado/src/xrt/auxiliary/util",
            "monado/src/xrt/auxiliary",
            "monado/src/xrt/include",
            "monado/src/external/nanopb",
            "monado/src/external",
            "monado/src/external/cjson",
        ]);
        dirs.push(AuxUtilBuild::append_out_dir("monado/src/xrt/auxiliary/bindings"));
        dirs.push(AuxUtilBuild::append_out_dir("monado/src/xrt/auxiliary"));
        dirs
    }

    fn extra_source_files() -> Vec<PathBuf> {
        vec![AuxUtilBuild::append_out_dir("monado/src/xrt/auxiliary/bindings/b_generated_bindings.c")]
    }

    // fn extra_header_files() -> Vec<PathBuf> {
    //     vec![AuxUtilBuild::append_out_dir("monado/src/xrt/auxiliary/bindings/b_generated_bindings.h")]
    // }
    fn conditionally_include() -> HashMap<PathBuf, bool> {
        let mut map = HashMap::new();
        map.insert(AuxUtilBuild::append_parent_root("monado/src/xrt/auxiliary/util/u_windows.c"),TargetOs::get() == Windows
        );
        map.insert(AuxUtilBuild::append_parent_root("monado/src/xrt/auxiliary/util/u_windows.h"),
            TargetOs::get() == Windows
        );
        map
    }
}

fn main() {
    generate_python_bindings();
    let mut build = AuxUtilBuild::setup();
    AuxUtilBuild::compile_and_link(build, "aux-util");
    AuxUtilBuild::generate_bindings();
}

fn generate_python_bindings() {
    gen_bindings("b_generated_bindings.h");
    gen_bindings("b_generated_bindings.c");
}
fn gen_bindings(str: &str) {
    let dir = AuxUtilBuild::append_parent_root("monado/src/xrt/auxiliary/bindings").to_str().unwrap().to_string();
    let out_dir = AuxUtilBuild::append_out_dir("monado/src/xrt/auxiliary/bindings").to_str().unwrap().to_string();
    fs::create_dir_all(out_dir.clone()).unwrap();
    let command = Command::new("python")
        .arg(dir.clone() + "/bindings.py")
        .arg(dir.clone() + "/bindings.json")
        .arg(format!("{}", out_dir + "/" + str))
        .status().unwrap();
}