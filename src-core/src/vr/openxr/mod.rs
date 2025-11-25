use std::{
    sync::{LazyLock, OnceLock},
    time::Duration,
};

use log::{debug, info};
use tokio::{spawn, sync::Mutex, task::spawn_blocking};
use xr_overlay::{
    model::AppContext,
    openxr::Vector3f,
    runner::{events::AppEvent, AppRunner, AppRunnerCreateInfo, OverlayCreateInfo, OverlayHandle},
    xr::ReferenceSpaceT,
    RgbaTexture,
};

use crate::{
    utils::send_event,
    vr::{
        gesture_detector::GestureDetector, model::VRStatus, sleep_detector::SleepDetector,
    },
};
pub static OXR_HANDLE: OnceLock<Mutex<AppRunner>> = OnceLock::new();
pub static OXR_BRIGHTNES_OVERLAY_HANDLE: Mutex<Option<OverlayHandle>> = Mutex::const_new(None);
pub static OXR_STATE: Mutex<VRStatus> = Mutex::const_new(VRStatus::Inactive);
async fn get_ctx() -> AppContext<xr_overlay::openxr::Vulkan> {
    let ctx = loop {
            let ctx = xr_overlay::xr::Init::default()
                .disable_hand_tracking()
                .sort_order(u16::MAX as u32)
                .user_presence_support(false)
                .with_app_name("Oyasumi VR");
            // if let Some(ref app)=app{
            //     ctx=ctx.with_instance(&unsafe { app.get_ctx() }.xr.instance);
            // }
            let ctx = ctx.init_overlay();
            if let Err(xr_overlay::error::Error::InitNotReady) = ctx {
                drop(ctx);
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            } else {
                break ctx.unwrap();
            }
        };
    update_status(VRStatus::Initializing).await;
    ctx
}
fn get_overlay_info() -> OverlayCreateInfo {
    OverlayCreateInfo {
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
    }
}
pub async fn init() {
    let _ = tokio::task::spawn(async {
        let ctx = get_ctx().await;

        info!("[Init] connected to openxr");

        let mut runner = xr_overlay::runner::AppRunner::new(AppRunnerCreateInfo {
            ctx,
            space_type: ReferenceSpaceT::VIEW,
            callback: openxr_callback,
            input: None,
        });
        let brightness_overlay_handle = runner.add_overlay(get_overlay_info());

        OXR_BRIGHTNES_OVERLAY_HANDLE
            .lock()
            .await
            .replace(brightness_overlay_handle);
        OXR_HANDLE.set(Mutex::new(runner)).unwrap();
        tokio::task::spawn(async {
            loop {
                if *OXR_STATE.lock().await == VRStatus::Initialized {
                    let mut xr_ctx = OXR_HANDLE.get().unwrap().lock().await;
                    match xr_ctx.run() {
                        xr_overlay::runner::PollResult::Success => (),
                        xr_overlay::runner::PollResult::UserNotPresent => {
                            drop(xr_ctx);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                        xr_overlay::runner::PollResult::Exit => {
                            drop(xr_ctx);
                            tokio::time::sleep(Duration::from_secs(10)).await;
                            debug_assert_eq!(*OXR_STATE.lock().await, VRStatus::Inactive);
                            continue;
                        }
                        xr_overlay::runner::PollResult::Starting => {
                            drop(xr_ctx);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                        xr_overlay::runner::PollResult::SuccessNoRender => {
                            drop(xr_ctx);
                            tokio::time::sleep(Duration::from_secs(60)).await
                        }
                        xr_overlay::runner::PollResult::SessionLost => unreachable!(),
                    }
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });

        tokio::task::spawn(async move {
            loop {
                if *OXR_STATE.lock().await == VRStatus::Initialized {
                    let ctx: &mut AppRunner = &mut *OXR_HANDLE.get().unwrap().lock().await;
                    if let Some(posef) = ctx.get_hmd_posef(ReferenceSpaceT::STAGE) {
                        let pos = posef.position;
                        let quat = posef.orientation;
                        SLEEP_DETECTOR
                            .lock()
                            .await
                            .log_pose(
                                [pos.x, pos.y, pos.z],
                                [quat.x as f64, quat.y as f64, quat.z as f64, quat.w as f64],
                            )
                            .await;
                    } else {
                        info!("[Core] Failed to get hmd Posef")
                    }
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
        debug!("[Init] openxr start (2)");
        update_status(VRStatus::Initialized).await;
    });
}
async fn session_restart() {
    let mut handle = OXR_HANDLE.get().as_ref().unwrap().lock().await;
    let ctx = get_ctx().await;
    unsafe { handle.replace_ctx(xr_overlay::runner::SessionRestartInfo::NoInput { ctx }) };
    let overlay_handle = handle.add_overlay(get_overlay_info());
    OXR_BRIGHTNES_OVERLAY_HANDLE.lock().await.replace(overlay_handle);
    let _ =handle.run();
    update_status(VRStatus::Initialized).await;
}
fn openxr_callback(event: AppEvent) {
    spawn_blocking(move || match event {
        AppEvent::SessionEnded | AppEvent::Killed => {
            log::debug!("[core] openxr disconnected");
            spawn(async {
                update_status(VRStatus::Inactive).await;
                unsafe { OXR_HANDLE.get().as_ref().unwrap().lock().await.drop_ctx() };
                OXR_BRIGHTNES_OVERLAY_HANDLE.lock().await.take();
                tokio::task::spawn(session_restart());
            });
        }
        AppEvent::Started => {
            log::debug!("[core] openxr ready");
            spawn(   update_status(VRStatus::Initialized));
        }
        _ => (),
    });
}
async fn update_status(new_status: VRStatus) {
    info!("[core] updating openxr status:{:?}",new_status);
    if *OXR_STATE.lock().await==VRStatus::Initialized && new_status==VRStatus::Initializing{
        unreachable!("possible race condition for update_status"); //panic instead of error since this is a logic error and need to be fixed
    }
    *OXR_STATE.lock().await = new_status.clone();
    send_event("VR_STATUS_UPDATE", new_status.to_string().to_uppercase()).await;
}
static ABORT_GESTURE_DETECTION: Mutex<bool> = Mutex::const_new(false);
static GESTURE_DETECTION_RUNNING: Mutex<bool> = Mutex::const_new(false);
pub async fn stop_head_shake_detection() {
    if *GESTURE_DETECTION_RUNNING.lock().await {
        *ABORT_GESTURE_DETECTION.lock().await = true;
    }
}
pub async fn start_head_shake_detection() {
    if *OXR_STATE.lock().await==VRStatus::Initialized{
    let frame_time = (1000.
        / OXR_HANDLE
            .get()
            .unwrap()
            .lock()
            .await
            .current_refresh_rate()) as u64;
    tokio::task::spawn(async move {
        *GESTURE_DETECTION_RUNNING.lock().await = true;
        *ABORT_GESTURE_DETECTION.lock().await = false;
        loop {
            if *ABORT_GESTURE_DETECTION.lock().await {
                break;
            }
            if *OXR_STATE.lock().await == VRStatus::Initialized {
                let ctx: &mut AppRunner = &mut *OXR_HANDLE.get().unwrap().lock().await;
                if let Some(posef) = ctx.get_hmd_posef(ReferenceSpaceT::STAGE) {
                    let pos = posef.position;
                    let quat = posef.orientation;
                    GESTURE_DETECTOR
                        .lock()
                        .await
                        .log_pose(
                            [pos.x, pos.y, pos.z],
                            [quat.x as f64, quat.y as f64, quat.z as f64, quat.w as f64],
                        )
                        .await;
                } else {
                    info!("[Core] Failed to get hmd Posef")
                }
            }
            tokio::time::sleep(Duration::from_millis(frame_time)).await;
        }
        *GESTURE_DETECTION_RUNNING.lock().await = false;
    });
}
}

pub async fn set_brightness(brightness: f64, perceived_brightness_adjustment_gamma: Option<f64>) {
    if *OXR_STATE.lock().await != VRStatus::Initialized {
        return;
    }
    let mut brightness = brightness.clamp(0.0, 1.0);
    // Adjust the brightness value for perceived brightness
    if let Some(gamma) = perceived_brightness_adjustment_gamma {
        brightness = adjust_for_perceived_brightness(brightness, gamma);
    }
    let brightness = ((1. - brightness) * 255.) as u8;

    let mut ctx = OXR_HANDLE.wait().lock().await;
    let overlay_handle = *OXR_BRIGHTNES_OVERLAY_HANDLE.lock().await;
    if overlay_handle.is_none() {
        return;
    }

    ctx.set_raw_texture(
        overlay_handle.unwrap(),
        // RgbaTexture::new(1, 1, [brightness, 0, 0, 255].to_vec()),
        RgbaTexture::new(1, 1, [0, 0, 0, brightness].to_vec()),
    );
    let _ = ctx.run();
}

fn adjust_for_perceived_brightness(linear_percent: f64, gamma: f64) -> f64 {
    linear_percent.powf(1.0 / gamma)
}
static SLEEP_DETECTOR: LazyLock<Mutex<SleepDetector>> =
    LazyLock::new(|| Mutex::new(SleepDetector::new()));
static GESTURE_DETECTOR: LazyLock<Mutex<GestureDetector>> =
    LazyLock::new(|| Mutex::new(GestureDetector::new()));
