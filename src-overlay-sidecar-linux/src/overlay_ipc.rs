use std::{sync::OnceLock, time::Duration};

use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, trace};
use rand::{
    distr::{Alphabetic, SampleString},
    rand_core::le,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use xr_overlay_cef::cef::{ImplBrowser, ImplFrame};

use crate::{model::Overlay, overlay_grpc::OyasumiSidecarState};
pub const IPC_SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/bundle.js"));
// pub const IPC_SCRIPT:&str=include_str!("../target/debug/build/src-overlay-sidecar-linux-8079ff49c704d0bc/out/bundle.js");
pub struct OverlayIPCAddNotification<'a> {
    message: &'a str,
    duration: Duration,
}
fn get_ipc_script(port: u16) -> String {
    let mut f = IPC_SCRIPT.splitn(2, "WS_ADDR");
    let mut out = String::with_capacity(IPC_SCRIPT.len() + 20);
    out.push_str(f.next().unwrap());
    out.push_str(&format!("localhost:{}", port));
    out.push_str(&f.next().unwrap());
    out
}
//OyasumiOverlayIPCIn
impl Overlay {
    pub fn execute_js(&self, js: &str) {
        trace!("running javascript:\n {}", js);
        self.browser
            .main_frame()
            .unwrap()
            .execute_java_script(Some(&js.into()), None, 0);
    }
    pub fn hide_dashboard(&self) {
        self.execute_js("window.OyasumiIPCIn.hideDashboard();");
    }
    pub fn show_dashboard(&self) {
        self.execute_js("window.OyasumiIPCIn.showDashboard();");
    }
    pub fn show_tool_tip(&self, string: Option<&str>) {
        self.execute_js(&format!(
            "window.OyasumiIPCIn.showToolTip({});",
            string.unwrap_or_default()
        ));
    }
    pub fn add_notification(&self, notification: OverlayIPCAddNotification) -> String {
        let id = Alphabetic.sample_string(&mut rand::rng(), 16);
        let args = format!(
            "{{id:{},message:{},duration:{}}}",
            id,
            notification.message,
            notification.duration.as_millis()
        );
        self.execute_js(&format!("window.OyasumiIPCIn.addNotification({});", args));
        id
    }
    pub fn clear_notification(&self, string: &str) {
        self.execute_js(&format!(
            "window.OyasumiIPCIn.clearNotification({});",
            string
        ));
    }
    pub fn inject_ipc(&self, port: u16) {
        self.execute_js(&get_ipc_script(port));
    }
    pub fn set_state(&self, state: OyasumiSidecarState) {
        //idk how to encode this
        todo!()
        // self.browser.main_frame().unwrap().execute_java_script(Some(&format!("window.OyasumiIPCIn.showToolTip({});",string.unwrap_or_default()).into()), None, 0);
    }
}
pub async fn start_websocket_server() -> u16 {
    let addr = "127.0.0.1:0";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    tokio::task::spawn(async move {
        info!("[websocket] listening on:{}",listener.local_addr().as_ref().unwrap());
        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();

            tokio::spawn(handle_connection(ws_stream));
        }
    });
    port
}
async fn handle_connection(ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    log::info!("[websocket] New client connected");
    let (mut sender, mut receiver) = ws_stream.split();

    while let Some(message) = receiver.next().await {
        match message {
            Ok(msg) => {
                if !msg.is_text(){
                    error!("[websocket] msg is not text:{:?}",msg);
                    continue;
                }
                let msg = msg.into_text().unwrap();
                info!("[websocket] ipc:{}", msg.as_str());
                let mut msg=msg.splitn(2, ":");
                let call_id=msg.next().unwrap().parse::<u8>().unwrap();
                let funtion=unsafe{*(&raw const call_id as *const FuntionCall )};
                match funtion {
                    FuntionCall::OnUiReady => {
                        debug_assert_eq!("{}",msg.next().unwrap());
                        debug!("[websocket] recived on ui ready");
                    }
                    _=>panic!("function with id:{} is not implemented",call_id)
                }
            }
            Err(e) => {
                log::error!("Error receiving message: {}", e);
                break;
            }
        }
    }
    log::info!("Client disconnected");
}
#[derive(Clone, Copy,PartialEq, PartialOrd,Debug)]
enum FuntionCall {
  SetSleepMpde = 0,
  OnUiReady = 1,
  SyncState = 2,
  SendEventVoid = 3,
  SendEventString = 4,
  SendEventBool = 5,
  SendEventInt = 6,
  SendEventDouble = 7,
  SendEventJson = 8,
  SendEvent = 9,
  AddNotification = 10,
  ShowToolTip = 11,
  Dispose = 12,
  GetDebugTranslations = 13,
  Close = 14,
}