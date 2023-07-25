use crate::bcn::consts::{S_BPTC_A2, S_BPTC_A3, S_BPTC_FACTORS, S_BPTC_P2, S_BPTC_P3};
use crate::bitreader::BitReader;
use crate::color::color;
use core::mem::swap;

struct Bc7ModeInfo {
    num_subsets: usize,
    partition_bits: usize,
    rotation_bits: usize,
    index_selection_bits: usize,
    color_bits: usize,
    alpha_bits: usize,
    endpoint_pbits: usize,
    shared_pbits: usize,
    index_bits: [usize; 2],
}

static S_BP7_MODE_INFO: [Bc7ModeInfo; 8] = [
    //  +---------------------------- num subsets
    //  |  +------------------------- partition bits
    //  |  |  +---------------------- rotation bits
    //  |  |  |  +------------------- index selection bits
    //  |  |  |  |  +---------------- color bits
    //  |  |  |  |  |  +------------- alpha bits
    //  |  |  |  |  |  |  +---------- endpoint P-bits
    //  |  |  |  |  |  |  |  +------- shared P-bits
    //  |  |  |  |  |  |  |  |    +-- 2x index bits
    // { 3, 4, 0, 0, 4, 0, 1, 0, { 3, 0 } }, // 0
    // { 2, 6, 0, 0, 6, 0, 0, 1, { 3, 0 } }, // 1
    // { 3, 6, 0, 0, 5, 0, 0, 0, { 2, 0 } }, // 2
    // { 2, 6, 0, 0, 7, 0, 1, 0, { 2, 0 } }, // 3
    // { 1, 0, 2, 1, 5, 6, 0, 0, { 2, 3 } }, // 4
    // { 1, 0, 2, 0, 7, 8, 0, 0, { 2, 2 } }, // 5
    // { 1, 0, 0, 0, 7, 7, 1, 0, { 4, 0 } }, // 6
    // { 2, 6, 0, 0, 5, 5, 1, 0, { 2, 0 } }, // 7
    Bc7ModeInfo {
        num_subsets: 3,
        partition_bits: 4,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 4,
        alpha_bits: 0,
        endpoint_pbits: 1,
        shared_pbits: 0,
        index_bits: [3, 0],
    },
    Bc7ModeInfo {
        num_subsets: 2,
        partition_bits: 6,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 6,
        alpha_bits: 0,
        endpoint_pbits: 0,
        shared_pbits: 1,
        index_bits: [3, 0],
    },
    Bc7ModeInfo {
        num_subsets: 3,
        partition_bits: 6,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 5,
        alpha_bits: 0,
        endpoint_pbits: 0,
        shared_pbits: 0,
        index_bits: [2, 0],
    },
    Bc7ModeInfo {
        num_subsets: 2,
        partition_bits: 6,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 7,
        alpha_bits: 0,
        endpoint_pbits: 1,
        shared_pbits: 0,
        index_bits: [2, 0],
    },
    Bc7ModeInfo {
        num_subsets: 1,
        partition_bits: 0,
        rotation_bits: 2,
        index_selection_bits: 1,
        color_bits: 5,
        alpha_bits: 6,
        endpoint_pbits: 0,
        shared_pbits: 0,
        index_bits: [2, 3],
    },
    Bc7ModeInfo {
        num_subsets: 1,
        partition_bits: 0,
        rotation_bits: 2,
        index_selection_bits: 0,
        color_bits: 7,
        alpha_bits: 8,
        endpoint_pbits: 0,
        shared_pbits: 0,
        index_bits: [2, 2],
    },
    Bc7ModeInfo {
        num_subsets: 1,
        partition_bits: 0,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 7,
        alpha_bits: 7,
        endpoint_pbits: 1,
        shared_pbits: 0,
        index_bits: [4, 0],
    },
    Bc7ModeInfo {
        num_subsets: 2,
        partition_bits: 6,
        rotation_bits: 0,
        index_selection_bits: 0,
        color_bits: 5,
        alpha_bits: 5,
        endpoint_pbits: 1,
        shared_pbits: 0,
        index_bits: [2, 0],
    },
];

#[inline]
fn expand_quantized(v: u8, bits: usize) -> u8 {
    let s = ((v as u16) << (8 - bits as u16)) as u8;
    s | s.overflowing_shr(bits as u32).0
}

pub fn decode_bc7_block(data: &[u8], outbuf: &mut [u32]) {
    let mut bit = BitReader::new(data, 0);
    let mode = {
        let mut mode = 0;
        while 0 == bit.read(1) && mode < 8 {
            mode += 1;
        }
        mode
    };

    if mode == 8 {
        outbuf[0..16].fill(0);
        return;
    }

    let mi: &Bc7ModeInfo = &S_BP7_MODE_INFO[mode];
    let mode_pbits: usize = if 0 != mi.endpoint_pbits {
        mi.endpoint_pbits
    } else {
        mi.shared_pbits
    };

    let partition_set_idx: usize = bit.read(mi.partition_bits) as usize;
    let rotation_mode: u8 = bit.read(mi.rotation_bits) as u8;
    let index_selection_mode: usize = bit.read(mi.index_selection_bits) as usize;

    let mut ep_r: [u8; 6] = [0; 6];
    let mut ep_g: [u8; 6] = [0; 6];
    let mut ep_b: [u8; 6] = [0; 6];
    let mut ep_a: [u8; 6] = [0; 6];

    (0..mi.num_subsets).for_each(|ii| {
        ep_r[ii * 2] = (bit.read(mi.color_bits) << mode_pbits) as u8;
        ep_r[ii * 2 + 1] = (bit.read(mi.color_bits) << mode_pbits) as u8;
    });

    (0..mi.num_subsets).for_each(|ii| {
        ep_g[ii * 2] = (bit.read(mi.color_bits) << mode_pbits) as u8;
        ep_g[ii * 2 + 1] = (bit.read(mi.color_bits) << mode_pbits) as u8;
    });

    (0..mi.num_subsets).for_each(|ii| {
        ep_b[ii * 2] = (bit.read(mi.color_bits) << mode_pbits) as u8;
        ep_b[ii * 2 + 1] = (bit.read(mi.color_bits) << mode_pbits) as u8;
    });

    if mi.alpha_bits > 0 {
        (0..mi.num_subsets).for_each(|ii| {
            ep_a[ii * 2] = (bit.read(mi.alpha_bits) << mode_pbits) as u8;
            ep_a[ii * 2 + 1] = (bit.read(mi.alpha_bits) << mode_pbits) as u8;
        });
    } else {
        ep_a = [0xff; 6];
    }

    if 0 != mode_pbits {
        (0..mi.num_subsets).for_each(|ii| {
            let pda: u8 = bit.read(mode_pbits) as u8;
            let pdb: u8 = if 0 == mi.shared_pbits {
                bit.read(mode_pbits) as u8
            } else {
                pda
            };

            ep_r[ii * 2] |= pda;
            ep_r[ii * 2 + 1] |= pdb;
            ep_g[ii * 2] |= pda;
            ep_g[ii * 2 + 1] |= pdb;
            ep_b[ii * 2] |= pda;
            ep_b[ii * 2 + 1] |= pdb;
            ep_a[ii * 2] |= pda;
            ep_a[ii * 2 + 1] |= pdb;
        });
    }

    let color_bits: usize = mi.color_bits + mode_pbits;

    (0..mi.num_subsets).for_each(|ii| {
        ep_r[ii * 2] = expand_quantized(ep_r[ii * 2], color_bits);
        ep_r[ii * 2 + 1] = expand_quantized(ep_r[ii * 2 + 1], color_bits);
        ep_g[ii * 2] = expand_quantized(ep_g[ii * 2], color_bits);
        ep_g[ii * 2 + 1] = expand_quantized(ep_g[ii * 2 + 1], color_bits);
        ep_b[ii * 2] = expand_quantized(ep_b[ii * 2], color_bits);
        ep_b[ii * 2 + 1] = expand_quantized(ep_b[ii * 2 + 1], color_bits);
    });

    if mi.alpha_bits > 0 {
        let alpha_bits = mi.alpha_bits + mode_pbits;

        (0..mi.num_subsets).for_each(|ii| {
            ep_a[ii * 2] = expand_quantized(ep_a[ii * 2], alpha_bits);
            ep_a[ii * 2 + 1] = expand_quantized(ep_a[ii * 2 + 1], alpha_bits);
        });
    }

    let has_index_bits1: bool = 0 != mi.index_bits[1];

    let factors: [[u8; 16]; 2] = [
        S_BPTC_FACTORS[mi.index_bits[0] - 2],
        if has_index_bits1 {
            S_BPTC_FACTORS[mi.index_bits[1] - 2]
        } else {
            S_BPTC_FACTORS[mi.index_bits[0] - 2]
        },
    ];

    let mut offset: [usize; 2] = [0, mi.num_subsets * (16 * mi.index_bits[0] - 1)];

    (0..4_usize).for_each(|yy| {
        (0..4_usize).for_each(|xx| {
            let idx = yy * 4 + xx;

            let mut subset_index: usize = 0;
            let mut index_anchor: usize = 0;
            match mi.num_subsets {
                2 => {
                    subset_index = (S_BPTC_P2[partition_set_idx] >> idx) & 1;
                    index_anchor = if 0 != subset_index {
                        S_BPTC_A2[partition_set_idx]
                    } else {
                        0
                    };
                }
                3 => {
                    subset_index = (S_BPTC_P3[partition_set_idx] >> (2 * idx)) & 3;
                    index_anchor = if 0 != subset_index {
                        S_BPTC_A3[subset_index - 1][partition_set_idx]
                    } else {
                        0
                    };
                }
                _ => {}
            }

            let anchor = idx == index_anchor;
            let num: [usize; 2] = [
                (mi.index_bits[0] - anchor as usize),
                if has_index_bits1 {
                    mi.index_bits[1] - anchor as usize
                } else {
                    0
                },
            ];

            let index: [usize; 2] = {
                let index_0 = bit.peek(offset[0], num[0]) as usize;
                [
                    index_0,
                    if has_index_bits1 {
                        bit.peek(offset[1], num[1]) as usize
                    } else {
                        index_0
                    },
                ]
            };

            offset[0] += num[0];
            offset[1] += num[1];

            // index selection mode 0 or 1
            // !index_selection_mode == 1-index_selection_mode
            let fc: u16 = factors[index_selection_mode][index[index_selection_mode]] as u16;
            let fa: u16 = factors[1 - index_selection_mode][index[1 - index_selection_mode]] as u16;

            let fca: u16 = 64 - fc;
            let fcb: u16 = fc;
            let faa: u16 = 64 - fa;
            let fab: u16 = fa;

            subset_index *= 2;
            let mut rr: u8 =
                ((ep_r[subset_index] as u16 * fca + ep_r[subset_index + 1] as u16 * fcb + 32) >> 6)
                    as u8;
            let mut gg: u8 =
                ((ep_g[subset_index] as u16 * fca + ep_g[subset_index + 1] as u16 * fcb + 32) >> 6)
                    as u8;
            let mut bb: u8 =
                ((ep_b[subset_index] as u16 * fca + ep_b[subset_index + 1] as u16 * fcb + 32) >> 6)
                    as u8;
            let mut aa: u8 =
                ((ep_a[subset_index] as u16 * faa + ep_a[subset_index + 1] as u16 * fab + 32) >> 6)
                    as u8;

            match rotation_mode {
                1 => {
                    swap(&mut aa, &mut rr);
                }
                2 => {
                    swap(&mut aa, &mut gg);
                }
                3 => {
                    swap(&mut aa, &mut bb);
                }
                _ => {}
            }
            outbuf[idx] = color(rr, gg, bb, aa);
        });
    });
}
