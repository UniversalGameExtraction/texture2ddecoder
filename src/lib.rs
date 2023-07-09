mod bitreader;
mod color;

mod atc;
mod bcn;
mod etc;
mod pvrtc;

pub use atc::{decode_atc_rgb4, decode_atc_rgba8};
pub use bcn::{decode_bc1, decode_bc3, decode_bc4, decode_bc5, decode_bc6, decode_bc7};
pub use etc::{
    decode_eacr, decode_eacr_signed, decode_eacrg, decode_eacrg_signed, decode_etc1, decode_etc2,
    decode_etc2a1, decode_etc2a8,
};

pub use pvrtc::decode_pvrtc;
pub fn decode_pvrtc_2bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_pvrtc(data, m_width, m_height, image, true);
}
pub fn decode_pvrtc_4bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_pvrtc(data, m_width, m_height, image, false);
}
