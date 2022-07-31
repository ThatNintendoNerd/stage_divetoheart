use std::sync::atomic::{AtomicBool, Ordering};

use atomic::Atomic;
use hash40::{hash40, Hash40};
use ninput::{any, Buttons};
use once_cell::sync::Lazy;
use skyline::{hook, patching::nop_data};
use smash::app;
use smash_stage::{
    common::{StageInfo, StageID},
    stages::trail_castle::{StageData, StageDataVisual},
};

mod config;
mod offsets;
mod scene;

use config::CONFIG;
use scene::Scene;
use offsets::{
    GET_SECRET_COMMAND_ID_OFFSET,
    TRAIL_CASTLE_GIMMICK_OFF_CHECK_OFFSET,
    TRAIL_CASTLE_PRELOAD_OFFSET,
    TRAIL_CASTLE_STDAT_PARSE_OFFSET,
    TRAIL_CASTLE_VISUAL_STDAT_PARSE_OFFSET,
};

static IS_PATCH_STAGE_DATA: AtomicBool = AtomicBool::new(false);
static SCENE: Atomic<Scene> = Atomic::new(Scene::Random);

#[hook(offset = GET_SECRET_COMMAND_ID_OFFSET)]
fn get_scene_from_input(ui_stage_db: &u64, ui_stage_id: Hash40, is_joycon: bool) {
    original!()(ui_stage_db, ui_stage_id, is_joycon);

    if (ui_stage_id.0 & 0xFFFFFFFFFF) == hash40("ui_stage_trail_castle").0 {
        if any::is_down(Buttons::L) && any::is_down(Buttons::UP) {
            SCENE.store(Scene::Riku, Ordering::Relaxed);
        } else if any::is_down(Buttons::L) && any::is_down(Buttons::LEFT) {
            SCENE.store(Scene::Roxas, Ordering::Relaxed);
        } else if any::is_down(Buttons::L) && any::is_down(Buttons::DOWN) {
            SCENE.store(Scene::Xion, Ordering::Relaxed);
        } else if any::is_down(Buttons::L) {
            SCENE.store(Scene::Sora, Ordering::Relaxed);
        } else if any::is_down(Buttons::R) && any::is_down(Buttons::UP) {
            SCENE.store(Scene::Terra, Ordering::Relaxed);
        } else if any::is_down(Buttons::R) && any::is_down(Buttons::RIGHT) {
            SCENE.store(Scene::Ventus, Ordering::Relaxed);
        } else if any::is_down(Buttons::R) && any::is_down(Buttons::DOWN) {
            SCENE.store(Scene::Aqua, Ordering::Relaxed);
        } else {
            SCENE.store(Scene::Random, Ordering::Relaxed);
        }
    }
}

#[hook(offset = TRAIL_CASTLE_PRELOAD_OFFSET)]
fn trail_castle_preload_hook(param_1: &u64, stage_info: &mut StageInfo) {
    let stage_id = (*stage_info).get_stage_id();

    // Only patch the stage data files if the stage should start on Dive to the Heart.
    // If the Hazards Off override flag is set for the corresponding stage form, force Hazards On behavior.
    IS_PATCH_STAGE_DATA.store(match stage_id {
        StageID::Trail_Castle => {
            if !(*stage_info).is_gimmick_on {
                if CONFIG.hazards_off_override.normal {
                    (*stage_info).is_gimmick_on = CONFIG.hazards_off_override.normal;
                }

                CONFIG.default_location.normal.is_dth_default() & CONFIG.hazards_off_override.normal
            } else {
                CONFIG.default_location.normal.is_dth_default()
            }
        },
        StageID::End_Trail_Castle => {
            if !(*stage_info).is_gimmick_on {
                if CONFIG.hazards_off_override.end {
                    (*stage_info).is_gimmick_on = CONFIG.hazards_off_override.end;
                }

                CONFIG.default_location.end.is_dth_default() & CONFIG.hazards_off_override.end
            } else {
                CONFIG.default_location.end.is_dth_default()
            }
        },
        StageID::Battle_Trail_Castle => {
            if !(*stage_info).is_gimmick_on {
                if CONFIG.hazards_off_override.battle {
                    (*stage_info).is_gimmick_on = CONFIG.hazards_off_override.battle;
                }

                CONFIG.default_location.battle.is_dth_default() & CONFIG.hazards_off_override.battle
            } else {
                CONFIG.default_location.battle.is_dth_default()
            }
        },
        _ => Default::default()
    }, Ordering::Relaxed);

    // If the station was not overriden by controller input, use the configuration's fallback station.
    // The fallback station may still be Random.
    if SCENE.load(Ordering::Relaxed) == Scene::Random {
        SCENE.store(match stage_id {
            StageID::Trail_Castle => CONFIG.default_station.normal,
            StageID::End_Trail_Castle => CONFIG.default_station.end,
            StageID::Battle_Trail_Castle => CONFIG.default_station.battle,
            _ => Default::default()
        }, Ordering::Relaxed);
    }

    // Apply the desired station if it was not randomly picked.
    // Otherwise, game logic will take care of randomly picking a station for us.
    if SCENE.load(Ordering::Relaxed) != Scene::Random {
        (*stage_info).stage_basic_setting.trail_castle = SCENE.load(Ordering::Relaxed).into();
    }

    // If the current game mode is Training and the stage should start on Dive to the Heart,
    // then make the stage start on an already-transitioned Dive to the Heart.
    if app::smashball::is_training_mode() && IS_PATCH_STAGE_DATA.load(Ordering::Relaxed) {
        (*stage_info).stage_additional_setting.trail_castle.is_start_on_dth = true;
    }

    original!()(param_1, stage_info);
}

#[hook(offset = TRAIL_CASTLE_STDAT_PARSE_OFFSET - 0x28)]
fn trail_castle_stdat_parse_hook(stage_data: &mut StageData, param_2: &u64) {
    original!()(stage_data, param_2);

    if IS_PATCH_STAGE_DATA.load(Ordering::Relaxed) && !app::smashball::is_training_mode() {
        stage_data.climax_delay_seconds = 0.0;
        stage_data.x22d0896d0b = f32::INFINITY;
        stage_data.x1fcd0859fd = u32::MAX;
        stage_data.x1f230638d1 = u32::MAX;
        stage_data.x1fbbcd035c = u32::MAX;
        stage_data.x1c316d7eab = u32::MAX;
        stage_data.x1cdf631f87 = u32::MAX;
        stage_data.x1ca8642f11 = u32::MAX;
        stage_data.x1c47a8240a = u32::MAX;
        stage_data.x1cdea175b0 = u32::MAX;
    }
}

#[hook(offset = TRAIL_CASTLE_VISUAL_STDAT_PARSE_OFFSET - 0x48)]
fn trail_castle_visual_stdat_parse_hook(stage_data_visual: &mut StageDataVisual, param_2: &u64) {
    original!()(stage_data_visual, param_2);

    if IS_PATCH_STAGE_DATA.load(Ordering::Relaxed) && !app::smashball::is_training_mode() {
        stage_data_visual.diving_period = 1.0;
        stage_data_visual.fading_period = 0.0;
        stage_data_visual.x156c914b3c = 0.0;
        stage_data_visual.smoke_fading_period = 0.0;
        stage_data_visual.pipe_fading_delay = 0.0;
        stage_data_visual.pipe_fading_period = 0.0;

        for model in stage_data_visual.climax_visibility_settings.iter_mut() {
            if model.start_frame == 43.0 {
                model.start_frame = 0.0;
            } else if model.end_frame == 42.0 || model.end_frame == 100.0 {
                model.start_frame = -1.0;
                model.end_frame = 0.0;
            }
        }

        for animation in stage_data_visual.climax_animation_settings.iter_mut() {
            if animation.anime_name == hash40("sky_dth") {
                animation.anime_name = hash40("sky_spirits");
                break;
            }
        }
    }
}

#[skyline::main(name = "stage_divetoheart")]
fn main() {
    offsets::search_offsets();
    skyline::install_hooks!(
        get_scene_from_input,
        trail_castle_preload_hook,
        trail_castle_stdat_parse_hook,
        trail_castle_visual_stdat_parse_hook,
    );
    unsafe {
        nop_data(TRAIL_CASTLE_GIMMICK_OFF_CHECK_OFFSET).unwrap_or_default();
    }
    Lazy::force(&CONFIG);
}
