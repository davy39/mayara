use std::{env, fs, path::PathBuf};

fn main() {
    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        .pure()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["src/protos"])
        // Inputs must reside in some of include paths.
        .input("src/protos/RadarMessage.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();

    let body = reqwest::blocking::get(
        "https://cdn.rawgit.com/dcodeIO/protobuf.js/6.11.0/dist/protobuf.min.js",
    )
    .unwrap()
    .text()
    .unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut dest_path = PathBuf::from(&out_dir);
    dest_path.push("web");
    fs::create_dir_all(&dest_path).unwrap();
    dest_path.push("protobuf.min.js");
    fs::write(&dest_path, body).unwrap();

    let body = reqwest::blocking::get(
        "https://cdn.rawgit.com/dcodeIO/protobuf.js/6.11.0/dist/protobuf.js",
    )
    .unwrap()
    .text()
    .unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut dest_path = PathBuf::from(&out_dir);
    dest_path.push("web");
    dest_path.push("protobuf.js");
    fs::write(&dest_path, body).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut src_path = PathBuf::from("src");
    src_path.push("protos");
    src_path.push("RadarMessage.proto");
    let mut dest_path = PathBuf::from(&out_dir);
    dest_path.push("web");
    dest_path.push("RadarMessage.proto");
    fs::copy(&src_path, &dest_path).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}
