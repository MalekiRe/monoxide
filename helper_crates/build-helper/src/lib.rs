pub mod target_helper;

use std::{env, fs};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use cc::Build;
use color_eyre::Result;
#[macro_export]
macro_rules! cargo_link {
	($feature:expr) => {
		println!("cargo:rustc-link-lib={}", $feature);
	};
}
#[macro_export]
macro_rules! cargo_link_s {
    ($feature:expr) => {
      println!("cargo:rustc-link-lib=static={}", $feature);
    };
}
#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        match name {
            "FP_NAN" => MacroParsingBehavior::Ignore,
            "FP_INFINITE" => MacroParsingBehavior::Ignore,
            "FP_ZERO" => MacroParsingBehavior::Ignore,
            "FP_SUBNORMAL" => MacroParsingBehavior::Ignore,
            "FP_NORMAL" => MacroParsingBehavior::Ignore,
            _ => MacroParsingBehavior::Default,
        }
    }
}

pub trait BuildHelper {
    fn setup() -> Build {
        let mut build = get_default_build();
        Self::get_system_libs().iter().for_each(|lib| {
            add_library(&mut build, lib.as_str());
        });
        Self::get_source_paths().iter().map(root_dir_prepend).for_each(|source_dir| {
            for source_file in get_source_files(source_dir) {
                if !Self::blacklist_source_files().contains(&source_file) {
                    build.file(source_file);
                } else {
                    panic!("contains: {}", source_file);
                }
            }
        });
        build.includes(Self::get_include_dirs().iter().map(root_dir_prepend));
        build
    }
    /// stuff like eigen3, system libraries
    fn get_system_libs() -> Vec<String>;
    /// where the directory where the source files are located, the .c and .cpp
    fn get_source_paths() -> Vec<String>;
    /// what include dirs to use
    fn get_include_dirs() -> Vec<String>;
    fn compile_and_link(build: Build, name: &str) {
        cargo_link!("dylib=stdc++");
        build.compile(name);
        cargo_link_s!(name);
    }
    /// used to exclude files otherwise included from `get_source_path`
    fn blacklist_source_files() -> Vec<String> {
        vec![]
    }
    /// used to add extra files individually
    fn extra_source_files() -> Vec<String> {
        vec![]
    }
    /// used to exclude header files from bindgen
    fn blacklist_header_files() -> Vec<String> {
        vec![]
    }
    fn gen_bindings() {
        let mut bindgen_builder = bindgen::Builder::default();
        let strings = Self::get_include_dirs().iter().map(root_dir_prepend).collect::<Vec<String>>();
        for dir in strings {
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", dir));
        }
        let strings =  Self::get_source_paths().iter().map(root_dir_prepend).collect::<Vec<String>>();
        for dir in strings {
            for header_file in get_header_files(dir).iter() {
                if !Self::blacklist_header_files().contains(header_file) {
                    bindgen_builder = bindgen_builder.header(header_file);
                }
            }
        }
        let macros = Arc::new(RwLock::new(HashSet::new()));
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindgen_builder = bindgen_builder.blocklist_type("FP_INFINITE")
            .blocklist_type("FP_ZERO")
            .blocklist_type("FP_SUBNORMAL")
            .blocklist_type("FP_NORMAL");
        bindgen_builder = bindgen_builder.parse_callbacks(Box::new(MacroCallback{macros}));
        bindgen_builder.generate().unwrap()
            .write_to_file(out_path.join("bindings.rs")).unwrap();
    }
}
pub fn get_default_build() -> Build{
    let mut build = cc::Build::new();
    build
        .warnings(false)
        .shared_flag(true)
        .static_flag(true);
    build
}
pub fn add_library(build: &mut Build, lib_name: &str) {
    let lib = pkg_config::Config::new().probe(lib_name).unwrap();
    build.include(lib.include_paths.first().unwrap());
}
pub fn get_out_dir() -> String {
    env::var("OUT_DIR").unwrap().as_str().to_string()
}
pub fn get_root_dir() -> String {
    let s = env::var("CARGO_MANIFEST_DIR").unwrap().as_str().to_string();
    let p = Path::new(&s);
    p.parent().unwrap().parent().unwrap().to_str().unwrap().to_string()
}
pub fn root_dir_prepend(str: impl Into<String>) -> String {
    get_root_dir() + "/" + str.into().as_str()
}
pub fn get_source_files(build_path: impl AsRef<Path>) -> Vec<String> {
    let paths = fs::read_dir(build_path).unwrap();
    let mut files = Vec::new();
    paths.into_iter().for_each(|path| {
        let path = path.unwrap();
        let name = path.file_name().to_str().unwrap().to_string();
        if name.ends_with(".c")
            || name.ends_with(".cpp") {
            files.push(path.path().to_str().unwrap().to_string());
        }
    });
    files
}
pub fn get_header_files(build_path: impl AsRef<Path>) -> Vec<String> {
    let paths = fs::read_dir(build_path).unwrap();
    let mut files = Vec::new();
    paths.into_iter().for_each(|path| {
        let path = path.unwrap();
        let name = path.file_name().to_str().unwrap().to_string();
        if name.ends_with(".h")
            /*|| name.ends_with(".hpp")*/ {
            files.push(path.path().to_str().unwrap().to_string());
        }
    });
    files
}