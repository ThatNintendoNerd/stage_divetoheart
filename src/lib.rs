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
        eprintln!(
            "[{}] The installed version of the target software is not compatible.",
            module_path!(),
        );
        return;
    }

    hooks::install();
    mem::write();
}
