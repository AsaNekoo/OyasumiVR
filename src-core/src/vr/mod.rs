#[cfg(windows)]
mod openvr;
pub mod commands;
pub async fn init(){
    #[cfg(windows)]
    openvr::init().await;
}