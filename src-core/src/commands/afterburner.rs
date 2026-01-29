use crate::Models::elevated_sidecar::SetMsiAfterburnerProfileError;

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn msi_afterburner_set_profile(
    _: String,
    profile: u32,
) -> Result<bool, SetMsiAfterburnerProfileError> {
    use crate::os::linux::lact;

    lact::set_lact_profile(profile).await
}
