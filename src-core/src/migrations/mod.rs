mod old_wix_uninstall;

pub async fn run_migrations() {
    #[cfg(target_os="windows")]
    old_wix_uninstall::run().await;
}
