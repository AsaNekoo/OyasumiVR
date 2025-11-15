use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut out_dir=PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // out_dir.push("modified_proto");
    // fs::create_dir(&out_dir).unwrap();
    // out_dir.push("modified-overlay-sidecar.proto");
    // let mut data=fs::read_to_string("../proto/overlay-sidecar.proto").unwrap();
    // data=data.replacen("SyncState", "sync_state", 1);
    // fs::write(&out_dir, data).unwrap();
    // tonic_prost_build::compile_protos(out_dir)?;
    tonic_prost_build::compile_protos("../proto/oyasumi-core.proto")?;
    tonic_prost_build::compile_protos("../proto/overlay-sidecar.proto")?;
    Ok(())
}