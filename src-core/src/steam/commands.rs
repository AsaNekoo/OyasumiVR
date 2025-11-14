#![cfg_attr(not(feature="steam"),expect(unused_variables))]
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn steam_active() -> bool {
    #[cfg(feature = "steam")]
    return crate::steam::STEAMWORKS_CLIENT.lock().await.is_some();
    false
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn steam_achievement_get(achievement_id: String) -> Result<bool, String> {
    #[cfg(feature = "steam")]
    {
        let mut client_guard = crate::steam::STEAMWORKS_CLIENT.lock().await;
        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => return Err("CLIENT_NOT_INITIALIZED".to_string()),
        };
        let stats = client.user_stats();
        let achievement = stats.achievement(achievement_id.as_str());
        return match achievement.get() {
            Ok(status) => Ok(status),
            Err(_) => return Err("FAILED_TO_GET_STATUS".to_string()),
        };
    }
    Err("STEAM_SUPPORT_DISABLED".to_string())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn steam_achievement_set(achievement_id: String, unlocked: bool) -> Result<(), String> {
    #[cfg(feature = "steam")]
    {
        let mut client_guard = crate::steam::STEAMWORKS_CLIENT.lock().await;
        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => return Err("CLIENT_NOT_INITIALIZED".to_string()),
        };
        let stats = client.user_stats();
        let achievement = stats.achievement(achievement_id.as_str());
        let status = match achievement.get() {
            Ok(status) => status,
            Err(_) => return Err("FAILED_TO_GET_STATUS".to_string()),
        };
        if status == unlocked {
            return Ok(());
        }
        if unlocked {
            if achievement.set().is_err() {
                return Err("FAILED_TO_SET_STATUS".to_string());
            }
        } else if achievement.clear().is_err() {
            return Err("FAILED_TO_SET_STATUS".to_string());
        }
        return match stats.store_stats() {
            Ok(_) => Ok(()),
            Err(_) => return Err("FAILED_TO_STORE_STATS".to_string()),
        };
    }
    Ok(())
}
