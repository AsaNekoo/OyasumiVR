use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
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
    pub distance_in_last_10_seconds: f32,
    pub start_time: u64,
    pub last_log: u64,
}
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)] 
pub enum SleepState{
    Awake=1,
    Preparing=2,
    Sleeping=3
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VRDevicePose {
    pub index: u32,
    pub quaternion: [f32; 4],
    pub position: [f32; 3],
}