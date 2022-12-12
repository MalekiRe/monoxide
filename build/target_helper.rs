use std::env;
use target_helper::TargetOs::{Android, Dragonfly, FreeBSD, IOS, Linux, MacOS, NetBSD, OpenBSD, Windows};


pub fn get_out_dir() -> String {
    env::var("OUT_DIR").unwrap().as_str().to_string()
}
pub fn get_root_dir() -> String {
    env::var("CARGO_MANIFEST_DIR").unwrap().as_str().to_string()
}
pub enum TargetFamily {
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
pub enum TargetOs {
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