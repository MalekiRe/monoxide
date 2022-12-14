use std::path::PathBuf;
use build_helper::{cargo_link_s, MonadoBuilder};

struct Builder;

// impl MonadoBuilder for Builder {
//     fn source_file_dirs() -> Vec<PathBuf> {
//         Builder::append_parent_root_vec(vec![
//             "monado/src/xrt/auxiliary/gstreamer"
//         ])
//     }
//
//     fn include_dirs() -> Vec<PathBuf> {
//         let mut dirs = Builder::append_parent_root_vec(vec![
//             "monado/src/xrt/auxiliary/gstreamer",
//             "monado/src/xrt/auxiliary",
//             "monado/src/xrt/include",
//             "fake_headers",
//         ]);
//         dirs.push("glib.h");
//         dirs
//     }
//
//     fn system_libs() -> Vec<String> {
//         vec!["gstreamer-1.0".to_string(), "gstreamer-app-1.0".to_string(), "gstreamer-video-1.0".to_string()]
//     }
// }
fn main() {
    // let build = Builder::setup();
    // Builder::compile_and_link(build, "aux-gstreamer");
    // Builder::generate_bindings();
    //cargo_link_s!("")

}
