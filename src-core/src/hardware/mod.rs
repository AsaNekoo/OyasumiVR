pub mod beyond;
#[cfg(windows)]
pub async fn init() {
    beyond::init().await;
}
