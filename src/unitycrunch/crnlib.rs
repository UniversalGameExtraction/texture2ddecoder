#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// Supported compressed pixel formats.
// Basically all the standard DX9 formats, with some swizzled DXT5 formats
// (most of them supported by ATI's Compressonator), along with some ATI/X360 GPU specific formats.
#[derive(PartialEq, PartialOrd)]
#[repr(C)]
pub enum crn_format {
    cCRNFmtInvalid = -1,

    cCRNFmtDXT1 = 0,

    // cCRNFmtFirstValid = crn_format::cCRNFmtDXT1 as isize, // Rust doesn't allow same value enums, and as far as I see this is not used in the lib.

    // cCRNFmtDXT3 is not currently supported when writing to CRN - only DDS.
    cCRNFmtDXT3,

    cCRNFmtDXT5,

    // Various DXT5 derivatives
    cCRNFmtDXT5_CCxY,    // Luma-chroma
    cCRNFmtDXT5_xGxR,    // Swizzled 2-component
    cCRNFmtDXT5_xGBR,    // Swizzled 3-component
    cCRNFmtDXT5_AGBR,    // Swizzled 4-component

    // ATI 3DC and X360 DXN
    cCRNFmtDXN_XY,
    cCRNFmtDXN_YX,

    // DXT5 alpha blocks only
    cCRNFmtDXT5A,

    cCRNFmtETC1,
    cCRNFmtETC2,
    cCRNFmtETC2A,
    cCRNFmtETC1S,
    cCRNFmtETC2AS,

    cCRNFmtTotal,

    cCRNFmtForceDWORD = 0xFFFFFFFF
}

pub const cCRNMaxLevels: u32 = 16;