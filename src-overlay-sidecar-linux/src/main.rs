use std::{sync::{LazyLock, Mutex, OnceLock}, time::Duration};

use argh::FromArgs;
use log::{error, info};
use tokio::join;
use xr_overlay_cef::{cef::{ImplBrowser, ImplFrame}, 
    pointless_cef_thread_spawner}
;

use crate::{
    core_grpc::{Empty, OverlaySidecarStartArgs, oyasumi_core_client::OyasumiCoreClient},
    grpc::{start_grpc_server, start_grpc_web_server}, vr::{OVERLAY_BROWSER, start_vr},
};
pub mod globals;
pub mod grpc;
pub mod input;
pub mod vr;
pub mod model;
pub mod overlay_ipc;
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
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let vr_thread = if !CMD_ARGS.get().unwrap().no_vr {
        Some(start_vr())
    } else {
        None
    };
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
    if let Some(vr_thread) = vr_thread {
        vr_thread.join().unwrap();
    }
}
static HANDLES: LazyLock<Mutex<Vec<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| Mutex::default());

async fn tokio_main() {
    let mut core_client =
        OyasumiCoreClient::connect(format!("http://127.0.0.1:{}", globals::CORE_GRPC_DEV_PORT))
            .await
            .unwrap();
    let http_port = core_client.get_http_server_port(Empty {}).await.unwrap();
    info!("got http port:{:?}", http_port);
    let grpc_server_port = start_grpc_server().await;
    let grpc_web_server_pos=start_grpc_web_server().await;
    core_client
        .on_overlay_sidecar_start(OverlaySidecarStartArgs {
            pid: std::process::id(),
            grpc_port: grpc_server_port as u32,
            grpc_web_port: grpc_web_server_pos as u32,
        })
        .await
        .unwrap();
    loop {
        OVERLAY_BROWSER.get().unwrap().main_frame().unwrap().execute_java_script(Some(&"window.OyasumiIPCIn.hideDashboard();".into()), None, 0);
        tokio::time::sleep(Duration::from_millis(500)).await;
        OVERLAY_BROWSER.get().unwrap().main_frame().unwrap().execute_java_script(Some(&"window.OyasumiIPCIn.showDashboard();".into()), None, 0);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    // let res=overlya_client.sync_state(request)
}
