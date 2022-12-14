fn main() {
    let mut cmake_config = cmake::Config::new("monado");
    let dst = cmake_config.build();
}