use std::net::SocketAddr;

use log::{error, info};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowOrigin};

use crate::{
    HANDLES,
    globals::{CORE_MODE, CoreMode},
    overlay_grpc::{
        self, AddNotificationRequest, AddNotificationResponse, ClearNotificationRequest, Empty,
        OverlayMenuOpenRequest, OyasumiSidecarState, SetDebugTranslationsRequest,
        SetMicrophoneActiveRequest,
        oyasumi_overlay_sidecar_server::{OyasumiOverlaySidecar, OyasumiOverlaySidecarServer},
    },
};
#[derive(Debug, Default, Clone)]
pub struct GrpcServer {}
#[tonic::async_trait]
impl OyasumiOverlaySidecar for GrpcServer {
    fn add_notification<'life0, 'async_trait>(
        &'async_trait self,
        request: tonic::Request<AddNotificationRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<AddNotificationResponse>,
                        tonic::Status,
                    >,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn clear_notification<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<ClearNotificationRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn sync_state<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<OyasumiSidecarState>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn set_debug_translations<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<SetDebugTranslationsRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn open_overlay_menu<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<OverlayMenuOpenRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn close_overlay_menu<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<Empty>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn toggle_overlay_menu<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<OverlayMenuOpenRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn set_microphone_active<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<SetMicrophoneActiveRequest>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                    Output = std::result::Result<tonic::Response<Empty>, tonic::Status>,
                > + ::core::marker::Send,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }
}
pub async fn start_grpc_server() -> u16 {
    let port: u16 = match CORE_MODE {
        CoreMode::Dev => crate::globals::OVERLAY_SIDECAR_GRPC_DEV_PORT,
        CoreMode::Release => 0,
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
    let port: u16 = match CORE_MODE {
        CoreMode::Dev => crate::globals::OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT,
        CoreMode::Release => 0,
    };
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    info!("Starting gRPC web server on {}", addr);
    let server = Server::builder()
        .accept_http1(true).layer(tower_http::cors::CorsLayer::new().allow_origin(AllowOrigin::any()).allow_headers(AllowHeaders::any()))
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
