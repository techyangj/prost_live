// use prost_build::compile_protos;

// fn main() {
//     // OUT_DIR
//     compile_protos(&["person.proto"], &["."]);

// }

use prost_build::Config;

fn main() {
    // 提高效率  参考：https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");

    Config::new()
        .out_dir("src/pb")
        // .bytes(&["."])
        .btree_map(&["scores"])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["person.proto"], &["."])
        .unwrap();
}
