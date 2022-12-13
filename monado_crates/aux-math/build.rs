use build_helper::{add_library, BuildHelper, get_default_build, get_root_dir, get_source_files};

struct AuxMathLib;

impl BuildHelper for AuxMathLib {
    fn get_system_libs() -> Vec<String> {
        vec![
            "eigen3"
        ].into_iter().map(|a| a.to_string()).collect()
    }

    fn get_source_paths() -> Vec<String> {
        vec![
            "monado/src/xrt/auxiliary/math"
        ].into_iter().map(|a| a.to_string()).collect()
    }

    fn get_include_dirs() -> Vec<String> {
        vec![
            "monado/src/xrt/auxiliary/math",
            "monado/src/xrt/auxiliary",
            "monado/src/xrt/include",
            "fake_headers",
        ].into_iter().map(|a| a.to_string()).collect()
    }

    fn blacklist_source_files() -> Vec<String> {
        vec![]
    }

    fn extra_source_files() -> Vec<String> {
        vec![]
    }
}

fn main() {
    let build = AuxMathLib::setup();
    AuxMathLib::compile_and_link(build, "aux_math");
    AuxMathLib::gen_bindings();
}

fn src_path() -> String {
    get_root_dir() + "/monado/src/xrt/auxiliary/math"
}