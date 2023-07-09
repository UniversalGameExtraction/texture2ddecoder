use crate::color::{color, copy_block_buffer, TRANSPARENT_MASK, TRANSPARENT_SHIFT};

static WRITE_ORDER_TABLE: [usize; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];
static WRITE_ORDER_TABLE_REV: [usize; 16] = [15, 11, 7, 3, 14, 10, 6, 2, 13, 9, 5, 1, 12, 8, 4, 0];
static ETC1_MODIFIER_TABLE: [[i16; 2]; 8] = [
    [2, 8],
    [5, 17],
    [9, 29],
    [13, 42],
    [18, 60],
    [24, 80],
    [33, 106],
    [47, 183],
];
static ETC2A_MODIFIER_TABLE: [[[i16; 2]; 8]; 2] = [
    [
        [0, 8],
        [0, 17],
        [0, 29],
        [0, 42],
        [0, 60],
        [0, 80],
        [0, 106],
        [0, 183],
    ],
    [
        [2, 8],
        [5, 17],
        [9, 29],
        [13, 42],
        [18, 60],
        [24, 80],
        [33, 106],
        [47, 183],
    ],
];
static ETC1_SUBBLOCK_TABLE: [[usize; 16]; 2] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1],
];
static ETC2_DISTANCE_TABLE: [i16; 8] = [3, 6, 11, 16, 23, 32, 41, 64];
static ETC2_ALPHA_MOD_TABLE: [[i8; 8]; 16] = [
    [-3, -6, -9, -15, 2, 5, 8, 14],
    [-3, -7, -10, -13, 2, 6, 9, 12],
    [-2, -5, -8, -13, 1, 4, 7, 12],
    [-2, -4, -6, -13, 1, 3, 5, 12],
    [-3, -6, -8, -12, 2, 5, 7, 11],
    [-3, -7, -9, -11, 2, 6, 8, 10],
    [-4, -7, -8, -11, 3, 6, 7, 10],
    [-3, -5, -8, -11, 2, 4, 7, 10],
    [-2, -6, -8, -10, 1, 5, 7, 9],
    [-2, -5, -8, -10, 1, 4, 7, 9],
    [-2, -4, -8, -10, 1, 3, 7, 9],
    [-2, -5, -7, -10, 1, 4, 6, 9],
    [-3, -4, -7, -10, 2, 3, 6, 9],
    [-1, -2, -3, -10, 0, 1, 2, 9],
    [-4, -6, -8, -9, 3, 5, 7, 8],
    [-3, -5, -7, -9, 2, 4, 6, 8],
];

#[inline]
const fn clamp(n: i32) -> u8 {
    (if n < 0 {
        0
    } else if n > 255 {
        255
    } else {
        n
    }) as u8
}

#[inline]
const fn applicate_color(c: [u8; 3], m: i16) -> u32 {
    color(
        clamp(c[0] as i32 + m as i32),
        clamp(c[1] as i32 + m as i32),
        clamp(c[2] as i32 + m as i32),
        255,
    )
}

#[inline]
const fn applicate_color_alpha(c: [u8; 3], m: i16, transparent: bool) -> u32 {
    color(
        clamp(c[0] as i32 + m as i32),
        clamp(c[1] as i32 + m as i32),
        clamp(c[2] as i32 + m as i32),
        if transparent { 0 } else { 255 },
    )
}

#[inline]
const fn applicate_color_raw(c: [u8; 3]) -> u32 {
    color(c[0], c[1], c[2], 255)
}

#[inline]
fn decode_etc1_block(data: &[u8], outbuf: &mut [u32]) {
    let code: [u8; 2] = [(data[3] >> 5), (data[3] >> 2 & 7)]; // Table codewords
    let table: [usize; 16] = ETC1_SUBBLOCK_TABLE[(data[3] & 1) as usize];
    let mut c: [[u8; 3]; 2] = [[0; 3]; 2];
    if (data[3] & 2) > 0 {
        // diff bit == 1
        c[0][0] = data[0] & 0xf8;
        c[0][1] = data[1] & 0xf8;
        c[0][2] = data[2] & 0xf8;
        c[1][0] = c[0][0].overflowing_add(data[0] << 3 & 0x18).0.overflowing_sub(data[0] << 3 & 0x20).0;
        c[1][1] = c[0][1].overflowing_add(data[1] << 3 & 0x18).0.overflowing_sub(data[1] << 3 & 0x20).0;
        c[1][2] = c[0][2].overflowing_add(data[2] << 3 & 0x18).0.overflowing_sub(data[2] << 3 & 0x20).0;
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

#[inline]
fn decode_etc2_block(data: &[u8], outbuf: &mut [u32]) {
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
fn decode_etc2a1_block(data: &[u8], outbuf: &mut [u32]) {
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
fn decode_etc2a8_block(data: &[u8], outbuf: &mut [u32]) {
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
fn decode_eac_block(data: &[u8], color: i32, outbuf: &mut [u32]) {
    let mut multiplier: i32 = (data[1] >> 1 & 0x78) as i32;
    if multiplier == 0 {
        multiplier = 1;
    }
    let table = ETC2_ALPHA_MOD_TABLE[(data[1] & 0xf) as usize];
    let mut l: usize = u64::from_be_bytes(data[0..8].try_into().unwrap()) as usize;
    let mut block: [u8; 4] = [0, 0, 0, 0];
    for i in 0..16 {
        let val: i32 = data[0] as i32 * 8 + multiplier * table[l & 7] as i32 + 4;
        block[color as usize] = if val < 0 {
            0
        } else if val >= 2048 {
            255
        } else {
            (val >> 3) as u8
        };
        outbuf[WRITE_ORDER_TABLE_REV[i]] |= u32::from_be_bytes(block);
        l >>= 3;
    }
}

#[inline]
fn decode_eac_signed_block(data: &[u8], color: i32, outbuf: &mut [u32]) {
    let base: i32 = (data[0] as i8) as i32;
    let mut multiplier: i32 = (data[1] >> 1 & 0x78) as i32;
    if multiplier == 0 {
        multiplier = 1;
    }
    let table = ETC2_ALPHA_MOD_TABLE[(data[1] & 0xf) as usize];
    let mut l: usize = u64::from_be_bytes(data[0..8].try_into().unwrap()) as usize;
    let mut block: [u8; 4] = [0, 0, 0, 0];
    for i in 0..16 {
        let val: i32 = base * 8 + multiplier * table[l & 7] as i32 + 1023;
        block[color as usize] = if val < 0 {
            0
        } else if val >= 2048 {
            255
        } else {
            (val >> 3) as u8
        };
        outbuf[WRITE_ORDER_TABLE_REV[i]] |= u32::from_be_bytes(block);
        l >>= 3;
    }
}

pub fn decode_etc1(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            decode_etc1_block(&data[data_pos..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 8;
        }
    }
}

pub fn decode_etc2(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            decode_etc2_block(&data[data_pos..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 8;
        }
    }
}

pub fn decode_etc2a1(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            decode_etc2a1_block(&data[data_pos..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 8;
        }
    }
}

pub fn decode_etc2a8(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            decode_etc2_block(&data[data_pos + 8..], &mut buffer);
            decode_etc2a8_block(&data[data_pos..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 16;
        }
    }
}

pub fn decode_eacr(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let base_buffer: [u32; 16] = [color(0, 0, 0, 255); 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            buffer.copy_from_slice(&base_buffer);
            decode_eac_block(&data[data_pos..], 2, &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 8;
        }
    }
}

pub fn decode_eacr_signed(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let base_buffer: [u32; 16] = [color(0, 0, 0, 255); 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            buffer.copy_from_slice(&base_buffer);
            decode_eac_signed_block(&data[data_pos..], 2, &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 8;
        }
    }
}

pub fn decode_eacrg(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let base_buffer: [u32; 16] = [color(0, 0, 0, 255); 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            buffer.copy_from_slice(&base_buffer);
            decode_eac_block(&data[data_pos..], 2, &mut buffer);
            decode_eac_block(&data[data_pos + 8..], 1, &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 16;
        }
    }
}

pub fn decode_eacrg_signed(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let base_buffer: [u32; 16] = [color(0, 0, 0, 255); 16];
    let mut data_pos = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            buffer.copy_from_slice(&base_buffer);
            decode_eac_signed_block(&data[data_pos..], 2, &mut buffer);
            decode_eac_signed_block(&data[data_pos + 8..], 1, &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_pos += 16;
        }
    }
}
