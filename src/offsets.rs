use std::sync::LazyLock;

use crate::env;

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
        let text = unsafe { env::text() };

        Self {
            trail_castle_set_random_settings: TRAIL_CASTLE_SET_RANDOM_SETTINGS_NEEDLE
                .find(text)
                .unwrap(),
            trail_castle_set_random_settings_gimmick_off_check:
                TRAIL_CASTLE_SET_RANDOM_SETTINGS_GIMMICK_OFF_CHECK_NEEDLE
                    .find(text)
                    .unwrap(),
            trail_castle_resource_preload: TRAIL_CASTLE_RESOURCE_PRELOAD_NEEDLE.find(text).unwrap(),
            trail_castle_climax_rule_check: TRAIL_CASTLE_CLIMAX_RULE_CHECK_NEEDLE
                .find(text)
                .unwrap(),
            trail_castle_stage_data_read: TRAIL_CASTLE_STAGE_DATA_READ_NEEDLE.find(text).unwrap(),
            trail_castle_stage_data_visual_read: TRAIL_CASTLE_STAGE_DATA_VISUAL_READ_NEEDLE
                .find(text)
                .unwrap(),
            get_secret_command_id: GET_SECRET_COMMAND_ID_NEEDLE.find(text).unwrap(),
        }
    }

    /// Returns a reference to a `LazyLock` containing the current instance of `Offsets`.
    pub const fn get() -> &'static LazyLock<Self> {
        static INSTANCE: LazyLock<Offsets> = LazyLock::new(Offsets::new);

        &INSTANCE
    }
}

/// A substring with an offset to displace the resulting index.
struct Needle {
    /// The byte slice to search for.
    bytes: &'static [u8],

    /// The offset of the resulting index, relative to the start of the byte slice.
    offset: isize,
}

impl Needle {
    /// Returns the index to the needle in the haystack.
    fn find(&self, haystack: &[u8]) -> Result<usize, FindNeedleError> {
        use memchr::memmem;

        let (Some(index), Some(rindex)) = (
            memmem::find(haystack, self.bytes),
            memmem::rfind(haystack, self.bytes),
        ) else {
            return Err(FindNeedleError::NotFound);
        };

        if index != rindex {
            return Err(FindNeedleError::MultipleResults { index, rindex });
        }

        let Some(index) = index.checked_add_signed(self.offset) else {
            return Err(FindNeedleError::Overflow);
        };

        Ok(index)
    }
}

/// An error returned from searching for a needle in a haystack.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum FindNeedleError {
    /// The needle was not found in the haystack.
    NotFound,

    /// The needle was found multiple times in the haystack.
    MultipleResults {
        /// The index of the first occurrence of a needle.
        index: usize,

        /// The index of the last occurrence of a needle.
        rindex: usize,
    },

    /// The resulting index overflowed when adding the offset to it.
    Overflow,
}

static TRAIL_CASTLE_SET_RANDOM_SETTINGS_NEEDLE: Needle = Needle {
    bytes: &[
        0x68, 0xBF, 0x41, 0x39, // ldrb w8, [x27, #0x6f]
        0x88, 0x21, 0x00, 0x34, // cbz w8, #0x434
    ],
    offset: 0x0,
};

static TRAIL_CASTLE_SET_RANDOM_SETTINGS_GIMMICK_OFF_CHECK_NEEDLE: Needle = Needle {
    bytes: &[
        0x68, 0x43, 0x40, 0x39, // ldrb w8, [x27, #0x10]
        0xC8, 0x58, 0x00, 0x34, // cbz w8, #0xb1c
    ],
    offset: 0x4,
};

static TRAIL_CASTLE_RESOURCE_PRELOAD_NEEDLE: Needle = Needle {
    bytes: &[
        0x14, 0x79, 0x97, 0xD2, // mov x20, #0xbbc8
        0x14, 0x30, 0xA6, 0xF2, // movk x20, #0x3180, lsl #16
        0x02, 0x16, 0x80, 0x52, // mov w2, #0xb0
    ],
    offset: -0x1C,
};

static TRAIL_CASTLE_CLIMAX_RULE_CHECK_NEEDLE: Needle = Needle {
    bytes: &[
        0xE8, 0x02, 0x40, 0x39, // ldrb w8, [x23]
        0x08, 0x02, 0x00, 0x35, // cbnz w8, #0x44
    ],
    offset: 0x0,
};

static TRAIL_CASTLE_STAGE_DATA_READ_NEEDLE: Needle = Needle {
    bytes: &[
        0x6F, 0xFD, 0x8F, 0xD2, // mov x15, #0x7feb
        0x2F, 0xA4, 0xA5, 0xF2, // movk x15, #0x2d21, lsl #16
        0x8F, 0x02, 0xC0, 0xF2, // movk x15, #0x14, lsl #32
    ],
    offset: -0x28,
};

static TRAIL_CASTLE_STAGE_DATA_VISUAL_READ_NEEDLE: Needle = Needle {
    bytes: &[
        0xCE, 0x4F, 0x98, 0xD2, // mov x14, #0xc27e
        0x0E, 0x19, 0xB4, 0xF2, // movk x14, #0xa0c8, lsl #16
        0xAE, 0x01, 0xC0, 0xF2, // movk x14, #0xd, lsl #32
    ],
    offset: -0x48,
};

static GET_SECRET_COMMAND_ID_NEEDLE: Needle = Needle {
    bytes: &[
        0xFF, 0x83, 0x01, 0xD1, // sub sp, sp, #0x60
        0xF9, 0x0B, 0x00, 0xF9, // str x25, [sp, #0x10]
        0xF8, 0x5F, 0x02, 0xA9, // stp x24, x23, [sp, #0x20]
        0xF6, 0x57, 0x03, 0xA9, // stp x22, x21, [sp, #0x30]
        0xF4, 0x4F, 0x04, 0xA9, // stp x20, x19, [sp, #0x40]
        0xFD, 0x7B, 0x05, 0xA9, // stp x29, x30, [sp, #0x50]
        0xFD, 0x43, 0x01, 0x91, // add x29, sp, #0x50
        0x28, 0x1C, 0x48, 0x92, // and x8, x1, #0xff00000000000000
        0x09, 0x20, 0xED, 0xD2, // mov x9, #0x6900000000000000
    ],
    offset: 0x0,
};
