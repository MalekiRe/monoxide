use std::ffi::c_void;
use std::ptr::null_mut;
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn main() {

}
// extern "C" {
//     pub fn main_1(
//         argc: ::std::os::raw::c_int,
//         argv: *mut *mut ::std::os::raw::c_char,
//     ) -> ::std::os::raw::c_int;
//
//     pub fn u_trace_marker_init();
// }
//
//
//
// fn main() {
//    // unsafe { run_things(); }
//     unsafe {run_things();}
//     unsafe {main_1(0, null_mut());}
//     unsafe {u_trace_marker_init()};
//     println!("Hello World!");
// }

// extern "C" {
//     fn run_things();
// }