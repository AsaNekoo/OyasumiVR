use lact_client::DaemonClient;
use log::warn;

use crate::{utils::send_event, Models::elevated_sidecar::GpuProfileError};

pub async fn init() {
    send_event("ELEVATED_SIDECAR_STARTED", 0).await;
}
// lact_client::DaemonClient is not Send, so run it on its own thread with a current-thread runtime
async fn run_local<F, Fut, T>(f: F) -> Result<T, GpuProfileError>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, GpuProfileError>>,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| GpuProfileError::UnknownError)?
            .block_on(f())
    })
    .await
    .map_err(|_| GpuProfileError::UnknownError)?
}
pub async fn set_lact_profile(profile: String) -> Result<bool, GpuProfileError> {
    async fn inner(profile: String) -> Result<bool, GpuProfileError> {
            match DaemonClient::connect().await {
                Ok(v) => {
                    if profile.is_empty(){
                        return Ok(true);
                    }
                    log::debug!("[core] LACT client started");
                    let profiles = v
                        .list_profiles(false)
                        .await
                        .map_err(|_| GpuProfileError::UnknownError)?;
                    let profile = profiles
                        .profiles
                        .iter()
                        .find(|p|*p.0==profile)
                        .ok_or(GpuProfileError::InvalidProfileIndex)?;
                    if let Err(err)=v.set_profile(Some(profile.0.to_string()), false)
                        .await{
                            log::error!("failed to set lact profile: {:?}",err);
                            return Err(GpuProfileError::UnknownError);
                        }
                    Ok(true)
                }
                Err(err) => {
                    warn!("[core] failed to connect to lact daemon{:?}", err);
                    Err(GpuProfileError::ExeCannotExecute)
                }
            }
    }
    run_local(move || inner(profile)).await
}
pub async fn get_lact_profiles() -> Result<Vec<String>, GpuProfileError> {
    async fn inner() -> Result<Vec<String>, GpuProfileError> {
     
            match DaemonClient::connect().await {
                Ok(v) => {
                    log::debug!("[core] LACT client started");
                    Ok(v.list_profiles(false)
                        .await
                        .map_err(|_| GpuProfileError::UnknownError)?
                        .profiles
                        .iter()
                        .map(|p| p.0)
                        .cloned()
                        .collect())
                }
                Err(err) => {
                    warn!("[core] failed to connect to lact daemon{:?}", err);
                    Err(GpuProfileError::ExeCannotExecute)
                }
            }
    }
    run_local(inner).await
}
