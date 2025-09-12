use std::ffi::CStr;

use semver::Version;
use skyline::nn;

/// Determines if the version of the target application is supported.
pub fn is_app_version_compatible() -> bool {
    let app_version = app_version();
    let app_version_min = Version::new(13, 0, 0);

    app_version >= app_version_min
}

/// Returns the version of the target application.
pub fn app_version() -> Version {
    let mut display_version = nn::oe::DisplayVersion { name: [b'\0'; 16] };

    unsafe {
        nn::oe::GetDisplayVersion(&mut display_version);
    }

    let text = CStr::from_bytes_until_nul(&display_version.name)
        .expect("trailing nul terminator should exist")
        .to_str()
        .expect("version text should contain valid UTF-8");

    Version::parse(text).expect("application version should use semantic versioning")
}
