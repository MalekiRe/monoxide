use std::env;
use std::path::PathBuf;
use TargetOs::{Android, Dragonfly, FreeBSD, IOS, Linux, MacOS, NetBSD, OpenBSD, Windows};

macro_rules! cargo_cmake_feat {
	($feature:literal) => {
		if cfg!(feature = $feature) {
			"ON"
		} else {
			"OFF"
		}
	};
}

macro_rules! cargo_link_static {
	($feature:expr) => {
		println!("cargo:rustc-link-lib=static={}", $feature);
	};
}

fn main() {
    let target_os = TargetOs::get();
    let target_family = TargetFamily::get();
    cmake_build();
    bindings_generate();
}

const MONADO_PATH: &'static str = "monado";

fn cmake_build() {
    let mut cmake_config = cmake::Config::new(MONADO_PATH);
    cmake_config.define("XRT_FEATURE_OPENXR", "ON");
    cmake_config.define("XRT_FEATURE_SERVICE", "ON");
    cmake_config.define("XRT_FEATURE_IPC","ON");
    let dst = cmake_config.build();
    println!("cargo:rustc-link-search=native={}/build/src/xrt/ipc", dst.display());
    cargo_link_static!("ipc_shared");
    cargo_link_static!("ipc_server");
    println!("cargo:rustc-link-search=native={}/build/src/xrt/auxiliary/util", dst.display());
    cargo_link_static!("aux_util");
    cargo_link_static!("aux_util_process");
    cargo_link_static!("aux_util_sink");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=monado-service");
    println!("cargo:rustc-link-lib=static=my_test");
}

fn bindings_generate() {
    let mut bindings = bindgen::Builder::default()
        .header("src/static_wrapper.h").generate().unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}



enum TargetFamily {
    Windows,
    Wasm,
    Unix,
}
impl TargetFamily {
    pub fn get() -> Self {
        Self::from(env::var("CARGO_CFG_TARGET_FAMILY").unwrap().as_str())
    }
    pub fn from(str: &str) -> Self {
        match str {
            "windows" => TargetFamily::Windows,
            "unix" => TargetFamily::Unix,
            "wasm" => TargetFamily::Wasm,
            &_=> panic!("unknown target family")
        }
    }
}
enum TargetOs {
    Windows,
    MacOS,
    IOS,
    Linux,
    Android,
    FreeBSD,
    Dragonfly,
    OpenBSD,
    NetBSD
}
impl TargetOs {
    pub fn get() -> Self {
        Self::from(env::var("CARGO_CFG_TARGET_OS").unwrap().as_str())
    }
    pub fn from(str: &str) -> Self {
        match str {
            "windows" => Windows,
            "macos" => MacOS,
            "ios" => IOS,
            "linux" => Linux,
            "android" => Android,
            "freebsd" => FreeBSD,
            "dragonfly" => Dragonfly,
            "openbsd" => OpenBSD,
            "netbsd" => NetBSD,
            &_ => panic!("unknown target os")
        }
    }
}