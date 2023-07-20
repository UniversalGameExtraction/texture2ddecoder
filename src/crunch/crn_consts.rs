#![allow(non_upper_case_globals)]
#![allow(dead_code)]

// Moved Consts from structs.
pub const cCRNHeaderMinSize: u32 = 62;

pub const cCRNSigValue: u16 = ('H' as u16) << 8 | 'x' as u16;

pub const cMaxExpectedCodeSize: u32 = 16;

pub const cMaxSupportedSyms: u32 = 8192;

pub const cMaxTableBits: u32 = 11;

pub const cMagicValue: u32 = 0x1EF9CABD;

pub const cBitBufSize: u32 = 32;

// The crnd library assumes all allocation blocks have at least CRND_MIN_ALLOC_ALIGNMENT alignment.
pub const CRND_MIN_ALLOC_ALIGNMENT: u32 = 8;

// Code length encoding symbols:
// 0-16 - actual code lengths
pub const cMaxCodelengthCodes: u32      = 21;

pub const cSmallZeroRunCode: u32        = 17;
pub const cLargeZeroRunCode: u32        = 18;
pub const cSmallRepeatCode: u32         = 19;
pub const cLargeRepeatCode: u32         = 20;

pub const cMinSmallZeroRunSize: u32     = 3;
pub const cMaxSmallZeroRunSize: u32     = 10;
pub const cMinLargeZeroRunSize: u32     = 11;
pub const cMaxLargeZeroRunSize: u32     = 138;

pub const cSmallMinNonZeroRunSize: u32  = 3;
pub const cSmallMaxNonZeroRunSize: u32  = 6;
pub const cLargeMinNonZeroRunSize: u32  = 7;
pub const cLargeMaxNonZeroRunSize: u32  = 70;

pub const cSmallZeroRunExtraBits: u32   = 3;
pub const cLargeZeroRunExtraBits: u32   = 7;
pub const cSmallNonZeroRunExtraBits: u32 = 2;
pub const cLargeNonZeroRunExtraBits: u32 = 6;

pub const g_most_probable_codelength_codes: [u8; 21] = [
   cSmallZeroRunCode as u8, cLargeZeroRunCode as u8,
   cSmallRepeatCode as u8,  cLargeRepeatCode as u8,
   0, 8,
   7, 9,
   6, 10,
   5, 11,
   4, 12,
   3, 13,
   2, 14,
   1, 15,
   16
];

pub const cNumMostProbableCodelengthCodes: u32 = 21;

#[cfg(target_endian = "little")]
pub const c_crnd_little_endian_platform: bool = true;
#[cfg(target_endian = "little")]
pub const c_crnd_big_endian_platform: bool = false;

#[cfg(target_endian = "big")]
pub const c_crnd_little_endian_platform: bool = false;
#[cfg(target_endian = "big")]
pub const c_crnd_big_endian_platform: bool = true;

pub const cDXTBlockShift: u32 = 2;
pub const cDXTBlockSize: u32 = 1 << cDXTBlockShift;
pub const cDXT1BytesPerBlock: u32 = 8;
pub const cDXT5NBytesPerBlock: u32 = 16;
pub const cDXT1SelectorBits: u32 = 2;
pub const cDXT1SelectorValues: u32 = 1 << cDXT1SelectorBits;
pub const cDXT1SelectorMask: u32 = cDXT1SelectorValues - 1;
pub const cDXT5SelectorBits: u32 = 3;
pub const cDXT5SelectorValues: u32 = 1 << cDXT5SelectorBits;
pub const cDXT5SelectorMask: u32 = cDXT5SelectorValues - 1;

pub const g_dxt1_to_linear:             [u8; cDXT1SelectorValues as usize]  = [0, 3, 1, 2];
pub const g_dxt1_from_linear:           [u8; cDXT1SelectorValues as usize]  = [0, 2, 3, 1];
pub const g_dxt5_to_linear:             [u8; cDXT5SelectorValues as usize]  = [0, 7, 1, 2, 3, 4, 5, 6];
pub const g_dxt5_from_linear:           [u8; cDXT5SelectorValues as usize]  = [0, 2, 3, 4, 5, 6, 7, 1];
pub const g_six_alpha_invert_table:     [u8; cDXT5SelectorValues as usize]  = [1, 0, 5, 4, 3, 2, 6, 7];
pub const g_eight_alpha_invert_table:   [u8; cDXT5SelectorValues as usize]  = [1, 0, 7, 6, 5, 4, 3, 2];

pub const cNumChunkEncodings: u32 = 8;

#[allow(non_camel_case_types)]
pub struct crnd_encoding_tile_indices{
    pub m_tiles: [u8; 4]
} 

pub const g_crnd_chunk_encoding_tiles: [crnd_encoding_tile_indices; cNumChunkEncodings as usize] = [
   { crnd_encoding_tile_indices{ m_tiles: [0, 0, 0, 0] } },
   { crnd_encoding_tile_indices{ m_tiles: [0, 0, 1, 1] } },
   { crnd_encoding_tile_indices{ m_tiles: [0, 1, 0, 1] } },
   { crnd_encoding_tile_indices{ m_tiles: [0, 0, 1, 2] } },
   { crnd_encoding_tile_indices{ m_tiles: [1, 2, 0, 0] } },
   { crnd_encoding_tile_indices{ m_tiles: [0, 1, 0, 2] } },
   { crnd_encoding_tile_indices{ m_tiles: [1, 0, 2, 0] } },
   { crnd_encoding_tile_indices{ m_tiles: [0, 1, 2, 3] } }
];

pub const g_crnd_chunk_encoding_num_tiles: [u8; cNumChunkEncodings as usize] = [ 1, 2, 2, 3, 3, 3, 3, 4 ];