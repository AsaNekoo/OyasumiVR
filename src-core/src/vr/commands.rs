#[cfg(windows)]
use super::openvr::{
    models::{BindingOriginData, OVRDevice, OVRFrameLimits},
    OVR_CONTEXT,
};
#[cfg(windows)]
use crate::globals::STEAM_APP_KEY;
use crate::{vr::model::{BindingOriginData, OVRDevice, OVRFrameLimits}, warn_unimplemented};
#[cfg(windows)]
use enumset::EnumSet;
#[cfg(windows)]
use log::error;
#[cfg(windows)]
use ovr::input::{InputString, InputValueHandle};
#[cfg(windows)]
use ovr_overlay as ovr;
#[cfg(windows)]
use substring::Substring;

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_set_app_framelimit(
    app_id: u32,
    limits: Option<OVRFrameLimits>,
) -> Result<(), String> {
    #[cfg(windows)]
    return super::framelimiter::set_app_framelimits(app_id, limits).await;
    #[cfg(unix)]
    crate::os::linux::mangohud::limit_frame_rate(app_id, limits).await;
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_app_framelimit(app_id: u32) -> Result<Option<OVRFrameLimits>, String> {
    #[cfg(windows)]
    return super::framelimiter::get_app_framelimits(app_id).await;
    #[cfg(unix)]
    Ok(crate::os::linux::mangohud::get_app_framelimit(app_id).await)
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_sleep_mode_check(value:bool){
    #[cfg(unix)]
    {match value {
        true => super::openxr::start_head_shake_detection().await,
        false => super::openxr::stop_head_shake_detection().await,
    }}
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn openvr_set_init_delay_fix(enabled: bool) {
    #[cfg(windows)]
    {
        *super::OVR_INIT_DELAY_FIX.lock().await = enabled;
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_devices() -> Vec<OVRDevice> {
    #[cfg(windows)]
    return super::devices::get_devices().await;
    #[cfg(unix)]
    //fixme
    Vec::new()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_status() -> String {
    #[cfg(windows)]
    {
       super::openvr::OVR_STATUS.lock().await.to_string().to_uppercase()
    }
    #[cfg(unix)]
    crate::vr::VR_STATE.lock().await.to_string().to_uppercase()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn openvr_set_analog_gain(analog_gain: f32) -> Result<(), String> {
    #[cfg(windows)]
    return super::brightness_analog::set_analog_gain(analog_gain).await;
    #[cfg(unix)]
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_analog_gain() -> Result<f32, String> {
    #[cfg(windows)]
    return super::brightness_analog::get_analog_gain().await;
    #[cfg(unix)]
    Ok(1.)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn vr_set_supersample_scale(supersample_scale: Option<f32>) -> Result<(), String> {
    #[cfg(windows)]
    return super::supersampling::set_supersample_scale(supersample_scale).await;
    #[cfg(unix)]
    //fixme: add this feature to libmonado
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_supersample_scale() -> Result<Option<f32>, String> {
    #[cfg(windows)]
    return super::supersampling::get_supersample_scale().await;
    #[cfg(unix)]
     //fixme: add this feature to libmonado
    Ok(None)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn openvr_set_fade_distance(fade_distance: f32) -> Result<(), String> {
    #[cfg(windows)]
    return super::chaperone::set_fade_distance(fade_distance).await;
    #[cfg(unix)]
    Ok(())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn openvr_get_fade_distance() -> Result<f32, String> {
    #[cfg(windows)]
    return super::chaperone::get_fade_distance().await;
    #[cfg(unix)]
    //is chapperon a thing in openxr?
    Ok(f32::INFINITY)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn vr_set_analog_color_temp(
    temperature: Option<u32>,
) -> Result<(f64, f64, f64), String> {
    #[cfg(windows)]
    return super::colortemp_analog::set_color_temp(temperature).await;
    #[cfg(unix)]
    warn_unimplemented!();
    Err("not implemented".into())

}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn vr_set_image_brightness(
    brightness: f64,
    perceived_brightness_adjustment_gamma: Option<f64>,
) {
    #[cfg(windows)]
    super::openvr::brightness_overlay::set_brightness(brightness, perceived_brightness_adjustment_gamma)
        .await;
    #[cfg(unix)]
    super::openxr::set_brightness(brightness, perceived_brightness_adjustment_gamma).await;
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn vr_launch_binding_configuration(show_on_desktop: bool) {
    #[cfg(windows)]
    {
        let context = OVR_CONTEXT.lock().await;
        let mut input = match context.as_ref() {
            Some(context) => context.input_mngr(),
            None => return,
        };
        let input_handle = match input.get_input_source_handle("/user/hand/right") {
            Ok(handle) => handle,
            Err(e) => {
                error!("[Core] Failed to get input source handle: {}", e);
                return;
            }
        };
        if let Err(e) = input.open_binding_ui(None, None, input_handle, show_on_desktop) {
            error!("[Core] Failed to open SteamVR binding UI: {}", e);
        }
    }
    //fixme: figure this out
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn openvr_is_dashboard_visible() -> bool {
    #[cfg(windows)]
    {
        let context = OVR_CONTEXT.lock().await;
        let mut manager = match context.as_ref() {
            Some(context) => context.overlay_mngr(),
            None => return false,
        };
        return manager.is_dashboard_visible();
    }
        #[cfg(unix)]
        {
            //there is no one dashboard on openxr
            false
        }
    
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn openvr_reregister_manifest() -> Result<(), String> {
    #[cfg(windows)]
    {
        let ctx = OVR_CONTEXT.lock().await;
        let mut applications = ctx.as_ref().unwrap().applications_mngr();
        let manifest_path_buf = std::fs::canonicalize("resources/manifest.vrmanifest").unwrap();
        let manifest_path: &std::path::Path = manifest_path_buf.as_ref();
        match applications.is_application_installed(STEAM_APP_KEY) {
            Ok(value) => {
                if !value {
                    return Err(String::from("MANIFEST_NOT_REGISTERED"));
                } else {
                    match applications.remove_application_manifest(manifest_path) {
                        Ok(_) => {
                            let install_for_flavours = [
                                crate::flavour::BuildFlavour::Standalone,
                                crate::flavour::BuildFlavour::Dev,
                            ];
                            let should_install_for_flavour =
                                install_for_flavours.contains(&crate::flavour::BUILD_FLAVOUR);
                            if should_install_for_flavour {
                                match applications.add_application_manifest(manifest_path, false) {
                                    Ok(_) => {
                                        return Ok(());
                                    }
                                    Err(e) => {
                                        error!("[Core] Failed to add VR manifest: {}", e);
                                        return Err(String::from("MANIFEST_ADD_FAILED"));
                                    }
                                };
                            } else {
                                return Err(String::from("FLAVOUR_NOT_ELIGIBLE"));
                            }
                        }
                        Err(e) => {
                            error!("[Core] Failed to remove VR manifest: {}", e);
                            return Err(String::from("MANIFEST_REMOVE_FAILED"));
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    "[Core] Failed to check if VR manifest is registered: {:#?}",
                    e.description()
                );
                return Err(String::from("MANIFEST_CHECK_FAILED"));
            }
        }
    }
    #[cfg(unix)]{
        //not used
        Ok(())
    }
}

#[tauri::command]
#[cfg_attr(unix, allow(unused_variables))]
#[oyasumivr_macros::command_profiling]
pub async fn vr_get_binding_origins(
    action_set_key: String,
    action_key: String,
) -> Option<Vec<BindingOriginData>> {
    #[cfg(windows)]
    {
        let mut input_ctx = super::OVR_INPUT_CONTEXT.lock().await;
        // Get action set by name
        let action_set = match input_ctx
            .action_sets
            .iter()
            .find(|a| a.name == action_set_key)
        {
            Some(action_set) => action_set.handle,
            None => return None,
        };
        // Get action by name
        let action = match input_ctx.actions.iter().find(|a| a.name == action_key) {
            Some(action) => action.handle,
            None => return None,
        };
        // Get the input service
        let context = OVR_CONTEXT.lock().await;
        let mut input = match context.as_ref() {
            Some(context) => context.input_mngr(),
            None => return None,
        };
        if let Err(e) = input.update_actions(input_ctx.active_sets.as_mut_slice()) {
            error!("[Core] Failed to update actions: {}", e);
            return None;
        }
        // Get all of the origins for this action
        let origins: Vec<u64> = match input.get_action_origins(action_set, action) {
            Ok(origins) => origins
                .iter()
                .filter(|origin| **origin > 0)
                .cloned()
                .collect(),
            Err(e) => {
                error!("[Core] Failed to get action origins: {}", e);
                return None;
            }
        };
        // Get the localized controller types for each origin
        let localized_controller_types: Vec<String> = origins
            .iter()
            .filter_map(|origin| {
                match input.get_origin_localized_name(
                    InputValueHandle(*origin),
                    EnumSet::only(InputString::ControllerType),
                ) {
                    Ok(name) => Some(name),
                    Err(e) => {
                        error!(
                            "[Core] Failed to get origin localized name controller types: {}",
                            e.description()
                        );
                        None
                    }
                }
            })
            .collect();
        // Get the localized hands for each origin
        let localized_hands: Vec<String> = origins
            .iter()
            .filter_map(|origin| {
                match input.get_origin_localized_name(
                    InputValueHandle(*origin),
                    EnumSet::only(InputString::Hand),
                ) {
                    Ok(name) => Some(name),
                    Err(e) => {
                        error!(
                            "[Core] Failed to get origin localized name controller types: {}",
                            e.description()
                        );
                        None
                    }
                }
            })
            .collect();
        // Get the localized input sources for each origin
        let localized_input_sources: Vec<String> = origins
            .iter()
            .filter_map(|origin| {
                match input.get_origin_localized_name(
                    InputValueHandle(*origin),
                    EnumSet::only(InputString::InputSource),
                ) {
                    Ok(name) => Some(name),
                    Err(e) => {
                        error!(
                            "[Core] Failed to get origin localized name controller types: {}",
                            e.description()
                        );
                        None
                    }
                }
            })
            .collect();

        // Get extra information about each binding
        let result = {
            let result: Vec<ovr::sys::InputBindingInfo_t> =
                match input.get_action_binding_info(action) {
                    Ok(result) => result,
                    Err(e) => {
                        error!("[Core] Failed to get action binding info: {}", e);
                        return None;
                    }
                };
            Some(result)
        };
        let binding_infos = match result {
            Some(infos) => infos,
            None => return None,
        };

        // Group the data for each origin
        let mut datas = vec![];
        for i in 0..origins.len() {
            let binding_info = &binding_infos[i];
            let data = BindingOriginData {
                localized_controller_type: localized_controller_types[i].clone(),
                localized_hand: localized_hands[i].clone(),
                localized_input_source: localized_input_sources[i].clone(),
                device_path_name: crate::utils::convert_char_array_to_string(
                    &binding_info.rchDevicePathName,
                )
                .expect("Failed to convert rchDevicePathName to string"),
                input_path_name: crate::utils::convert_char_array_to_string(
                    &binding_info.rchInputPathName,
                )
                .expect("Failed to convert rchInputPathName to string"),
                mode_name: crate::utils::convert_char_array_to_string(&binding_info.rchModeName)
                    .expect("Failed to convert rchModeName to string"),
                slot_name: crate::utils::convert_char_array_to_string(&binding_info.rchSlotName)
                    .expect("Failed to convert rchSlotName to string"),
                input_source_type: crate::utils::convert_char_array_to_string(
                    &binding_info.rchInputSourceType,
                )
                .expect("Failed to convert rchInputSourceType to string"),
            };
            datas.push(data);
        }

        Some(datas)
    }
    #[cfg(unix)]{
        //fixme: look at it and see wha it does
        None
    }
}
