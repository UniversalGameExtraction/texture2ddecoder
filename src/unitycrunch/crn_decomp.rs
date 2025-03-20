use super::crn_unpacker::*;
use crate::crnlib::CrnFormat;
use crate::crunch::crn_consts::*;
extern crate alloc;

pub fn crnd_unpack_begin(p_data: &[u8], data_size: u32) -> Result<CrnUnpacker, &'static str> {
    if data_size < CRNHEADER_MIN_SIZE as u32 {
        return Err("Data size is below the minimum allowed.");
    }
    let mut p = CrnUnpacker::default();
    if !p.init(p_data, data_size) {
        return Err("Failed to initialize Crunch decompressor.");
    }
    Ok(p)
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut CrnFormat) -> Result<u32, &'static str> {
    match fmt {
        CrnFormat::Dxt1
        | CrnFormat::Dxt5a
        | CrnFormat::Etc1
        | CrnFormat::Etc2
        | CrnFormat::Etc1s => Ok(4),

        CrnFormat::Dxt3
        | CrnFormat::CCrnfmtDxt5
        | CrnFormat::DxnXy
        | CrnFormat::DxnYx
        | CrnFormat::Dxt5CcxY
        | CrnFormat::Dxt5XGxR
        | CrnFormat::Dxt5XGbr
        | CrnFormat::Dxt5Agbr
        | CrnFormat::Etc2a
        | CrnFormat::Etc2as => Ok(8),

        _ => Err("Texture format is not supported."),
    }
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut CrnFormat) -> Result<u32, &'static str> {
    Ok((crnd_get_crn_format_bits_per_texel(fmt)? << 4) >> 3)
}
