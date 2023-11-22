// Moved Consts from structs.
pub const CRNHEADER_MIN_SIZE: usize = 62;

pub const CRNSIG_VALUE: u16 = ('H' as u16) << 8 | 'x' as u16;

pub const MAX_EXPECTED_CODE_SIZE: usize = 16;

pub const MAX_SUPPORTED_SYMS: usize = 8192;

pub const MAX_TABLE_BITS: usize = 11;

pub const MAGIC_VALUE: u32 = 0x1EF9CABD;

pub const BIT_BUF_SIZE: usize = 32;

pub const CRNMAX_LEVELS: u32 = 16;

// The crnd library assumes all allocation blocks have at least CRND_MIN_ALLOC_ALIGNMENT alignment.
// pub const CRND_MIN_ALLOC_ALIGNMENT: usize = 8;

// Code length encoding symbols:
// 0-16 - actual code lengths
pub const MAX_CODELENGTH_CODES: usize = 21;

pub const SMALL_ZERO_RUN_CODE: usize = 17;
pub const LARGE_ZERO_RUN_CODE: usize = 18;
pub const SMALL_REPEAT_CODE: usize = 19;
pub const LARGE_REPEAT_CODE: usize = 20;

pub const MIN_SMALL_ZERO_RUN_SIZE: usize = 3;
// pub const cMaxSmallZeroRunSize: usize     = 10;
pub const MIN_LARGE_ZERO_RUN_SIZE: usize = 11;
// pub const cMaxLargeZeroRunSize: usize     = 138;

pub const SMALL_MIN_NON_ZERO_RUN_SIZE: usize = 3;
// pub const cSmallMaxNonZeroRunSize: usize  = 6;
pub const LARGE_MIN_NON_ZERO_RUN_SIZE: usize = 7;
// pub const cLargeMaxNonZeroRunSize: usize  = 70;

pub const SMALL_ZERO_RUN_EXTRA_BITS: usize = 3;
pub const LARGE_ZERO_RUN_EXTRA_BITS: usize = 7;
pub const SMALL_NON_ZERO_RUN_EXTRA_BITS: usize = 2;
pub const LARGE_NON_ZERO_RUN_EXTRA_BITS: usize = 6;

pub const MOST_PROBABLE_CODELENGTH_CODES: [u8; 21] = [
    SMALL_ZERO_RUN_CODE as u8,
    LARGE_ZERO_RUN_CODE as u8,
    SMALL_REPEAT_CODE as u8,
    LARGE_REPEAT_CODE as u8,
    0,
    8,
    7,
    9,
    6,
    10,
    5,
    11,
    4,
    12,
    3,
    13,
    2,
    14,
    1,
    15,
    16,
];

// pub const cNumMostProbableCodelengthCodes: usize = 21;

#[cfg(target_endian = "little")]
pub const CRND_LITTLE_ENDIAN_PLATFORM: bool = true;
// #[cfg(target_endian = "little")]
// pub const c_crnd_big_endian_platform: bool = false;

#[cfg(target_endian = "big")]
pub const CRND_LITTLE_ENDIAN_PLATFORM: bool = false;
// #[cfg(target_endian = "big")]
// pub const c_crnd_big_endian_platform: bool = true;

// pub const cDXTBlockShift: usize = 2;
// pub const cDXTBlockSize: usize = 1 << cDXTBlockShift;
// pub const cDXT1BytesPerBlock: usize = 8;
// pub const cDXT5NBytesPerBlock: usize = 16;
pub const DXT1_SELECTOR_BITS: usize = 2;
pub const DXT1_SELECTOR_VALUES: usize = 1 << DXT1_SELECTOR_BITS;
// pub const cDXT1SelectorMask: usize = cDXT1SelectorValues - 1;
pub const DXT5_SELECTOR_BITS: usize = 3;
pub const DXT5_SELECTOR_VALUES: usize = 1 << DXT5_SELECTOR_BITS;
// pub const cDXT5SelectorMask: usize = cDXT5SelectorValues - 1;

// pub const g_dxt1_to_linear:             [u8; cDXT1SelectorValues as usize]  = [0, 3, 1, 2];
pub const DXT1_FROM_LINEAR: [u8; DXT1_SELECTOR_VALUES] = [0, 2, 3, 1];
// pub const g_dxt5_to_linear:             [u8; cDXT5SelectorValues as usize]  = [0, 7, 1, 2, 3, 4, 5, 6];
pub const DXT5_FROM_LINEAR: [u8; DXT5_SELECTOR_VALUES] = [0, 2, 3, 4, 5, 6, 7, 1];
// pub const g_six_alpha_invert_table:     [u8; cDXT5SelectorValues as usize]  = [1, 0, 5, 4, 3, 2, 6, 7];
// pub const g_eight_alpha_invert_table:   [u8; cDXT5SelectorValues as usize]  = [1, 0, 7, 6, 5, 4, 3, 2];

pub const NUM_CHUNK_ENCODINGS: usize = 8;

#[allow(non_camel_case_types)]
pub struct crnd_encoding_tile_indices {
    pub tiles: [u8; 4],
}

pub const CRND_CHUNK_ENCODING_TILES: [crnd_encoding_tile_indices; NUM_CHUNK_ENCODINGS] = [
    {
        crnd_encoding_tile_indices {
            tiles: [0, 0, 0, 0],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [0, 0, 1, 1],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [0, 1, 0, 1],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [0, 0, 1, 2],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [1, 2, 0, 0],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [0, 1, 0, 2],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [1, 0, 2, 0],
        }
    },
    {
        crnd_encoding_tile_indices {
            tiles: [0, 1, 2, 3],
        }
    },
];

pub const CRND_CHUNK_ENCODING_NUM_TILES: [u8; NUM_CHUNK_ENCODINGS] = [1, 2, 2, 3, 3, 3, 3, 4];
