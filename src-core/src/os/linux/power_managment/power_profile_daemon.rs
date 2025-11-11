use std::process::Command;

use crate::os::linux::power_managment::{PowerPolicyProvider, PowerPolicyProviderEnum};
pub struct PowerProfileDaemon;
impl PowerPolicyProvider for PowerProfileDaemon {
    fn is_avalible(&self) -> bool {
        Command::new("powerprofilesctl").spawn().is_ok()
    }

    fn conflicts_with(&self) -> Vec<super::PowerPolicyProviderEnum> {
        Vec::new()
    }

    fn set_power_profile(&mut self,profile: super::PowerProfile) -> Result<(), ()> {
        Command::new("powerprofilesctl")
            .arg("set")
            .arg(match profile {
                super::PowerProfile::UltraPowerSave => "power-saver",
                super::PowerProfile::PowerSave => "power-saver",
                super::PowerProfile::Balanced => "balanced",
                super::PowerProfile::Pefromance => "performance",
                super::PowerProfile::UltraPerformance => "performance",
            })
            .spawn()
            .map_err(|_| ())?;
        Ok(())
    }

    fn which(&self)->PowerPolicyProviderEnum {
        PowerPolicyProviderEnum::PowerProfileDaemon
    }
}
