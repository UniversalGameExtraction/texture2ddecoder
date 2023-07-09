mod bitreader;
mod color;

mod atc;
mod bcn;
mod etc;
mod pvrtc;

pub use atc::{decode_atc_rgb4, decode_atc_rgba8};
pub use bcn::{decode_bc1, decode_bc3, decode_bc4, decode_bc5, decode_bc6, decode_bc7};
pub use etc::{
    decode_etc1,
    decode_etc2,
    decode_etc2a1,
    decode_etc2a8,
    decode_eacr,
    decode_eacr_signed,
    decode_eacrg,
    decode_eacrg_signed,
};

pub use pvrtc::decode_pvrtc;
