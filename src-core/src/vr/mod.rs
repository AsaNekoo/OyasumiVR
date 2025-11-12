#[cfg(windows)]
pub mod openvr;
#[cfg(unix)]
pub mod openxr;
pub mod commands;
pub mod model;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
    #[cfg(unix)]
    openxr::init().await;
}