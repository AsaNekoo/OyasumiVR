use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use glam::Vec2;
use tokio::sync::Mutex;
use xr_overlay::{
    input::{
        model::{BindingCreateInfo, InputHandlerActionSetCreateInfo},
        InputHandler,
    },
    model::AppContext,
    openxr::Vulkan,
    runner::{actions::ProfilesMetaSwitchAction, builders::parse_config},
    xr_input::{self, ActionType},
};
const DETECTION_SET: usize = 0;
// const MIC_SET: usize = 0;
pub static INPUT_CONTEXT: Mutex<Option<(ProfilesMetaSwitchAction, InputHandler<Vulkan>)>> =
    Mutex::const_new(None);
pub fn get_input_handlers(
    ctx: AppContext<Vulkan>,
) -> Option<(ProfilesMetaSwitchAction, InputHandler<Vulkan>)> {
    const DEFAULT_BINDINGS_CONFIG: &str =
        include_str!("../../../../bindings_config.toml");

        if  !PathBuf::from("bindings_config.toml").exists()  &&!PathBuf::from("src-core").exists() {
            fs::write("bindings_config.toml", DEFAULT_BINDINGS_CONFIG).unwrap();
        }
    let config = match PathBuf::from("src-core").exists() {
        true => DEFAULT_BINDINGS_CONFIG.to_string(),
        false => read_to_string("bindings_config.toml").unwrap(),
    };

    let config = parse_config(config, Some(|k| ["main", "overlay"].contains(&k)));
    if let Err(err) = &config {
        log::error!("failed to load bindings config: {:#?}", err);
        return None;
    }
    let config = config.unwrap();
    let mic_actions = config
        .profiles
        .into_iter()
        .filter_map(|mut p| p.actions.remove("microphone_switch").map(|s| (p.name, s)))
        .filter_map(|s| match s.1 {
            xr_overlay::runner::builders::ParsedAction::MetaSwitch(meta_switch_action) => {
                Some((s.0, meta_switch_action))
            }
            xr_overlay::runner::builders::ParsedAction::Disabled => None,
            xr_overlay::runner::builders::ParsedAction::Linear(linear_action) => {
                log::warn!(
                    "wrong config format for: {:?}:{:?} skipping",
                    s.0,
                    linear_action.source()
                );
                None
            }
            xr_overlay::runner::builders::ParsedAction::PosefSouce(input_source) => {
                log::warn!(
                    "wrong config format for: {:?}:{:?} skipping",
                    s.0,
                    input_source
                );
                None
            }
            xr_overlay::runner::builders::ParsedAction::Haptic(parsed_haptic) => {
                log::warn!(
                    "wrong config format for: {:?}:{:?} skipping",
                    s.0,
                    parsed_haptic
                );
                None
            }
        })
        .collect::<Vec<_>>();
    let mic_actions = ProfilesMetaSwitchAction::new(&mic_actions);
    let mut manager = InputHandler::new_empty(ctx);
    manager
        .add_action_set(
            [
                //multiple actions set are broken rn
                // InputHandlerActionSetCreateInfo {
                //     actions: mic_actions
                //         .sources()
                //         .iter()
                //         .map(|s| BindingCreateInfo {
                //             binding: *s,
                //             name: None,
                //             localized_name: None,
                //         })
                //         .collect(),
                //     name: Some("oyasumi mic mute".to_string()),
                //     localized_name: None,
                // },
                InputHandlerActionSetCreateInfo {
                    actions: [
                        xr_input::OCULUS_TOUCH,
                        xr_input::OCULUS_TOUCH_PRO,
                        xr_input::PICO_4,
                        xr_input::PICO_NEO3,
                        // xr_input::VIVE,
                        xr_input::VIVE_COSMOS,
                        xr_input::VIVE_FOCUS3,
                        xr_input::KHRONOS_SIMPLE,
                        xr_input::VALVE_INDEX,
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|s| {
                        s.path().ends_with("click")
                            || s.path().ends_with("thumbstick")
                            || s.path().ends_with("thumbstick/x")
                            || s.path().ends_with("thumbstick/y")
                            || s.path().ends_with("trigger/value")
                        //no grip to avoid false positives
                    })
                    .map(|s| BindingCreateInfo {
                        binding: *s,
                        name: None,
                        localized_name: None,
                    })
                    .collect::<Vec<BindingCreateInfo>>(),
                    name: Some("oyasumi vr activity detection".to_owned()),
                    localized_name: Some("oyasumi vr activity detection".to_owned()),
                }
                
            ]
            .to_vec(),
        )
        .unwrap();
    debug_assert!(!manager.actions().is_empty());

    // String:
    Some((mic_actions, manager))
}
pub fn check_user_activity(handler: &mut InputHandler<Vulkan>) -> Option<bool> {
    let changed = handler
        .update_specfic(xr_overlay::input::ActionSetsToUpdate::Specific(&[DETECTION_SET]))
        .ok()?;
    for c in changed {
        if c.action_type() == ActionType::Bool {
            if handler.get_bool(*c) {
                log::debug!("detected a press of:{:?}", c);
                return Some(true);
            }
        } else if c.action_type() == ActionType::F32 {
            if handler.get_f32(*c).abs() >= 0.9 {
                log::debug!("detected a press of:{:?}", c);
                return Some(true);
            }
        } else if c.action_type() == ActionType::Vector2f {
            let vec = handler.get_vector2f(*c);
            let vec = Vec2::new(vec.x, vec.y);
            //avoid calculating square root, for some reason compile doesnt do that on it's onw
            //https://godbolt.org/z/74e4qvfr7
            if vec.length_squared() >= 0.9_f32.powi(2) {
                log::debug!("detected a press of:{:?}", c);
                return Some(true);
            }
        }
    }
    Some(false)
}
