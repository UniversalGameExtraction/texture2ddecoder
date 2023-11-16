use crate::crnlib::CrnFormat;
use crate::crunch::crn_consts::*;
use super::crn_unpacker::*;
extern crate alloc;

pub fn crnd_unpack_begin(p_data: &[u8], data_size: u32) -> Result<CrnUnpacker, &'static str>{
    if data_size < C_CRNHEADER_MIN_SIZE as u32{
        return Err("Data size is below the minimum allowed.");
    }
    let mut p = CrnUnpacker::default();
    if p.init(p_data, data_size) == false{
        return Err("Failed to initialize Crunch decompressor.");
    }
    Ok(p)
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut CrnFormat) -> Result<u32, &'static str>{
    match fmt {
        CrnFormat::CCrnfmtDxt1 |
        CrnFormat::CCrnfmtDxt5a |
        CrnFormat::CCrnfmtEtc1 |
        CrnFormat::CCrnfmtEtc2 |
        CrnFormat::CCrnfmtEtc1s => Ok(4),

        CrnFormat::CCrnfmtDxt3 |
        CrnFormat::CCrnfmtDxt5 |
        CrnFormat::CCrnfmtDxnXy |
        CrnFormat::CCrnfmtDxnYx |
        CrnFormat::CCrnfmtDxt5CcxY |
        CrnFormat::CCrnfmtDxt5XGxR |
        CrnFormat::CCrnfmtDxt5XGbr |
        CrnFormat::CCrnfmtDxt5Agbr |
        CrnFormat::CCrnfmtEtc2a |
        CrnFormat::CCrnfmtEtc2as => Ok(8),

        _ => Err("Texture format is not supported.")
    }
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut CrnFormat) -> Result<u32, &'static str>{
    Ok((match crnd_get_crn_format_bits_per_texel(fmt){
        Ok(s) => s,
        Err(e) => return Err(e)
    } << 4) >> 3)
}
