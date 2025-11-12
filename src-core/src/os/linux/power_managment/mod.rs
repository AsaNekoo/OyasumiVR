use std::sync::LazyLock;

use strum_macros::Display;
use tokio::sync::Mutex;

use crate::os::{
    linux::power_managment::power_profile_daemon::PowerProfileDaemon, models::WindowsPowerPolicy,
};
pub static LINUX_POWER_POLICY_MANAGER: LazyLock<Mutex<LinuxPowerPolicyManager>> =
    LazyLock::new(|| Mutex::const_new(LinuxPowerPolicyManager::new()));
mod power_profile_daemon;
enum PowerPolicyProviderEnum {
    PowerProfileDaemon,
    ///to be used only by conflicts_with
    #[allow(dead_code)]
    All,
}
///since there is a many power managment methods on linux (https://wiki.archlinux.org/title/Power_management) and the is no "standard"
#[derive(Display, Debug, PartialEq, Clone, Copy)]
pub enum PowerProfile {
    UltraPowerSave,
    PowerSave,
    Balanced,
    Pefromance,
    UltraPerformance,
}
impl PowerProfile {
    pub const fn all() -> &'static [PowerProfile] {
        &[
            PowerProfile::UltraPowerSave,
            PowerProfile::PowerSave,
            PowerProfile::Balanced,
            PowerProfile::Pefromance,
            PowerProfile::UltraPerformance,
        ]
    }
}
impl From<PowerProfile> for WindowsPowerPolicy {
    fn from(value: PowerProfile) -> Self {
        WindowsPowerPolicy {
            guid: value.to_string().to_uppercase(),
            name: value.to_string(),
        }
    }
}
impl From<String> for PowerProfile {
    fn from(value: String) -> Self {
        match value.to_uppercase() {
            val if val == PowerProfile::UltraPowerSave.to_string().to_uppercase() => PowerProfile::UltraPowerSave,
            val if val == PowerProfile::PowerSave.to_string().to_uppercase() => PowerProfile::PowerSave,
            val if val == PowerProfile::Balanced.to_string().to_uppercase() => PowerProfile::Balanced,
            val if val == PowerProfile::Pefromance.to_string().to_uppercase() => PowerProfile::Pefromance,
            val if val == PowerProfile::UltraPerformance.to_string().to_uppercase() => {
                PowerProfile::UltraPerformance
            }
            _ => panic!("bad policy name:{}", value),
        }
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
    ///returns conflict with other PowerPolicyProviders
    fn conflicts_with(&self) -> Vec<PowerPolicyProviderEnum>;
    fn set_power_profile(&mut self, profile: PowerProfile) -> Result<(), ()>;
}
//todo: multiple providers
pub struct LinuxPowerPolicyManager {
    state: PowerProfile,
    providers: Vec<Box<dyn PowerPolicyProvider>>,
}
impl LinuxPowerPolicyManager {
    pub fn new() -> Self {
        let mut providers = Vec::new();
        let power_profile_daemon = PowerProfileDaemon {};
        if power_profile_daemon.is_avalible() {
            let power_profile_daemon: Box<dyn PowerPolicyProvider> = Box::new(power_profile_daemon);
            providers.push(power_profile_daemon);
        }
        Self {
            state: PowerProfile::Balanced,
            providers,
        }
    }
    pub fn get_current(&self) -> PowerProfile {
        self.state
    }
    pub fn set_policy<T: Into<PowerProfile> + Copy>(&mut self, value: T) {
        self.state=value.into();
        for provider in &mut self.providers {
            provider.set_power_profile(value.into()).ok();
        }
    }
}
