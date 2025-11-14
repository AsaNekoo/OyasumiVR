fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("../proto/overlay-sidecar.proto")?;
    tonic_prost_build::compile_protos("../proto/oyasumi-core.proto")?;
    Ok(())
}