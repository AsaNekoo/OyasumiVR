pub mod openxr;
pub mod commands;
pub mod model;
pub mod sleep_detector;
pub mod gesture_detector;
pub static mut SLEEP_DETECTION_ENABLED:bool=true;
pub async fn init(){
    openxr::init().await;
}