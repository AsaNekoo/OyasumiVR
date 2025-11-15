use std::time::Duration;

use log::trace;
use rand::{distr::{Alphabetic, SampleString}, rand_core::le};
use serde::Serialize;
use xr_overlay_cef::cef::{ImplBrowser, ImplFrame};

use crate::{model::Overlay, overlay_grpc::OyasumiSidecarState};

pub struct OverlayIPCAddNotification<'a>{
    message:&'a str,
    duration:Duration
}
//OyasumiOverlayIPCIn
impl Overlay{
    pub fn execute_js(&self,js:&str){
        trace!("[ipc] {}", js);
        self.browser.main_frame().unwrap().execute_java_script(Some(&js.into()), None, 0);
    }
    pub fn hide_dashboard(&self){
        self.execute_js("window.OyasumiIPCIn.hideDashboard();");
    }
    pub fn show_dashboard(&self){
        self.execute_js("window.OyasumiIPCIn.showDashboard();");
    }
    pub fn show_tool_tip(&self,string:Option<&str>){
        self.execute_js(&format!("window.OyasumiIPCIn.showToolTip({});",string.unwrap_or_default()));
    }
    pub fn add_notification(&self,notification:OverlayIPCAddNotification)->String{
        let id=Alphabetic.sample_string(&mut rand::rng(), 16);
        let args=format!("{{id:{},message:{},duration:{}}}",id,notification.message,notification.duration.as_millis());
        self.execute_js(&format!("window.OyasumiIPCIn.addNotification({});",args));
        id
    }
    pub fn clear_notification(&self,string:&str){
        self.execute_js(&format!("window.OyasumiIPCIn.clearNotification({});",string));
    }
    pub fn set_state(&self,state:OyasumiSidecarState){
        //idk how to encode this
        todo!()
        // self.browser.main_frame().unwrap().execute_java_script(Some(&format!("window.OyasumiIPCIn.showToolTip({});",string.unwrap_or_default()).into()), None, 0);
    }


}
