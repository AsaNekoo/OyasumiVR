use std::fs;

use xr_overlay::runner::builders::ControllersCreateInfo;

use crate::{
    ARGS,
    vr::{BINDING_FILE_PATH, DEFAULT_BINDINGS_CONFIG},
};

pub fn get_controller_create_info() -> ControllersCreateInfo {
    let bindings = ControllersCreateInfo::default();
    let bindings_ = bindings.clone();
    match bindings.parse_config(match ARGS.get().as_ref().unwrap().core_pid == 0 {
        true => DEFAULT_BINDINGS_CONFIG.to_string(),
        false => fs::read_to_string(BINDING_FILE_PATH.as_path()).unwrap(),
    }) {
        Ok(v) => v,
        Err(err) => {
            log::error!("failed to parse binding config with:{:?}", err);
            bindings_
        }
    }
}
