#![allow(clippy::identity_op)]
use crate::bcn::decode_bc3_alpha;
use crate::color::{color, copy_block_buffer};
use core::cmp::max;

#[inline]
const fn expand_quantized(v: u8, bits: u8) -> u8 {
    let v = v << (8 - bits);
    v | (v >> bits)
}

fn decode_atc_block(_src: &[u8], _dst: &mut [u32]) {
    let mut colors: [u8; 16] = [0; 16];
    let c0: u32 = u16::from_le_bytes([_src[0], _src[1]]) as u32;
    let c1: u32 = u16::from_le_bytes([_src[2], _src[3]]) as u32;

    if 0 == (c0 & 0x8000) {
        colors[0] = expand_quantized(((c0 >> 0) & 0x1f) as u8, 5);
        colors[1] = expand_quantized(((c0 >> 5) & 0x1f) as u8, 5);
        colors[2] = expand_quantized(((c0 >> 10) & 0x1f) as u8, 5);

        colors[12] = expand_quantized(((c1 >> 0) & 0x1f) as u8, 5);
        colors[13] = expand_quantized(((c1 >> 5) & 0x3f) as u8, 6);
        colors[14] = expand_quantized(((c1 >> 11) & 0x1f) as u8, 5);

        #[inline]
        fn interop_colors(c0: u8, c1: u8) -> u8 {
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

        colors[4] = max(0, colors[8] - colors[12] / 4);
        colors[5] = max(0, colors[9] - colors[13] / 4);
        colors[6] = max(0, colors[10] - colors[14] / 4);
    }

    let mut next = 8 * 4;
    (0..16).for_each(|i| {
        let idx = (((_src[next >> 3] >> (next & 7)) & 3) * 4) as usize;
        _dst[i] = color(colors[idx + 2], colors[idx + 1], colors[idx + 0], 255);
        next += 2;
    });
}

pub fn decode_atc_rgb4(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x: usize = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y: usize = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0; 16];

    let mut data_offset = 0;
    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_atc_block(&data[data_offset..], &mut buffer);
            copy_block_buffer(
                bx,
                by,
                m_width,
                m_height,
                m_block_width,
                m_block_height,
                &buffer,
                image,
            );
            data_offset += 8;
        });
    });
}

pub fn decode_atc_rgba8(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x: usize = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y: usize = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0; 16];

    let mut data_offset = 0;
    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_atc_block(&data[data_offset + 8..], &mut buffer);
            decode_bc3_alpha(&data[data_offset..], &mut buffer, 3);
            copy_block_buffer(
                bx,
                by,
                m_width,
                m_height,
                m_block_width,
                m_block_height,
                &buffer,
                image,
            );
            data_offset += 16;
        });
    });
}
