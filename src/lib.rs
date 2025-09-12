mod command;
mod config;
mod dth;
mod env;
mod hooks;
mod location;
mod mem;
mod offsets;
mod scene;

#[skyline::main(name = "stage_divetoheart")]
fn main() {
    if !env::is_app_version_compatible() {
        println!(
            "stage_divetoheart is not compatible with the installed version of this software."
        );
        return;
    }

    hooks::install();
    mem::write();
}
