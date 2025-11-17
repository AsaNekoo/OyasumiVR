#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_osc_address() -> Option<String> {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_oscquery_address() -> Option<String> {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn stop_osc_server() {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn start_osc_server() -> Option<(String, String)> {
    None
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn add_osc_method(method: OSCMethod) {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_osc_method_value(address: String, value: String) {}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn osc_send_command(
    addr: String,
    osc_addr: String,
    types: Vec<SupportedOscType>,
    values: Vec<String>,
) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn osc_valid_addr(addr: String) -> bool {
    true
}
async fn osc_send(addr: String, osc_addr: String, data: Vec<OscType>) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_osc_receive_address_whitelist(whitelist: Vec<String>) {}
