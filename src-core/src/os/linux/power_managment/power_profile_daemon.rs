use std::process::Command;

use crate::os::linux::power_managment::{PowerPolicyProvider, PowerPolicyProviderEnum};
pub struct PowerProfileDaemon;
const PROFILES: [&str;3] = ["power-saver", "balanced", "performance"];
impl PowerPolicyProvider for PowerProfileDaemon {
    fn is_avalible(&self) -> bool {
        Command::new("powerprofilesctl").spawn().is_ok()
    }
    
    fn set_power_profile(&mut self, mut profile: String) -> Result<(), ()> {
        profile=profile.to_lowercase();
        if !PROFILES.contains(&profile.as_str()) {
            log::error!(
                "incorrect power profile: {} correct values are:{:?}",
                profile,
                PROFILES
            );
            return Err(());
        }

        Command::new("powerprofilesctl")
            .arg("set")
            .arg(profile)
            .spawn()
            .map_err(|_| ())?;
        Ok(())
    }

    fn which(&self) -> PowerPolicyProviderEnum {
        PowerPolicyProviderEnum::PowerProfileDaemon
    }
    
    fn get_avalible_profiles(&self)->Option<Vec<String>> {
        Some(PROFILES.iter().map(|x|x.to_string()).collect())
    }
    
    fn get_current_profile(&self)->Option<String> {
        let out=Command::new("powerprofilesctl").arg("get").output().ok()?.stdout;
        Some(String::from_utf8_lossy(&out).to_uppercase().trim().to_string())
    }
}
