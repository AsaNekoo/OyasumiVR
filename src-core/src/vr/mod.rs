#[cfg(windows)]
mod openvr;
#[cfg(unix)]
mod openxr;
pub mod commands;
pub mod model;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
}