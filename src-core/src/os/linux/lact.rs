use lact_client::DaemonClient;
use log::warn;

use crate::{utils::send_event, Models::elevated_sidecar::SetMsiAfterburnerProfileError};
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
pub async fn set_lact_profile(profile: u32) -> Result<bool, SetMsiAfterburnerProfileError> {
    fn inner(profile: u32) -> Result<bool, SetMsiAfterburnerProfileError> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            match DaemonClient::connect().await {
                Ok(v) => {
                    // LACT.lock().await.replace(SendWapper { inner: v });
                    log::debug!("[core] LACT client started");
                    let profiles = v
                        .list_profiles(false)
                        .await
                        .map_err(|_| SetMsiAfterburnerProfileError::UnknownError)?;
                    let profile = profiles
                    .profiles
                    .iter()
                    .find(|p| p.0.starts_with(&format!("{}_", profile)))
                    .ok_or(SetMsiAfterburnerProfileError::InvalidProfileIndex)?;
                    v.set_profile(Some(profile.0.to_string()), false)
                    .await.unwrap();
                    Ok(true)
                }
                Err(err) => {
                    warn!("[core] failed to connect to lact daemon{:?}", err);
                    Err(SetMsiAfterburnerProfileError::ExeCannotExecute)
                }
            }
        })
    }
    if profile==0{
        return Ok(true);
    }
    std::thread::spawn(move || inner(profile)).join().unwrap()?;

    Ok(true)
}
