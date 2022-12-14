use std::collections::HashMap;
use std::path::PathBuf;
use build_helper::MonadoBuilder;


struct Builder;

impl MonadoBuilder for Builder {
    fn source_file_dirs() -> Vec<PathBuf> {
        Builder::append_parent_root_vec(vec![
            "monado/src/xrt/auxiliary/os"
        ])
    }

    fn include_dirs() -> Vec<PathBuf> {
        Builder::append_parent_root_vec(vec![
            "monado/src/xrt/auxiliary/os",
            "monado/src/xrt/auxiliary",
            "monado/src/xrt/include",
            "fake_headers",
        ])
    }

    fn extra_source_files() -> Vec<PathBuf> {
        Builder::append_parent_root_vec(vec!["monado/src/xrt/auxiliary/os/os_ble_stubs.c"])
    }
    fn blacklist() -> Vec<PathBuf> {
        Builder::append_parent_root_vec(vec!["monado/src/xrt/auxiliary/os/os_ble_dbus.c"])
    }
}
fn main() {
    let build = Builder::setup();
    Builder::compile_and_link(build, "aux-os");
    Builder::generate_bindings();
}
