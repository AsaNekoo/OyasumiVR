use std::sync::LazyLock;

use tokio::sync::Mutex;

use crate::os::linux::power_managment::{power_profile_daemon::PowerProfileDaemon, tplctl::TLPCTL};
pub static LINUX_POWER_POLICY_MANAGER: LazyLock<Mutex<LinuxPowerPolicyManager>> =
    LazyLock::new(|| Mutex::const_new(LinuxPowerPolicyManager::new()));
mod power_profile_daemon;
mod tplctl;
#[allow(dead_code)]
#[derive(Debug)]
enum PowerPolicyProviderEnum {
    PowerProfileDaemon,
    TLPCTL,
}
impl From<String> for PowerPolicyProviderEnum {
    fn from(value: String) -> Self {
        match value.as_str() {
            "powerprofilesctl" => Self::PowerProfileDaemon,
            "tlpctl" => Self::TLPCTL,
            _ => panic!("unknown power policy provider: {}", value),
        }
    }
}
impl From<PowerPolicyProviderEnum> for String {
    fn from(value: PowerPolicyProviderEnum) -> Self {
        match value {
            PowerPolicyProviderEnum::PowerProfileDaemon => "powerprofilesctl",
            PowerPolicyProviderEnum::TLPCTL => "tlpctl",
        }
        .to_string()
    }
}
unsafe impl Send for LinuxPowerPolicyManager {}
unsafe impl Sync for LinuxPowerPolicyManager {}
///specific implementations should use user editable configuration file if possible
#[allow(dead_code)]
#[allow(private_interfaces)]
pub trait PowerPolicyProvider {
    fn which(&self) -> PowerPolicyProviderEnum;
    fn is_avalible(&self) -> bool;
    //capitalization is upperscae
    fn set_power_profile(&mut self, profile: String) -> Result<(), ()>;
    fn get_avalible_profiles(&self) -> Option<Vec<String>>;
    //return as upper case
    fn get_current_profile(&self) -> Option<String>;
}
//todo: multiple providers
pub struct LinuxPowerPolicyManager {
    provider: Option<Box<dyn PowerPolicyProvider>>,
}
impl LinuxPowerPolicyManager {
    pub fn new() -> Self {
        let mut s = Self { provider: None };
        let mut providers = s.get_providers();
        if !providers.is_empty() {
            s.set_provider(providers.swap_remove(0));
        }
        s
    }
    pub fn get_current_profile(&self) -> String {
        if let Some(provider) = &self.provider {
            provider.get_current_profile().unwrap_or_else(|| {
                log::error!("failed to get active power policy");
                Default::default()
            })
        } else {
            log::error!("tried to get active power policy but provider is None");
            Default::default()
        }
    }
    pub fn get_avalible_profiles(&self) -> Vec<String> {
        if let Some(provider) = &self.provider {
            provider.get_avalible_profiles().unwrap_or_else(|| {
                log::error!("failed to get avalible power policies");
                Default::default()
            })
        } else {
            log::error!("tried to get avalible power policies but provider is None");
            Default::default()
        }
    }
    pub fn get_providers(&self) -> Vec<String> {
        let mut providers = Vec::new();
        if PowerProfileDaemon.is_avalible() {
            providers.push(PowerPolicyProviderEnum::PowerProfileDaemon.into());
        }
        if TLPCTL.is_avalible() {
            providers.push(PowerPolicyProviderEnum::TLPCTL.into());
        }
        providers
    }
    pub fn set_provider(&mut self, name: String) {
        match PowerPolicyProviderEnum::from(name) {
            PowerPolicyProviderEnum::PowerProfileDaemon => {
                self.provider = Some(Box::new(PowerProfileDaemon));
            }
            PowerPolicyProviderEnum::TLPCTL => {
                self.provider = Some(Box::new(TLPCTL));
            }
        }
    }
    pub fn set_policy(&mut self, value: String) {
        if let Some(provider) = &mut self.provider {
            if provider.set_power_profile(value.clone()).is_err() {
                log::error!(
                    "failed to set power policy: {} using: {:?}",
                    value,
                    provider.which()
                )
            }
        } else {
            log::error!("tried to set power policy but provider is None");
            Default::default()
        }
    }
}
