pub mod commands;

use crate::utils::sidecar_manager::SidecarManager;
use crate::{
    Models::overlay_sidecar::oyasumi_overlay_sidecar_client::OyasumiOverlaySidecarClient,
    Models::oyasumi_core::OverlaySidecarStartArgs, utils::send_event,
};
use oyasumi_shared::RESOURCES_PATH;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub static SIDECAR_GRPC_CLIENT: LazyLock<Mutex<Option<OyasumiOverlaySidecarClient<Channel>>>> =
    LazyLock::new(Default::default);
static SIDECAR_MANAGER: LazyLock<Mutex<Option<SidecarManager>>> = LazyLock::new(Default::default);

pub async fn init() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    *SIDECAR_MANAGER.lock().await = Some(SidecarManager::new(
        "OVERLAY".to_string(),
        RESOURCES_PATH.join("sidecars").to_string_lossy().to_string(),
        "oyasumivr-overlay-sidecar".to_string(),
        tx,
        true,
        vec![],
    ));

    // Listen for sidecar stop signals
    tokio::spawn(async move {
        while (rx.recv().await).is_some() {
            *SIDECAR_GRPC_CLIENT.lock().await = None;
            send_event("OVERLAY_SIDECAR_STOPPED", ()).await;
        }
    });
}

pub async fn handle_overlay_sidecar_start(
    args: &OverlaySidecarStartArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let manager_guard = SIDECAR_MANAGER.lock().await;
    let manager = manager_guard.as_ref().unwrap();
    // Ignore this signal if it is invalid
    if !manager
        .handle_start_signal(
            Some(args.grpc_port),
            Some(args.grpc_web_port),
            args.pid,
            None,
        )
        .await
    {
        return Ok(());
    }
    // Create new GRPC client
    let grpc_client =
        OyasumiOverlaySidecarClient::connect(format!("http://127.0.0.1:{}", args.grpc_port))
            .await?;
    *SIDECAR_GRPC_CLIENT.lock().await = Some(grpc_client);
    send_event("OVERLAY_SIDECAR_STARTED", args.grpc_web_port).await;
    Ok(())
}
