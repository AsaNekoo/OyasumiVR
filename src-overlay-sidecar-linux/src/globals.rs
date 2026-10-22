use tokio::sync::Mutex;

use crate::overlay_grpc::OyasumiSidecarState;

pub const CORE_GRPC_DEV_PORT: u16 = 5176;
// pub const CORE_HTTP_DEV_PORT: u16 = 5177;
pub const OVERLAY_SIDECAR_GRPC_DEV_PORT: u16 = 5174;
pub const OVERLAY_SIDECAR_GRPC_WEB_DEV_PORT: u16 = 5175;
pub static STATE: Mutex<Option<OyasumiSidecarState>> = Mutex::const_new(None);
pub mod textures {
    use std::{io::Cursor, sync::LazyLock};

    use xr_overlay::{Texture, vulkano::format::Format};

    pub static MIC_MUTE: LazyLock<Texture> =
        LazyLock::new(|| decode_texture(include_bytes!("Resources/mic_mute.png")));
    pub static MIC_UNMUTE: LazyLock<Texture> =
        LazyLock::new(|| decode_texture(include_bytes!("Resources/mic_unmute.png")));
    // drawn as a raw openxr layer, which expects premultiplied alpha
    pub static POINTER: LazyLock<Texture> =
        LazyLock::new(|| decode_texture_premultiplied(include_bytes!("Resources/pointer.png")));
    // #[cfg(debug_assertions)]
    // #[allow(dead_code)]
    // pub static TEST: LazyLock<Texture> =
    //     LazyLock::new(|| decode_texture(include_bytes!("Resources/test.png")));
    #[inline(never)]
    fn decode_texture_premultiplied(bytes: &[u8]) -> Texture {
        let texture = decode_texture(bytes);
        let (width, height) = (texture.width(), texture.height());
        let mut bytes = texture.to_raw();
        for px in bytes.chunks_exact_mut(4) {
            let alpha = px[3] as u16;
            for c in &mut px[..3] {
                *c = ((*c as u16 * alpha + 127) / 255) as u8;
            }
        }
        Texture::new(width, height, bytes, Format::R8G8B8A8_UNORM)
    }
    #[inline(never)]
    fn decode_texture(bytes: &[u8]) -> Texture {
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
        Texture::new(width, height, bytes, Format::R8G8B8A8_UNORM)
    }
}
