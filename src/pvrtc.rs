extern crate alloc;
use alloc::vec::Vec;

use crate::color::{color, copy_block_buffer};

#[derive(Clone, Copy)]
struct PVRTCTexelColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PVRTCTexelColor {
    const fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct PVRTCTexelColorInt {
    r: i32,
    g: i32,
    b: i32,
    a: i32,
}

impl PVRTCTexelColorInt {
    const fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct PVRTCTexelInfo {
    a: PVRTCTexelColor,
    b: PVRTCTexelColor,
    weight: [i8; 32],
    punch_through_flag: u32,
}

impl PVRTCTexelInfo {
    const fn default() -> Self {
        Self {
            a: PVRTCTexelColor::default(),
            b: PVRTCTexelColor::default(),
            weight: [0; 32],
            punch_through_flag: 0,
        }
    }
}

static PVRTC1_STANDARD_WEIGHT: [i8; 4] = [0, 3, 5, 8];
static PVRTC1_PUNCHTHROUGH_WEIGHT: [i8; 4] = [0, 4, 4, 8];

#[inline]
const fn morton_index(x: usize, y: usize, min_dim: usize) -> usize {
    let mut offset: usize = 0;
    let mut shift: usize = 0;
    let mut mask: usize = 1;
    while mask < min_dim {
        offset |= ((y & mask) | ((x & mask) << 1)) << shift;
        mask <<= 1;
        shift += 1;
    }
    offset |= ((x | y) >> shift) << (shift * 2);
    offset
}

fn get_texel_colors(data: &[u8], info: &mut PVRTCTexelInfo) {
    let ca: u16 = u16::from_le_bytes(data[4..6].try_into().unwrap());
    let cb: u16 = u16::from_le_bytes(data[6..8].try_into().unwrap());
    if ca & 0x8000 != 0 {
        info.a.r = (ca >> 10 & 0x1f) as u8;
        info.a.g = (ca >> 5 & 0x1f) as u8;
        info.a.b = ((ca & 0x1e) | (ca >> 4 & 1)) as u8;
        info.a.a = 0xf;
    } else {
        info.a.r = ((ca >> 7 & 0x1e) | (ca >> 11 & 1)) as u8;
        info.a.g = ((ca >> 3 & 0x1e) | (ca >> 7 & 1)) as u8;
        info.a.b = ((ca << 1 & 0x1c) | (ca >> 2 & 3)) as u8;
        info.a.a = (ca >> 11 & 0xe) as u8;
    }
    if cb & 0x8000 != 0 {
        info.b.r = (cb >> 10 & 0x1f) as u8;
        info.b.g = (cb >> 5 & 0x1f) as u8;
        info.b.b = (cb & 0x1f) as u8;
        info.b.a = 0xf;
    } else {
        info.b.r = ((cb >> 7 & 0x1e) | (cb >> 11 & 1)) as u8;
        info.b.g = ((cb >> 3 & 0x1e) | (cb >> 7 & 1)) as u8;
        info.b.b = ((cb << 1 & 0x1e) | (cb >> 3 & 1)) as u8;
        info.b.a = (cb >> 11 & 0xe) as u8;
    }
}

fn get_texel_weights_4bpp(data: &[u8], info: &mut PVRTCTexelInfo) {
    info.punch_through_flag = 0;

    let mod_mode: bool = data[4] & 1 != 0;
    let mut mod_bits: usize = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;

    if mod_mode {
        for i in 0..16 {
            info.weight[i] = PVRTC1_PUNCHTHROUGH_WEIGHT[mod_bits & 3];
            if (mod_bits & 3) == 2 {
                info.punch_through_flag |= 1 << i;
            }
            mod_bits >>= 2;
        }
    } else {
        for i in 0..16 {
            info.weight[i] = PVRTC1_STANDARD_WEIGHT[mod_bits & 3];
            mod_bits >>= 2;
        }
    }
}

fn get_texel_weights_2bpp(data: &[u8], info: &mut PVRTCTexelInfo) {
    info.punch_through_flag = 0;

    let mod_mode: bool = data[4] & 1 != 0;
    let mut mod_bits: usize = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;

    if mod_mode {
        let fillflag = if (data[0] & 1) != 0 {
            if (data[2] & 0x10) != 0 {
                -1
            } else {
                -2
            }
        } else {
            -3
        };

        let mut i: usize = 0;
        for y in 0..4 {
            if y & 1 == 0 {
                i += 1;
            } else {
                i -= 1;
            }
            for _x in 0..4 {
                info.weight[i] = fillflag;
                i += 2;
            }
        }

        i = 1;
        for y in 0..4 {
            if y & 1 == 0 {
                i -= 1;
            } else {
                i += 1;
            }
            for _x in 0..4 {
                info.weight[i] = PVRTC1_STANDARD_WEIGHT[mod_bits & 3];
                i += 2;
                mod_bits >>= 2;
            }
        }
        info.weight[0] = (info.weight[0] + 3) & 8;
        if (data[0] & 1) != 0 {
            info.weight[20] = (info.weight[20] + 3) & 8;
        }
    } else {
        for i in 0..32 {
            info.weight[i] = if (mod_bits & 1) != 0 { 8 } else { 0 };
            mod_bits >>= 1;
        }
    }
}

#[inline]
const fn interpolate_color(c1: i32, c2: i32, w: i8) -> u8 {
    let w = w as i32;
    let c = c1 * (8 - w) + c2 * w;
    (c / 8) as u8
}

fn applicate_color_4bpp(_data: &[u8], info: &mut [PVRTCTexelInfo; 9], buf: &mut [u32; 32]) {
    static INTERP_WEIGHT: [[i32; 3]; 4] = [[2, 2, 0], [1, 3, 0], [0, 4, 0], [0, 3, 1]];
    let mut clr_a: [PVRTCTexelColorInt; 16] = [PVRTCTexelColorInt::default(); 16];
    let mut clr_b: [PVRTCTexelColorInt; 16] = [PVRTCTexelColorInt::default(); 16];

    let mut i = 0;
    (0..4).for_each(|y| {
        (0..4).for_each(|x| {
            let mut ac = 0;
            for acy in 0..3 {
                for acx in 0..3 {
                    let interp_weight: i32 = INTERP_WEIGHT[x][acx] * INTERP_WEIGHT[y][acy];
                    clr_a[i].r += info[ac].a.r as i32 * interp_weight;
                    clr_a[i].g += info[ac].a.g as i32 * interp_weight;
                    clr_a[i].b += info[ac].a.b as i32 * interp_weight;
                    clr_a[i].a += info[ac].a.a as i32 * interp_weight;
                    clr_b[i].r += info[ac].b.r as i32 * interp_weight;
                    clr_b[i].g += info[ac].b.g as i32 * interp_weight;
                    clr_b[i].b += info[ac].b.b as i32 * interp_weight;
                    clr_b[i].a += info[ac].b.a as i32 * interp_weight;

                    ac += 1;
                }
            }
            clr_a[i].r = (clr_a[i].r >> 1) + (clr_a[i].r >> 6);
            clr_a[i].g = (clr_a[i].g >> 1) + (clr_a[i].g >> 6);
            clr_a[i].b = (clr_a[i].b >> 1) + (clr_a[i].b >> 6);
            clr_a[i].a = (clr_a[i].a) + (clr_a[i].a >> 4);
            clr_b[i].r = (clr_b[i].r >> 1) + (clr_b[i].r >> 6);
            clr_b[i].g = (clr_b[i].g >> 1) + (clr_b[i].g >> 6);
            clr_b[i].b = (clr_b[i].b >> 1) + (clr_b[i].b >> 6);
            clr_b[i].a = (clr_b[i].a) + (clr_b[i].a >> 4);

            i += 1;
        });
    });

    let self_info: &PVRTCTexelInfo = &info[4];
    let mut punch_through_flag: u32 = self_info.punch_through_flag;
    for i in 0..16 {
        buf[i] = color(
            interpolate_color(clr_a[i].r, clr_b[i].r, self_info.weight[i]),
            interpolate_color(clr_a[i].g, clr_b[i].g, self_info.weight[i]),
            interpolate_color(clr_a[i].b, clr_b[i].b, self_info.weight[i]),
            if punch_through_flag & 1 != 0 {
                0
            } else {
                interpolate_color(clr_a[i].a, clr_b[i].a, self_info.weight[i])
            },
        );

        punch_through_flag >>= 1;
    }
}

fn applicate_color_2bpp(_data: &[u8], info: &mut [PVRTCTexelInfo; 9], buf: &mut [u32; 32]) {
    static INTERP_WEIGHT_X: [[i32; 3]; 8] = [
        [4, 4, 0],
        [3, 5, 0],
        [2, 6, 0],
        [1, 7, 0],
        [0, 8, 0],
        [0, 7, 1],
        [0, 6, 2],
        [0, 5, 3],
    ];
    static INTERP_WEIGHT_Y: [[i32; 3]; 4] = [[2, 2, 0], [1, 3, 0], [0, 4, 0], [0, 3, 1]];
    let mut clr_a: [PVRTCTexelColorInt; 32] = [PVRTCTexelColorInt::default(); 32];
    let mut clr_b: [PVRTCTexelColorInt; 32] = [PVRTCTexelColorInt::default(); 32];

    let mut i = 0;
    (0..4).for_each(|y| {
        (0..8).for_each(|x| {
            let mut ac = 0;
            for acy in 0..3 {
                for acx in 0..3 {
                    let interp_weight: i32 = INTERP_WEIGHT_X[x][acx] * INTERP_WEIGHT_Y[y][acy];
                    clr_a[i].r += info[ac].a.r as i32 * interp_weight;
                    clr_a[i].g += info[ac].a.g as i32 * interp_weight;
                    clr_a[i].b += info[ac].a.b as i32 * interp_weight;
                    clr_a[i].a += info[ac].a.a as i32 * interp_weight;
                    clr_b[i].r += info[ac].b.r as i32 * interp_weight;
                    clr_b[i].g += info[ac].b.g as i32 * interp_weight;
                    clr_b[i].b += info[ac].b.b as i32 * interp_weight;
                    clr_b[i].a += info[ac].b.a as i32 * interp_weight;
                    ac += 1;
                }
            }

            clr_a[i].r = (clr_a[i].r >> 2) + (clr_a[i].r >> 7);
            clr_a[i].g = (clr_a[i].g >> 2) + (clr_a[i].g >> 7);
            clr_a[i].b = (clr_a[i].b >> 2) + (clr_a[i].b >> 7);
            clr_a[i].a = (clr_a[i].a >> 1) + (clr_a[i].a >> 5);
            clr_b[i].r = (clr_b[i].r >> 2) + (clr_b[i].r >> 7);
            clr_b[i].g = (clr_b[i].g >> 2) + (clr_b[i].g >> 7);
            clr_b[i].b = (clr_b[i].b >> 2) + (clr_b[i].b >> 7);
            clr_b[i].a = (clr_b[i].a >> 1) + (clr_b[i].a >> 5);

            i += 1;
        });
    });

    static POSYA: [[i32; 2]; 4] = [[1, 24], [4, -8], [4, -8], [4, -8]];
    static POSYB: [[i32; 2]; 4] = [[4, 8], [4, 8], [4, 8], [7, -24]];
    static POSXL: [[i32; 2]; 8] = [
        [3, 7],
        [4, -1],
        [4, -1],
        [4, -1],
        [4, -1],
        [4, -1],
        [4, -1],
        [4, -1],
    ];
    static POSXR: [[i32; 2]; 8] = [
        [4, 1],
        [4, 1],
        [4, 1],
        [4, 1],
        [4, 1],
        [4, 1],
        [4, 1],
        [5, -7],
    ];

    let mut self_info: PVRTCTexelInfo = info[4];
    let mut punch_through_flag: u32 = self_info.punch_through_flag;

    let mut i = 0;
    for y in 0..4 {
        for x in 0..8 {
            match self_info.weight[i] {
                -1 => {
                    self_info.weight[i] = ((info[POSYA[y][0] as usize].weight
                        [(i as i32 + POSYA[y][1]) as usize]
                        as i32
                        + info[POSYB[y][0] as usize].weight[(i as i32 + POSYB[y][1]) as usize]
                            as i32
                        + 1)
                        / 2) as i8;
                }
                -2 => {
                    self_info.weight[i] = ((info[POSXL[x][0] as usize].weight
                        [(i as i32 + POSXL[x][1]) as usize]
                        as i32
                        + info[POSXR[x][0] as usize].weight[(i as i32 + POSXR[x][1]) as usize]
                            as i32
                        + 1)
                        / 2) as i8;
                }
                -3 => {
                    self_info.weight[i] = ((info[POSYA[y][0] as usize].weight
                        [(i as i32 + POSYA[y][1]) as usize]
                        as i32
                        + info[POSYB[y][0] as usize].weight[(i as i32 + POSYB[y][1]) as usize]
                            as i32
                        + info[POSXL[x][0] as usize].weight[(i as i32 + POSXL[x][1]) as usize]
                            as i32
                        + info[POSXR[x][0] as usize].weight[(i as i32 + POSXR[x][1]) as usize]
                            as i32
                        + 2)
                        / 4) as i8;
                }
                _ => {}
            }

            buf[i] = color(
                interpolate_color(clr_a[i].r, clr_b[i].r, self_info.weight[i]),
                interpolate_color(clr_a[i].g, clr_b[i].g, self_info.weight[i]),
                interpolate_color(clr_a[i].b, clr_b[i].b, self_info.weight[i]),
                if punch_through_flag & 1 != 0 {
                    0
                } else {
                    interpolate_color(clr_a[i].a, clr_b[i].a, self_info.weight[i])
                },
            );
            i += 1;
            punch_through_flag >>= 1;
        }
    }
}

pub fn decode_pvrtc(data: &[u8], w: usize, h: usize, image: &mut [u32], is2bpp: bool) {
    let bw: usize = if is2bpp { 8 } else { 4 };
    let num_blocks_x: usize = if is2bpp { (w + 7) / 8 } else { (w + 3) / 4 };
    let num_blocks_y: usize = (h + 3) / 4;
    let num_blocks: usize = num_blocks_x * num_blocks_y;
    let min_num_blocks: usize = if num_blocks_x <= num_blocks_y {
        num_blocks_x
    } else {
        num_blocks_y
    };

    if ((num_blocks_x & (num_blocks_x - 1)) != 0) || ((num_blocks_y & (num_blocks_y - 1)) != 0) {
        panic!("the number of blocks of each side must be a power of 2");
    }

    let mut texel_info: Vec<PVRTCTexelInfo> = Vec::with_capacity(num_blocks);
    texel_info.fill(PVRTCTexelInfo::default());

    let get_texel_weights_func = if is2bpp {
        get_texel_weights_2bpp
    } else {
        get_texel_weights_4bpp
    };
    let applicate_color_func = if is2bpp {
        applicate_color_2bpp
    } else {
        applicate_color_4bpp
    };

    let mut data_offset: usize = 0;
    texel_info.iter_mut().for_each(|info| {
        get_texel_colors(&data[data_offset..], info);
        get_texel_weights_func(&data[data_offset..], info);
        data_offset += 8;
    });

    let mut buffer: [u32; 32] = [0; 32];
    let mut local_info: [PVRTCTexelInfo; 9] = [PVRTCTexelInfo::default(); 9];
    let mut pos_x: [usize; 3] = [0; 3];
    let mut pos_y: [usize; 3] = [0; 3];

    for by in 0..num_blocks_y {
        pos_y[0] = if by == 0 { num_blocks_y - 1 } else { by - 1 };
        pos_y[1] = by;
        pos_y[2] = if by == num_blocks_y - 1 { 0 } else { by + 1 };

        for bx in 0..num_blocks_x {
            pos_x[0] = if bx == 0 { num_blocks_x - 1 } else { bx - 1 };
            pos_x[1] = bx;
            pos_x[2] = if bx == num_blocks_x - 1 { 0 } else { bx + 1 };

            let mut c: usize = 0;
            for cy in 0..3 {
                for cx in 0..3 {
                    local_info[c] = texel_info[morton_index(pos_x[cx], pos_y[cy], min_num_blocks)];
                    c += 1;
                }
            }

            applicate_color_func(
                &data[morton_index(bx, by, min_num_blocks) * 8..],
                &mut local_info,
                &mut buffer,
            );
            copy_block_buffer(bx, by, w, h, bw, 4, &buffer, image);
        }
    }
}
