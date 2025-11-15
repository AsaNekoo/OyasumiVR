use std::{
    net::SocketAddr,
    sync::{Arc, LazyLock, Mutex, OnceLock, RwLock},
    thread::JoinHandle,
    time::Duration,
};

use argh::FromArgs;
use log::{error, info};
use tonic::{Request, transport::Server};
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
    create_cef_overlay, pointless_cef_thread_spawner,
};

use crate::{
    core_grpc::{Empty, OverlaySidecarStartArgs, oyasumi_core_client::OyasumiCoreClient},
    globals::{CORE_GRPC_DEV_PORT, CORE_MODE, CoreMode, OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT},
    grpc::GrpcServer,
    input::get_controller_create_info,
    overlay_grpc::{
        oyasumi_overlay_sidecar_client::OyasumiOverlaySidecarClient,
        oyasumi_overlay_sidecar_server::OyasumiOverlaySidecarServer,
    },
};
pub mod globals;
pub mod grpc;
pub mod input;
pub mod core_grpc {
    tonic::include_proto!("oyasumi_core");
}
pub mod overlay_grpc {
    tonic::include_proto!("oyasumi_overlay_sidecar");
}
#[derive(FromArgs, Debug)]
/// s
pub struct StartArgs {
    /// dev option
    #[argh(switch)]
    no_vr: bool,
}
static CMD_ARGS: OnceLock<StartArgs> = OnceLock::new();
static OVERLAY_BROWSER: OnceLock<Browser> = OnceLock::new();
static mut KILL_VR: bool = false;
fn kill_vr() {
    unsafe { KILL_VR = true };
}
fn vr() -> JoinHandle<()> {
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
fn main() {
    pointless_cef_thread_spawner();
    CMD_ARGS.set(argh::from_env()).unwrap();
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    let vr_thread = if !CMD_ARGS.get().unwrap().no_vr {
        Some(vr())
    } else {
        None
    };
    let runtime_thread = std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(tokio_main());
      
    });

    runtime_thread.join().unwrap();
    if let Some(vr_thread) = vr_thread {
        vr_thread.join().unwrap();
    }
}
static HANDLES: LazyLock<Mutex<Vec<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| Mutex::default());
fn openxr_callback(event: AppEvent) {}
pub async fn init_server() -> u16 {
    let port: u16 = match CORE_MODE {
        CoreMode::Dev => crate::globals::OVERLAY_SIDECAR_GRPC_DEV_PORT,
        CoreMode::Release => 0,
    };
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    info!("Starting gRPC server on {}", addr);

    let server = Server::builder()
        .add_service(OyasumiOverlaySidecarServer::new(GrpcServer::default()))
        .serve(addr);
    HANDLES.lock().unwrap().push(tokio::task::spawn(async move {
        {
            if let Err(err) = server.await {
                error!("Failed to start gRPC server: {}", err);
            }
        }
    }));
    addr.port()
}
async fn tokio_main() {
    let mut core_client =
        OyasumiCoreClient::connect(format!("http://127.0.0.1:{}", globals::CORE_GRPC_DEV_PORT))
            .await
            .unwrap();
    let http_port = core_client.get_http_server_port(Empty {}).await.unwrap();
    info!("got http port:{:?}", http_port);
    let grpc_server_port = init_server().await;
    info!("server running on 127.0.0.1:{}", grpc_server_port);
    core_client.on_overlay_sidecar_start(OverlaySidecarStartArgs{ pid: std::process::id(), grpc_port: grpc_server_port as u32, grpc_web_port: OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT as u32 }).await.unwrap();
    tokio::time::sleep(Duration::from_secs(100000)).await;
    // let res=overlya_client.sync_state(request)
    
}
