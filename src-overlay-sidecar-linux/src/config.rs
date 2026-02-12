use std::fmt::Debug;

use xr_overlay::{
    openxr::Vector3f,
    runner::{DeviceRole, ShowMode},
    xr::ReferenceSpaceT,
};

use crate::vr::openxr_show_hand;

pub const DEFAULT_OVERLAY_CONFIG: &str = include_str!("../../overlay_config.toml");
#[derive(Clone, Debug)]
pub struct OverlayConfig {
    pub pointers: PointersConfig,
    // pub misc: MiscSettings,
    pub main_overlay: MainOverlay,
    pub notification_overlay: NotificationsOverlay,
    pub mute_indicator_overlay: MuteIndicatorOverlay,
}
impl OverlayConfig {
    pub fn default(fps: u8) -> Self {
        Self {
            pointers: PointersConfig {
                overlay_move_speed: 0.01,
                left_color: [1.0, 1.0, 1., 0.2],
                right_color: [1.0, 1.0, 1., 0.2],
                draw_only_when_on_overlay: true,
            },
            // misc: MiscSettings {
            //     app_name: "Oyasumi VR Overlay".into(),
            //     xr_sort_order: 4089,
            // },
            main_overlay: MainOverlay {
                framerate: fps / 2,
                position: [0., 0., -0.4],
                resolution: [1024, 1024],
                size: [0.6, 0.6],
                reference_space: ReferenceSpaceT::STAGE,
                show_mode: ShowMode::DeviceCallback {
                    role_callback: openxr_show_hand,
                    pos: Vector3f {
                        x: 0.0,
                        y: 0.0,
                        z: -0.4,
                    },
                    rot: None,
                },
            },
            notification_overlay: NotificationsOverlay {
                framerate: fps / 2,
                position: [0., -0.3, -0.6],
                resolution: [1024, 1024],
                size: [0.5, 0.5],
                reference_space: ReferenceSpaceT::VIEW,
            },
            mute_indicator_overlay: MuteIndicatorOverlay {
                position: [-0.3, -0.3, -0.6],
                size: [0.04, 0.04],
                reference_space: ReferenceSpaceT::VIEW,
            },
        }
    }
    pub fn default_with_config(data: &str, default_fps: u8) -> Result<Self, ConfigParseError> {
        let mut de = Self::default(default_fps);
        let m = toml::from_str::<toml::Value>(&data)
            .map_err(|_| ConfigParseError::FailedToParseToml)?;
        let t = m.as_table().unwrap();
        if let Some(main) = t.get("main_overlay").map(|t| t.as_table().unwrap()) {
            if let Some(framerate) = main.get("framerate") {
                let fps: u8;
                if framerate.is_str() {
                    fps = parse_fps(default_fps, framerate.as_str().unwrap())?;
                } else {
                    fps = framerate.as_integer().unwrap() as u8;
                }
                de.main_overlay.framerate = fps;
            }

            if let Some(position) = main.get("position").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.main_overlay.position = position.try_into().unwrap();
            }
            if let Some(resolution) = main.get("resolution").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_integer().unwrap() as u32)
                    .collect::<Vec<u32>>()
            }) {
                de.main_overlay.resolution = resolution.try_into().unwrap();
            }
            if let Some(size) = main.get("size").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.main_overlay.size = size.try_into().unwrap();
            }
            if let Some(reference_space) = main.get("reference_space").map(|r| r.as_str().unwrap())
            {
                de.main_overlay.reference_space = parse_reference_space(reference_space)?;
            }
            if let Some(show_mode) = main.get("show_mode").map(|r| r.as_str().unwrap()) {
                let v = de.main_overlay.position;
                de.main_overlay.show_mode = parse_show_mode(
                    show_mode,
                    Vector3f {
                        x: v[0],
                        y: v[1],
                        z: v[2],
                    },
                )?;
            }
        }
        if let Some(noti) = t.get("notification_overlay").map(|t| t.as_table().unwrap()) {
            if let Some(framerate) = noti.get("framerate") {
                let fps: u8;
                if framerate.is_str() {
                    fps = parse_fps(default_fps, framerate.as_str().unwrap())?;
                } else {
                    fps = framerate.as_integer().unwrap() as u8;
                }
                de.notification_overlay.framerate = fps;
            }
            if let Some(position) = noti.get("position").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.notification_overlay.position = position.try_into().unwrap();
            }
            if let Some(resolution) = noti.get("resolution").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_integer().unwrap() as u32)
                    .collect::<Vec<u32>>()
            }) {
                de.notification_overlay.resolution = resolution.try_into().unwrap();
            }
            if let Some(size) = noti.get("size").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.notification_overlay.size = size.try_into().unwrap();
            }
            if let Some(reference_space) = noti.get("reference_space").map(|r| r.as_str().unwrap())
            {
                de.notification_overlay.reference_space = parse_reference_space(reference_space)?;
            }
        }
        if let Some(mute) = t
            .get("mute_indicator_overlay")
            .map(|t| t.as_table().unwrap())
        {
            if let Some(position) = mute.get("position").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.mute_indicator_overlay.position = position.try_into().unwrap();
            }
            if let Some(size) = mute.get("size").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.mute_indicator_overlay.size = size.try_into().unwrap();
            }
            if let Some(reference_space) = mute.get("reference_space").map(|r| r.as_str().unwrap())
            {
                de.mute_indicator_overlay.reference_space = parse_reference_space(reference_space)?;
            }
        }
        // if let Some(misc) = t.get("misc").map(|t| t.as_table().unwrap()) {
        //     if let Some(sort) = misc
        //         .get("xr_sort_order")
        //         .map(|s| s.as_integer().unwrap() as u32)
        //     {
        //         de.misc.xr_sort_order = sort;
        //     }
        //     if let Some(app) = misc.get("app_name").map(|s| s.as_str().unwrap()) {
        //         de.misc.app_name = app.into();
        //     }
        // }
        if let Some(pointers) = t.get("pointers").map(|t| t.as_table().unwrap()) {
            if let Some(left) = pointers.get("left_color").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.pointers.left_color = left.try_into().unwrap();
            }
            if let Some(right) = pointers.get("right_color").map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|f| f.as_float().unwrap() as f32)
                    .collect::<Vec<f32>>()
            }) {
                de.pointers.right_color = right.try_into().unwrap();
            }
            if let Some(on_hit) = pointers
                .get("draw_only_when_on_overlay")
                .map(|d| d.as_bool().unwrap())
            {
                de.pointers.draw_only_when_on_overlay = on_hit;
            }
            if let Some(move_speed) = pointers
                .get("overlay_move_speed")
                .map(|s| s.as_float().unwrap() as f32)
            {
                de.pointers.overlay_move_speed = move_speed;
            }
        }

        Ok(de)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PointersConfig {
    pub left_color: [f32; 4],
    pub right_color: [f32; 4],
    pub draw_only_when_on_overlay: bool,
    pub overlay_move_speed: f32,
}
#[derive(Clone)]
pub struct MainOverlay {
    pub framerate: u8,
    pub position: [f32; 3],
    pub resolution: [u32; 2],
    pub size: [f32; 2],
    pub reference_space: ReferenceSpaceT,
    pub show_mode: ShowMode,
}
impl Debug for MainOverlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MainOverlay")
            .field("framerate", &self.framerate)
            .field("position", &self.position)
            .field("resolution", &self.resolution)
            .field("size", &self.size)
            .field("reference_space", &self.reference_space)
            .field(
                "show_mode",
                &match self.show_mode {
                    ShowMode::Keep => "Keep",
                    ShowMode::Callback {
                        callback: _,
                        relative: _,
                    } => "Callback",
                    ShowMode::DeviceCallback {
                        role_callback: _,
                        pos: _,
                        rot: _,
                    } => "DeviceCallback",
                    ShowMode::Device {
                        role: _,
                        pos: _,
                        rot: _,
                    } => "Device",
                },
            )
            .finish()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct NotificationsOverlay {
    pub framerate: u8,
    pub position: [f32; 3],
    pub resolution: [u32; 2],
    pub size: [f32; 2],
    pub reference_space: ReferenceSpaceT,
}
#[derive(Clone, Copy, Debug)]
pub struct MuteIndicatorOverlay {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub reference_space: ReferenceSpaceT,
}
#[derive(Clone, Debug)]
pub struct MiscSettings {
    // pub app_name: Box<str>,
    // pub xr_sort_order: u32,
}
pub fn parse_reference_space(str: &str) -> Result<ReferenceSpaceT, ConfigParseError> {
    Ok(match str {
        "stage" => ReferenceSpaceT::STAGE,
        "view" => ReferenceSpaceT::VIEW,
        "local" => ReferenceSpaceT::LOCAL,
        _ => {
            return Err(ConfigParseError::NotAReferenceSpace(str.to_string()));
        }
    })
}
pub fn parse_show_mode(str: &str, pos: Vector3f) -> Result<ShowMode, ConfigParseError> {
    Ok(match str {
        "last_controller" => ShowMode::DeviceCallback {
            role_callback: openxr_show_hand,
            pos,
            rot: None,
        },
        "hmd" => ShowMode::Device {
            role: DeviceRole::Hmd,
            pos,
            rot: None,
        },
        "left_controller" => ShowMode::Device {
            role: DeviceRole::RightHand,
            pos,
            rot: None,
        },
        "right_controller" => ShowMode::Device {
            role: DeviceRole::LeftHand,
            pos,
            rot: None,
        },
        "keep" => ShowMode::Keep,
        _ => {
            return Err(ConfigParseError::NotAShowMode(str.to_string()));
        }
    })
}
pub fn parse_fps(current: u8, str: &str) -> Result<u8, ConfigParseError> {
    let (operator, value) = str.split_at(1);
    let value = value
        .parse::<f32>()
        .map_err(|_| ConfigParseError::FPSNotANumber(value.to_string()))?;
    Ok(match operator {
        "+" => current + value as u8,
        "-" => current - value as u8,
        "*" => (current as f32 * value) as u8,
        "/" => (current as f32 / value) as u8,
        _ => {
            return Err(ConfigParseError::FPSNotAnOperator(operator.to_string()));
        }
    })
}
// pub enumu
#[derive(Clone, Debug)]
pub enum ConfigParseError {
    NotAShowMode(String),
    NotAReferenceSpace(String),
    FPSNotAnOperator(String),
    FPSNotANumber(String),
    FailedToParseToml,
}
