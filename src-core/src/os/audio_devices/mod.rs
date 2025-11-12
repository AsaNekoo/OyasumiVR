use serde::Serialize;

#[cfg(windows)]
#[allow(dead_code, unused_variables, non_upper_case_globals)]
pub mod device;
#[cfg(windows)]
#[allow(dead_code, unused_variables, non_upper_case_globals)]
#[cfg(windows)]
pub mod manager;
#[cfg(windows)]
#[allow(dead_code, unused_variables, non_upper_case_globals)]
mod wrappers;
#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
pub enum AudioDeviceType {
    Capture,
    Render,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDeviceDto {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub volume: f32,
    pub mute: bool,
    pub default: bool,
    pub default_communications: bool,
}