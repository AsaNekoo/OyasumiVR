use crate::overlay_grpc::{
    self, AddNotificationRequest, AddNotificationResponse, ClearNotificationRequest, Empty,
    OverlayMenuOpenRequest, OyasumiSidecarState, SetDebugTranslationsRequest,
    SetMicrophoneActiveRequest,
    oyasumi_overlay_sidecar_server::{OyasumiOverlaySidecar, OyasumiOverlaySidecarServer},
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
