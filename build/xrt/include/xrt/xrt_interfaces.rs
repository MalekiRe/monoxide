use std::fs;
use std::fs::FileType;
use cc::Build;

macro_rules! dir {
    ($str:literal) => {
        ("monado/src/xrt/include/xrt/".to_owned() + $str)
    }
}

// pub fn build() -> &'static str {
//     // let mut build = Build::new();
//     // build.file("monado/src/xrt/include/xrt/xrt_compiler.h");
//     // //build.file("monado/src/xrt/include/xrt/xrt_defines.h");
//     // //build.file("monado/src/xrt/include/xrt/xrt_device.h");
//     // //build.files(get_build_files());
//     // build.shared_flag(true);
//     // build.static_flag(true);
//     // build.
//     // let name = "xrt_interfaces";
//     // build.compile(name);
//     // name
// }

fn get_build_files() -> Vec<String> {
    let paths = fs::read_dir("monado/src/xrt/include/xrt").unwrap();
    let mut files = Vec::new();
    paths.into_iter().for_each(|path| {
        let path = path.unwrap();
        let name = path.file_name().to_str().unwrap().to_string();
        if name.ends_with(".h")
        || name.ends_with(".hpp") {
            files.push(path.path().to_str().unwrap().to_string());
        }
    });
    files
}