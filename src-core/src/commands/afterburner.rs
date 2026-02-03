use crate::os::linux::lact;
use crate::Models::elevated_sidecar::GpuProfileError;
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn gpu_set_profile(profile: String) -> Result<bool, GpuProfileError> {
    lact::set_lact_profile(profile).await
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn gpu_get_profiles() -> Result<Vec<String>, GpuProfileError> {
    lact::get_lact_profiles().await
}
