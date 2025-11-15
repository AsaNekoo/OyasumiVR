use tokio::sync::Mutex;

use crate::overlay_grpc::{OyasumiSidecarAutomationsState, OyasumiSidecarState};

pub const CORE_GRPC_DEV_PORT: u16 = 5176;
pub const CORE_HTTP_DEV_PORT: u16 = 5177;
pub const OVERLAY_SIDECAR_GRPC_DEV_PORT: u16 = 5174;
pub const OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT: u16 = 5175;



pub static CORE_MODE:CoreMode=CoreMode::Dev;

#[derive(Clone, Copy,Debug,PartialEq, PartialOrd)]
pub enum CoreMode{
    Dev,
    Release
}
pub static STATE:Mutex<Option<OyasumiSidecarState>>=Mutex::const_new(None);
