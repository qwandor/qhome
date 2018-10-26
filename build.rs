extern crate protoc_rust_grpc;

fn main() {
  protoc_rust_grpc::run(protoc_rust_grpc::Args {
    out_dir: "src",
    includes: &["googleapis"],
    input: &["googleapis/google/assistant/embedded/v1alpha2/embedded_assistant.proto", "googleapis/google/type/latlng.proto"],
    rust_protobuf: true, // also generate protobuf messages, not just services
    ..Default::default()
  }).expect("protoc-rust-grpc");
}
