use super::models::Output;
use super::models::WindowsPowerPolicy;
use crate::os::linux::audio::LinuxAudioError;
use crate::os::linux::audio::get_linux_audio_manager;
use crate::os::models::AudioDeviceDto;
use crate::utils::VRCHAT_ACTIVE;
use crate::warn_unimplemented;
use log::error;

use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tauri::command]
pub async fn is_windows() -> bool {
    cfg!(windows)
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn play_sound(name: String, volume: f32) {
    log::debug!("[core] playing: {} volume:{}", name, volume);
    if volume == 0.0 {
        return;
    }
    let guard = super::PLAY_SOUND_TX.lock().await;
    let tx = match guard.as_ref() {
        Some(tx) => tx,
        None => {
            error!("[Core] Could not play sound, as sound player was not initialized");
            return;
        }
    };
    let _ = tx.send((name, volume)).await;
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn quit_steamvr(kill: bool) {
    crate::utils::quit_steamvr(kill).await;
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn is_vrchat_active() -> bool {
    unsafe { VRCHAT_ACTIVE }
    // true
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn run_command(command: String, args: Vec<String>) -> Result<Output, String> {
    log::debug!("running: {} with args:{:?}", command, args);

    Command::new(command)
        .args(args)
        .output()
        .map_err(|err| err.to_string())
        .map(|res| Output {
            stdout: String::from_utf8_lossy(&res.stdout).to_string(),
            stderr: String::from_utf8_lossy(&res.stderr).to_string(),
            status: res.status.code().unwrap_or_default(),
        })
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn run_cmd_commands(commands: String) {
    async fn inner(commands: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::options()
            .write(true)
            .create_new(true)
            .open("/tmp/Oyasumi_bash_script.sh")
            .await?;
        file.write_all(commands.as_bytes()).await?;
        Command::new("chmod")
            .arg("+x")
            .arg("/tmp/Oyasumi_bash_script.sh")
            .output()?;
        let out = Command::new("bash")
            .arg("/tmp/Oyasumi_bash_script.sh")
            .output()?;
        if !out.status.success() {
            log::warn!("[core] command failed:{:?}", out);
        }
        Ok(())
    }
    tokio::task::spawn(async {
        use tokio::fs;

        if let Err(err) = inner(commands).await {
            error!("[core] command failed with:{:?}", err);
        }
        fs::remove_file("/tmp/Oyasumi_bash_script.sh").await.ok();
    });
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn show_in_folder(path: String) {
    if let Err(err) = Command::new("xdg-open").arg(&path).spawn() {
        log::error!(
            "Failed to spawn file explorer with path:{} : {:?}",
            path,
            err
        );
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_system_power_policy(guid: String) {
    use crate::os::linux::power_managment::LINUX_POWER_POLICY_MANAGER;

    LINUX_POWER_POLICY_MANAGER.lock().await.set_policy(guid);
}
#[tauri::command]
pub async fn set_power_policy_provider(name: String) {
    use crate::os::linux::power_managment::LINUX_POWER_POLICY_MANAGER;

    LINUX_POWER_POLICY_MANAGER.lock().await.set_provider(name);
}
#[tauri::command]
pub async fn get_power_policy_providers() -> Vec<String> {
    use crate::os::linux::power_managment::LINUX_POWER_POLICY_MANAGER;

    LINUX_POWER_POLICY_MANAGER.lock().await.get_providers()
}
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn active_system_power_policy() -> Option<WindowsPowerPolicy> {
    use crate::os::linux::power_managment::LINUX_POWER_POLICY_MANAGER;
    let current = LINUX_POWER_POLICY_MANAGER
        .lock()
        .await
        .get_current_profile();
    Some(WindowsPowerPolicy {
        guid: current.clone(),
        name: current,
    })
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_system_power_policies() -> Vec<WindowsPowerPolicy> {
    use crate::os::linux::power_managment::LINUX_POWER_POLICY_MANAGER;

    LINUX_POWER_POLICY_MANAGER
        .lock()
        .await
        .get_avalible_profiles()
        .into_iter()
        .map(|p| WindowsPowerPolicy {
            guid: p.clone(),
            name: p,
        })
        .collect()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn windows_is_elevated() -> bool {
    true
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn system_shutdown(message: String, timeout: u32, force_close_apps: bool) {
    let _ = system_shutdown::shutdown();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, expect(unused_variables))]
pub async fn system_reboot(message: String, timeout: u32, force_close_apps: bool) {
    let _ = system_shutdown::reboot();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn system_sleep() {
    let _ = system_shutdown::sleep();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn system_hibernate() {
    let _ = system_shutdown::hibernate();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn system_logout() {
    let _ = system_shutdown::logout();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_audio_devices(refresh: bool) -> Vec<AudioDeviceDto> {
    let mut manager_guard = get_linux_audio_manager().await;
    if let Some(manager) = manager_guard.as_mut() {
        if refresh {
            if let Err(err) = manager.refresh_devices() {
                //something broke
                if matches!(err, LinuxAudioError::OutOfOrder) {
                    error!("[core] recived unexpected sequence number when refresh audio devices");
                    *manager_guard = None;
                }
                error!("[core] Failed to refresh audio devices: {:?}", err);
                return Vec::new();
            }
        }
        manager
            .devices()
            .iter()
            .map(|device: &super::linux::audio::LinuxAudioDevice| {
                AudioDeviceDto::from(device.clone())
            })
            .collect::<Vec<AudioDeviceDto>>()
    } else {
        error!("[Core] Could not get audio devices, as audio device manager was not initialized");
        Vec::new()
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_audio_device_volume(device_id: String, volume: f32) {
    let mut manager_guard =get_linux_audio_manager().await;
    if let Some(manager) = manager_guard.as_mut() {
        if let Err(error) = manager.set_audio_device_volume(device_id, volume) {
            //something broke
            if matches!(error, LinuxAudioError::OutOfOrder) {
                error!("[core] recived unexpected sequence number when refresh audio devices");
                *manager_guard = None;
            } else {
                error!("[Core] Could not set audio device volume, {:?}", error);
            }
        }
    } else {
        error!(
            "[Core] Could not set audio device volume, as audio device manager was not initialized"
        );
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_audio_device_mute(device_id: String, mute: bool) {
    let mut manager_guard = get_linux_audio_manager().await;
    if let Some(manager) = manager_guard.as_mut() {
        if let Err(error) = manager.set_audio_device_mute(device_id, mute) {
            //something broke
            if matches!(error, LinuxAudioError::OutOfOrder) {
                error!("[core] recived unexpected sequence number when refresh audio devices");
                *manager_guard = None;
            }else {
                error!("[Core] Could not set audio device mute state, {:?}", error);
            }
        }
    } else {
        error!("[Core] Could not set audio device mute state, as audio device manager was not initialized");
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn set_hardware_mic_activity_enabled(enabled: bool) {
    warn_unimplemented!();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn set_hardware_mic_activivation_threshold(threshold: f32) {
    warn_unimplemented!();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn set_mic_activity_device_id(device_id: Option<String>) {
    warn_unimplemented!();
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn is_elevation_security_disabled() -> bool {
    true
}
#[tauri::command]
pub async fn pause_mpris_players() {
    use mpris::PlayerFinder;

    let players = match match PlayerFinder::new() {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to connect to dbus: {:?}", e);
            return;
        }
    }
    .find_all()
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to find mpris compatible player: {:?}", e);
            return;
        }
    };
    for player in players {
        if let Err(err) = player.pause() {
            log::warn!("failed to pause: {:?} with: {:?}", player.bus_name(), err);
        }
    }
}
