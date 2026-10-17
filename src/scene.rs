use libc2::app::stage::trail_castle;
use ninput::Controller;
use serde::{Deserialize, Serialize};

use crate::command::StageSelectSecretCommand;

/// A stained-glass design.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum Scene {
    /// A random stained-glass design.
    #[default]
    Random,

    /// The stained-glass design for Sora.
    Sora,

    /// The stained-glass design for Riku.
    Riku,

    /// The stained-glass design for Roxas.
    Roxas,

    /// The stained-glass design for Xion.
    Xion,

    /// The stained-glass design for Terra.
    Terra,

    /// The stained-glass design for Ventus.
    Ventus,

    /// The stained-glass design for Aqua.
    Aqua,
}

impl Scene {
    /// Returns a stained-glass design based on any one controller's held buttons.
    pub fn from_any_controller() -> Self {
        const NPAD_ID_HANDHELD: u32 = 0x20;
        const NPAD_ID_MAX: u32 = 0x8;

        if let Some(controller) = Controller::get_from_id(NPAD_ID_HANDHELD) {
            let scene = Self::from(&controller);

            if scene != Self::Random {
                return scene;
            }
        }

        for npad_id in 0..NPAD_ID_MAX {
            if let Some(controller) = Controller::get_from_id(npad_id) {
                let scene = Self::from(&controller);

                if scene != Self::Random {
                    return scene;
                }
            }
        }

        Self::Random
    }
}

impl From<&Controller> for Scene {
    fn from(controller: &Controller) -> Self {
        let secret_command = StageSelectSecretCommand::from(controller);

        secret_command.scene(controller.buttons)
    }
}

impl From<Scene> for trail_castle::Scene {
    fn from(scene: Scene) -> Self {
        match scene {
            Scene::Random => Self::Random,
            Scene::Sora => Self::Sora,
            Scene::Riku => Self::Riku,
            Scene::Roxas => Self::Roxas,
            Scene::Xion => Self::Xion,
            Scene::Terra => Self::Terra,
            Scene::Ventus => Self::Ventus,
            Scene::Aqua => Self::Aqua,
        }
    }
}
