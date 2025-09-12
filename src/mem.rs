use skyline::patching::Patch;

use crate::{config::Config, offsets::Offsets};

/// Overwrites the instruction to branch when stage hazards are disabled on the Hollow Bastion stage with a no-op.
fn patch_trail_castle_gimmick_off_check() {
    let override_gimmick_off = &Config::get().override_gimmick_off;

    if !override_gimmick_off.normal && !override_gimmick_off.battle && !override_gimmick_off.end {
        return;
    }

    let offset = Offsets::get().trail_castle_set_random_settings_gimmick_off_check;

    Patch::in_text(offset).nop().unwrap();
}

/// Writes all the memory patches.
pub fn write() {
    patch_trail_castle_gimmick_off_check();
}
