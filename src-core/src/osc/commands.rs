use log::debug;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::{spawn, sync::Mutex};
use vrchat_osc::rosc::{OscMessage, OscPacket, OscType};
use vrchat_osc::{
    VRChatOSC,
    models::{OscNode, OscRootNode},
};

use crate::{
    osc::models::{OSCMessage, OSCMethod, OSCValue, SupportedOscType},
    utils::{self, send_event},
    warn_unimplemented,
};
static VRCHAT_OSC_ADDR: std::sync::Mutex<Option<SocketAddr>> = std::sync::Mutex::new(None);
static VRCHAT_OSCQUERY_ADDR: std::sync::Mutex<Option<SocketAddr>> = std::sync::Mutex::new(None);
//VRChatOSC internally uses RwLock
pub static OSC_SERVER: Mutex<Option<Arc<VRChatOSC>>> = Mutex::const_new(None);
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
#[allow(static_mut_refs)]
pub async fn add_osc_method(method: OSCMethod) {
    //its called only "few" times at the start so cloning string is fine
    if !unsafe { WHITELIST.contains(&method.address.clone().into()) } {
        #[cfg(debug_assertions)]
        panic!("{:?}", method);
        #[cfg(not(debug_assertions))]
        warn_unimplemented!("not on the whitelist: {:?}", method);
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

//#Safety; it's set once and any reads will happen after the write
static mut WHITELIST: Vec<Box<str>> = Vec::new();
#[tauri::command]
#[oyasumivr_macros::command_profiling]
#[allow(static_mut_refs)]
pub async fn set_osc_receive_address_whitelist(whitelist: Vec<String>) {
    let mut server_guard = OSC_SERVER.lock().await;
    debug!("starting osc server with paths:{:?}", whitelist);
    unsafe {
        for s in &whitelist {
            WHITELIST.push(s.clone().into_boxed_str());
        }
    }

    let vrchat_osc = match VRChatOSC::new(None).await {
        Ok(v) => v,
        Err(err) => {
            log::error!("[osc] Failed to start osc server:{:?}", err);
            return;
        }
    };
    let mut root_node = OscRootNode::new();
    for path in whitelist {
        root_node = root_node.add_node(OscNode {
            full_path: path,
            ..Default::default()
        })
    }
    vrchat_osc
        .on_connect(|service| match service {
            vrchat_osc::ServiceType::Osc(name, socket_addr) => {
                if name.starts_with("VRChat-Client-") {
                    log::debug!("new osc service {}:{}", name, socket_addr);
                    spawn(send_event(
                        "VRC_OSC_ADDRESS_CHANGED",
                        socket_addr.to_string(),
                    ));
                    VRCHAT_OSC_ADDR.lock().unwrap().replace(socket_addr);
                }
            }
            vrchat_osc::ServiceType::OscQuery(name, socket_addr) => {
                log::debug!("new osc query service {}:{}", name, socket_addr);
                spawn(send_event(
                    "VRC_OSCQUERY_ADDRESS_CHANGED",
                    socket_addr.to_string(),
                ));
                VRCHAT_OSCQUERY_ADDR.lock().unwrap().replace(socket_addr);
            }
        })
        .await;
    vrchat_osc
        .register("OyasumiVR", root_node, |msg| match msg {
            OscPacket::Message(osc_message) => {
                // println!("{:?}",osc_message);
                if !unsafe { WHITELIST.iter().any(|s| **s == *osc_message.addr.as_str()) } {
                    //vrchat seems to be sending more then requested
                    return;
                }
                if osc_message.args.len() > 1 {
                    return;
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
            OscPacket::Bundle(_osc_bundle) => warn_unimplemented!("osc bundle"),
        })
        .await
        .ok()
        .unwrap();
    server_guard.replace(vrchat_osc);
    tokio::time::sleep(Duration::from_secs(1)).await
}
