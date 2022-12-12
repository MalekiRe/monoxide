use cc::Build;
use target_helper::get_out_dir;

pub fn build() -> &'static str {
    let mut build = cc::Build::new();
    build.files(get_build_files());
    build.include("monado/src/xrt/auxiliary");
    build.include("monado/src/xrt/auxiliary/math");
    build.include("monado/src/xrt/include");
    build.include("monado/src/external/nanopb");
    build.include("monado/src/external");
    build.include("monado/src/external/cjson");
    build.include("fake_headers");
    build.include(get_out_dir()+"/monado/src/xrt/auxiliary");
    build.object(get_out_dir() + "/libaux_math.a");
    build.static_flag(true);
    build.warnings(false);
    build.shared_flag(true);
    let name = "aux_util";
    build.compile(name);
    name
}
fn get_build_files() -> Vec<String>{
    let mut files = vec![
        "u_autoexpgain.c",
        "u_bitwise.c",
        "u_builders.c",
        "u_debug.c",
        "u_deque.cpp",
        "u_device.c",
        "u_distortion.c",
        "u_distortion_mesh.c",
        "u_file.c",
        "u_file.cpp",
        "u_format.c",
        "u_frame.c",
        "u_git_tag.h",
        "u_hand_tracking.c",
        "u_hand_simulation.c",
        "u_handles.c",
        "u_hashmap.cpp",
        "u_hashset.cpp",
        "u_id_ringbuffer.cpp",
        "u_imu_sink_split.c",
        "u_imu_sink_force_monotonic.c",
        "u_json.c",
        "u_logging.c",
        "u_metrics.c",
        "u_misc.c",
        "u_pacing_app.c",
        "u_pacing_compositor.c",
        "u_pacing_compositor_fake.c",
        "u_pretty_print.c",
        "u_prober.c",
        "u_string_list.cpp",
        "u_system_helpers.c",
        "u_time.cpp",
        "u_trace_marker.c",
        "u_tracked_imu_3dof.c",
        "u_var.cpp",
        "u_vector.cpp",
        "u_config_json.c",
        "u_win32_com_guard.cpp",
        "u_worker.c",
        "u_worker.cpp",
    ];
    files.into_iter().map(|mut file| {
        "monado/src/xrt/auxiliary/util/".to_string() + file
    }).collect()
}