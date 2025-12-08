use std::{
    fs,
    path::PathBuf,
    sync::{LazyLock, Mutex, OnceLock},
    time::Duration,
};

use log::{info, trace};
use tonic::transport::Channel;
use xr_overlay_cef::{
    cef::{ImplBrowser, ImplFrame},
    disable_vr, pointless_cef_thread_spawner,
};

use crate::{
    core_grpc::{Empty, OverlaySidecarStartArgs, oyasumi_core_client::OyasumiCoreClient},
    grpc::{start_grpc_server, start_grpc_web_server},
    overlay_ipc::start_websocket_server,
    ui::serve_ui,
    vr::{BINDING_FILE_PATH, DEFAULT_BINDINGS_CONFIG, OVERLAY, show_dashboard, start_vr},
};
pub mod globals;
pub mod grpc;
pub mod input;
pub mod model;
pub mod overlay_ipc;
pub mod ui;
pub mod vr;
pub mod core_grpc {
    tonic::include_proto!("oyasumi_core");
}
pub mod overlay_grpc {
    tonic::include_proto!("oyasumi_overlay_sidecar");
}
#[macro_export]
macro_rules! warn_unimplemented {
    () => {
        log::error!("unimplemented: {}:{}:{}",file!(),line!(),column!())
    };
    ($($arg:tt)+) => {
        log::error!("unimplemented: {}:{}:{}\n{:?}",file!(),line!(),column!(),format_args!($($arg)+))
    };
}
static NO_VR: OnceLock<bool> = OnceLock::new();
static ARGS: OnceLock<Args> = OnceLock::new();
#[derive(Clone, Copy, Debug)]
pub struct Args {
    core_grpc_port: u16,
    core_pid: u64,
    disable_gpu: bool,
}
fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("xr_overlay_cef", log::LevelFilter::Debug)
        .filter_module("xr_overlay", log::LevelFilter::Debug)
        .filter_module("tokio_tungstenite", log::LevelFilter::Warn)
        .filter_module("tungstenite", log::LevelFilter::Warn)
        .init();
    trace!("args: {:?}", std::env::args());
    trace!(
        "thread_id:{:?},pid:{:?}",
        std::thread::current().id(),
        std::process::id()
    );
    fs::write("/proc/self/oom_score_adj", "1000").ok();
    pointless_cef_thread_spawner();
    trace!(
        "main thread_id:{:?},pid:{:?}",
        std::thread::current().id(),
        std::process::id()
    );
    log::trace!("args:{:?}", std::env::args());
    let args = std::env::args().collect::<Vec<_>>();
    if !(args.len() == 4 || args.len() == 3) {
        panic!("Usage: oyasumivr-overlay-sidecar <core grpc port> <core process id>")
    }

    let mut args = Args {
        core_grpc_port: args[1].parse().unwrap(),
        core_pid: args[2].parse().unwrap(),
        disable_gpu: args.get(3).cloned().unwrap_or_default() == "--disable-gpu-acceleration",
    };
    if args.disable_gpu {
        panic!("software rendering no in implemented");
    }
    if args.core_grpc_port == 0 && args.core_pid == 0 {
        args.core_grpc_port = globals::CORE_GRPC_DEV_PORT;
    }
    log::debug!("using arguments:{:?}", args);
    ARGS.set(args).unwrap();
    NO_VR
        .set(std::env::var("NO_VR").unwrap_or_default().to_lowercase() == "true")
        .unwrap();
    log::debug!("NO_VR:{:?}", NO_VR.get().as_ref().unwrap());
    if *NO_VR.get().unwrap() {
        disable_vr();
    }
    if ARGS.get().as_ref().unwrap().core_pid!=0{
    if !BINDING_FILE_PATH.is_file() {
        fs::write(&*BINDING_FILE_PATH, DEFAULT_BINDINGS_CONFIG).unwrap();
    }
}
    let vr_thread = match start_vr() {
        Some(v) => v,
        None => {
            log::trace!("early kill");
            return;
        }
    };
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(tokio_main());

    vr_thread.join().unwrap();
    trace!("exiting");
    unsafe { xr_overlay_cef::shutdown() };
    trace!("cef shutdown");
    runtime.shutdown_timeout(Duration::from_millis(100));
    trace!("runtime exited");
    std::thread::sleep(Duration::from_millis(10)); //just in case
}
static HANDLES: LazyLock<Mutex<Vec<tokio::task::JoinHandle<()>>>> = LazyLock::new(Mutex::default);

static CORE_CLIENT: OnceLock<tokio::sync::Mutex<OyasumiCoreClient<Channel>>> = OnceLock::new();
async fn tokio_main() {
    trace!("tokio_main");
    tokio::task::spawn(async {
        let pid = ARGS.get().as_ref().unwrap().core_pid as u32;
        if pid == 0 {
            trace!("core_pid 0");
            return;
        }
        log::trace!("watching:{} pid", pid);
        loop {
            if killed() {
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    let mut core_client = OyasumiCoreClient::connect(format!(
        "http://127.0.0.1:{}",
        ARGS.get().as_ref().unwrap().core_grpc_port
    ))
    .await
    .unwrap();
    CORE_CLIENT.set(core_client.clone().into()).unwrap();
    let http_port = core_client
        .get_http_server_port(Empty {})
        .await
        .unwrap()
        .into_inner()
        .port;
    info!("got http port:{:?}", http_port);
    let ui_port = match ARGS.get().as_ref().unwrap().core_grpc_port == 0 {
        true => 5173,
        false => serve_ui().await,
    };
    let url = format!(
        "http://localhost:{}/dashboard?corePort={}",
        ui_port, http_port
    );
    trace!("navigating to:{}", url);
    OVERLAY
        .get()
        .as_ref()
        .unwrap()
        .browser
        .main_frame()
        .unwrap()
        .load_url(Some(&(url.as_str()).into()));
    let grpc_server_port = start_grpc_server().await;
    let grpc_web_server_pos = start_grpc_web_server().await;
    core_client
        .on_overlay_sidecar_start(OverlaySidecarStartArgs {
            pid: std::process::id(),
            grpc_port: grpc_server_port as u32,
            grpc_web_port: grpc_web_server_pos as u32,
        })
        .await
        .unwrap();
    assert_ne!(grpc_web_server_pos, 0);
    assert_ne!(grpc_server_port, 0);
    let ws_port = start_websocket_server().await;
    OVERLAY.wait().inject_ipc(ws_port);
    tokio::time::sleep(Duration::from_millis(2000)).await;
    println!("showing");
    show_dashboard();
}
static mut KILL: bool = false;
// #[allow(dead_code)]
pub fn kill() {
    if !killed() {
        trace!("killing overlay");
    }
    unsafe { KILL = true };
}
pub fn killed() -> bool {
    let pid = ARGS.get().as_ref().unwrap().core_pid as u32;
    if pid != 0 && !PathBuf::from(format!("/proc/{}", pid)).exists() {
        unsafe { KILL = true };
    }

    unsafe { KILL }
}
