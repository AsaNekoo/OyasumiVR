use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct SystemPowerPolicy {
    pub name: String,
}
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
}
#[derive(Serialize,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewSound {
    pub name: String,
    pub duration_ms: u32,
}
