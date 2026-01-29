use crate::Models::elevated_sidecar::{NvmlDevice, NvmlSetPowerManagementLimitError, NvmlStatus};

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn nvml_status() -> NvmlStatus {
    NvmlStatus::InitComplete
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn nvml_get_devices() -> Vec<NvmlDevice> {
    Vec::new()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn nvml_set_power_management_limit(
    uuid: String,
    power_limit: u32,
) -> Result<bool, NvmlSetPowerManagementLimitError> {
    Ok(true)
}
