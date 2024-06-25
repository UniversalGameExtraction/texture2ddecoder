use crate::macros::block_decoder;

pub(crate) mod consts;
pub(crate) mod eac;
pub(crate) mod etc1;
pub(crate) mod etc2;

pub use eac::{
    decode_eac_block, decode_eac_signed_block, decode_eacr_block, decode_eacr_signed_block,
    decode_eacrg_block, decode_eacrg_signed_block,
};
pub use etc1::decode_etc1_block;
pub use etc2::{
    decode_etc2_a8_block, decode_etc2_rgb_block, decode_etc2_rgba1_block, decode_etc2_rgba8_block,
};

block_decoder!("etc1", 4, 4, 8, decode_etc1_block);
block_decoder!("etc2_rgb", 4, 4, 8, decode_etc2_rgb_block);
block_decoder!("etc2_rgba1", 4, 4, 8, decode_etc2_rgba1_block);
block_decoder!("etc2_rgba8", 4, 4, 16, decode_etc2_rgba8_block);

// TODO: set alpha to 0xff
block_decoder!("eacr", 4, 4, 8, decode_eacr_block);
block_decoder!("eacr_signed", 4, 4, 8, decode_eacr_signed_block);
block_decoder!("eacrg", 4, 4, 16, decode_eacrg_block);
block_decoder!("eacrg_signed", 4, 4, 16, decode_eacrg_signed_block);
