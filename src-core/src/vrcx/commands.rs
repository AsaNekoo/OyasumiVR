
#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[cfg_attr(unix, allow(unused_variables))]
pub async fn vrcx_log(msg: String) -> bool {
    //fixme: expose ipc in vrcx
    true
}
