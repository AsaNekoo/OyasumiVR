use std::net::SocketAddr;

use log::{error, info};
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
    vr::{DASBOARD_VISIBLE, OVERLAY, hide_dashboard, show_dashboard},
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
        //core calls this to spawn notification overlay
        Ok(AddNotificationResponse {
            notification_id: None,
        }
        .into())
    }

    async fn clear_notification(
        &self,
        request: tonic::Request<ClearNotificationRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
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
        todo!()
    }
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
