use crate::{
    vr::{
        model::{BindingOriginData, OVRDevice, OVRFrameLimits, SleepState},
        openxr::SLEEP_DETECTOR,
        SLEEP_DETECTION_ENABLED,
    },
    warn_unimplemented,
};

pub static mut SLEEP_STATE: SleepState = SleepState::Awake;
#[tauri::command]
pub async fn set_sleep_state(state: SleepState) {
    unsafe {
        SLEEP_STATE = state;
    }
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_set_app_framelimit(
    app_id: u32,
    limits: Option<OVRFrameLimits>,
) -> Result<(), String> {
    crate::os::linux::mangohud::limit_frame_rate(app_id, limits).await;
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_app_framelimit(app_id: u32) -> Result<Option<OVRFrameLimits>, String> {
    Ok(crate::os::linux::mangohud::get_app_framelimit(app_id).await)
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_sleep_mode_check(value: bool) {
    match value {
        true => super::openxr::start_head_shake_detection().await,
        false => super::openxr::stop_head_shake_detection().await,
    }
}
#[tauri::command]
pub async fn vr_sleep_detection_enabled(value: bool) {
    unsafe {
        if !SLEEP_DETECTION_ENABLED && value {
            SLEEP_DETECTOR.lock().await.reset_start_time().await;
        }
        SLEEP_DETECTION_ENABLED = value;
    }
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn openvr_set_init_delay_fix(enabled: bool) {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_devices() -> Vec<OVRDevice> {
    //fixme
    Vec::new()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_status() -> String {
    super::openxr::OXR_STATE
        .lock()
        .await
        .to_string()
        .to_uppercase()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn openvr_set_analog_gain(analog_gain: f32) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_analog_gain() -> Result<f32, String> {
    Ok(1.)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn vr_set_analog_color_temp(temperature: Option<u32>) -> Result<(f64, f64, f64), String> {
    warn_unimplemented!();
    Err("not implemented".into())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_set_image_brightness(
    brightness: f64,
    perceived_brightness_adjustment_gamma: Option<f64>,
) {
    super::openxr::set_brightness(brightness, perceived_brightness_adjustment_gamma).await;
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn vr_launch_binding_configuration(show_on_desktop: bool) {
    //fixme: figure this out
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn openvr_is_dashboard_visible() -> bool {
    false
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn openvr_reregister_manifest() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
#[cfg_attr(unix, allow(unused_variables))]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_binding_origins(
    action_set_key: String,
    action_key: String,
) -> Option<Vec<BindingOriginData>> {
    //fixme: look at it and see wha it does
    None
}
