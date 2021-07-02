extern crate protobuf_codegen_pure;

// Example custom build script.
fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src")
        .inputs(&["pb/protos.proto"])
        .include("pb")
        .run()
        .expect("protoc");
}
