use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, IntoStaticStr};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OVRFrameLimits {
    pub additional_frames_to_predict: u8,
    pub frames_to_throttle: u8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OVRDevice {
    pub index: u32,
    pub class: TrackedDeviceClass,
    pub role: TrackedControllerRole,
    pub battery: Option<f32>,
    pub provides_battery_status: Option<bool>,
    pub can_power_off: Option<bool>,
    pub is_charging: Option<bool>,
    pub dongle_id: Option<String>,
    pub serial_number: Option<String>,
    pub hardware_revision: Option<String>,
    pub manufacturer_name: Option<String>,
    pub model_number: Option<String>,
    pub handle_type: Option<OVRHandleType>,
    pub hmd_on_head: Option<bool>,
    pub hmd_activity: Option<String>,
    pub display_frequency: Option<f32>,
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum TrackedControllerRole {
    Invalid,
    LeftHand,
    RightHand,
    OptOut,
    Treadmill,
    Stylus,
}
#[derive(Clone, Serialize, Deserialize, IntoStaticStr, EnumIter)]
pub enum OVRHandleType {
    HandPrimary,
    HandSecondary,
    Head,
    Gamepad,
    Treadmill,
    Stylus,
    FootLeft,
    FootRight,
    ShoulderLeft,
    ShoulderRight,
    ElbowLeft,
    ElbowRight,
    KneeLeft,
    KneeRight,
    WristLeft,
    WristRight,
    AnkleLeft,
    AnkleRight,
    Waist,
    Chest,
    Camera,
    Keyboard,
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TrackedDeviceClass {
    Invalid,
    HMD,
    Controller,
    GenericTracker,
    TrackingReference,
    DisplayRedirect,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingOriginData {
    pub localized_controller_type: String,
    pub localized_hand: String,
    pub localized_input_source: String,
    pub device_path_name: String,
    pub input_path_name: String,
    pub mode_name: String,
    pub slot_name: String,
    pub input_source_type: String,
}
#[derive(Serialize, Clone,Display,Copy,PartialEq,Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum VRStatus {
    Inactive,
    Initializing,
    Initialized,
}
#[derive(Clone, Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct SleepDetectorStateReport {
    pub distance_in_last_15_minutes: f32,
    pub distance_in_last_10_minutes: f32,
    pub distance_in_last_5_minutes: f32,
    pub distance_in_last_1_minute: f32,
    pub distance_in_last_10_seconds: f32,
    // pub rotation_in_last_15_minutes: f32,
    // pub rotation_in_last_10_minutes: f32,
    // pub rotation_in_last_5_minutes: f32,
    // pub rotation_in_last_1_minute: f32,
    // pub rotation_in_last_10_seconds: f32,
    pub start_time: u64,
    pub last_log: u64,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GestureDetected {
    pub gesture: String,
}