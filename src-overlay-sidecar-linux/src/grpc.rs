use std::{f32::consts::E, net::SocketAddr, time::Duration};

use log::{error, info};
use tokio::{sync::Mutex, task::JoinHandle};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin};

use crate::{
    ARGS, HANDLES,
    globals::STATE,
    overlay_grpc::{
        AddNotificationRequest, AddNotificationResponse, ClearNotificationRequest, Empty,
        OverlayMenuOpenRequest, OyasumiSidecarState, SetDebugTranslationsRequest,
        SetMicrophoneActiveRequest,
        oyasumi_overlay_sidecar_server::{OyasumiOverlaySidecar, OyasumiOverlaySidecarServer},
    },
    overlay_ipc::OverlayIPCAddNotification,
    vr::{DASBOARD_VISIBLE, NOTIFICATION_OVERLAY, OVERLAY, XR_CTX, hide_dashboard, set_mic_state, show_dashboard},
};
#[derive(Debug, Default, Clone)]
pub struct GrpcServer {}
#[allow(unused_variables)]
#[tonic::async_trait]
impl OyasumiOverlaySidecar for GrpcServer {
    async fn add_notification(
        &self,
        request: tonic::Request<AddNotificationRequest>,
    ) -> Result<tonic::Response<AddNotificationResponse>, tonic::Status> {
        let req = request.into_inner();
        let id = NOTIFICATION_OVERLAY
            .wait()
            .add_notification(OverlayIPCAddNotification {
                message: &req.message,
                duration: Duration::from_millis(req.duration as u64),
            });
        Ok(AddNotificationResponse {
            notification_id: Some(id),
        }
        .into())
    }

    async fn clear_notification(
        &self,
        request: tonic::Request<ClearNotificationRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        NOTIFICATION_OVERLAY
            .wait()
            .clear_notification(&request.into_inner().notification_id);
        Ok(Empty {}.into())
    }

    async fn sync_state(
        &self,
        request: tonic::Request<OyasumiSidecarState>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        #[cfg(debug_assertions)]
        log::trace!("got new state from core:{:?}", request);
        let req = request.into_inner();
        STATE.lock().await.replace(req.clone());
        OVERLAY.wait().set_state(req);
        Ok(Empty::default().into())
    }

    async fn set_debug_translations(
        &self,
        request: tonic::Request<SetDebugTranslationsRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        todo!()
    }

    async fn open_overlay_menu(
        &self,
        request: tonic::Request<OverlayMenuOpenRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        show_dashboard();
        Ok(Empty {}.into())
    }

    async fn close_overlay_menu(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        hide_dashboard().await;
        Ok(Empty {}.into())
    }

    async fn toggle_overlay_menu(
        &self,
        request: tonic::Request<OverlayMenuOpenRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        match unsafe { DASBOARD_VISIBLE } {
            true => hide_dashboard().await,
            false => show_dashboard(),
        }
        Ok(Empty {}.into())
    }

    async fn set_microphone_active(
        &self,
        request: tonic::Request<SetMicrophoneActiveRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let req = request.into_inner();
        if req.mode == 0 { // 0= hardware
                set_mic_state_fadeout(req.active).await;
        }else {
            set_mic_state(false, 1., false);
        }
        Ok(Empty{}.into())
    }
}
async fn set_mic_state_fadeout(mute:bool){
    static FUT:Mutex<Option<JoinHandle<()>>>=Mutex::const_new(None);
    let mut guard=FUT.lock().await;
    let task=async move{
        const START:f32=1.0;
        const END:f32=0.;
        const DURATION:Duration=Duration::from_millis(2500);
        let step_size=Duration::from_millis(1000/XR_CTX.get().as_ref().unwrap().read().unwrap().current_refresh_rate() as u64);
        let step_count=DURATION.as_millis() as u64/step_size.as_millis() as u64;
        let mut alpha=START;
        for _ in 0..step_count{
            set_mic_state(mute, alpha, true);
            alpha-=(START-END)/step_count as f32;
            tokio::time::sleep(step_size).await;

        }
        set_mic_state(mute, alpha, false);
    };
    // if guard.is_none(){
        guard.replace(tokio::task::spawn(task));
    // }else if let Some(handler) {
        
    // }

}
pub async fn start_grpc_server() -> u16 {
    let port: u16 = match ARGS.get().as_ref().unwrap().core_pid == 0 {
        true => crate::globals::OVERLAY_SIDECAR_GRPC_DEV_PORT,
        false => portpicker::pick_unused_port().unwrap(),
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
pub async fn start_grpc_web_server() -> u16 {
    let port: u16 = match ARGS.get().as_ref().unwrap().core_pid == 0 {
        true => crate::globals::OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT,
        false => portpicker::pick_unused_port().unwrap(),
    };
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    info!("Starting gRPC web server on {}", addr);
    let server = Server::builder()
        .accept_http1(true)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(AllowOrigin::any())
                .allow_headers(AllowHeaders::any()),
        )
        .layer(GrpcWebLayer::new())
        .add_service(OyasumiOverlaySidecarServer::new(GrpcServer::default()))
        .serve(addr);
    HANDLES.lock().unwrap().push(tokio::task::spawn(async move {
        {
            if let Err(err) = server.await {
                error!("Failed to start web gRPC server: {}", err);
            }
        }
    }));
    addr.port()
}
