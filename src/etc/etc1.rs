use crate::color::color;
use crate::etc::consts::{ETC1_MODIFIER_TABLE, ETC1_SUBBLOCK_TABLE, WRITE_ORDER_TABLE};

#[inline]
pub(crate) const fn clamp(n: i32) -> u8 {
    (if n < 0 {
        0
    } else if n > 255 {
        255
    } else {
        n
    }) as u8
}

#[inline]
pub(crate) const fn applicate_color(c: [u8; 3], m: i16) -> u32 {
    color(
        clamp(c[0] as i32 + m as i32),
        clamp(c[1] as i32 + m as i32),
        clamp(c[2] as i32 + m as i32),
        255,
    )
}

#[inline]
pub(crate) const fn applicate_color_alpha(c: [u8; 3], m: i16, transparent: bool) -> u32 {
    color(
        clamp(c[0] as i32 + m as i32),
        clamp(c[1] as i32 + m as i32),
        clamp(c[2] as i32 + m as i32),
        if transparent { 0 } else { 255 },
    )
}

#[inline]
pub(crate) const fn applicate_color_raw(c: [u8; 3]) -> u32 {
    color(c[0], c[1], c[2], 255)
}

#[inline]
pub fn decode_etc1_block(data: &[u8], outbuf: &mut [u32]) {
    let code: [u8; 2] = [(data[3] >> 5), (data[3] >> 2 & 7)]; // Table codewords
    let table: [usize; 16] = ETC1_SUBBLOCK_TABLE[(data[3] & 1) as usize];
    let mut c: [[u8; 3]; 2] = [[0; 3]; 2];
    if (data[3] & 2) > 0 {
        // diff bit == 1
        c[0][0] = data[0] & 0xf8;
        c[0][1] = data[1] & 0xf8;
        c[0][2] = data[2] & 0xf8;
        c[1][0] = c[0][0]
            .overflowing_add(data[0] << 3 & 0x18)
            .0
            .overflowing_sub(data[0] << 3 & 0x20)
            .0;
        c[1][1] = c[0][1]
            .overflowing_add(data[1] << 3 & 0x18)
            .0
            .overflowing_sub(data[1] << 3 & 0x20)
            .0;
        c[1][2] = c[0][2]
            .overflowing_add(data[2] << 3 & 0x18)
            .0
            .overflowing_sub(data[2] << 3 & 0x20)
            .0;
        c[0][0] |= c[0][0] >> 5;
        c[0][1] |= c[0][1] >> 5;
        c[0][2] |= c[0][2] >> 5;
        c[1][0] |= c[1][0] >> 5;
        c[1][1] |= c[1][1] >> 5;
        c[1][2] |= c[1][2] >> 5;
    } else {
        // diff bit == 0
        c[0][0] = (data[0] & 0xf0) | data[0] >> 4;
        c[1][0] = (data[0] & 0x0f) | data[0] << 4;
        c[0][1] = (data[1] & 0xf0) | data[1] >> 4;
        c[1][1] = (data[1] & 0x0f) | data[1] << 4;
        c[0][2] = (data[2] & 0xf0) | data[2] >> 4;
        c[1][2] = (data[2] & 0x0f) | data[2] << 4;
    }

    let mut j: usize = u16::from_be_bytes([data[6], data[7]]) as usize; // less significant pixel index bits
    let mut k: usize = u16::from_be_bytes([data[4], data[5]]) as usize; // more significant pixel index bits

    for i in 0..16 {
        let s: usize = table[i];
        let m: i16 = ETC1_MODIFIER_TABLE[code[s] as usize][j & 1];
        outbuf[WRITE_ORDER_TABLE[i]] = applicate_color(c[s], if k & 1 > 0 { -m } else { m });
        j >>= 1;
        k >>= 1
    }
}
