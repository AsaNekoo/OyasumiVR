use std::{os, sync::Arc};

use log::{debug, info, trace};
use rosc::OscType;
use tokio::sync::Mutex;
use vrchat_osc::{
    models::{OscNode, OscRootNode},
    VRChatOSC,
};

use crate::{
    osc::{
        self,
        models::{OSCMessage, OSCMethod, OSCValue, SupportedOscType},
    },
    utils,
};
//VRChatOSC internally uses RwLock
static OSC_SERVER: Mutex<Option<Arc<VRChatOSC>>> = Mutex::const_new(None);
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_osc_address() -> Option<String> {
    log::debug!("get_vrchat_osc_address");
    OSC_SERVER
        .lock()
        .await
        .as_ref()?
        .list_mdns_services()
        .await
        .iter()
        .find(|service| {
            let name = service
                .0
                .to_string()
                .starts_with("VRChat-Client")
                .to_string();
            name.starts_with("VRChat-Client") && name.contains("udp")
        })
        .map(|x| x.1.to_string())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn get_vrchat_oscquery_address() -> Option<String> {
    log::debug!("get_vrchat_oscquery_address");
    OSC_SERVER
        .lock()
        .await
        .as_ref()?
        .list_mdns_services()
        .await
        .iter()
        .find(|service| {
            let name = service
                .0
                .to_string()
                .starts_with("VRChat-Client")
                .to_string();
            name.starts_with("VRChat-Client") && name.contains("tcp")
        })
        .map(|x| x.1.to_string())
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn stop_osc_server() {
    log::debug!("stop_osc_server");
    if let Some(server) = OSC_SERVER.lock().await.take() {
        server.shutdown().await.ok();
    }
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]

pub async fn start_osc_server() -> Option<(String, String)> {
    log::debug!("start_osc_server");
    //todo: fix
    Some((
        String::from("i'm so confused"),
        String::from("how this works"),
    ))
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn add_osc_method(method: OSCMethod) {
    log::debug!("add_osc_method");
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
    unimplemented!("{:?}\n{:?}\n{:?}\n:{:?}", addr, osc_addr, types, values);
}

#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn osc_valid_addr(addr: String) -> bool {
    unimplemented!("{:?}", addr);
}
// async fn osc_send(addr: String, osc_addr: String, data: Vec<OscType>) -> Result<bool, String> {
//     unimplemented!("{:?}\n{:?}\n{:?}",addr,osc_addr,data);
// }
#[cfg(debug_assertions)]
static WHITELIST: Mutex<Vec<String>> = Mutex::const_new(Vec::new());
#[tauri::command]
#[oyasumivr_macros::command_profiling]
pub async fn set_osc_receive_address_whitelist(whitelist: Vec<String>) {
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
    vrchat_osc
        .register("OyasumiVR", root_node, |msg| {
            // info!("Received OSC message:{:?}", msg); //trace is broken for some reason
            match msg {
                rosc::OscPacket::Message(osc_message) => {
                    if osc_message.args.len() > 1 {
                        unimplemented!("{:?}", osc_message);
                    }
                    let msg= osc_message.args.into_iter().map(|arg| match arg {
                            OscType::Int(v) => OSCValue{ kind: "int".into(), value: Some(v.to_string()) },
                            OscType::Float(v) => OSCValue{ kind: "float".into(), value: Some(v.to_string()) },
                            OscType::String(v) => OSCValue{ kind: "string".into(), value: Some(v.to_string()) },
                            OscType::Bool(v) => OSCValue{ kind: "bool".into(), value: Some(v.to_string()) },
                            _=>unimplemented!("{:?}",arg),
                        }).collect::<Vec<_>>();
                    tokio::task::spawn(utils::send_event(
                        "OSC_MESSAGE",
                       OSCMessage{ address: osc_message.addr, values:msg},
                    ));
                }
                rosc::OscPacket::Bundle(osc_bundle) => unimplemented!(),
            }
        })
        .await
        .ok()
        .unwrap();
    OSC_SERVER.lock().await.replace(vrchat_osc);
}
