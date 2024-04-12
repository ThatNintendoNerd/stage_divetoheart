mod command;
mod config;
mod controller;
mod env;
mod hooks;
mod location;
mod offsets;
mod scene;

#[skyline::main(name = "stage_divetoheart")]
fn main() {
    use skyline::patching::Patch;

    use config::Config;
    use offsets::Offsets;

    if env::app_version() < env::APP_VERSION_MIN {
        return;
    }

    hooks::install();

    if !Config::get().is_override_gimmick_off() {
        return;
    }

    Patch::in_text(Offsets::get().trail_castle_set_random_settings_gimmick_off_check)
        .nop()
        .unwrap();
}
