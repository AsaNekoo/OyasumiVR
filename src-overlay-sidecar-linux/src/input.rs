use xr_overlay::runner::builders::{ControllersCreateInfo, ProfileControllersCreateInfo};

use crate::vr::BINDING_FILE_PATH;

pub fn get_controller_create_info() -> ControllersCreateInfo {
    let bindings = ControllersCreateInfo {
        profiles: [ProfileControllersCreateInfo::default_oculus_touch()].into(),
    };
    let bindings_ = bindings.clone();
    match bindings.parse_config(BINDING_FILE_PATH.to_path_buf()) {
        Ok(v) => v,
        Err(err) => {
            log::error!("failed to parse binding config with:{:?}", err);
            bindings_
        }
    }
}
