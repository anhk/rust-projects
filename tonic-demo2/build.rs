fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
    Ok(())
}
