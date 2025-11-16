use std::{path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=ts-src/index.ts");
    let mut out_dir=PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // out_dir.push("modified_proto");
    // fs::create_dir(&out_dir).unwrap();
    // out_dir.push("modified-overlay-sidecar.proto");
    // let mut data=fs::read_to_string("../proto/overlay-sidecar.proto").unwrap();
    // data=data.replacen("SyncState", "sync_state", 1);
    // fs::write(&out_dir, data).unwrap();
    // tonic_prost_build::compile_protos(out_dir)?;
    // tonic_prost_build::configure()
    //     .type_attribute("*", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_protos(
    //         &[
    //             "../proto/oyasumi-core.proto",
    //             "../proto/overlay-sidecar.proto",
    //         ],
    //         &["../proto/"],
    //     )?; 
    tonic_prost_build::compile_protos("../proto/oyasumi-core.proto")?;
    tonic_prost_build::compile_protos("../proto/overlay-sidecar.proto")?;
    // panic!("{}",format!("esbuild ts-src/index.ts --bundle --outfile={}/bundle.js --minify=true",out_dir.to_str().unwrap()));
    #[cfg(not(debug_assertions))]
    Command::new("npx").args(format!("esbuild ts-src/index.ts --bundle --outfile={}/bundle.js --minify=true",out_dir.to_str().unwrap()).split(" ")).output().unwrap();
    #[cfg(debug_assertions)]
    Command::new("npx").args(format!("esbuild ts-src/index.ts --bundle --outfile={}/bundle.js --minify=false",out_dir.to_str().unwrap()).split(" ")).output().unwrap();
    Ok(())
}
