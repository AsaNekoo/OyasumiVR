use tokio::sync::Mutex;

use crate::overlay_grpc::OyasumiSidecarState;

pub const CORE_GRPC_DEV_PORT: u16 = 5176;
// pub const CORE_HTTP_DEV_PORT: u16 = 5177;
pub const OVERLAY_SIDECAR_GRPC_DEV_PORT: u16 = 5174;
pub const OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT: u16 = 5175;
pub static STATE: Mutex<Option<OyasumiSidecarState>> = Mutex::const_new(None);
pub mod textures {
    use std::{io::Cursor, sync::LazyLock};

    use xr_overlay::RgbaTexture;

    pub static MIC_MUTE: LazyLock<RgbaTexture> =
        LazyLock::new(|| decode_texture(include_bytes!("Resources/mic_mute.png")));
    pub static MIC_UNMUTE: LazyLock<RgbaTexture> =
        LazyLock::new(|| decode_texture(include_bytes!("Resources/mic_unmute.png")));
    pub static POINTER: LazyLock<RgbaTexture> =
        LazyLock::new(|| decode_texture(include_bytes!("Resources/pointer.png")));
    // #[cfg(debug_assertions)]
    // #[allow(dead_code)]
    // pub static TEST: LazyLock<RgbaTexture> =
    //     LazyLock::new(|| decode_texture(include_bytes!("Resources/test.png")));
    #[inline(never)]
    fn decode_texture(bytes: &[u8]) -> RgbaTexture {
        let image = image::ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let height = image.height();
        let width = image.width();
        let mut bytes = image.into_raw();
        bytes.shrink_to_fit();
        RgbaTexture::new(width, height, bytes)
    }
}
