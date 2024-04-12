use skyline::hooks::InlineCtx;
use smash_stage::{
    app::{
        stage::trail_castle::{StageData, StageDataVisual},
        GlobalStageParameter, SecretCommandID,
    },
    lib::ParameterResource,
};

use crate::{controller::DiveToHeartController, offsets::Offsets, scene::Scene};

#[skyline::hook(offset = Offsets::get().trail_castle_set_random_settings, inline)]
fn trail_castle_set_random_settings(ctx: &InlineCtx) {
    let seed = unsafe { *ctx.registers[20].w.as_ref() };

    DiveToHeartController::get().lock().set_seed(seed);
}

#[skyline::hook(offset = Offsets::get().trail_castle_resource_preload)]
fn trail_castle_resource_preload(arg1: usize, stage_parameter: &mut GlobalStageParameter) {
    if DiveToHeartController::is_invalid_melee_mode() {
        DiveToHeartController::get().lock().reset();
    } else {
        DiveToHeartController::get().lock().update(stage_parameter);
    }

    original!()(arg1, stage_parameter);
}

#[skyline::hook(offset = Offsets::get().trail_castle_climax_rule_check, inline)]
fn trail_castle_climax_rule_check(ctx: &InlineCtx) {
    if DiveToHeartController::get().lock().is_force_climax() {
        unsafe {
            let is_climax = *ctx.registers[23].x.as_ref() as *mut bool;

            *is_climax = true;
        }
    }
}

#[skyline::hook(offset = Offsets::get().trail_castle_stage_data_read)]
fn trail_castle_stage_data_read(stage_data: &mut StageData, resource: &ParameterResource) {
    original!()(stage_data, resource);

    if DiveToHeartController::get().lock().is_patch_stage_data() {
        DiveToHeartController::patch_stage_data(stage_data);
    }
}

#[skyline::hook(offset = Offsets::get().trail_castle_stage_data_visual_read)]
fn trail_castle_stage_data_visual_read(
    stage_data_visual: &mut StageDataVisual,
    resource: &ParameterResource,
) {
    original!()(stage_data_visual, resource);

    if DiveToHeartController::get().lock().is_patch_stage_data() {
        DiveToHeartController::patch_stage_data_visual(stage_data_visual);
    }
}

#[skyline::hook(offset = Offsets::get().get_secret_command_id)]
fn get_secret_command_id(ui_stage_db: usize, ui_stage_id: u64, is_joycon: bool) -> SecretCommandID {
    use hash40::Hash40;

    if Hash40(ui_stage_id & 0xFFFFFFFFFF) == Hash40::new("ui_stage_trail_castle") {
        let scene = Scene::from_any_controller();

        DiveToHeartController::get().lock().set_scene(scene);
    }

    original!()(ui_stage_db, ui_stage_id, is_joycon)
}

/// Installs all the function hooks and inline hooks.
pub fn install() {
    skyline::install_hooks!(
        trail_castle_set_random_settings,
        trail_castle_resource_preload,
        trail_castle_climax_rule_check,
        trail_castle_stage_data_read,
        trail_castle_stage_data_visual_read,
        get_secret_command_id,
    );
}
