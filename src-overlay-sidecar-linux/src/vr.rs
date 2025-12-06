use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, OnceLock, RwLock},
    thread::JoinHandle,
    time::Duration,
};

use log::trace;
use xr_overlay::{
    openxr::Vector3f,
    runner::{
        AppRunner, AppRunnerCreateInfo, AppRunnerCreateInfoInput, DeviceRole, ShowMode,
        events::AppEvent,
    },
};
use xr_overlay_cef::{CefOverlayCreateInfo, create_cef_overlay};
pub const DEFAULT_BINDINGS_CONFIG: &str = include_str!("bindings_overwrite_default.toml");
pub static BINDING_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("../../bindings_overwrite.toml"));
use crate::{KILL, input::get_controller_create_info, killed, model::Overlay};
pub static OVERLAY: OnceLock<Overlay> = OnceLock::new();
pub static XR_CTX: OnceLock<Arc<RwLock<AppRunner>>> = OnceLock::new();
pub fn start_vr() -> Option<JoinHandle<()>> {
    trace!("start_vr");
    let ctx = loop {
        if killed(){
            return None;
        }
        match xr_overlay::xr::Init::default()
            .enable_drm_support()
            .user_presence_support(true)
            .sort_order(4089)
            .with_app_name("Oyasumi VR Overlay")
            .init_overlay()
        {
            Ok(v) => break v,
            Err(_) => {
                std::thread::sleep(Duration::from_secs(1));
            }
        };
    };
    let app = AppRunner::new(AppRunnerCreateInfo {
        ctx,
        space_type: xr_overlay::xr::ReferenceSpaceT::STAGE,
        callback: openxr_callback,
        input: Some(AppRunnerCreateInfoInput {
            l_pointer_color: [1., 1., 1., 0.2],
            r_pointer_color: [1., 1., 1., 0.],
            controllers: get_controller_create_info(),
            draw_pointers_only_when_hit: true,
            overlay_move_speed: 0.01,
        }),
    });
    let app = Arc::new(RwLock::new(app));
    XR_CTX.set(app.clone()).unwrap();
    let pos = Vector3f {
        x: 0.1,
        y: -0.2,
        z: -0.8,
    };
    let framerate = app.read().unwrap().current_refresh_rate() as u32;
    let overlay = create_cef_overlay(
        app.clone(),
        CefOverlayCreateInfo {
            size: [0.6, 0.6],
            spawn_visible: true,
            interactable: true,
            movable: true,
            allow_visibility_switch: true,
            pos,
            framerate,
            resolution: [1024, 1024],
            show_mode: ShowMode::DeviceCallback {
                role_callback: openxr_show_hand,
                pos,
                rot: None,
            },
            name: Some("oyasumi".into()),
            disable_dragging:true,
            ..Default::default()
        },
    );
    let delay = Duration::from_millis(500).as_millis() as f32
        / (1000. / app.read().unwrap().current_refresh_rate() as f32);
    app.write()
        .unwrap()
        .set_delay_hide(overlay.overlay_handle, delay as u8);
    assert!(
        OVERLAY
            .set(Overlay {
                browser: overlay.browser.clone(),
                xr_handle: overlay.overlay_handle
            })
            .is_ok()
    );

    Some(std::thread::spawn(move || {
        let frame_time = (1000. / app.write().unwrap().current_refresh_rate() as f32) as u64;
        loop {
            if unsafe { KILL } {
                break;
            }
            match app.write().unwrap().run() {
                xr_overlay::runner::PollResult::Success => {
                    //it already waits for next frame
                    // std::thread::sleep(Duration::from_millis(frame_time))
                }
                xr_overlay::runner::PollResult::SuccessNoRender => {
                    std::thread::sleep(Duration::from_millis(frame_time))
                }
                xr_overlay::runner::PollResult::UserNotPresent => {
                    std::thread::sleep(Duration::from_secs(1))
                }
                xr_overlay::runner::PollResult::Starting => (),
                xr_overlay::runner::PollResult::Exit => {
                    //no session resuming bc google's trash doesn't support restarting after calling shutdown
                    // unsafe { xr_overlay_cef::shutdown() };
                    break;
                }
                xr_overlay::runner::PollResult::SessionLost => {
                    break;
                }
            }
        }
    }))
}
fn openxr_callback(event: AppEvent) {
    if event != AppEvent::ButtonsUpdated {
        trace!("[openxr] {:?}", event);
    }
    match event {
        AppEvent::OverlayVisibilityChanged { handle: _, visible } => {
            if visible {
                OVERLAY.get().as_ref().unwrap().show_dashboard();
            }
        }
        AppEvent::OverlayHiding {
            handle: _,
            frames_left: _,
        } => OVERLAY.get().as_ref().unwrap().hide_dashboard(),
        _ => (),
    }
}
fn openxr_show_hand() -> DeviceRole {
    DeviceRole::Hmd
}
pub static mut DASBOARD_VISIBLE: bool = false;
pub fn show_dashboard() {
    trace!("show_dashboard");
    unsafe { DASBOARD_VISIBLE = true };
    XR_CTX
        .wait()
        .write()
        .unwrap()
        .show(OVERLAY.wait().xr_handle, true);
    OVERLAY.wait().show_dashboard();
}
pub async fn hide_dashboard() {
    trace!("hide_dashboard");
    unsafe { DASBOARD_VISIBLE = false };
    OVERLAY.wait().hide_dashboard();
    tokio::time::sleep(Duration::from_millis(500)).await; //give animation some time
    XR_CTX
        .wait()
        .write()
        .unwrap()
        .show(OVERLAY.wait().xr_handle, false);
}
