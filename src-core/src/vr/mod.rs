#[cfg(windows)]
pub mod openvr;
#[cfg(unix)]
pub mod openxr;
pub mod commands;
pub mod model;
pub mod sleep_detector;
pub mod gesture_detector;
pub static mut SLEEP_DETECTION_ENABLED:bool=true;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
    #[cfg(unix)]
    openxr::init().await;
}