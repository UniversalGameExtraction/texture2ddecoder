pub(crate) mod crn_consts;
pub(crate) mod crn_decomp;
pub(crate) mod crn_static_huffman_data_model;
pub(crate) mod crn_symbol_codec;
pub(crate) mod crn_unpacker;
pub(crate) mod crn_utils;
use super::crnlib::{CrnFormat, CrnTextureInfo};
use crate::bcn;
use core::cmp::max;
extern crate alloc;

pub struct CrunchDecodeHandler {
    pub format: CrnFormat,
    pub dxt_data: alloc::vec::Vec<u8>,
    pub faces: u32,
}

pub fn crunch_unpack_level(
    data: &[u8],
    data_size: u32,
    level_index: u32,
) -> Result<CrunchDecodeHandler, &'static str> {
    let mut tex_info: CrnTextureInfo = CrnTextureInfo::default();
    if !tex_info.crnd_get_texture_info(data, data_size) {
        return Err("Invalid crunch texture encoding.");
    }
    let mut p_context: crn_unpacker::CrnUnpacker<'_> =
        crn_decomp::crnd_unpack_begin(data, data_size)?;
    let width = max(1, tex_info.width >> level_index);
    let height = max(1, tex_info.height >> level_index);
    let blocks_x: u32 = max(1, (width + 3) >> 2);
    let blocks_y: u32 = max(1, (height + 3) >> 2);
    let row_pitch: u32 = blocks_x * crn_decomp::crnd_get_bytes_per_dxt_block(&mut tex_info.format)?;
    let total_face_size: u32 = row_pitch * blocks_y;
    match p_context.crnd_unpack_level(total_face_size, row_pitch, level_index) {
        Ok(res) => Ok(CrunchDecodeHandler {
            format: tex_info.format,
            dxt_data: res,
            faces: tex_info.faces,
        }),
        Err(err) => Err(err),
    }
}

pub fn decode_crunch(
    data: &[u8],
    width: usize,
    height: usize,
    image: &mut [u32],
) -> Result<(), &'static str> {
    let handler = crunch_unpack_level(data, data.len() as u32, 0)?;
    match handler.format {
        CrnFormat::Dxt1 => bcn::decode_bc1(&handler.dxt_data, width, height, image),

        CrnFormat::CCrnfmtDxt5
        | CrnFormat::Dxt5CcxY
        | CrnFormat::Dxt5XGbr
        | CrnFormat::Dxt5Agbr
        | CrnFormat::Dxt5XGxR => bcn::decode_bc3(&handler.dxt_data, width, height, image),

        CrnFormat::Dxt5a => bcn::decode_bc4(&handler.dxt_data, width, height, image),

        CrnFormat::DxnXy | CrnFormat::DxnYx => {
            bcn::decode_bc5(&handler.dxt_data, width, height, image)
        }
        _ => Err("Invalid crunch format."),
    }
}
