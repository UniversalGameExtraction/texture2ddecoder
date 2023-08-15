pub(crate) mod crn_utils;
pub(crate) mod crn_consts;
pub(crate) mod crn_static_huffman_data_model;
pub(crate) mod crn_symbol_codec;
pub(crate) mod crn_unpacker;
pub(crate) mod crn_decomp;
use super::crnlib::{crn_texture_info, crn_format};
use core::cmp::max;
use crate::bcn;
extern crate alloc;

struct CrunchDecodeHandler{
    format: crn_format,
    dxt_data: alloc::vec::Vec<u8>
}

fn crunch_unpack_level<'vec>(data: &[u8], data_size: u32, level_index: u32) -> Result<CrunchDecodeHandler, &'static str> {
    let mut tex_info: crn_texture_info = crn_texture_info::default();
    if tex_info.crnd_get_texture_info(data, data_size) == false {
        return Err("Invalid crunch texture encoding.");
    }
    if tex_info.m_faces != 1 {
        // I think cubemaps have 6, but they are not in the same ballpark as Texture2D?
        return Err("Texture2D must only have 1 number of faces.");
    }
    let mut p_context: crn_unpacker::CrnUnpacker<'_> = match crn_decomp::crnd_unpack_begin(&data, data_size){
        Ok(p_context) => p_context,
        Err(res) => return Err(res)
    };
    let width = max(1, tex_info.m_width >> level_index);
    let height = max(1, tex_info.m_height >> level_index);
    let blocks_x: u32 = max(1, ((width + 3) >> 2) as u32);
    let blocks_y: u32 = max(1, ((height + 3) >> 2) as u32);
    let row_pitch: u32 = blocks_x * match crn_decomp::crnd_get_bytes_per_dxt_block(&mut tex_info.m_format){
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    let total_face_size: u32 = row_pitch * blocks_y;
    return match p_context.crnd_unpack_level(total_face_size, row_pitch, level_index){
        Ok(res) => Ok(CrunchDecodeHandler{
            format: tex_info.m_format,
            dxt_data: res
        }),
        Err(err) => Err(err)
    };
}

pub fn decode_crunch(data: &[u8], width: usize, height: usize, image: &mut [u32]) -> Result<(), &'static str>{
    let handler = match crunch_unpack_level(data, data.len() as u32, 0){
        Ok(handler) => handler,
        Err(s) => return Err(s)
    };
    match handler.format{
        crn_format::cCRNFmtDXT1 => bcn::decode_bc1(&handler.dxt_data, width, height, image),

        crn_format::cCRNFmtDXT5 |
        crn_format::cCRNFmtDXT5_CCxY |
        crn_format::cCRNFmtDXT5_xGBR |
        crn_format::cCRNFmtDXT5_AGBR |
        crn_format::cCRNFmtDXT5_xGxR => bcn::decode_bc3(&handler.dxt_data, width, height, image),

        crn_format::cCRNFmtDXT5A => bcn::decode_bc4(&handler.dxt_data, width, height, image),
        
        crn_format::cCRNFmtDXN_XY |
        crn_format::cCRNFmtDXN_YX => bcn::decode_bc5(&handler.dxt_data, width, height, image),
        _ => Err("Invalid crunch format.")
    }
}
