mod old_wix_uninstall;

pub async fn run_migrations() {
    #[cfg(disabled)]
    old_wix_uninstall::run().await;
}
