use crate::color::{color, TRANSPARENT_MASK, TRANSPARENT_SHIFT};
use crate::etc::consts::{
    ETC1_MODIFIER_TABLE, ETC1_SUBBLOCK_TABLE, ETC2A_MODIFIER_TABLE, ETC2_ALPHA_MOD_TABLE,
    ETC2_DISTANCE_TABLE, WRITE_ORDER_TABLE, WRITE_ORDER_TABLE_REV,
};
use crate::etc::etc1::{applicate_color, applicate_color_alpha, applicate_color_raw, clamp};

#[inline]
pub fn decode_etc2_rgb_block(data: &[u8], outbuf: &mut [u32]) {
    let mut j: usize = u16::from_be_bytes([data[6], data[7]]) as usize; // 15 -> 0
    let mut k: usize = u16::from_be_bytes([data[4], data[5]]) as usize; // 31 -> 16
    let mut c: [[u8; 3]; 3] = [[0; 3]; 3];

    if (data[3] & 2) > 0 {
        // diff bit == 1
        let r: i32 = (data[0] & 0xf8) as i32;
        let dr: i32 = ((data[0] as i32) << 3 & 0x18) - ((data[0] as i32) << 3 & 0x20);
        let g: i32 = (data[1] & 0xf8) as i32;
        let dg: i32 = ((data[1] as i32) << 3 & 0x18) - ((data[1] as i32) << 3 & 0x20);
        let b: i32 = (data[2] & 0xf8) as i32;
        let db: i32 = ((data[2] as i32) << 3 & 0x18) - ((data[2] as i32) << 3 & 0x20);
        if r + dr < 0 || r + dr > 255 {
            // T
            c[0][0] = (data[0] << 3 & 0xc0)
                | (data[0] << 4 & 0x30)
                | (data[0] >> 1 & 0xc)
                | (data[0] & 3);
            c[0][1] = (data[1] & 0xf0) | data[1] >> 4;
            c[0][2] = (data[1] & 0x0f) | data[1] << 4;
            c[1][0] = (data[2] & 0xf0) | data[2] >> 4;
            c[1][1] = (data[2] & 0x0f) | data[2] << 4;
            c[1][2] = (data[3] & 0xf0) | data[3] >> 4;
            let d: i16 = ETC2_DISTANCE_TABLE[((data[3] >> 1 & 6) | (data[3] & 1)) as usize];
            let color_set: [u32; 4] = [
                applicate_color_raw(c[0]),
                applicate_color(c[1], d),
                applicate_color_raw(c[1]),
                applicate_color(c[1], -d),
            ];
            k <<= 1;
            for i in 0..16 {
                outbuf[WRITE_ORDER_TABLE[i]] = color_set[(k & 2) | (j & 1)];
                j >>= 1;
                k >>= 1
            }
        } else if g + dg < 0 || g + dg > 255 {
            // H
            c[0][0] = (data[0] << 1 & 0xf0) | (data[0] >> 3 & 0xf);
            c[0][1] = (data[0] << 5 & 0xe0) | (data[1] & 0x10);
            c[0][1] |= c[0][1] >> 4;
            c[0][2] = (data[1] & 8) | (data[1] << 1 & 6) | data[2] >> 7;
            c[0][2] |= c[0][2] << 4;
            c[1][0] = (data[2] << 1 & 0xf0) | (data[2] >> 3 & 0xf);
            c[1][1] = (data[2] << 5 & 0xe0) | (data[3] >> 3 & 0x10);
            c[1][1] |= c[1][1] >> 4;
            c[1][2] = (data[3] << 1 & 0xf0) | (data[3] >> 3 & 0xf);
            let mut d: u8 = (data[3] & 4) | (data[3] << 1 & 2);
            if c[0][0] > c[1][0]
                || (c[0][0] == c[1][0]
                    && (c[0][1] > c[1][1] || (c[0][1] == c[1][1] && c[0][2] >= c[1][2])))
            {
                d += 1;
            }
            let d: i16 = ETC2_DISTANCE_TABLE[d as usize];
            let color_set: [u32; 4] = [
                applicate_color(c[0], d),
                applicate_color(c[0], -d),
                applicate_color(c[1], d),
                applicate_color(c[1], -d),
            ];
            k <<= 1;
            for i in 0..16 {
                outbuf[WRITE_ORDER_TABLE[i]] = color_set[(k & 2) | (j & 1)];
                j >>= 1;
                k >>= 1
            }
        } else if b + db < 0 || b + db > 255 {
            // planar
            c[0][0] = (data[0] << 1 & 0xfc) | (data[0] >> 5 & 3);
            c[0][1] = (data[0] << 7 & 0x80) | (data[1] & 0x7e) | (data[0] & 1);
            c[0][2] = (data[1] << 7 & 0x80)
                | (data[2] << 2 & 0x60)
                | (data[2] << 3 & 0x18)
                | (data[3] >> 5 & 4);
            c[0][2] |= c[0][2] >> 6;
            c[1][0] = (data[3] << 1 & 0xf8) | (data[3] << 2 & 4) | (data[3] >> 5 & 3);
            c[1][1] = (data[4] & 0xfe) | data[4] >> 7;
            c[1][2] = (data[4] << 7 & 0x80) | (data[5] >> 1 & 0x7c);
            c[1][2] |= c[1][2] >> 6;
            c[2][0] = (data[5] << 5 & 0xe0) | (data[6] >> 3 & 0x1c) | (data[5] >> 1 & 3);
            c[2][1] = (data[6] << 3 & 0xf8) | (data[7] >> 5 & 0x6) | (data[6] >> 4 & 1);
            c[2][2] = data[7] << 2 | (data[7] >> 4 & 3);
            let mut i: usize = 0;
            for y in 0..4 {
                for x in 0..4 {
                    let r: u8 = clamp(
                        (x * (c[1][0] as i32 - c[0][0] as i32)
                            + y * (c[2][0] as i32 - c[0][0] as i32)
                            + 4 * c[0][0] as i32
                            + 2)
                            >> 2,
                    );
                    let g: u8 = clamp(
                        (x * (c[1][1] as i32 - c[0][1] as i32)
                            + y * (c[2][1] as i32 - c[0][1] as i32)
                            + 4 * c[0][1] as i32
                            + 2)
                            >> 2,
                    );
                    let b: u8 = clamp(
                        (x * (c[1][2] as i32 - c[0][2] as i32)
                            + y * (c[2][2] as i32 - c[0][2] as i32)
                            + 4 * c[0][2] as i32
                            + 2)
                            >> 2,
                    );
                    outbuf[i] = color(r, g, b, 255);
                    i += 1;
                }
            }
        } else {
            // differential
            let code: [u8; 2] = [(data[3] >> 5), (data[3] >> 2 & 7)];
            let table = ETC1_SUBBLOCK_TABLE[(data[3] & 1) as usize];
            c[0][0] = (r | r >> 5) as u8;
            c[0][1] = (g | g >> 5) as u8;
            c[0][2] = (b | b >> 5) as u8;
            c[1][0] = (r + dr) as u8;
            c[1][1] = (g + dg) as u8;
            c[1][2] = (b + db) as u8;
            c[1][0] |= c[1][0] >> 5;
            c[1][1] |= c[1][1] >> 5;
            c[1][2] |= c[1][2] >> 5;
            for i in 0..16 {
                let s: usize = table[i];
                let m: i16 = ETC1_MODIFIER_TABLE[code[s] as usize][j & 1];
                outbuf[WRITE_ORDER_TABLE[i]] =
                    applicate_color(c[s], if k & 1 > 0 { -m } else { m });
                j >>= 1;
                k >>= 1;
            }
        }
    } else {
        // individual (diff bit == 0)
        let code: [u8; 2] = [(data[3] >> 5), (data[3] >> 2 & 7)];
        let table = ETC1_SUBBLOCK_TABLE[(data[3] & 1) as usize];
        c[0][0] = (data[0] & 0xf0) | data[0] >> 4;
        c[1][0] = (data[0] & 0x0f) | data[0] << 4;
        c[0][1] = (data[1] & 0xf0) | data[1] >> 4;
        c[1][1] = (data[1] & 0x0f) | data[1] << 4;
        c[0][2] = (data[2] & 0xf0) | data[2] >> 4;
        c[1][2] = (data[2] & 0x0f) | data[2] << 4;
        for i in 0..16 {
            let s: usize = table[i];
            let m: i16 = ETC1_MODIFIER_TABLE[code[s] as usize][j & 1];
            outbuf[WRITE_ORDER_TABLE[i]] = applicate_color(c[s], if k & 1 > 0 { -m } else { m });
            j >>= 1;
            k >>= 1;
        }
    }
}

#[inline]
pub fn decode_etc2_rgba1_block(data: &[u8], outbuf: &mut [u32]) {
    let mut j: usize = u16::from_be_bytes([data[6], data[7]]) as usize; // 15 -> 0
    let mut k: usize = u16::from_be_bytes([data[4], data[5]]) as usize; // 31 -> 16
    let mut c: [[u8; 3]; 3] = [[0; 3]; 3];

    let obaq: bool = (data[3] >> 1 & 1) > 0;

    // diff bit == 1
    let r: i32 = (data[0] & 0xf8) as i32;
    let dr: i32 = ((data[0] as i32) << 3 & 0x18) - ((data[0] as i32) << 3 & 0x20);
    let g: i32 = (data[1] & 0xf8) as i32;
    let dg: i32 = ((data[1] as i32) << 3 & 0x18) - ((data[1] as i32) << 3 & 0x20);
    let b: i32 = (data[2] & 0xf8) as i32;
    let db: i32 = ((data[2] as i32) << 3 & 0x18) - ((data[2] as i32) << 3 & 0x20);
    if r + dr < 0 || r + dr > 255 {
        // T
        c[0][0] =
            (data[0] << 3 & 0xc0) | (data[0] << 4 & 0x30) | (data[0] >> 1 & 0xc) | (data[0] & 3);
        c[0][1] = (data[1] & 0xf0) | data[1] >> 4;
        c[0][2] = (data[1] & 0x0f) | data[1] << 4;
        c[1][0] = (data[2] & 0xf0) | data[2] >> 4;
        c[1][1] = (data[2] & 0x0f) | data[2] << 4;
        c[1][2] = (data[3] & 0xf0) | data[3] >> 4;
        let d: i16 = ETC2_DISTANCE_TABLE[((data[3] >> 1 & 6) | (data[3] & 1)) as usize];
        let color_set: [u32; 4] = [
            applicate_color_raw(c[0]),
            applicate_color(c[1], d),
            applicate_color_raw(c[1]),
            applicate_color(c[1], -d),
        ];
        k <<= 1;
        for i in 0..16 {
            let index = (k & 2) | (j & 1);
            outbuf[WRITE_ORDER_TABLE[i]] = color_set[index];
            if !obaq && index == 2 {
                outbuf[WRITE_ORDER_TABLE[i]] &= TRANSPARENT_MASK;
            }
            j >>= 1;
            k >>= 1;
        }
    } else if g + dg < 0 || g + dg > 255 {
        // H
        c[0][0] = (data[0] << 1 & 0xf0) | (data[0] >> 3 & 0xf);
        c[0][1] = (data[0] << 5 & 0xe0) | (data[1] & 0x10);
        c[0][1] |= c[0][1] >> 4;
        c[0][2] = (data[1] & 8) | (data[1] << 1 & 6) | data[2] >> 7;
        c[0][2] |= c[0][2] << 4;
        c[1][0] = (data[2] << 1 & 0xf0) | (data[2] >> 3 & 0xf);
        c[1][1] = (data[2] << 5 & 0xe0) | (data[3] >> 3 & 0x10);
        c[1][1] |= c[1][1] >> 4;
        c[1][2] = (data[3] << 1 & 0xf0) | (data[3] >> 3 & 0xf);
        let mut d: u8 = (data[3] & 4) | (data[3] << 1 & 2);
        if c[0][0] > c[1][0]
            || (c[0][0] == c[1][0]
                && (c[0][1] > c[1][1] || (c[0][1] == c[1][1] && c[0][2] >= c[1][2])))
        {
            d += 1;
        }
        let d = ETC2_DISTANCE_TABLE[d as usize];
        let color_set: [u32; 4] = [
            applicate_color(c[0], d),
            applicate_color(c[0], -d),
            applicate_color(c[1], d),
            applicate_color(c[1], -d),
        ];
        k <<= 1;
        for i in 0..16 {
            let index: usize = (k & 2) | (j & 1);
            outbuf[WRITE_ORDER_TABLE[i]] = color_set[index];
            if !obaq && index == 2 {
                outbuf[WRITE_ORDER_TABLE[i]] &= TRANSPARENT_MASK;
            }
            j >>= 1;
            k >>= 1;
        }
    } else if b + db < 0 || b + db > 255 {
        // planar
        c[0][0] = (data[0] << 1 & 0xfc) | (data[0] >> 5 & 3);
        c[0][1] = (data[0] << 7 & 0x80) | (data[1] & 0x7e) | (data[0] & 1);
        c[0][2] = (data[1] << 7 & 0x80)
            | (data[2] << 2 & 0x60)
            | (data[2] << 3 & 0x18)
            | (data[3] >> 5 & 4);
        c[0][2] |= c[0][2] >> 6;
        c[1][0] = (data[3] << 1 & 0xf8) | (data[3] << 2 & 4) | (data[3] >> 5 & 3);
        c[1][1] = (data[4] & 0xfe) | data[4] >> 7;
        c[1][2] = (data[4] << 7 & 0x80) | (data[5] >> 1 & 0x7c);
        c[1][2] |= c[1][2] >> 6;
        c[2][0] = (data[5] << 5 & 0xe0) | (data[6] >> 3 & 0x1c) | (data[5] >> 1 & 3);
        c[2][1] = (data[6] << 3 & 0xf8) | (data[7] >> 5 & 0x6) | (data[6] >> 4 & 1);
        c[2][2] = data[7] << 2 | (data[7] >> 4 & 3);
        let mut i: usize = 0;
        for y in 0..4 {
            for x in 0..4 {
                let r: u8 = clamp(
                    (x * (c[1][0] as i32 - c[0][0] as i32)
                        + y * (c[2][0] as i32 - c[0][0] as i32)
                        + 4 * c[0][0] as i32
                        + 2)
                        >> 2,
                );
                let g: u8 = clamp(
                    (x * (c[1][1] as i32 - c[0][1] as i32)
                        + y * (c[2][1] as i32 - c[0][1] as i32)
                        + 4 * c[0][1] as i32
                        + 2)
                        >> 2,
                );
                let b: u8 = clamp(
                    (x * (c[1][2] as i32 - c[0][2] as i32)
                        + y * (c[2][2] as i32 - c[0][2] as i32)
                        + 4 * c[0][2] as i32
                        + 2)
                        >> 2,
                );
                outbuf[i] = color(r, g, b, 255);
                i += 1;
            }
        }
    } else {
        // differential
        let code: [u8; 2] = [(data[3] >> 5), (data[3] >> 2 & 7)];
        let table = ETC1_SUBBLOCK_TABLE[(data[3] & 1) as usize];
        c[0][0] = (r | r >> 5) as u8;
        c[0][1] = (g | g >> 5) as u8;
        c[0][2] = (b | b >> 5) as u8;
        c[1][0] = (r + dr) as u8;
        c[1][1] = (g + dg) as u8;
        c[1][2] = (b + db) as u8;
        c[1][0] |= c[1][0] >> 5;
        c[1][1] |= c[1][1] >> 5;
        c[1][2] |= c[1][2] >> 5;
        for i in 0..16 {
            let s: usize = table[i];
            let m: i16 = ETC2A_MODIFIER_TABLE[obaq as usize][code[s] as usize][j & 1];
            outbuf[WRITE_ORDER_TABLE[i]] = applicate_color_alpha(
                c[s],
                if k & 1 > 0 { -m } else { m },
                !obaq && (k & 1 != 0) && j & 1 == 0,
            );
            j >>= 1;
            k >>= 1;
        }
    }
}

#[inline]
pub fn decode_etc2_a8_block(data: &[u8], outbuf: &mut [u32]) {
    // default alpha value is 255 from previous step
    if data[1] & 0xf0 > 0 {
        // multiplier != 0
        let multiplier: i32 = (data[1] >> 4) as i32;
        let table = ETC2_ALPHA_MOD_TABLE[(data[1] & 0xf) as usize];
        let mut l: usize = u64::from_be_bytes(data[0..8].try_into().unwrap()) as usize;
        for i in 0..16 {
            let alpha = (clamp((data[0] as i32) + multiplier * (table[l & 7] as i32)) as u32)
                << TRANSPARENT_SHIFT;
            let x: &mut u32 = &mut outbuf[WRITE_ORDER_TABLE_REV[i]];
            *x = (*x & TRANSPARENT_MASK) | alpha;
            l >>= 3;
        }
    } else {
        // multiplier == 0 (always same as base codeword)
        let alpha = (data[0] as u32) << TRANSPARENT_SHIFT;
        for x in outbuf.iter_mut().take(16) {
            *x = (*x & TRANSPARENT_MASK) | alpha;
        }
    }
}

#[inline]
pub fn decode_etc2_rgba8_block(data: &[u8], outbuf: &mut [u32]) {
    decode_etc2_rgb_block(&data[8..], outbuf);
    decode_etc2_a8_block(data, outbuf);
}
