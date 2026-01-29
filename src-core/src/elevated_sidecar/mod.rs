#![cfg_attr(unix, allow(unused_imports))]
#![cfg_attr(unix, allow(unused_variables))]
pub mod commands;
use crate::utils::sidecar_manager::SidecarManager;
use crate::{
    utils::send_event,
    Models::elevated_sidecar::oyasumi_elevated_sidecar_client::OyasumiElevatedSidecarClient,
    Models::oyasumi_core::ElevatedSidecarStartArgs,
};
use log::info;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tonic::transport::Channel;

#[allow(dead_code)]
pub async fn request_stop() {}

pub async fn handle_elevated_sidecar_start(
    args: &ElevatedSidecarStartArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
