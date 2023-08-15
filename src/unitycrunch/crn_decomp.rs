use crate::crnlib::crn_format;
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
    return Ok(p);
}

pub fn crnd_get_crn_format_bits_per_texel(fmt: &mut crn_format) -> Result<u32, &'static str>{
    return match fmt {
        crn_format::cCRNFmtDXT1 |
        crn_format::cCRNFmtDXT5A |
        crn_format::cCRNFmtETC1 |
        crn_format::cCRNFmtETC2 |
        crn_format::cCRNFmtETC1S => Ok(4),

        crn_format::cCRNFmtDXT3 |
        crn_format::cCRNFmtDXT5 |
        crn_format::cCRNFmtDXN_XY |
        crn_format::cCRNFmtDXN_YX |
        crn_format::cCRNFmtDXT5_CCxY |
        crn_format::cCRNFmtDXT5_xGxR |
        crn_format::cCRNFmtDXT5_xGBR |
        crn_format::cCRNFmtDXT5_AGBR |
        crn_format::cCRNFmtETC2A |
        crn_format::cCRNFmtETC2AS => Ok(8),

        _ => Err("Texture format is not supported.")
    };
}

pub fn crnd_get_bytes_per_dxt_block(fmt: &mut crn_format) -> Result<u32, &'static str>{
    return Ok((match crnd_get_crn_format_bits_per_texel(fmt){
        Ok(s) => s,
        Err(e) => return Err(e)
    } << 4) >> 3);
}
