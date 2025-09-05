use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    // Source file:
    // https://github.com/STMicroelectronics/st-mems-machine-learning-core/blob/main/examples/gym_activity_recognition/lsm6dsv320x/lsm6dsv320x_gym_activity_recognition_right.json
    let input_file = Path::new("lsm6dsv320x_gym_activity_recognition_right.json");
    let output_file = Path::new("src/config.rs");
    parser::generate_rs_from_json(&input_file, &output_file, "GYM_RIGHT", "LSM6DSV320X", false);

    println!("cargo:rerun-if-changed=lsm6dsv320x_gym_activity_recognition_right.json");
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
