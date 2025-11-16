use std::{
    sync::{LazyLock, Mutex, OnceLock},
    time::Duration,
};

use argh::FromArgs;
use log::{error, info, trace};
use tokio::join;
use tonic::transport::Channel;
use xr_overlay_cef::{
    cef::{ImplBrowser, ImplFrame},
    disable_vr, pointless_cef_thread_spawner,
};

use crate::{
    core_grpc::{Empty, EventParams, OverlaySidecarStartArgs, oyasumi_core_client::OyasumiCoreClient}, globals::STATE, grpc::{start_grpc_server, start_grpc_web_server}, overlay_grpc::OyasumiSidecarState, overlay_ipc::start_websocket_server, vr::{OVERLAY, start_vr}
};
pub mod globals;
pub mod grpc;
pub mod input;
pub mod model;
pub mod overlay_ipc;
pub mod vr;
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

fn main() {
    pointless_cef_thread_spawner();
    CMD_ARGS.set(argh::from_env()).unwrap();
    if CMD_ARGS.get().unwrap().no_vr {
        disable_vr();
    }
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("xr_overlay_cef", log::LevelFilter::Debug)
        .init();
    let vr_thread = start_vr();
    let runtime_thread = std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(tokio_main());
        loop {
            let handle = {
                let mut g = HANDLES.lock().unwrap();
                let l = g.len();
                if l == 0 {
                    break;
                }
                g.swap_remove(l - 1)
            };
            runtime.block_on(async move { join!(handle).0.unwrap() });
        }
    });

    runtime_thread.join().unwrap();
    vr_thread.join().unwrap();
}
static HANDLES: LazyLock<Mutex<Vec<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| Mutex::default());

static CORE_CLIENT:OnceLock<tokio::sync::Mutex<OyasumiCoreClient<Channel>>>=OnceLock::new();
async fn tokio_main() {
    let mut core_client =
        OyasumiCoreClient::connect(format!("http://127.0.0.1:{}", globals::CORE_GRPC_DEV_PORT))
            .await
            .unwrap();
    CORE_CLIENT.set(core_client.clone().into()).unwrap();
    let http_port = core_client.get_http_server_port(Empty {}).await.unwrap();
    info!("got http port:{:?}", http_port);
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
    let ws_port = start_websocket_server().await;
    trace!("inject");
    OVERLAY.wait().inject_ipc(ws_port);
    tokio::time::sleep(Duration::from_secs(1)).await;
    loop {
    //     let mut state=STATE.lock().await.as_ref().unwrap().clone();
    //     state.sleep_mode=true;
    //     OVERLAY.wait().set_state(state.clone());
    //     // OVERLAY.wait().hide_dashboard();
    //     tokio::time::sleep(Duration::from_millis(2000)).await;
    //     // OVERLAY.wait().show_dashboard();
    //     tokio::time::sleep(Duration::from_millis(2000)).await;
    //     // state.sleep_mode=true;
    //    OVERLAY.wait().set_state(state);
    }
    // let res=overlya_client.sync_state(request)
}
