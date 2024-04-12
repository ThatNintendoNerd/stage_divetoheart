use parking_lot::Mutex;
use smash_stage::app::{
    stage::trail_castle::{StageData, StageDataVisual},
    GlobalParameter, GlobalStageParameter,
};

use crate::{config::Config, location::Location, scene::Scene};

/// The controller for configuring the behavior of Dive to the Heart on the Hollow Bastion stage.
pub struct DiveToHeartController {
    /// Determines if the stage should enable Dive to the Heart.
    is_climax: bool,

    /// Determines if the stage should start on Dive to the Heart.
    is_force_climax: bool,

    /// Determines if the current melee mode is Training.
    is_training_mode: bool,

    /// The working stained-glass design.
    scene: Scene,

    /// The working random seed.
    seed: u32,
}

impl DiveToHeartController {
    /// Constructs a new instance of `DiveToHeartController`.
    const fn new() -> Self {
        Self {
            is_climax: false,
            is_force_climax: false,
            is_training_mode: false,
            scene: Scene::Random,
            seed: 0,
        }
    }

    /// Returns a reference to a `Mutex` containing the current instance of `DiveToHeartController`.
    pub fn get() -> &'static Mutex<Self> {
        static INSTANCE: Mutex<DiveToHeartController> = Mutex::new(DiveToHeartController::new());

        &INSTANCE
    }

    /// Updates the state of both `self` and the stage's settings based on the current game state.
    pub fn update(&mut self, stage_parameter: &mut GlobalStageParameter) {
        self.update_state(stage_parameter);
        self.update_stage_settings(stage_parameter);
    }

    /// Updates the state of `self` based on the current game state.
    fn update_state(&mut self, stage_parameter: &mut GlobalStageParameter) {
        use smash_stage::app::StageID;

        let config = Config::get();

        match stage_parameter.stage_id() {
            StageID::Trail_Castle => self.update_state_from_param(
                stage_parameter.is_gimmick,
                config.override_gimmick_off.normal,
                config.override_gimmick_on.normal,
                config.default_location.normal,
                config.default_scene.normal,
            ),
            StageID::End_Trail_Castle => self.update_state_from_param(
                stage_parameter.is_gimmick,
                config.override_gimmick_off.end,
                config.override_gimmick_on.end,
                config.default_location.end,
                config.default_scene.end,
            ),
            StageID::Battle_Trail_Castle => self.update_state_from_param(
                stage_parameter.is_gimmick,
                config.override_gimmick_off.battle,
                config.override_gimmick_on.battle,
                config.default_location.battle,
                config.default_scene.battle,
            ),
            _ => return,
        }

        self.is_training_mode = GlobalParameter::global_melee_parameter().is_training_mode();
    }

    /// Updates the state of `self` based on select variables from both the configuration and current game state.
    fn update_state_from_param(
        &mut self,
        is_gimmick: bool,
        override_gimmick_off: bool,
        override_gimmick_on: bool,
        default_location: Location,
        default_scene: Scene,
    ) {
        self.is_climax =
            Self::is_valid_climax(is_gimmick, override_gimmick_off, override_gimmick_on);
        self.is_force_climax = self.is_valid_force_climax(default_location);

        if self.scene == Scene::Random {
            self.scene = default_scene;
        }
    }

    /// Updates the state of the stage's settings based on the current game state.
    fn update_stage_settings(&self, stage_parameter: &mut GlobalStageParameter) {
        stage_parameter.is_gimmick = self.is_climax;

        if !self.is_climax {
            return;
        }

        if self.scene != Scene::Random {
            stage_parameter.random_settings.trail_castle.scene = self.scene.into();
        }

        if !self.is_force_climax {
            return;
        }

        if self.is_training_mode {
            stage_parameter
                .additional_settings
                .trail_castle
                .is_post_climax = true;
        }
    }

    /// Returns `true` if the stage should enable Dive to the Heart based on the given arguments.
    fn is_valid_climax(
        is_gimmick: bool,
        override_gimmick_off: bool,
        override_gimmick_on: bool,
    ) -> bool {
        if is_gimmick {
            !override_gimmick_on
        } else {
            override_gimmick_off
        }
    }

    /// Returns `true` if the stage should start on Dive to the Heart based on the given arguments.
    fn is_valid_force_climax(&self, default_location: Location) -> bool {
        if !self.is_climax {
            return false;
        }

        self.location(default_location) == Location::DiveToHeart
    }

    /// Returns the starting location of the stage.
    fn location(&self, default_location: Location) -> Location {
        use smash_stage::app::StageRandom;

        if default_location != Location::Random {
            return default_location;
        }

        if StageRandom::new(self.seed).next_bool() {
            return Location::DiveToHeart;
        }

        Location::HollowBastion
    }

    /// Returns `true` if the stage should start on Dive to the Heart.
    pub fn is_force_climax(&self) -> bool {
        self.is_force_climax
    }

    /// Sets the working stained-glass design.
    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = scene;
    }

    /// Sets the working random seed.
    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }

    /// Returns `true` if the stage is being played on in a mode unsupported by this plugin.
    pub fn is_invalid_melee_mode() -> bool {
        use smash_stage::app::MeleeMode;

        matches!(
            GlobalParameter::global_melee_parameter().melee_mode(),
            MeleeMode::Standard
                | MeleeMode::StandardMulti
                | MeleeMode::SpiritsBattle
                | MeleeMode::SpiritsBattleMulti
        )
    }

    /// Sets all variables that are read from within update loops to their respective default state.
    pub fn reset(&mut self) {
        self.is_force_climax = Default::default();
    }

    /// Returns `true` if values in both the `StageData` and `StageDataVisual` instances should be overwritten.
    pub fn is_patch_stage_data(&self) -> bool {
        self.is_force_climax && !self.is_training_mode
    }

    /// Overwrites values in the given instance of `StageData` to start the stage on Dive to the Heart.
    pub fn patch_stage_data(stage_data: &mut StageData) {
        stage_data.root.climax_delay_seconds = 0.0;
    }

    /// Overwrites values in the given instance of `StageDataVisual` to start the stage on Dive to the Heart.
    pub fn patch_stage_data_visual(stage_data_visual: &mut StageDataVisual) {
        use hash40::Hash40;

        stage_data_visual.root.diving_period = 1.0;
        stage_data_visual.root.fading_period = 0.0;
        stage_data_visual.root.x156c914b3c = 0.0;
        stage_data_visual.root.smoke_fading_period = 0.0;
        stage_data_visual.root.pipe_fading_delay = 0.0;
        stage_data_visual.root.pipe_fading_period = 0.0;

        for model in stage_data_visual.root.climax_visibility_settings.iter_mut() {
            if model.start_frame == 43.0 {
                model.start_frame = 0.0;
            } else if model.end_frame == 42.0 || model.end_frame == 100.0 {
                model.start_frame = -1.0;
                model.end_frame = 0.0;
            }
        }

        for animation in stage_data_visual.root.climax_animation_settings.iter_mut() {
            if animation.anime_name == Hash40::new("sky_dth") {
                animation.anime_name = Hash40::new("sky_spirits");
                break;
            }
        }
    }
}
