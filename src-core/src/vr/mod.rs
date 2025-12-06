#[cfg(windows)]
pub mod openvr;
#[cfg(unix)]
pub mod openxr;
pub mod commands;
pub mod model;
pub mod sleep_detector;
pub mod gesture_detector;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
    #[cfg(unix)]
    openxr::init().await;
}