pub(crate) mod crn_unpacker;
pub(crate) mod crn_decomp;
use super::crnlib::{CrnTextureInfo, CrnFormat};
use crate::{decode_etc1, decode_etc2_rgb, decode_etc2_rgba8};
use crate::bcn;
extern crate alloc;

struct CrunchDecodeHandler{
    format: CrnFormat,
    dxt_data: alloc::vec::Vec<u8>
}

fn unity_crunch_unpack_level(data: &[u8], data_size: u32, level_index: u32) -> Result<CrunchDecodeHandler, &'static str> {
    let mut tex_info: CrnTextureInfo = CrnTextureInfo::default();
    if !tex_info.crnd_get_texture_info(data, data_size) {
        return Err("Invalid crunch texture encoding.");
    }
    if tex_info.faces != 1 {
        return Err("Texture2D must only have 1 number of faces.");
    }
    let mut p_context: crn_unpacker::CrnUnpacker<'_> = match crn_decomp::crnd_unpack_begin(data, data_size){
        Ok(p_context) => p_context,
        Err(res) => return Err(res)
    };
    let width = core::cmp::max(1, tex_info.width >> level_index);
    let height = core::cmp::max(1, tex_info.height >> level_index);
    let blocks_x: u32 = core::cmp::max(1, ((width + 3) >> 2) as u32);
    let blocks_y: u32 = core::cmp::max(1, ((height + 3) >> 2) as u32);
    let row_pitch: u32 = blocks_x * match crn_decomp::crnd_get_bytes_per_dxt_block(&mut tex_info.format){
        Ok(s) => s,
        Err(e) => return Err(e)
    };
    let total_face_size: u32 = row_pitch * blocks_y;
    match p_context.crnd_unpack_level(total_face_size, row_pitch, level_index){
        Ok(res) => Ok(CrunchDecodeHandler{
            format: tex_info.format,
            dxt_data: res
        }),
        Err(err) => Err(err)
    }
}

pub fn decode_unity_crunch(data: &[u8], width: usize, height: usize, image: &mut [u32]) -> Result<(), &'static str>{
    let handler = match unity_crunch_unpack_level(data, data.len() as u32, 0){
        Ok(handler) => handler,
        Err(s) => return Err(s)
    };
    match handler.format{
        CrnFormat::Dxt1 => bcn::decode_bc1(&handler.dxt_data, width, height, image),

        CrnFormat::Etc1 |
        CrnFormat::Etc1s => decode_etc1(&handler.dxt_data, width, height, image),

        CrnFormat::CCrnfmtDxt5 |
        CrnFormat::Dxt5CcxY |
        CrnFormat::Dxt5XGbr |
        CrnFormat::Dxt5Agbr |
        CrnFormat::Dxt5XGxR => bcn::decode_bc3(&handler.dxt_data, width, height, image),

        CrnFormat::Dxt5a => bcn::decode_bc4(&handler.dxt_data, width, height, image),
        
        CrnFormat::DxnXy |
        CrnFormat::DxnYx => bcn::decode_bc5(&handler.dxt_data, width, height, image),

        CrnFormat::Etc2 => decode_etc2_rgb(&handler.dxt_data, width, height, image),

        CrnFormat::Etc2a |
        CrnFormat::Etc2as => decode_etc2_rgba8(&handler.dxt_data, width, height, image),

        _ => Err("Invalid crunch format.")
    }
}
