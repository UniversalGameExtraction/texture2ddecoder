use crate::color::color;
use crate::etc::consts::{ETC2_ALPHA_MOD_TABLE, WRITE_ORDER_TABLE_REV};

#[inline]
pub fn decode_eac_block(data: &[u8], color: usize, outbuf: &mut [u32]) {
    let mut multiplier: i32 = (data[1] >> 1 & 0x78) as i32;
    if multiplier == 0 {
        multiplier = 1;
    }
    let table = ETC2_ALPHA_MOD_TABLE[(data[1] & 0xf) as usize];
    let mut l: usize = u64::from_le_bytes(data[0..8].try_into().unwrap()) as usize;
    let mut block: [u8; 4] = [0, 0, 0, 0];
    for i in 0..16 {
        let val: i32 = data[0] as i32 * 8 + multiplier * table[l & 7] as i32 + 4;
        block[color] = if val < 0 {
            0
        } else if val >= 2048 {
            255
        } else {
            (val >> 3) as u8
        };
        outbuf[WRITE_ORDER_TABLE_REV[i]] |= u32::from_le_bytes(block);
        l >>= 3;
    }
}

#[inline]
pub fn decode_eac_signed_block(data: &[u8], color: usize, outbuf: &mut [u32]) {
    let base: i32 = (data[0] as i8) as i32;
    let mut multiplier: i32 = (data[1] >> 1 & 0x78) as i32;
    if multiplier == 0 {
        multiplier = 1;
    }
    let table = ETC2_ALPHA_MOD_TABLE[(data[1] & 0xf) as usize];
    let mut l: usize = u64::from_le_bytes(data[0..8].try_into().unwrap()) as usize;
    let mut block: [u8; 4] = [0, 0, 0, 0];
    for i in 0..16 {
        let val: i32 = base * 8 + multiplier * table[l & 7] as i32 + 1023;
        block[color] = if val < 0 {
            0
        } else if val >= 2048 {
            255
        } else {
            (val >> 3) as u8
        };
        outbuf[WRITE_ORDER_TABLE_REV[i]] |= u32::from_le_bytes(block);
        l >>= 3;
    }
}

#[inline]
pub fn decode_eacr_block(data: &[u8], outbuf: &mut [u32]) {
    outbuf[0..16].fill(color(0, 0, 0, 255));
    decode_eac_block(data, 2, outbuf);
}

#[inline]
pub fn decode_eacr_signed_block(data: &[u8], outbuf: &mut [u32]) {
    outbuf[0..16].fill(color(0, 0, 0, 255));
    decode_eac_signed_block(data, 2, outbuf);
}

#[inline]
pub fn decode_eacrg_block(data: &[u8], outbuf: &mut [u32]) {
    outbuf[0..16].fill(color(0, 0, 0, 255));
    decode_eac_block(data, 2, outbuf);
    decode_eac_block(&data[8..], 1, outbuf);
}

#[inline]
pub fn decode_eacrg_signed_block(data: &[u8], outbuf: &mut [u32]) {
    outbuf[0..16].fill(color(0, 0, 0, 255));
    decode_eac_signed_block(data, 2, outbuf);
    decode_eac_signed_block(&data[8..], 1, outbuf);
}
