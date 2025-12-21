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
impl From<String> for PowerPolicyProviderEnum{
    fn from(value: String) -> Self {
        match value.as_str(){
            "powerprofilesctl"=>Self::PowerProfileDaemon,
            "tlpctl"=>Self::TLPCTL,
            _=>panic!("unknown power policy provider: {}",value)
        }
    }
}
impl From<PowerPolicyProviderEnum> for String{
    fn from(value: PowerPolicyProviderEnum) -> Self {
        match value{
            PowerPolicyProviderEnum::PowerProfileDaemon => "powerprofilesctl",
            PowerPolicyProviderEnum::TLPCTL => "tlpctl",
        }.to_string()
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
    fn get_avalible_profiles(&self)->Vec<String>;
    //return as upper case
    fn get_current_profile(&self)->String;
}
//todo: multiple providers
pub struct LinuxPowerPolicyManager {
    provider: Box<dyn PowerPolicyProvider>,
}
impl LinuxPowerPolicyManager {
    pub fn new() -> Self {
        Self {
            provider: Box::new(PowerProfileDaemon),
        }
    }
    pub fn get_current_profile(&self) -> String {
        self.provider.get_current_profile()
    }
    pub fn get_avalible_profiles(&self)->Vec<String>{
        self.provider.get_avalible_profiles()
    }
    pub fn get_providers(&self)->Vec<String>{
        let mut providers=Vec::new();
        if PowerProfileDaemon.is_avalible(){
            providers.push(PowerPolicyProviderEnum::PowerProfileDaemon.into());
        }
        if TLPCTL.is_avalible(){
            providers.push(PowerPolicyProviderEnum::TLPCTL.into());
        }
        providers
    }
    pub fn set_provider(&mut self,name:String){
        match PowerPolicyProviderEnum::from(name){
            PowerPolicyProviderEnum::PowerProfileDaemon => {self.provider=Box::new(PowerProfileDaemon)},
            PowerPolicyProviderEnum::TLPCTL => {self.provider=Box::new(TLPCTL)},
        }

    }
    pub fn set_policy(&mut self, value: String) {
        if self.provider.set_power_profile(value.clone()).is_err() {
            log::error!(
                "failed to set power policy: {} using: {:?}",
                value,
                self.provider.which()
            )
        }
    }
}
