use xr_overlay::runner::OverlayHandle;
use xr_overlay_cef::cef::Browser;
pub struct Overlay{
    pub browser:Browser,
    pub xr_handle:OverlayHandle,
}