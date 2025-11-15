use std::{
    sync::{Arc, OnceLock, RwLock},
    thread::JoinHandle,
    time::Duration,
};

use xr_overlay::{
    input::{InputHandler, InputHandlerCreateInfo, InputSettings},
    openxr::Vector3f,
    runner::{
        AppRunner, AppRunnerCreateInfo, AppRunnerCreateInfoInput, ShowMode, events::AppEvent,
    },
};
use xr_overlay_cef::{
    CefOverlayCreateInfo,
    cef::{Browser, ImplBrowser, ImplFrame},
    create_cef_overlay,
};

use crate::input::get_controller_create_info;
static mut KILL_VR: bool = false;
pub static OVERLAY_BROWSER: OnceLock<Browser> = OnceLock::new();
#[allow(dead_code)]
pub fn kill_vr() {
    unsafe { KILL_VR = true };
}
pub fn start_vr() -> JoinHandle<()> {
    let ctx = loop {
        match xr_overlay::xr::Init::default()
            .enable_drm_support()
            .user_presence_support(false)
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
    let input_handler = InputHandler::new(InputHandlerCreateInfo {
        ctx: ctx.clone(),
        setting: InputSettings {
            oculus_touch: true,
            valve_index: true,
            hand_tracking: true,
            palm_pose: false,
        },
        actionset_name: None,
        tracked_actions: None,
    })
    .unwrap();
    let app = AppRunner::new(AppRunnerCreateInfo {
        ctx,
        space_type: xr_overlay::xr::ReferenceSpaceT::VIEW,
        callback: openxr_callback,
        input: Some(AppRunnerCreateInfoInput {
            input_handler,
            l_pointer_color: [1., 1., 1., 0.2],
            r_pointer_color: [1., 1., 1., 0.],
            controllers: get_controller_create_info(),
            draw_pointers_only_when_hit: true,
            overlay_move_speed: 0.01,
        }),
    });
    let app = Arc::new(RwLock::new(app));
    let pos = Vector3f {
        x: -0.0,
        y: -0.0,
        z: -1.2,
    };
    let framerate = app.read().unwrap().current_refresh_rate() as u32;
    let overlay = create_cef_overlay(
        app.clone(),
        CefOverlayCreateInfo {
            size: [0.6, 0.6],
            visibility_switch_actions: None,
            spawn_visible: true,
            interactable: true,
            movable: true,
            allow_visibility_switch: false,
            pos,
            framerate,
            resolution: [1024, 1024],
            show_mode: ShowMode::CamerPos((pos, true)),
            ..Default::default()
        },
    );
    overlay
        .browser
        .main_frame()
        .unwrap()
        .load_url(Some(&"http://localhost:5173/dashboard".into()));
    assert!(OVERLAY_BROWSER.set(overlay.browser.clone()).is_ok());
    let overlay_thread = std::thread::spawn(move || {
        let frame_time = (1000. / app.write().unwrap().current_refresh_rate()) as u64;
        loop {
            if unsafe { KILL_VR } {
                app.write().unwrap().request_end_session();
            }
            match app.write().unwrap().run() {
                xr_overlay::runner::PollResult::Success => {
                    std::thread::sleep(Duration::from_millis(frame_time))
                }
                xr_overlay::runner::PollResult::SuccessNoRender => {
                    std::thread::sleep(Duration::from_millis(frame_time * 3))
                }
                xr_overlay::runner::PollResult::UserNotPresent => {
                    std::thread::sleep(Duration::from_secs(1))
                }
                xr_overlay::runner::PollResult::Starting => (),
                xr_overlay::runner::PollResult::Exit => {
                    //no session resuming bc google's trash doesn't support restarting after calling shutdown
                    unsafe { xr_overlay_cef::shutdown() };
                    break;
                }
            }
        }
    });
    overlay_thread
}
fn openxr_callback(event: AppEvent) {}