pub mod target_helper;

use std::{env, fs};
use std::collections::{HashMap, HashSet};
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


pub trait MonadoBuilder {
    const DISTANCE_FROM_PARENT_DIR: i32 = 2;
    fn setup() -> Build {
        let mut build = create_default_build();
        Self::system_libs().iter().for_each(|lib| {
           add_library(&mut build, lib);
        });
        build.files(Self::source_files_found());
        //we create a runtime header file that includes all the headers we want
        build.includes(Self::include_dirs());
        build
    }
    fn generate_bindings() {
        generate_bindings(Self::include_dirs(), Self::header_files_found());
    }
    fn compile_and_link(build: Build, name: &str) {
        cargo_link!("dylib=stdc++");
        build.compile(name);
        cargo_link_s!(name);
    }
    fn current_root_dir() -> PathBuf {
        let path = env::var("CARGO_MANIFEST_DIR");
        let path = path.unwrap();
        Path::new(path.as_str())
            .to_path_buf()
    }
    fn out_dir() -> PathBuf {
        Path::new(env::var("OUT_DIR").unwrap().as_str()).to_path_buf()
    }
    fn parent_root_dir() -> PathBuf {
        let mut path = Self::current_root_dir();
        for _i in 0..Self::DISTANCE_FROM_PARENT_DIR {
            path = path.parent().unwrap().to_path_buf();
        }
        path
    }
    fn append_parent_root(str: &str) -> PathBuf {
        let mut path_buf= Self::parent_root_dir();
        path_buf.push(Path::new(str));
        path_buf
    }
    fn append_out_dir(str: &str) -> PathBuf {
        let mut path_buf = Self::out_dir();
        path_buf.push(Path::new(str));
        path_buf
    }
    fn append_parent_root_vec(things: Vec<&str>) -> Vec<PathBuf> {
        things.into_iter().map(Self::append_parent_root).collect()
    }
    fn append_out_dir_vec(things: Vec<&str>) -> Vec<PathBuf> {
        things.into_iter().map(Self::append_out_dir).collect()
    }

    fn source_files_found() -> Vec<PathBuf> {
        let mut files_found = Self::files_found(Self::source_file_dirs(), vec![".c".to_string(), ".cpp".to_string()]);
        files_found.append(&mut Self::extra_source_files());
        files_found
    }
    fn header_files_found() -> Vec<PathBuf> {
        let mut files_found = Self::files_found(Self::include_dirs(), vec![".h".to_string()]);
        files_found.append(&mut Self::extra_header_files());
        files_found
    }

    fn files_found(file_dirs: Vec<PathBuf>, patterns: Vec<String>) -> Vec<PathBuf>{
        let mut files = vec![];
        for file_dir in file_dirs {
            let matching_files = Self::matching_files_from_dir(file_dir, patterns.clone());
            for file in matching_files {
                if Self::blacklist().contains(&file) {
                    continue
                }
                if Self::conditionally_include().contains_key(&file) {
                    if !Self::conditionally_include().get(&file).unwrap() {
                        continue
                    }
                }
                files.push(file)
            }
        }
        files
    }

    fn matching_files_from_dir(dir: PathBuf, patterns: Vec<String>) -> Vec<PathBuf> {
        let mut matching_files = vec![];
        let dir = fs::read_dir(dir).unwrap();
        for possible_file in dir.into_iter() {
            let file = possible_file.unwrap();
            let file_name = file.file_name().to_str().unwrap().to_string();
            patterns.iter().for_each(|pattern| {
                if file_name.ends_with(pattern.as_str()) {
                    matching_files.push(file.path());
                    return;
                }
            });
        }
        matching_files
    }

    fn source_file_dirs() -> Vec<PathBuf>;
    fn include_dirs() -> Vec<PathBuf>;
    fn extra_source_files() -> Vec<PathBuf> {
        vec![]
    }
    fn extra_header_files() -> Vec<PathBuf> {
        vec![]
    }
    fn system_libs() -> Vec<String> {
        vec![]
    }
    fn conditionally_include() -> HashMap<PathBuf, bool> {
        HashMap::new()
    }
    fn blacklist() -> Vec<PathBuf> {
        vec![]
    }
}

// pub trait BuildHelper {
//     fn setup() -> Build {
//         let mut build = get_default_build();
//         Self::get_system_libs().iter().for_each(|lib| {
//             add_library(&mut build, lib.as_str());
//         });
//         Self::get_source_paths().iter().map(root_dir_prepend).for_each(|source_dir| {
//             for source_file in get_source_files(source_dir) {
//                 if !Self::blacklist_source_files().contains(&source_file) {
//                     build.file(source_file);
//                 } else {
//                     panic!("contains: {}", source_file);
//                 }
//             }
//         });
//         build.includes(Self::get_include_dirs().iter().map(root_dir_prepend));
//         build
//     }
//     /// stuff like eigen3, system libraries
//     fn get_system_libs() -> Vec<String>;
//     /// where the directory where the source files are located, the .c and .cpp
//     fn get_source_paths() -> Vec<String>;
//     /// what include dirs to use
//     fn get_include_dirs() -> Vec<String>;
//     fn compile_and_link(build: Build, name: &str) {
//         cargo_link!("dylib=stdc++");
//         build.compile(name);
//         cargo_link_s!(name);
//     }
//     /// used to exclude files otherwise included from `get_source_path`
//     fn blacklist_source_files() -> Vec<String> {
//         vec![]
//     }
//     /// used to add extra files individually
//     fn extra_source_files() -> Vec<String> {
//         vec![]
//     }
//     /// used to exclude header files from bindgen
//     fn blacklist_header_files() -> Vec<String> {
//         vec![]
//     }
//     fn gen_bindings() {
//         let mut bindgen_builder = bindgen::Builder::default();
//         let strings = Self::get_include_dirs().iter().map(root_dir_prepend).collect::<Vec<String>>();
//         for dir in strings {
//             bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", dir));
//         }
//         let strings =  Self::get_source_paths().iter().map(root_dir_prepend).collect::<Vec<String>>();
//         for dir in strings {
//             for header_file in get_header_files(dir).iter() {
//                 if !Self::blacklist_header_files().contains(header_file) {
//                     bindgen_builder = bindgen_builder.header(header_file);
//                 }
//             }
//         }
//         let macros = Arc::new(RwLock::new(HashSet::new()));
//         let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//         bindgen_builder = bindgen_builder.blocklist_type("FP_INFINITE")
//             .blocklist_type("FP_ZERO")
//             .blocklist_type("FP_SUBNORMAL")
//             .blocklist_type("FP_NORMAL");
//         bindgen_builder = bindgen_builder.parse_callbacks(Box::new(MacroCallback{macros}));
//         bindgen_builder.generate().unwrap()
//             .write_to_file(out_path.join("bindings.rs")).unwrap();
//     }
// }

fn generate_bindings(include_dirs: Vec<PathBuf>, header_files: Vec<PathBuf>) {
    let mut bindgen_builder = bindgen::Builder::default();
    for directory in include_dirs {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", directory.to_str().unwrap()));
    }
    for header_file in header_files {
        bindgen_builder = bindgen_builder.header(header_file.to_str().unwrap());
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

pub fn create_default_build() -> Build{
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