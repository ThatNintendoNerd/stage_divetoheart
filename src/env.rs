use semver::Version;

/// The minimum supported version of the target application.
pub const APP_VERSION_MIN: Version = Version::new(13, 0, 0);

/// Returns the version of the target application.
pub fn app_version() -> Version {
    let display_version = display_version();

    Version::parse(&display_version).expect("application version should use semantic versioning")
}

/// Returns the display version of the target application.
fn display_version() -> String {
    use skyline::nn;

    let mut display_version = nn::oe::DisplayVersion { name: [0x00; 16] };

    unsafe {
        nn::oe::GetDisplayVersion(&mut display_version);

        skyline::from_c_str(display_version.name.as_ptr())
    }
}
