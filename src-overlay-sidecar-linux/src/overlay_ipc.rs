use std::time::Duration;

use base64::{Engine, prelude::BASE64_STANDARD};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use prost::Message;
use rand::distr::{Alphabetic, SampleString};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite;
use xr_overlay_cef::cef::{ImplBrowser, ImplFrame};

use crate::{
    CORE_CLIENT,
    core_grpc::EventParams,
    globals::STATE,
    kill, killed,
    model::Overlay,
    overlay_grpc::OyasumiSidecarState,
    vr::{OVERLAY, hide_dashboard},
    warn_unimplemented,
};
pub const IPC_SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/bundle.js"));
pub struct OverlayIPCAddNotification<'a> {
    pub message: &'a str,
    pub duration: Duration,
}
fn get_ipc_script(port: u16) -> String {
    let mut f = IPC_SCRIPT.splitn(2, "WS_ADDR");
    let mut out = String::with_capacity(IPC_SCRIPT.len() + 20);
    out.push_str(f.next().unwrap());
    out.push_str(&format!("localhost:{}", port));
    out.push_str(f.next().unwrap());
    out
}
//OyasumiOverlayIPCIn
impl Overlay {
    pub fn execute_js(&self, js: &str) {
        log::trace!("running javascript:\n {}", js);
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
            "{{id:\"{}\",message:\"{}\",duration:{}}}",
            id,
            notification.message,
            notification.duration.as_millis()
        );
        self.execute_js(&format!("window.OyasumiIPCIn.addNotification({});", args));
        id
    }
    pub fn clear_notification(&self, id: &str) {
        self.execute_js(&format!("window.OyasumiIPCIn.clearNotification(\"{}\");", id));
    }
    pub fn inject_ipc(&self, port: u16) {
        self.execute_js(&get_ipc_script(port));
    }
    pub fn set_state(&self, state: OyasumiSidecarState) {
        let state = unsafe { String::from_utf8_unchecked(state.encode_to_vec()) };
        let state = BASE64_STANDARD.encode(state);
        self.execute_js(&format!("window.OyasumiIPCIn.setState(\"{}\");", state));
    }
}
pub async fn start_websocket_server() -> u16 {
    let addr = "127.0.0.1:0";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    tokio::task::spawn(async move {
        info!(
            "[websocket] listening on:{}",
            listener.local_addr().as_ref().unwrap()
        );
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
    tokio::time::sleep(Duration::from_millis(200)).await;
    #[derive(Deserialize)]
    struct Event<T> {
        #[serde(rename(deserialize = "eventName"))]
        pub event_name: String,
        pub data: T,
    }
    while let Some(message) = receiver.next().await {
        match message {
            Ok(msg) => {
                if killed() {
                    break;
                }
                if matches!(msg, tungstenite::Message::Close(_)) {
                    break;
                }
                if !msg.is_text() {
                    error!("[websocket] msg is not text:{:?}", msg);
                    continue;
                }
                let msg = msg.into_text().unwrap();
                log::trace!("[websocket] ipc:{}", msg.as_str());
                let mut msg = msg.splitn(2, ":");
                let call_id = msg.next().unwrap().parse::<u8>().unwrap();
                let funtion = unsafe { *(&raw const call_id as *const FuntionCall) };
                match funtion {
                    FuntionCall::OnUiReady => {
                        debug_assert_eq!("{}", msg.next().unwrap());
                        if STATE.lock().await.as_ref().is_none() {
                            continue;
                        }
                        debug!("[websocket] recived on ui ready");
                        OVERLAY
                            .wait()
                            .set_state(STATE.lock().await.as_ref().unwrap().clone());
                    }
                    FuntionCall::ShowToolTip => { /*idk */ }
                    FuntionCall::AddNotification => {
                        #[derive(Deserialize)]
                        struct AddNotificationArgs {
                            pub message: String,
                            pub duration: u32,
                            pub seq: u32,
                        }
                        let args = serde_json::from_str::<AddNotificationArgs>(msg.next().unwrap())
                            .unwrap();
                        #[derive(Serialize)]
                        struct AddNotificationResponse {
                            notification_id: Option<String>,
                            seq: u32,
                        }
                        match CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .add_notification(crate::core_grpc::AddNotificationRequest {
                                message: args.message,
                                duration: args.duration,
                            })
                            .await
                        {
                            Ok(v) => {
                                let data = v.into_inner();
                                sender.send(tokio_tungstenite::tungstenite::Message::Text(
                                    serde_json::to_string(&AddNotificationResponse {
                                        notification_id: data.notification_id,
                                        seq: args.seq,
                                    }).unwrap().into(),
                                )).await.unwrap_or_else(|err| error!("[websocket] failed to send reponse to AddNotification:{:?}",err));
                            }
                            Err(err) => error!(
                                "[websocket] forwarding AddNotification failed with:{:?}",
                                err
                            ),
                        }
                    }
                    FuntionCall::Close => {
                        hide_dashboard().await;
                    }
                    FuntionCall::Dispose=>{},
                    FuntionCall::SendEventBool => {
                        let args =
                            serde_json::from_str::<Event<bool>>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: Some(
                                    crate::core_grpc::event_params::EventData::BoolData(args.data),
                                ),
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventBool:{:?}", err);
                        }
                    }
                    FuntionCall::SendEventInt => {
                        let args = serde_json::from_str::<Event<i32>>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: Some(
                                    crate::core_grpc::event_params::EventData::IntData(args.data),
                                ),
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventInt:{:?}", err);
                        }
                    }
                    FuntionCall::SendEventDouble => {
                        let args = serde_json::from_str::<Event<f64>>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: Some(
                                    crate::core_grpc::event_params::EventData::DoubleData(
                                        args.data,
                                    ),
                                ),
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventDouble:{:?}", err);
                        }
                    }
                    FuntionCall::SendEventJson => {
                        let args =
                            serde_json::from_str::<Event<String>>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: Some(
                                    crate::core_grpc::event_params::EventData::JsonData(args.data),
                                ),
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventJson:{:?}", err);
                        }
                    }
                    FuntionCall::SendEventString => {
                        let args =
                            serde_json::from_str::<Event<String>>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: Some(
                                    crate::core_grpc::event_params::EventData::StringData(
                                        args.data,
                                    ),
                                ),
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventString:{:?}", err);
                        }
                    }
                    FuntionCall::SendEventVoid => {
                        #[derive(Deserialize)]
                        struct VoidEvent {
                            #[serde(rename(deserialize = "eventName"))]
                            pub event_name: String,
                        }
                        let args = serde_json::from_str::<VoidEvent>(msg.next().unwrap()).unwrap();
                        if let Err(err) = CORE_CLIENT
                            .wait()
                            .lock()
                            .await
                            .send_event(EventParams {
                                event_name: args.event_name,
                                event_data: None,
                            })
                            .await
                        {
                            error!("[websocket] failed to forward SendEventVoid:{:?}", err);
                        }
                    }
                    // FuntionCall::SetSleepMpde => {
                    //     #[derive(Deserialize)]
                    //     struct SetSleepMpdeArgs{
                    //         pub enabled:bool
                    //     }
                    //     let args=serde_json::from_str::<SetSleepMpdeArgs>(msg.next().unwrap()).unwrap();

                    // },
                    _ => warn_unimplemented!("maybe_name:{:?},id:{}", funtion, call_id),
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
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[allow(dead_code)]
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
