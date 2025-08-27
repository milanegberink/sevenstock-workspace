fn main() {
    tonic_prost_build::compile_protos("../../../proto/refresh_tokens.proto").unwrap();
}
