use std::sync::OnceLock;

use ppd::PpdProxy;
static MANANGER: OnceLock<LinuxPowerPolicyManager> = OnceLock::new();
pub async fn get_power_manager() -> Option<&'static LinuxPowerPolicyManager<'static>> {
    if let Some(manager) = MANANGER.get() {
        return Some(manager);
    } else {
        match LinuxPowerPolicyManager::new().await {
            Ok(v) => {
                assert!(MANANGER.set(v).is_ok());
                Some(MANANGER.get().unwrap())
            }
            Err(err) => {
                log::error!(
                    "[core] failed to initialized power policy manager: {:?}",
                    err
                );
                None
            }
        }
    }
}
pub struct LinuxPowerPolicyManager<'a> {
    proxy: PpdProxy<'a>,
}
impl LinuxPowerPolicyManager<'_> {
    pub async fn new() -> Result<Self, zbus::Error> {
        let connection = zbus::Connection::system().await?;
        let proxy = PpdProxy::new(&connection).await?;
        Ok(Self { proxy })
    }
    pub async fn get_current_profile(&self) -> Option<String> {
        match self.proxy.active_profile().await {
            Ok(v) => {
                log::info!("active power profile: {}", v);
                Some(v)
            }
            Err(err) => {
                log::error!("failed to get active power policy: {:?}", err);
                None
            }
        }
    }
    pub async fn get_avalible_profiles(&self) -> Option<Vec<String>> {
        match self.proxy.profiles().await {
            Ok(v) => Some(v.into_iter().map(|v| v.profile).collect()),
            Err(err) => {
                log::error!("failed to get avalible power profiles: {:?}", err);
                None
            }
        }
    }
    pub async fn set_policy(&self, value: String) {
        log::info!("setting power profile: {}", value);
        if let Err(err) = self.proxy.set_active_profile(value).await {
            log::error!("failed to set power profile: {:?}", err);
        }
    }
}
