use cc::Build;

pub fn build() -> &'static str{
    let x = pkg_config::Config::new().probe("eigen3").unwrap();
    let mut build = cc::Build::new();
    build.cpp(true);
    build.include("/usr/include/eigen3");
    //build.file("monado/src/xrt/auxiliary/math/m_base.cpp");
    build.include("monado/src/xrt/auxiliary/math");
    build.include("monado/src/xrt/auxiliary");
    build.include("monado/src/xrt/include");
    build.include("fake_headers");
    build.files(get_build_files());
    let name = "aux_math";
    build.compile(name);
    name
}

pub fn get_build_files() -> Vec<String> {
    let files = vec![
        "m_base.cpp",
        "m_filter_fifo.c",
        "m_filter_one_euro.c",
        "m_hash.cpp",
        "m_imu_3dof.c",
        "m_imu_pre.c",
        "m_lowpass_float.cpp",
        "m_lowpass_integer.cpp",
        "m_optics.c",
        "m_permutation.c",
        "m_predict.c",
        "m_quatexpmap.cpp",
        "m_relation_history.cpp",
        "m_space.cpp",
    ];
    files.iter().map(|file| {
        "monado/src/xrt/auxiliary/math/".to_string() + file
    }).collect()
}