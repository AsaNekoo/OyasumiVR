use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
#[cfg(windows)]
use std::os;
use log::debug;
use rosc::{OscMessage, OscPacket, OscType};
use tokio::sync::Mutex;
use vrchat_osc::{
    models::{OscNode, OscRootNode},
    VRChatOSC,
};

use crate::{
    osc::models::{OSCMessage, OSCMethod, OSCValue, SupportedOscType},
    utils, warn_unimplemented,
};
static VRCHAT_OSC_ADDR: std::sync::Mutex<Option<SocketAddr>> = std::sync::Mutex::new(None);
static VRCHAT_OSCQUERY_ADDR: std::sync::Mutex<Option<SocketAddr>> = std::sync::Mutex::new(None);
//VRChatOSC internally uses RwLock
static OSC_SERVER: Mutex<Option<Arc<VRChatOSC>>> = Mutex::const_new(None);
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_osc_address() -> Option<String> {
    VRCHAT_OSC_ADDR
        .lock()
        .unwrap()
        .map(|a| a.to_string())
        .clone()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_oscquery_address() -> Option<String> {
    VRCHAT_OSCQUERY_ADDR
        .lock()
        .unwrap()
        .map(|a| a.to_string())
        .clone()
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn stop_osc_server() {
    if let Some(server) = OSC_SERVER.lock().await.take() {
        server.shutdown().await.ok();
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]

pub async fn start_osc_server() -> Option<(String, String)> {
    Some((
        get_vrchat_osc_address().await?,
        get_vrchat_oscquery_address().await?,
    ))
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn add_osc_method(method: OSCMethod) {
    #[cfg(debug_assertions)]
    if !WHITELIST.lock().await.contains(&method.address) {
        panic!("{:?}", method);
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_osc_method_value(address: String, value: String) {
    log::error!("unimplemeneted: {:?}:{:?}", address, value);
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn osc_send_command(
    addr: String,
    osc_addr: String,
    types: Vec<SupportedOscType>,
    values: Vec<String>,
) -> Result<bool, String> {
    OSC_SERVER
        .lock()
        .await
        .as_mut()
        .unwrap()
        .send_to_addr(
            OscPacket::Message(OscMessage {
                addr: osc_addr,
                args: values
                    .into_iter()
                    .enumerate()
                    .map(|(idx, value)| match types[idx] {
                        SupportedOscType::Int => OscType::Int(value.parse().unwrap()),
                        SupportedOscType::Float => OscType::Float(value.parse().unwrap()),
                        SupportedOscType::Boolean => OscType::Bool(value.parse().unwrap()),
                        SupportedOscType::String => OscType::String(value),
                    })
                    .collect(),
            }),
            SocketAddr::from_str(&addr).unwrap(),
        )
        .await
        .map_err(|err| err.to_string())?;
    Ok(true)
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn osc_valid_addr(addr: String) -> bool {
    SocketAddr::from_str(&addr).is_ok()
}
#[cfg(debug_assertions)]
static WHITELIST: Mutex<Vec<String>> = Mutex::const_new(Vec::new());
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_osc_receive_address_whitelist(whitelist: Vec<String>) {
    let mut server_guard = OSC_SERVER.lock().await;
    #[cfg(debug_assertions)]
    {
        *WHITELIST.lock().await = whitelist.clone();
    }
    debug!("starting osc server with paths:{:?}", whitelist);
    let vrchat_osc = VRChatOSC::new().await.unwrap();
    let mut root_node = OscRootNode::new();
    for path in whitelist {
        root_node = root_node.add_node(OscNode {
            full_path: path,
            ..Default::default()
        })
    }
    vrchat_osc.on_connect(|type_| {
        match type_ {
            vrchat_osc::ServiceType::Osc(_, socket_addr) => {
                VRCHAT_OSC_ADDR.lock().unwrap().replace(socket_addr)
            }
            vrchat_osc::ServiceType::OscQuery(_, socket_addr) => {
                VRCHAT_OSCQUERY_ADDR.lock().unwrap().replace(socket_addr)
            }
        };
    }).await;
    vrchat_osc
        .register("OyasumiVR", root_node, |msg| match msg {
            rosc::OscPacket::Message(osc_message) => {
                if osc_message.args.len() > 1 {
                    //seems to only be used by camerapos
                    return;
                    // unimplemented!("{:?}", osc_message);
                }
                let msg = osc_message
                    .args
                    .into_iter()
                    .map(|arg| match arg {
                        OscType::Int(v) => OSCValue {
                            kind: SupportedOscType::Int,
                            value: Some(v.to_string()),
                        },
                        OscType::Float(v) => OSCValue {
                            kind: SupportedOscType::Float,
                            value: Some(v.to_string()),
                        },
                        OscType::String(v) => OSCValue {
                            kind: SupportedOscType::String,
                            value: Some(v.to_string()),
                        },
                        OscType::Bool(v) => OSCValue {
                            kind: SupportedOscType::Boolean,
                            value: Some(v.to_string()),
                        },
                        _ => unimplemented!("{:?}", arg),
                    })
                    .collect::<Vec<_>>();
                tokio::task::spawn(utils::send_event(
                    "OSC_MESSAGE",
                    OSCMessage {
                        address: osc_message.addr,
                        values: msg,
                    },
                ));
            }
            rosc::OscPacket::Bundle(_osc_bundle) => warn_unimplemented!("osc bundle"),
        })
        .await
        .ok()
        .unwrap();
    server_guard.replace(vrchat_osc);
    tokio::time::sleep(Duration::from_secs(1)).await
}
