use std::fs;

use oyasumi_shared::XR_BINDING_FILE_PATH;
use xr_overlay::runner::builders::{ControllersCreateInfo, parse_config};

use crate::{
    ARGS,
    vr::{DEFAULT_BINDINGS_CONFIG},
};

pub fn get_controller_create_info() -> ControllersCreateInfo {
    let bindings = ControllersCreateInfo::default();
    let bindings_ = bindings.clone();
    match bindings.load_config(parse_config(match ARGS.get().as_ref().unwrap().core_pid == 0 {
        true => DEFAULT_BINDINGS_CONFIG.to_string(),
        false => fs::read_to_string(XR_BINDING_FILE_PATH.as_path()).unwrap(),
    },Some(|k|k=="main"||k=="overlay")).unwrap()) {
        Ok(v) => v,
        Err(err) => {
            log::error!("failed to parse binding config with:{:?}", err);
            bindings_
        }
    }
}
