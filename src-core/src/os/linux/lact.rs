use lact_client::DaemonClient;
use log::warn;

use crate::{utils::send_event, Models::elevated_sidecar::GpuProfileError};
// unsafe impl<T> Send for SendWapper<T> {}
// pub struct SendWapper<T> {
//     inner: T,
// }
// impl<T> Deref for SendWapper<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
// impl<T> DerefMut for SendWapper<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }
pub async fn init() {
    send_event("ELEVATED_SIDECAR_STARTED", 0).await;
    // let h=tokio::runtime::Handle::current();
    // h.spa

    // tokio::task::spawn( tokio::task::LocalSet::new().run_until(spawn_local(inner())));
    // let local = tokio::task::LocalSet::new()
}
//fixme: fix this attrocity
pub async fn set_lact_profile(profile: String) -> Result<bool, GpuProfileError> {
    fn inner(profile: String) -> Result<bool, GpuProfileError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            match DaemonClient::connect().await {
                Ok(v) => {
                    if profile.is_empty(){
                        return Ok(true);
                    }
                    // LACT.lock().await.replace(SendWapper { inner: v });
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
                    v.set_profile(Some(profile.0.to_string()), false)
                        .await
                        .unwrap();
                    Ok(true)
                }
                Err(err) => {
                    warn!("[core] failed to connect to lact daemon{:?}", err);
                    Err(GpuProfileError::ExeCannotExecute)
                }
            }
        })
    }
  
    std::thread::spawn(move || inner(profile)).join().unwrap()?;

    Ok(true)
}
pub async fn get_lact_profiles() -> Result<Vec<String>, GpuProfileError> {
    fn inner() -> Result<Vec<String>, GpuProfileError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            match DaemonClient::connect().await {
                Ok(v) => {
                    // LACT.lock().await.replace(SendWapper { inner: v });
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
        })
    }

    std::thread::spawn(inner).join().unwrap()
}
