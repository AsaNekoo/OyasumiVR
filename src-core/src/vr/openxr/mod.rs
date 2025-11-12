use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use log::{debug, info};
use tokio::{sync::Mutex, task::spawn_blocking};
use xr_overlay::{
    openxr::Vector3f,
    runner::{events::AppEvent, AppRunner, AppRunnerCreateInfo, OverlayCreateInfo, OverlayHandle},
    xr::ReferenceSpaceT,
    RgbaTexture,
};

use crate::vr::model::VRStatus;
pub static OXR_HANDLE: OnceLock<Mutex<AppRunner>> = OnceLock::new();
pub static OXR_BRIGHTNES_OVERLAY_HANDLE: OnceLock<Mutex<OverlayHandle>> = OnceLock::new();
pub static OXR_STATE: Mutex<VRStatus> = Mutex::const_new(VRStatus::Inactive);
pub async fn init() {
    let _ = tokio::task::spawn(async {
        info!("[Init] initializing openxr");
        *OXR_STATE.lock().await = VRStatus::Initializing;
        let ctx = xr_overlay::xr::Init::default()
            .enable_drm_support()
            .sort_order(u16::MAX as u32)
            .user_presence_support(false)
            .with_app_name("Oyasumi VR")
            .init_overlay(Duration::MAX.into())
            .unwrap();
        info!("[Init] connected to openxr");

        let mut runner = xr_overlay::runner::AppRunner::new(AppRunnerCreateInfo {
            ctx,
            space_type: ReferenceSpaceT::VIEW,
            callback: openxr_callback,
            input: None,
        });
        let brightness_overlay_handle = runner.add_overlay(OverlayCreateInfo {
            type_: xr_overlay::runner::OverlayCreateInfoType::Unmanaged {
                space: None,
                size: [1., 1.],
            },
            spawn_visible: true,
            interactable: false,
            movable: false,
            allow_visibility_switch: false,
            //make sure it's in front of the user
            pos: Vector3f {
                x: 0.0,
                y: 0.0,
                z: -0.1,
            },
            ..Default::default()
        });

        info!("[Init] openxr start");
        OXR_BRIGHTNES_OVERLAY_HANDLE
            .set(Mutex::new(brightness_overlay_handle))
            .unwrap();
        OXR_HANDLE.set(Mutex::new(runner)).unwrap();
        tokio::task::spawn(async {
            loop {
                match OXR_HANDLE.get().unwrap().lock().await.run() {
                    xr_overlay::runner::PollResult::Success => continue,
                    xr_overlay::runner::PollResult::Sleep(duration) => {
                        tokio::time::sleep(duration).await
                    }
                    xr_overlay::runner::PollResult::Exit => break,
                }
            }
        });
        debug!("[Init] openxr start (2)");
    });
}
fn openxr_callback(event: AppEvent) {
    spawn_blocking(move || {
        match event {
            AppEvent::SessionEnded | AppEvent::Killed => {
                log::debug!("[core] openxr disconnected");
                *OXR_STATE.blocking_lock() = VRStatus::Inactive;
            }
            AppEvent::Started => {
                log::debug!("[core] openxr ready");
                *OXR_STATE.blocking_lock() = VRStatus::Initialized;
            }
            _ => (),
        }
    });
}
pub async fn set_brightness(brightness: f64, perceived_brightness_adjustment_gamma: Option<f64>) {
    if OXR_HANDLE.get().is_none() {
        return;
    }
    let mut brightness = brightness.clamp(0.0, 1.0);
    // Adjust the brightness value for perceived brightness
    if let Some(gamma) = perceived_brightness_adjustment_gamma {
        brightness = adjust_for_perceived_brightness(brightness, gamma);
    }
    let brightness = ((1. - brightness) * 255.) as u8;
    let mut ctx = OXR_HANDLE.wait().lock().await;
    let overlay_handle = OXR_BRIGHTNES_OVERLAY_HANDLE.wait().lock().await;

    ctx.set_raw_texture(
        *overlay_handle,
        // RgbaTexture::new(1, 1, [brightness, 0, 0, 255].to_vec()),
        RgbaTexture::new(1, 1, [0, 0, 0, brightness].to_vec()),
    );
}

fn adjust_for_perceived_brightness(linear_percent: f64, gamma: f64) -> f64 {
    linear_percent.powf(1.0 / gamma)
}
