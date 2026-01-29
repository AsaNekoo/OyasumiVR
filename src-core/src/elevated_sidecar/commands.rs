#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn start_elevated_sidecar() {
    use tokio::task::spawn_local;

    use crate::os::linux::lact;
    lact::init().await;
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_started() -> bool {
    true
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_get_grpc_web_port() -> Option<u32> {
    None
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_get_grpc_port() -> Option<u32> {
    None
}
