// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
//
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate aux_math;

use aux_math::{math_vec3_normalize, xrt_vec3};

pub fn main() {
    let mut vec = xrt_vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    unsafe {
        math_vec3_normalize(&mut vec);
    }
    println!("{:?}", vec);
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