#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn start_elevated_sidecar() {
    #[cfg(windows)]
    {
        let mut sidecar_manager_guard = super::SIDECAR_MANAGER.lock().await;
        let sidecar_manager = sidecar_manager_guard.as_mut().unwrap();
        sidecar_manager.start().await;
    }
    #[cfg(unix)]
    {
        use tokio::task::spawn_local;

        use crate::os::linux::lact;
        lact::init().await;
      
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_started() -> bool {
    #[cfg(windows)]
    {
        let mut sidecar_manager_guard = super::SIDECAR_MANAGER.lock().await;
        let sidecar_manager = sidecar_manager_guard.as_mut().unwrap();
        sidecar_manager.has_started().await
    }
    #[cfg(unix)]
    {
        // use crate::os::linux::lact;
        // lact::LACT.lock().await.is_some()
        true
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_get_grpc_web_port() -> Option<u32> {
    #[cfg(windows)]
    {
        let manager_guard = super::SIDECAR_MANAGER.lock().await;
        let manager = manager_guard.as_ref();
        match manager {
            Some(manager) => {
                let grpc_web_port = manager.grpc_web_port.lock().await;
                grpc_web_port.as_ref().map(|grpc_web_port| *grpc_web_port)
            }
            None => None,
        }
    }
    #[cfg(unix)]
    {
        None
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn elevated_sidecar_get_grpc_port() -> Option<u32> {
    #[cfg(windows)]
    {
        let manager_guard = super::SIDECAR_MANAGER.lock().await;
        let manager = manager_guard.as_ref();
        match manager {
            Some(manager) => {
                let grpc_port = manager.grpc_port.lock().await;
                grpc_port.as_ref().map(|grpc_port| *grpc_port)
            }
            None => None,
        }
    }
    #[cfg(unix)]
    {
        None
    }
}
