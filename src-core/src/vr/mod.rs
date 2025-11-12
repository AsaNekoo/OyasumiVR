#[cfg(windows)]
mod openvr;
pub mod commands;
pub mod model;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
}