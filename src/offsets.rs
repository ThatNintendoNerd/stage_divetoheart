use once_cell::sync::Lazy;

/// The container for cached offsets to code.
pub struct Offsets {
    pub trail_castle_set_random_settings: usize,
    pub trail_castle_set_random_settings_gimmick_off_check: usize,
    pub trail_castle_resource_preload: usize,
    pub trail_castle_climax_rule_check: usize,
    pub trail_castle_stage_data_read: usize,
    pub trail_castle_stage_data_visual_read: usize,
    pub get_secret_command_id: usize,
}

impl Offsets {
    /// Constructs a new instance of `Offsets`.
    fn new() -> Self {
        let text = text();

        Self {
            trail_castle_set_random_settings: Self::find(
                text,
                TRAIL_CASTLE_SET_RANDOM_SETTINGS_SEARCH_CODE,
            )
            .unwrap_or(0x177B364),
            trail_castle_set_random_settings_gimmick_off_check: Self::find(
                text,
                TRAIL_CASTLE_SET_RANDOM_SETTINGS_GIMMICK_OFF_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x177B7C0),
            trail_castle_resource_preload: Self::find(
                text,
                TRAIL_CASTLE_RESOURCE_PRELOAD_SEARCH_CODE,
            )
            .unwrap_or(0x2F6D810),
            trail_castle_climax_rule_check: Self::find(
                text,
                TRAIL_CASTLE_CLIMAX_RULE_CHECK_SEARCH_CODE,
            )
            .unwrap_or(0x2F7380C),
            trail_castle_stage_data_read: Self::find(
                text,
                TRAIL_CASTLE_STAGE_DATA_READ_SEARCH_CODE,
            )
            .unwrap_or(0x2F78280),
            trail_castle_stage_data_visual_read: Self::find(
                text,
                TRAIL_CASTLE_STAGE_DATA_VISUAL_READ_SEARCH_CODE,
            )
            .unwrap_or(0x2F7CEE0),
            get_secret_command_id: Self::find(text, GET_SECRET_COMMAND_ID_SEARCH_CODE)
                .unwrap_or(0x32B34C0),
        }
    }

    /// Returns a reference to a `Lazy` containing the current instance of `Offsets`.
    pub fn get() -> &'static Lazy<Self> {
        static INSTANCE: Lazy<Offsets> = Lazy::new(Offsets::new);

        &INSTANCE
    }

    /// Returns the offset to the needle in the haystack, or `None` if it was not found.
    fn find(haystack: &[u8], needle: (&[u8], isize)) -> Option<usize> {
        use memchr::memmem;

        memmem::find(haystack, needle.0).map(|o| (o as isize + needle.1) as usize)
    }
}

/// Returns a byte slice representing the code segment of the target application.
fn text() -> &'static [u8] {
    use std::slice;

    use skyline::hooks::{getRegionAddress, Region};

    unsafe {
        let ptr = getRegionAddress(Region::Text) as *const u8;
        let len = (getRegionAddress(Region::Rodata) as usize) - (ptr as usize);

        slice::from_raw_parts(ptr, len)
    }
}

#[rustfmt::skip]
static TRAIL_CASTLE_SET_RANDOM_SETTINGS_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x68, 0xBF, 0x41, 0x39,
        0x88, 0x21, 0x00, 0x34,
    ],
    0x00,
);

#[rustfmt::skip]
static TRAIL_CASTLE_SET_RANDOM_SETTINGS_GIMMICK_OFF_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x68, 0x43, 0x40, 0x39,
        0xC8, 0x58, 0x00, 0x34,
    ],
    0x04,
);

#[rustfmt::skip]
static TRAIL_CASTLE_RESOURCE_PRELOAD_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x14, 0x79, 0x97, 0xD2,
        0x14, 0x30, 0xA6, 0xF2,
        0x02, 0x16, 0x80, 0x52,
    ],
    -0x1C,
);

#[rustfmt::skip]
static TRAIL_CASTLE_CLIMAX_RULE_CHECK_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xE8, 0x02, 0x40, 0x39,
        0x08, 0x02, 0x00, 0x35,
    ],
    0x00,
);

#[rustfmt::skip]
static TRAIL_CASTLE_STAGE_DATA_READ_SEARCH_CODE: (&[u8], isize) = (
    &[
        0x6F, 0xFD, 0x8F, 0xD2,
        0x2F, 0xA4, 0xA5, 0xF2,
        0x8F, 0x02, 0xC0, 0xF2,
    ],
    -0x28,
);

#[rustfmt::skip]
static TRAIL_CASTLE_STAGE_DATA_VISUAL_READ_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xCE, 0x4F, 0x98, 0xD2,
        0x0E, 0x19, 0xB4, 0xF2,
        0xAE, 0x01, 0xC0, 0xF2,
    ],
    -0x48,
);

#[rustfmt::skip]
static GET_SECRET_COMMAND_ID_SEARCH_CODE: (&[u8], isize) = (
    &[
        0xFF, 0x83, 0x01, 0xD1,
        0xF9, 0x0B, 0x00, 0xF9,
        0xF8, 0x5F, 0x02, 0xA9,
        0xF6, 0x57, 0x03, 0xA9,
        0xF4, 0x4F, 0x04, 0xA9,
        0xFD, 0x7B, 0x05, 0xA9,
        0xFD, 0x43, 0x01, 0x91,
        0x28, 0x1C, 0x48, 0x92,
        0x09, 0x20, 0xED, 0xD2,
    ],
    0x00,
);
