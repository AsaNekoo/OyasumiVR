use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use tonic::Request;
use xr_overlay::{
    input::{InputHandler, InputHandlerCreateInfo, InputSettings},
    openxr::Vector3f,
    runner::{
        AppRunner, AppRunnerCreateInfo, AppRunnerCreateInfoInput, ShowMode, events::AppEvent,
    },
};
use xr_overlay_cef::{CefOverlayCreateInfo, cef::{ImplBrowser, ImplFrame}, create_cef_overlay, pointless_cef_thread_spawner};

use crate::{
    core_grpc::{Empty, oyasumi_core_client::OyasumiCoreClient},
    input::get_controller_create_info,
    overlay_grpc::oyasumi_overlay_sidecar_client::OyasumiOverlaySidecarClient,
};
pub mod globals;
pub mod input;
pub mod core_grpc {
    tonic::include_proto!("oyasumi_core");
}
pub mod overlay_grpc {
    tonic::include_proto!("oyasumi_overlay_sidecar");
}

fn main() {
    pointless_cef_thread_spawner();
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
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
    let framerate=app.read().unwrap().current_refresh_rate() as u32;
    let overlay = create_cef_overlay(
        app.clone(),
        CefOverlayCreateInfo {
            size: [0.6, 0.6],
            visibility_switch_actions: None,
            spawn_visible: true,
            interactable: true,
            movable: true,
            allow_visibility_switch: false,
            pos: Vector3f {
                x: -0.0,
                y: -0.0,
                z: -1.2,
            },
            framerate,
            resolution:[1024,1024],
            // show_mode: ShowMode::CamerPos((
            //     Vector3f {
            //         x: -0.1,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     true,
            // )),
            ..Default::default()
        },
    );
    overlay.browser.main_frame().unwrap().load_url(Some(&"http://localhost:5173/dashboard".into()));
    let runtime_thread = std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(tokio_main());
    });
    let overlay_thread = std::thread::spawn(move || {
        let frame_time=(1000./app.write().unwrap().current_refresh_rate()) as u64;
        loop {
            match app.write().unwrap().run() {
                xr_overlay::runner::PollResult::Success =>   std::thread::sleep(Duration::from_millis(frame_time)),
                xr_overlay::runner::PollResult::SuccessNoRender => std::thread::sleep(Duration::from_millis(frame_time*3)),
                xr_overlay::runner::PollResult::UserNotPresent => std::thread::sleep(Duration::from_secs(1)),
                xr_overlay::runner::PollResult::Starting => (),
                xr_overlay::runner::PollResult::Exit => todo!(),
            }
        }
    });
    runtime_thread.join().unwrap();
    overlay_thread.join().unwrap();
}
fn openxr_callback(event: AppEvent) {}
async fn tokio_main() {
    let mut core_client =
        OyasumiCoreClient::connect(format!("http://127.0.0.1:{}", globals::CORE_GRPC_DEV_PORT))
            .await
            .unwrap(); 
    let mut overlya_client = OyasumiOverlaySidecarClient::connect(format!(
        "http://127.0.0.1:{}",
        globals::CORE_GRPC_DEV_PORT
    ))
    .await
    .unwrap();
    let port = core_client.get_http_server_port(Empty {}).await.unwrap();
    println!("Hello, world!:{:?}", port);
}
