use skyline::hooks::{getRegionAddress, Region};

// Default 13.0.1 offsets
pub static mut GET_SECRET_COMMAND_ID_OFFSET: usize = 0x32B2840;
pub static mut TRAIL_CASTLE_GIMMICK_OFF_CHECK_OFFSET: usize = 0x177ACF0;
pub static mut TRAIL_CASTLE_PRELOAD_OFFSET: usize = 0x2F6CB90;
pub static mut TRAIL_CASTLE_STDAT_PARSE_OFFSET: usize = 0x2F77600;
pub static mut TRAIL_CASTLE_VISUAL_STDAT_PARSE_OFFSET: usize = 0x2F7C260;

static GET_SECRET_COMMAND_ID_SEARCH_CODE: &[u8] = &[
    0xFF, 0x83, 0x01, 0xD1,
    0xF9, 0x0B, 0x00, 0xF9,
    0xF8, 0x5F, 0x02, 0xA9,
    0xF6, 0x57, 0x03, 0xA9,
    0xF4, 0x4F, 0x04, 0xA9,
    0xFD, 0x7B, 0x05, 0xA9,
    0xFD, 0x43, 0x01, 0x91,
    0x28, 0x1C, 0x48, 0x92,
    0x09, 0x20, 0xED, 0xD2,
];

static TRAIL_CASTLE_GIMMICK_OFF_CHECK_SEARCH_CODE: &[u8] = &[
    0xC8, 0x58, 0x00, 0x34,
    0x55, 0xCA, 0x01, 0xD0,
];

static TRAIL_CASTLE_PRELOAD_SEARCH_CODE: &[u8] = &[
    0xFF, 0xC3, 0x06, 0xD1,
    0xFC, 0x67, 0x16, 0xA9,
    0xF8, 0x5F, 0x17, 0xA9,
    0xF6, 0x57, 0x18, 0xA9,
    0xF4, 0x4F, 0x19, 0xA9,
    0xFD, 0x7B, 0x1A, 0xA9,
    0xFD, 0x83, 0x06, 0x91,
    0x14, 0x79, 0x97, 0xD2,
    0x14, 0x30, 0xA6, 0xF2,
];

static TRAIL_CASTLE_STDAT_PARSE_SEARCH_CODE: &[u8] = &[
    0x6F, 0xFD, 0x8F, 0xD2,
    0x2F, 0xA4, 0xA5, 0xF2,
    0x8F, 0x02, 0xC0, 0xF2,
];

static TRAIL_CASTLE_VISUAL_STDAT_PARSE_SEARCH_CODE: &[u8] = &[
    0xCE, 0x4F, 0x98, 0xD2,
    0x0E, 0x19, 0xB4, 0xF2,
    0xAE, 0x01, 0xC0, 0xF2,
];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

macro_rules! find_offsets {
    (
        $(
            ($out_variable:expr, $search_pattern:expr)
        ),*
        $(,)?
    ) => {
        $(
            unsafe {
                let text_ptr = getRegionAddress(Region::Text) as *const u8;
                let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
                let text = std::slice::from_raw_parts(text_ptr, text_size);

                if let Some(offset) = find_subsequence(text, $search_pattern) {
                    $out_variable = offset;
                } else {
                    println!("[stage_divetoheart::offsets] Failed to find offset for '{}'. Defaulting to 13.0.1 offset...", stringify!($out_variable));
                }
            }
        )*
    };
}

pub fn search_offsets() {
    find_offsets! (
        (GET_SECRET_COMMAND_ID_OFFSET, GET_SECRET_COMMAND_ID_SEARCH_CODE),
        (TRAIL_CASTLE_GIMMICK_OFF_CHECK_OFFSET, TRAIL_CASTLE_GIMMICK_OFF_CHECK_SEARCH_CODE),
        (TRAIL_CASTLE_PRELOAD_OFFSET, TRAIL_CASTLE_PRELOAD_SEARCH_CODE),
        (TRAIL_CASTLE_STDAT_PARSE_OFFSET, TRAIL_CASTLE_STDAT_PARSE_SEARCH_CODE),
        (TRAIL_CASTLE_VISUAL_STDAT_PARSE_OFFSET, TRAIL_CASTLE_VISUAL_STDAT_PARSE_SEARCH_CODE),
    );
}
