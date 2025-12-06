use tokio::sync::Mutex;

use crate::vr::model::VRStatus;

#[cfg(windows)]
pub mod openvr;
#[cfg(unix)]
pub mod openxr;
pub mod commands;
pub mod model;
pub mod sleep_detector;
pub mod gesture_detector;
pub static VR_STATE: Mutex<VRStatus> = Mutex::const_new(VRStatus::Inactive);
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
    #[cfg(unix)]
    openxr::init().await;
}