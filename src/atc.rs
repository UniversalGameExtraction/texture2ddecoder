#![allow(clippy::identity_op)]
use crate::bcn::bc3::decode_bc3_alpha;
use crate::color::color;
use crate::macros::block_decoder;
use core::cmp::max;
use core::result::Result;

#[inline]
const fn expand_quantized(v: u8, bits: u8) -> u8 {
    let v = v << (8 - bits);
    v | (v >> bits)
}

#[inline]
pub fn decode_atc_rgb4_block(data: &[u8], outbuf: &mut [u32]) {
    let mut colors: [u8; 16] = [0; 16];
    let c0: u32 = u16::from_le_bytes([data[0], data[1]]) as u32;
    let c1: u32 = u16::from_le_bytes([data[2], data[3]]) as u32;

    if 0 == (c0 & 0x8000) {
        colors[0] = expand_quantized(((c0 >> 0) & 0x1f) as u8, 5);
        colors[1] = expand_quantized(((c0 >> 5) & 0x1f) as u8, 5);
        colors[2] = expand_quantized(((c0 >> 10) & 0x1f) as u8, 5);

        colors[12] = expand_quantized(((c1 >> 0) & 0x1f) as u8, 5);
        colors[13] = expand_quantized(((c1 >> 5) & 0x3f) as u8, 6);
        colors[14] = expand_quantized(((c1 >> 11) & 0x1f) as u8, 5);

        #[inline]
        const fn interop_colors(c0: u8, c1: u8) -> u8 {
            ((5 * c0 as u16 + 3 * c1 as u16) / 8) as u8
        }
        // colors[4] = (5 * colors[0] + 3 * colors[12]) / 8;
        // colors[5] = (5 * colors[1] + 3 * colors[13]) / 8;
        // colors[6] = (5 * colors[2] + 3 * colors[14]) / 8;

        // colors[8] = (3 * colors[0] + 5 * colors[12]) / 8;
        // colors[9] = (3 * colors[1] + 5 * colors[13]) / 8;
        // colors[10] = (3 * colors[2] + 5 * colors[14]) / 8;

        colors[4] = interop_colors(colors[0], colors[12]);
        colors[5] = interop_colors(colors[1], colors[13]);
        colors[6] = interop_colors(colors[2], colors[14]);

        colors[8] = interop_colors(colors[12], colors[0]);
        colors[9] = interop_colors(colors[13], colors[1]);
        colors[10] = interop_colors(colors[14], colors[2]);
    } else {
        colors[0] = 0;
        colors[1] = 0;
        colors[2] = 0;

        colors[8] = expand_quantized(((c0 >> 0) & 0x1f) as u8, 5);
        colors[9] = expand_quantized(((c0 >> 5) & 0x1f) as u8, 5);
        colors[10] = expand_quantized(((c0 >> 10) & 0x1f) as u8, 5);

        colors[12] = expand_quantized(((c1 >> 0) & 0x1f) as u8, 5);
        colors[13] = expand_quantized(((c1 >> 5) & 0x3f) as u8, 6);
        colors[14] = expand_quantized(((c1 >> 11) & 0x1f) as u8, 5);

        colors[4] = max(
            0,
            ((colors[8] as u16).overflowing_sub(colors[12] as u16).0 / 4) as u8,
        );
        colors[5] = max(
            0,
            ((colors[9] as u16).overflowing_sub(colors[13] as u16).0 / 4) as u8,
        );
        colors[6] = max(
            0,
            ((colors[10] as u16).overflowing_sub(colors[14] as u16).0 / 4) as u8,
        );
    }

    let mut next = 8 * 4;
    (0..16).for_each(|i| {
        let idx = (((data[next >> 3] >> (next & 7)) & 3) * 4) as usize;
        outbuf[i] = color(colors[idx + 2], colors[idx + 1], colors[idx + 0], 255);
        next += 2;
    });
}

#[inline]
pub fn decode_atc_rgba8_block(data: &[u8], outbuf: &mut [u32]) {
    decode_atc_rgb4_block(&data[8..], outbuf);
    decode_bc3_alpha(data, outbuf, 3);
}

block_decoder!("atc_rgb4", 4, 4, 8, decode_atc_rgb4_block);
block_decoder!("atc_rgba8", 4, 4, 16, decode_atc_rgba8_block);
