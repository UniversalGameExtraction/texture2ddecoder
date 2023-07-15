#![no_std]

mod bitreader;
mod color;
mod f16;
mod macros;

mod astc;
mod atc;
mod bcn;
mod etc;
mod pvrtc;

// import decode functions
pub use astc::*;
pub use atc::{decode_atc_rgb4, decode_atc_rgb4_block, decode_atc_rgba8, decode_atc_rgba8_block};
pub use bcn::{
    decode_bc1, decode_bc1_block, decode_bc3, decode_bc3_block, decode_bc4, decode_bc4_block,
    decode_bc5, decode_bc5_block, decode_bc6, decode_bc6_block, decode_bc6_block_signed,
    decode_bc6_signed, decode_bc6_unsigned, decode_bc7, decode_bc7_block,
};
pub use etc::{
    decode_eacr, decode_eacr_signed, decode_eacrg, decode_eacrg_signed, decode_etc1, decode_etc2,
    decode_etc2a1, decode_etc2a8,
};
pub use pvrtc::{decode_pvrtc, decode_pvrtc_2bpp, decode_pvrtc_4bpp};
