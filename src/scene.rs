use serde::{Deserialize, Serialize};
use smash_stage::stages::trail_castle;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[repr(u8)]
pub enum Scene {
    Random,
    Sora,
    Riku,
    Roxas,
    Xion,
    Terra,
    Ventus,
    Aqua,
}

impl Default for Scene {
    fn default() -> Self {
        Self::Random
    }
}

impl Into<trail_castle::Scene> for Scene {
    fn into(self) -> trail_castle::Scene {
        match self {
            Self::Random => trail_castle::Scene::Common,
            Self::Sora => trail_castle::Scene::Sora,
            Self::Riku => trail_castle::Scene::Riku,
            Self::Roxas => trail_castle::Scene::Roxas,
            Self::Xion => trail_castle::Scene::Xion,
            Self::Terra => trail_castle::Scene::Terra,
            Self::Ventus => trail_castle::Scene::Ventus,
            Self::Aqua => trail_castle::Scene::Aqua,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Location(pub String);

impl Location {
    pub fn is_dth_default(&self) -> bool {
        match self.0.as_str() {
            "Dive to the Heart" => true,
            _ => false
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self("Hollow Bastion".to_string())
    }
}
