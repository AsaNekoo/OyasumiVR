#![cfg(any(windows,linux))]
use serde::Serialize;
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
#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
pub enum AudioDeviceType {
    Capture,
    Render,
}