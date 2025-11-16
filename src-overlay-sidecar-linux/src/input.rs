use xr_overlay::runner::builders::{ControllersCreateInfo, ProfileControllersCreateInfo};

pub fn get_controller_create_info() -> ControllersCreateInfo {
    ControllersCreateInfo {
        profiles: [ProfileControllersCreateInfo::default_oculus_touch()].into(),
    }
}
