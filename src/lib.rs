mod bitreader;
mod color;

mod astc;
mod atc;
mod bcn;
mod etc;
mod pvrtc;

// import decode functions
pub use astc::decode_astc;
pub use atc::{decode_atc_rgb4, decode_atc_rgba8};
pub use bcn::{decode_bc1, decode_bc3, decode_bc4, decode_bc5, decode_bc6, decode_bc7};
pub use etc::{
    decode_eacr, decode_eacr_signed, decode_eacrg, decode_eacrg_signed, decode_etc1, decode_etc2,
    decode_etc2a1, decode_etc2a8,
};
pub use pvrtc::decode_pvrtc;
// generate helper function for identical function signatures
pub fn decode_astc_4_4(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 4, 4, image);
}
pub fn decode_astc_5_4(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 5, 4, image);
}
pub fn decode_astc_5_5(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 5, 5, image);
}
pub fn decode_astc_6_5(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 6, 5, image);
}
pub fn decode_astc_6_6(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 6, 6, image);
}
pub fn decode_astc_8_5(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 8, 5, image);
}
pub fn decode_astc_8_6(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 8, 6, image);
}
pub fn decode_astc_8_8(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_astc(data, m_width, m_height, 8, 8, image);
}

pub fn decode_pvrtc_2bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_pvrtc(data, m_width, m_height, image, true);
}
pub fn decode_pvrtc_4bpp(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    decode_pvrtc(data, m_width, m_height, image, false);
}
