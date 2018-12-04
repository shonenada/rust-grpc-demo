extern crate protoc_grpcio;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let proto_root = "protos";
    let proto_files = ["echo.proto"];
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=protos");

    protoc_grpcio::compile_grpc_protos(&proto_files, &[proto_root], &out_dir)
        .expect("Failed to compile gRPC definitions!");

    // Create mod.rs accordingly
    let mod_file_content = proto_files
        .iter()
        .map(|proto_file| {
            let protofile = format!("{}/{}", &proto_root, &proto_file);
            let proto_path = Path::new(&protofile);
            let mod_name = proto_path
                    .file_stem()
                    .expect("Unable to extract stem")
                    .to_str()
                    .expect("Unable to extract filename");
            format!(
                "pub mod {};\npub mod {}_grpc;", mod_name, mod_name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut mod_file = File::create(Path::new(&out_dir).join("mod.rs")).unwrap();
    mod_file
        .write_all(mod_file_content.as_bytes())
        .expect("Unable to write mod file");
}
