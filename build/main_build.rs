extern crate cc;

mod target_helper;
mod xrt;

fn main() {
    xrt::auxiliary::math::build();
}