use crate::bitreader::BitReader;
use crate::color::{color, copy_block_buffer, rgb565_le};
use core::mem::swap;
use half::f16;

#[inline]
fn decode_bc1_block(data: &[u8], outbuf: &mut [u32]) {
    let q0 = u16::from_le_bytes([data[0], data[1]]);
    let q1 = u16::from_le_bytes([data[2], data[3]]);
    let (r0, g0, b0) = rgb565_le(q0);
    let (r1, g1, b1) = rgb565_le(q1);

    let mut c: [u32; 4] = [color(r0, g0, b0, 255), color(r1, g1, b1, 255), 0, 0];

    // C insanity.....
    let r0 = r0 as u16;
    let g0 = g0 as u16;
    let b0 = b0 as u16;
    let r1 = r1 as u16;
    let g1 = g1 as u16;
    let b1 = b1 as u16;

    if q0 > q1 {
        c[2] = color(
            ((r0 * 2 + r1) / 3) as u8,
            ((g0 * 2 + g1) / 3) as u8,
            ((b0 * 2 + b1) / 3) as u8,
            255,
        );
        c[3] = color(
            ((r0 + r1 * 2) / 3) as u8,
            ((g0 + g1 * 2) / 3) as u8,
            ((b0 + b1 * 2) / 3) as u8,
            255,
        );
    } else {
        c[2] = color(
            ((r0 + r1) / 2) as u8,
            ((g0 + g1) / 2) as u8,
            ((b0 + b1) / 2) as u8,
            255,
        );
        c[3] = color(0, 0, 0, 255);
    }
    let mut d: usize = u32::from_be_bytes(data[4..8].try_into().unwrap()) as usize;
    (0..16).for_each(|i| {
        outbuf[i] = c[d & 3];
        d >>= 2;
    });
}

pub fn decode_bc1(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];

    let mut data_offset: usize = 0;
    for by in 0..num_blocks_y {
        for bx in 0..num_blocks_x {
            decode_bc1_block(&data[data_offset..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_offset += 8;
        }
    }
}

pub fn decode_bc3_alpha(data: &[u8], outbuf: &mut [u32], channel: usize) {
    // use u16 to avoid overflow and replicate equivalent behavior to C++ code
    let mut a: [u16; 8] = [data[0] as u16, data[1] as u16, 0, 0, 0, 0, 0, 0];
    if a[0] > a[1] {
        a[2] = (a[0] * 6 + a[1]) / 7;
        a[3] = (a[0] * 5 + a[1] * 2) / 7;
        a[4] = (a[0] * 4 + a[1] * 3) / 7;
        a[5] = (a[0] * 3 + a[1] * 4) / 7;
        a[6] = (a[0] * 2 + a[1] * 5) / 7;
        a[7] = (a[0] + a[1] * 6) / 7;
    } else {
        a[2] = (a[0] * 4 + a[1]) / 5;
        a[3] = (a[0] * 3 + a[1] * 2) / 5;
        a[4] = (a[0] * 2 + a[1] * 3) / 5;
        a[5] = (a[0] + a[1] * 4) / 5;
        a[6] = 0;
        a[7] = 255;
    }

    let mut d: usize = (u64::from_le_bytes(data[..8].try_into().unwrap()) >> 16) as usize;

    let channel_shift = channel * 8;
    let channel_mask = 0xFFFFFFFF ^ (0xFF << channel_shift);
    outbuf.iter_mut().for_each(|p| {
        *p = (*p & channel_mask) | (a[d & 7] as u32) << channel_shift;
        d >>= 3;
    });
}

#[inline]
fn decode_bc3_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc1_block(&data[8..], outbuf);
    decode_bc3_alpha(data, outbuf, 3);
}

pub fn decode_bc3(data: &[u8], w: usize, h: usize, image: &mut [u32]) {
    let num_blocks_x: usize = (w + 3) / 4;
    let num_blocks_y: usize = (h + 3) / 4;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_offset: usize = 0;

    (0..num_blocks_y).for_each(|by| {
        (0..num_blocks_x).for_each(|bx| {
            decode_bc3_block(&data[data_offset..], &mut buffer);
            copy_block_buffer(bx, by, w, h, 4, 4, &buffer, image);
            data_offset += 16;
        });
    });
}

#[inline]
fn decode_bc4_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc3_alpha(data, outbuf, 2);
}

pub fn decode_bc4(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0xff000000; 16];
    let mut data_offset: usize = 0;

    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_bc4_block(&data[data_offset..], &mut buffer);
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

#[inline]
fn decode_bc5_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc3_alpha(data, outbuf, 2);
    decode_bc3_alpha(&data[8..], outbuf, 1);
}

pub fn decode_bc5(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0xff000000; 16];
    let mut data_offset: usize = 0;

    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_bc5_block(&data[data_offset..], &mut buffer);
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

// ===================== BC 6 & 7 =====================
static S_BPTC_P2: [usize; 64] = [
    //  3210     0000000000   1111111111   2222222222   3333333333
    0xcccc, // 0, 0, 1, 1,  0, 0, 1, 1,  0, 0, 1, 1,  0, 0, 1, 1
    0x8888, // 0, 0, 0, 1,  0, 0, 0, 1,  0, 0, 0, 1,  0, 0, 0, 1
    0xeeee, // 0, 1, 1, 1,  0, 1, 1, 1,  0, 1, 1, 1,  0, 1, 1, 1
    0xecc8, // 0, 0, 0, 1,  0, 0, 1, 1,  0, 0, 1, 1,  0, 1, 1, 1
    0xc880, // 0, 0, 0, 0,  0, 0, 0, 1,  0, 0, 0, 1,  0, 0, 1, 1
    0xfeec, // 0, 0, 1, 1,  0, 1, 1, 1,  0, 1, 1, 1,  1, 1, 1, 1
    0xfec8, // 0, 0, 0, 1,  0, 0, 1, 1,  0, 1, 1, 1,  1, 1, 1, 1
    0xec80, // 0, 0, 0, 0,  0, 0, 0, 1,  0, 0, 1, 1,  0, 1, 1, 1
    0xc800, // 0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1,  0, 0, 1, 1
    0xffec, // 0, 0, 1, 1,  0, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1
    0xfe80, // 0, 0, 0, 0,  0, 0, 0, 1,  0, 1, 1, 1,  1, 1, 1, 1
    0xe800, // 0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 1,  0, 1, 1, 1
    0xffe8, // 0, 0, 0, 1,  0, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1
    0xff00, // 0, 0, 0, 0,  0, 0, 0, 0,  1, 1, 1, 1,  1, 1, 1, 1
    0xfff0, // 0, 0, 0, 0,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1
    0xf000, // 0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  1, 1, 1, 1
    0xf710, // 0, 0, 0, 0,  1, 0, 0, 0,  1, 1, 1, 0,  1, 1, 1, 1
    0x008e, // 0, 1, 1, 1,  0, 0, 0, 1,  0, 0, 0, 0,  0, 0, 0, 0
    0x7100, // 0, 0, 0, 0,  0, 0, 0, 0,  1, 0, 0, 0,  1, 1, 1, 0
    0x08ce, // 0, 1, 1, 1,  0, 0, 1, 1,  0, 0, 0, 1,  0, 0, 0, 0
    0x008c, // 0, 0, 1, 1,  0, 0, 0, 1,  0, 0, 0, 0,  0, 0, 0, 0
    0x7310, // 0, 0, 0, 0,  1, 0, 0, 0,  1, 1, 0, 0,  1, 1, 1, 0
    0x3100, // 0, 0, 0, 0,  0, 0, 0, 0,  1, 0, 0, 0,  1, 1, 0, 0
    0x8cce, // 0, 1, 1, 1,  0, 0, 1, 1,  0, 0, 1, 1,  0, 0, 0, 1
    0x088c, // 0, 0, 1, 1,  0, 0, 0, 1,  0, 0, 0, 1,  0, 0, 0, 0
    0x3110, // 0, 0, 0, 0,  1, 0, 0, 0,  1, 0, 0, 0,  1, 1, 0, 0
    0x6666, // 0, 1, 1, 0,  0, 1, 1, 0,  0, 1, 1, 0,  0, 1, 1, 0
    0x366c, // 0, 0, 1, 1,  0, 1, 1, 0,  0, 1, 1, 0,  1, 1, 0, 0
    0x17e8, // 0, 0, 0, 1,  0, 1, 1, 1,  1, 1, 1, 0,  1, 0, 0, 0
    0x0ff0, // 0, 0, 0, 0,  1, 1, 1, 1,  1, 1, 1, 1,  0, 0, 0, 0
    0x718e, // 0, 1, 1, 1,  0, 0, 0, 1,  1, 0, 0, 0,  1, 1, 1, 0
    0x399c, // 0, 0, 1, 1,  1, 0, 0, 1,  1, 0, 0, 1,  1, 1, 0, 0
    0xaaaa, // 0, 1, 0, 1,  0, 1, 0, 1,  0, 1, 0, 1,  0, 1, 0, 1
    0xf0f0, // 0, 0, 0, 0,  1, 1, 1, 1,  0, 0, 0, 0,  1, 1, 1, 1
    0x5a5a, // 0, 1, 0, 1,  1, 0, 1, 0,  0, 1, 0, 1,  1, 0, 1, 0
    0x33cc, // 0, 0, 1, 1,  0, 0, 1, 1,  1, 1, 0, 0,  1, 1, 0, 0
    0x3c3c, // 0, 0, 1, 1,  1, 1, 0, 0,  0, 0, 1, 1,  1, 1, 0, 0
    0x55aa, // 0, 1, 0, 1,  0, 1, 0, 1,  1, 0, 1, 0,  1, 0, 1, 0
    0x9696, // 0, 1, 1, 0,  1, 0, 0, 1,  0, 1, 1, 0,  1, 0, 0, 1
    0xa55a, // 0, 1, 0, 1,  1, 0, 1, 0,  1, 0, 1, 0,  0, 1, 0, 1
    0x73ce, // 0, 1, 1, 1,  0, 0, 1, 1,  1, 1, 0, 0,  1, 1, 1, 0
    0x13c8, // 0, 0, 0, 1,  0, 0, 1, 1,  1, 1, 0, 0,  1, 0, 0, 0
    0x324c, // 0, 0, 1, 1,  0, 0, 1, 0,  0, 1, 0, 0,  1, 1, 0, 0
    0x3bdc, // 0, 0, 1, 1,  1, 0, 1, 1,  1, 1, 0, 1,  1, 1, 0, 0
    0x6996, // 0, 1, 1, 0,  1, 0, 0, 1,  1, 0, 0, 1,  0, 1, 1, 0
    0xc33c, // 0, 0, 1, 1,  1, 1, 0, 0,  1, 1, 0, 0,  0, 0, 1, 1
    0x9966, // 0, 1, 1, 0,  0, 1, 1, 0,  1, 0, 0, 1,  1, 0, 0, 1
    0x0660, // 0, 0, 0, 0,  0, 1, 1, 0,  0, 1, 1, 0,  0, 0, 0, 0
    0x0272, // 0, 1, 0, 0,  1, 1, 1, 0,  0, 1, 0, 0,  0, 0, 0, 0
    0x04e4, // 0, 0, 1, 0,  0, 1, 1, 1,  0, 0, 1, 0,  0, 0, 0, 0
    0x4e40, // 0, 0, 0, 0,  0, 0, 1, 0,  0, 1, 1, 1,  0, 0, 1, 0
    0x2720, // 0, 0, 0, 0,  0, 1, 0, 0,  1, 1, 1, 0,  0, 1, 0, 0
    0xc936, // 0, 1, 1, 0,  1, 1, 0, 0,  1, 0, 0, 1,  0, 0, 1, 1
    0x936c, // 0, 0, 1, 1,  0, 1, 1, 0,  1, 1, 0, 0,  1, 0, 0, 1
    0x39c6, // 0, 1, 1, 0,  0, 0, 1, 1,  1, 0, 0, 1,  1, 1, 0, 0
    0x639c, // 0, 0, 1, 1,  1, 0, 0, 1,  1, 1, 0, 0,  0, 1, 1, 0
    0x9336, // 0, 1, 1, 0,  1, 1, 0, 0,  1, 1, 0, 0,  1, 0, 0, 1
    0x9cc6, // 0, 1, 1, 0,  0, 0, 1, 1,  0, 0, 1, 1,  1, 0, 0, 1
    0x817e, // 0, 1, 1, 1,  1, 1, 1, 0,  1, 0, 0, 0,  0, 0, 0, 1
    0xe718, // 0, 0, 0, 1,  1, 0, 0, 0,  1, 1, 1, 0,  0, 1, 1, 1
    0xccf0, // 0, 0, 0, 0,  1, 1, 1, 1,  0, 0, 1, 1,  0, 0, 1, 1
    0x0fcc, // 0, 0, 1, 1,  0, 0, 1, 1,  1, 1, 1, 1,  0, 0, 0, 0
    0x7744, // 0, 0, 1, 0,  0, 0, 1, 0,  1, 1, 1, 0,  1, 1, 1, 0
    0xee22, // 0, 1, 0, 0,  0, 1, 0, 0,  0, 1, 1, 1,  0, 1, 1, 1
];

static S_BPTC_A2: [usize; 64] = [
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 2, 8, 2, 2, 8, 8, 15, 2, 8,
    2, 2, 8, 8, 2, 2, 15, 15, 6, 8, 2, 8, 15, 15, 2, 8, 2, 2, 2, 15, 15, 6, 6, 2, 6, 8, 15, 15, 2,
    2, 15, 15, 15, 15, 15, 2, 2, 15,
];

static S_BPTC_FACTORS: [[u8; 16]; 3] = [
    [0, 21, 43, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 9, 18, 27, 37, 46, 55, 64, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64],
];

static S_BPTC_P3: [usize; 64] = [
    //    76543210     0000   1111   2222   3333   4444   5555   6666   7777
    0xaa685050, // 0, 0,  1, 1,  0, 0,  1, 1,  0, 2,  2, 1,  2, 2,  2, 2
    0x6a5a5040, // 0, 0,  0, 1,  0, 0,  1, 1,  2, 2,  1, 1,  2, 2,  2, 1
    0x5a5a4200, // 0, 0,  0, 0,  2, 0,  0, 1,  2, 2,  1, 1,  2, 2,  1, 1
    0x5450a0a8, // 0, 2,  2, 2,  0, 0,  2, 2,  0, 0,  1, 1,  0, 1,  1, 1
    0xa5a50000, // 0, 0,  0, 0,  0, 0,  0, 0,  1, 1,  2, 2,  1, 1,  2, 2
    0xa0a05050, // 0, 0,  1, 1,  0, 0,  1, 1,  0, 0,  2, 2,  0, 0,  2, 2
    0x5555a0a0, // 0, 0,  2, 2,  0, 0,  2, 2,  1, 1,  1, 1,  1, 1,  1, 1
    0x5a5a5050, // 0, 0,  1, 1,  0, 0,  1, 1,  2, 2,  1, 1,  2, 2,  1, 1
    0xaa550000, // 0, 0,  0, 0,  0, 0,  0, 0,  1, 1,  1, 1,  2, 2,  2, 2
    0xaa555500, // 0, 0,  0, 0,  1, 1,  1, 1,  1, 1,  1, 1,  2, 2,  2, 2
    0xaaaa5500, // 0, 0,  0, 0,  1, 1,  1, 1,  2, 2,  2, 2,  2, 2,  2, 2
    0x90909090, // 0, 0,  1, 2,  0, 0,  1, 2,  0, 0,  1, 2,  0, 0,  1, 2
    0x94949494, // 0, 1,  1, 2,  0, 1,  1, 2,  0, 1,  1, 2,  0, 1,  1, 2
    0xa4a4a4a4, // 0, 1,  2, 2,  0, 1,  2, 2,  0, 1,  2, 2,  0, 1,  2, 2
    0xa9a59450, // 0, 0,  1, 1,  0, 1,  1, 2,  1, 1,  2, 2,  1, 2,  2, 2
    0x2a0a4250, // 0, 0,  1, 1,  2, 0,  0, 1,  2, 2,  0, 0,  2, 2,  2, 0
    0xa5945040, // 0, 0,  0, 1,  0, 0,  1, 1,  0, 1,  1, 2,  1, 1,  2, 2
    0x0a425054, // 0, 1,  1, 1,  0, 0,  1, 1,  2, 0,  0, 1,  2, 2,  0, 0
    0xa5a5a500, // 0, 0,  0, 0,  1, 1,  2, 2,  1, 1,  2, 2,  1, 1,  2, 2
    0x55a0a0a0, // 0, 0,  2, 2,  0, 0,  2, 2,  0, 0,  2, 2,  1, 1,  1, 1
    0xa8a85454, // 0, 1,  1, 1,  0, 1,  1, 1,  0, 2,  2, 2,  0, 2,  2, 2
    0x6a6a4040, // 0, 0,  0, 1,  0, 0,  0, 1,  2, 2,  2, 1,  2, 2,  2, 1
    0xa4a45000, // 0, 0,  0, 0,  0, 0,  1, 1,  0, 1,  2, 2,  0, 1,  2, 2
    0x1a1a0500, // 0, 0,  0, 0,  1, 1,  0, 0,  2, 2,  1, 0,  2, 2,  1, 0
    0x0050a4a4, // 0, 1,  2, 2,  0, 1,  2, 2,  0, 0,  1, 1,  0, 0,  0, 0
    0xaaa59090, // 0, 0,  1, 2,  0, 0,  1, 2,  1, 1,  2, 2,  2, 2,  2, 2
    0x14696914, // 0, 1,  1, 0,  1, 2,  2, 1,  1, 2,  2, 1,  0, 1,  1, 0
    0x69691400, // 0, 0,  0, 0,  0, 1,  1, 0,  1, 2,  2, 1,  1, 2,  2, 1
    0xa08585a0, // 0, 0,  2, 2,  1, 1,  0, 2,  1, 1,  0, 2,  0, 0,  2, 2
    0xaa821414, // 0, 1,  1, 0,  0, 1,  1, 0,  2, 0,  0, 2,  2, 2,  2, 2
    0x50a4a450, // 0, 0,  1, 1,  0, 1,  2, 2,  0, 1,  2, 2,  0, 0,  1, 1
    0x6a5a0200, // 0, 0,  0, 0,  2, 0,  0, 0,  2, 2,  1, 1,  2, 2,  2, 1
    0xa9a58000, // 0, 0,  0, 0,  0, 0,  0, 2,  1, 1,  2, 2,  1, 2,  2, 2
    0x5090a0a8, // 0, 2,  2, 2,  0, 0,  2, 2,  0, 0,  1, 2,  0, 0,  1, 1
    0xa8a09050, // 0, 0,  1, 1,  0, 0,  1, 2,  0, 0,  2, 2,  0, 2,  2, 2
    0x24242424, // 0, 1,  2, 0,  0, 1,  2, 0,  0, 1,  2, 0,  0, 1,  2, 0
    0x00aa5500, // 0, 0,  0, 0,  1, 1,  1, 1,  2, 2,  2, 2,  0, 0,  0, 0
    0x24924924, // 0, 1,  2, 0,  1, 2,  0, 1,  2, 0,  1, 2,  0, 1,  2, 0
    0x24499224, // 0, 1,  2, 0,  2, 0,  1, 2,  1, 2,  0, 1,  0, 1,  2, 0
    0x50a50a50, // 0, 0,  1, 1,  2, 2,  0, 0,  1, 1,  2, 2,  0, 0,  1, 1
    0x500aa550, // 0, 0,  1, 1,  1, 1,  2, 2,  2, 2,  0, 0,  0, 0,  1, 1
    0xaaaa4444, // 0, 1,  0, 1,  0, 1,  0, 1,  2, 2,  2, 2,  2, 2,  2, 2
    0x66660000, // 0, 0,  0, 0,  0, 0,  0, 0,  2, 1,  2, 1,  2, 1,  2, 1
    0xa5a0a5a0, // 0, 0,  2, 2,  1, 1,  2, 2,  0, 0,  2, 2,  1, 1,  2, 2
    0x50a050a0, // 0, 0,  2, 2,  0, 0,  1, 1,  0, 0,  2, 2,  0, 0,  1, 1
    0x69286928, // 0, 2,  2, 0,  1, 2,  2, 1,  0, 2,  2, 0,  1, 2,  2, 1
    0x44aaaa44, // 0, 1,  0, 1,  2, 2,  2, 2,  2, 2,  2, 2,  0, 1,  0, 1
    0x66666600, // 0, 0,  0, 0,  2, 1,  2, 1,  2, 1,  2, 1,  2, 1,  2, 1
    0xaa444444, // 0, 1,  0, 1,  0, 1,  0, 1,  0, 1,  0, 1,  2, 2,  2, 2
    0x54a854a8, // 0, 2,  2, 2,  0, 1,  1, 1,  0, 2,  2, 2,  0, 1,  1, 1
    0x95809580, // 0, 0,  0, 2,  1, 1,  1, 2,  0, 0,  0, 2,  1, 1,  1, 2
    0x96969600, // 0, 0,  0, 0,  2, 1,  1, 2,  2, 1,  1, 2,  2, 1,  1, 2
    0xa85454a8, // 0, 2,  2, 2,  0, 1,  1, 1,  0, 1,  1, 1,  0, 2,  2, 2
    0x80959580, // 0, 0,  0, 2,  1, 1,  1, 2,  1, 1,  1, 2,  0, 0,  0, 2
    0xaa141414, // 0, 1,  1, 0,  0, 1,  1, 0,  0, 1,  1, 0,  2, 2,  2, 2
    0x96960000, // 0, 0,  0, 0,  0, 0,  0, 0,  2, 1,  1, 2,  2, 1,  1, 2
    0xaaaa1414, // 0, 1,  1, 0,  0, 1,  1, 0,  2, 2,  2, 2,  2, 2,  2, 2
    0xa05050a0, // 0, 0,  2, 2,  0, 0,  1, 1,  0, 0,  1, 1,  0, 0,  2, 2
    0xa0a5a5a0, // 0, 0,  2, 2,  1, 1,  2, 2,  1, 1,  2, 2,  0, 0,  2, 2
    0x96000000, // 0, 0,  0, 0,  0, 0,  0, 0,  0, 0,  0, 0,  2, 1,  1, 2
    0x40804080, // 0, 0,  0, 2,  0, 0,  0, 1,  0, 0,  0, 2,  0, 0,  0, 1
    0xa9a8a9a8, // 0, 2,  2, 2,  1, 2,  2, 2,  0, 2,  2, 2,  1, 2,  2, 2
    0xaaaaaa44, // 0, 1,  0, 1,  2, 2,  2, 2,  2, 2,  2, 2,  2, 2,  2, 2
    0x2a4a5254, // 0, 1,  1, 1,  2, 0,  1, 1,  2, 2,  0, 1,  2, 2,  2, 0
];

static S_BPTC_A3: [[usize; 64]; 2] = [
    [
        3, 3, 15, 15, 8, 3, 15, 15, 8, 8, 6, 6, 6, 5, 3, 3, 3, 3, 8, 15, 3, 3, 6, 10, 5, 8, 8, 6,
        8, 5, 15, 15, 8, 15, 3, 5, 6, 10, 8, 15, 15, 3, 15, 5, 15, 15, 15, 15, 3, 15, 5, 5, 5, 8,
        5, 10, 5, 10, 8, 13, 15, 12, 3, 3,
    ],
    [
        15, 8, 8, 3, 15, 15, 3, 8, 15, 15, 15, 15, 15, 15, 15, 8, 15, 8, 15, 3, 15, 8, 15, 8, 3,
        15, 6, 10, 15, 15, 10, 8, 15, 3, 15, 10, 10, 8, 9, 10, 6, 15, 8, 15, 3, 6, 6, 8, 15, 3, 15,
        15, 15, 15, 15, 15, 15, 15, 15, 15, 3, 15, 15, 8,
    ],
];

struct Bc6hModeInfo {
    transformed: usize,
    partition_bits: usize,
    endpoint_bits: usize,
    delta_bits: [usize; 3],
}

static S_BC6H_MODE_INFO: [Bc6hModeInfo; 32] = [
    //  +--------------------------- transformed
    //  |  +------------------------ partition bits
    //  |  |  +--------------------- endpoint bits
    //  |  |  |      +-------------- delta bits
    // { 1, 5, 10, {  5,  5,  5 } }, // 00    2-bits
    // { 1, 5,  7, {  6,  6,  6 } }, // 01
    // { 1, 5, 11, {  5,  4,  4 } }, // 00010 5-bits
    // { 0, 0, 10, { 10, 10, 10 } }, // 00011
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5, 11, {  4,  5,  4 } }, // 00110
    // { 1, 0, 11, {  9,  9,  9 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5, 11, {  4,  4,  5 } }, // 00010
    // { 1, 0, 12, {  8,  8,  8 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5,  9, {  5,  5,  5 } }, // 00010
    // { 1, 0, 16, {  4,  4,  4 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5,  8, {  6,  5,  5 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5,  8, {  5,  6,  5 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 1, 5,  8, {  5,  5,  6 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // { 0, 5,  6, {  6,  6,  6 } }, // 00010
    // { 0, 0,  0, {  0,  0,  0 } }, // -
    // 00    2-bits
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 10,
        delta_bits: [5, 5, 5],
    },
    // 01
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 7,
        delta_bits: [6, 6, 6],
    },
    // 00010 5-bits
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [5, 4, 4],
    },
    // 00011
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 10,
        delta_bits: [10, 10, 10],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00110
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [4, 5, 4],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 0,
        endpoint_bits: 11,
        delta_bits: [9, 9, 9],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [4, 4, 5],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 0,
        endpoint_bits: 12,
        delta_bits: [8, 8, 8],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 9,
        delta_bits: [5, 5, 5],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 0,
        endpoint_bits: 16,
        delta_bits: [4, 4, 4],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [6, 5, 5],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [5, 6, 5],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 1,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [5, 5, 6],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 5,
        endpoint_bits: 6,
        delta_bits: [6, 6, 6],
    },
    // -
    Bc6hModeInfo {
        transformed: 0,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
];

fn unquantize(_value: u16, _signed: bool, _endpoint_bits: usize) -> u16 {
    let max_value = 1 << (_endpoint_bits - 1);

    if _signed {
        if _endpoint_bits >= 16 {
            return _value;
        }

        let sign: bool = _value & 0x8000 != 0;
        let _value = _value & 0x7fff;

        let unq: u16;

        if 0 == _value {
            unq = 0;
        } else if _value >= max_value - 1 {
            unq = 0x7fff;
        } else {
            unq = ((_value << 15) + 0x4000) >> (_endpoint_bits - 1);
        }

        return if sign { u16::MAX - unq + 1 } else { unq };
    }

    if _endpoint_bits >= 15 {
        return _value;
    }

    if 0 == _value {
        return 0;
    }

    if _value == max_value {
        return u16::MAX;
    }

    ((_value << 15) + 0x4000) >> (_endpoint_bits - 1)
}

fn finish_unquantize(_value: u16, _signed: bool) -> u16 {
    if _signed {
        let sign: u16 = _value & 0x8000;
        (((_value & 0x7fff) as u32 * 31) >> 5) as u16 | sign
    } else {
        ((_value as u32 * 31) >> 6) as u16
    }
}

fn sign_extend(_value: u16, _num_bits: usize) -> u16 {
    let mask: u16 = 1 << (_num_bits - 1);
    (_value ^ mask).overflowing_sub(mask).0
}

#[inline]
fn f32_to_u8(f: f32) -> u8 {
    let c = f * 255.0;
    if c < 0.0 {
        0
    } else if c > 255.0 {
        255
    } else {
        c as u8
    }
}

#[inline]
fn f16_to_u8(h: u16) -> u8 {
    f32_to_u8(f16::from_bits(h).to_f32())
}

fn decode_bc6_block(_src: &[u8], _dst: &mut [u32], _signed: bool) {
    let mut bit: BitReader = BitReader::new(_src, 0);

    let mut mode: u8 = bit.read(2) as u8;

    let mut ep_r: [u16; 4] = [0; 4]; //{ /* rw, rx, ry, rz */ };
    let mut ep_g: [u16; 4] = [0; 4]; //{ /* gw, gx, gy, gz */ };
    let mut ep_b: [u16; 4] = [0; 4]; //{ /* bw, bx, by, bz */ };

    if mode & 2 != 0 {
        // 5-bit mode
        mode |= (bit.read(3) << 2) as u8;

        if 0 == S_BC6H_MODE_INFO[mode as usize].endpoint_bits {
            _dst[0..64].fill(0);
            return;
        }

        match mode {
            2 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(5);
                ep_r[0] |= bit.read(1) << 10;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(4);
                ep_g[0] |= bit.read(1) << 10;
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(4);
                ep_b[0] |= bit.read(1) << 10;
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 3;
            }

            3 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(10);
                ep_g[1] |= bit.read(10);
                ep_b[1] |= bit.read(10);
            }

            6 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(4);
                ep_r[0] |= bit.read(1) << 10;
                ep_g[3] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(5);
                ep_g[0] |= bit.read(1) << 10;
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(4);
                ep_b[0] |= bit.read(1) << 10;
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(4);
                ep_b[3] |= bit.read(1);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(4);
                ep_g[2] |= bit.read(1) << 4;
                ep_b[3] |= bit.read(1) << 3;
            }

            7 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(9);
                ep_r[0] |= bit.read(1) << 10;
                ep_g[1] |= bit.read(9);
                ep_g[0] |= bit.read(1) << 10;
                ep_b[1] |= bit.read(9);
                ep_b[0] |= bit.read(1) << 10;
            }

            10 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(4);
                ep_r[0] |= bit.read(1) << 10;
                ep_b[2] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(4);
                ep_g[0] |= bit.read(1) << 10;
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(5);
                ep_b[0] |= bit.read(1) << 10;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(4);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(4);
                ep_b[3] |= bit.read(1) << 4;
                ep_b[3] |= bit.read(1) << 3;
            }

            11 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(8);
                ep_r[0] |= bit.read(1) << 11;
                ep_r[0] |= bit.read(1) << 10;
                ep_g[1] |= bit.read(8);
                ep_g[0] |= bit.read(1) << 11;
                ep_g[0] |= bit.read(1) << 10;
                ep_b[1] |= bit.read(8);
                ep_b[0] |= bit.read(1) << 11;
                ep_b[0] |= bit.read(1) << 10;
            }

            14 => {
                ep_r[0] |= bit.read(9);
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(9);
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(9);
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(5);
                ep_g[3] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(5);
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 3;
            }

            15 => {
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(4);
                ep_r[0] |= bit.read(1) << 15;
                ep_r[0] |= bit.read(1) << 14;
                ep_r[0] |= bit.read(1) << 13;
                ep_r[0] |= bit.read(1) << 12;
                ep_r[0] |= bit.read(1) << 11;
                ep_r[0] |= bit.read(1) << 10;
                ep_g[1] |= bit.read(4);
                ep_g[0] |= bit.read(1) << 15;
                ep_g[0] |= bit.read(1) << 14;
                ep_g[0] |= bit.read(1) << 13;
                ep_g[0] |= bit.read(1) << 12;
                ep_g[0] |= bit.read(1) << 11;
                ep_g[0] |= bit.read(1) << 10;
                ep_b[1] |= bit.read(4);
                ep_b[0] |= bit.read(1) << 15;
                ep_b[0] |= bit.read(1) << 14;
                ep_b[0] |= bit.read(1) << 13;
                ep_b[0] |= bit.read(1) << 12;
                ep_b[0] |= bit.read(1) << 11;
                ep_b[0] |= bit.read(1) << 10;
            }

            18 => {
                ep_r[0] |= bit.read(8);
                ep_g[3] |= bit.read(1) << 4;
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(8);
                ep_b[3] |= bit.read(1) << 2;
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(8);
                ep_b[3] |= bit.read(1) << 3;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(6);
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(5);
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(6);
                ep_r[3] |= bit.read(6);
            }

            22 => {
                ep_r[0] |= bit.read(8);
                ep_b[3] |= bit.read(1);
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(8);
                ep_g[2] |= bit.read(1) << 5;
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(8);
                ep_g[3] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(5);
                ep_g[3] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(6);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 3;
            }

            26 => {
                ep_r[0] |= bit.read(8);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(8);
                ep_b[2] |= bit.read(1) << 5;
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(8);
                ep_b[3] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(5);
                ep_g[3] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(5);
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(6);
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 3;
            }

            30 => {
                ep_r[0] |= bit.read(6);
                ep_g[3] |= bit.read(1) << 4;
                ep_b[3] |= bit.read(1);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(6);
                ep_g[2] |= bit.read(1) << 5;
                ep_b[2] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 2;
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(6);
                ep_g[3] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 3;
                ep_b[3] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(6);
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(6);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(6);
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(6);
                ep_r[3] |= bit.read(6);
            }
            _ => {}
        }
    } else {
        match mode {
            0 => {
                ep_g[2] |= bit.read(1) << 4;
                ep_b[2] |= bit.read(1) << 4;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[0] |= bit.read(10);
                ep_g[0] |= bit.read(10);
                ep_b[0] |= bit.read(10);
                ep_r[1] |= bit.read(5);
                ep_g[3] |= bit.read(1) << 4;
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(5);
                ep_b[3] |= bit.read(1);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 2;
                ep_r[3] |= bit.read(5);
                ep_b[3] |= bit.read(1) << 3;
            }

            1 => {
                ep_g[2] |= bit.read(1) << 5;
                ep_g[3] |= bit.read(1) << 4;
                ep_g[3] |= bit.read(1) << 5;
                ep_r[0] |= bit.read(7);
                ep_b[3] |= bit.read(1);
                ep_b[3] |= bit.read(1) << 1;
                ep_b[2] |= bit.read(1) << 4;
                ep_g[0] |= bit.read(7);
                ep_b[2] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 2;
                ep_g[2] |= bit.read(1) << 4;
                ep_b[0] |= bit.read(7);
                ep_b[3] |= bit.read(1) << 3;
                ep_b[3] |= bit.read(1) << 5;
                ep_b[3] |= bit.read(1) << 4;
                ep_r[1] |= bit.read(6);
                ep_g[2] |= bit.read(4);
                ep_g[1] |= bit.read(6);
                ep_g[3] |= bit.read(4);
                ep_b[1] |= bit.read(6);
                ep_b[2] |= bit.read(4);
                ep_r[2] |= bit.read(6);
                ep_r[3] |= bit.read(6);
            }
            _ => {}
        }
    }

    let mi: &Bc6hModeInfo = &S_BC6H_MODE_INFO[mode as usize];

    if _signed {
        ep_r[0] = sign_extend(ep_r[0], mi.endpoint_bits);
        ep_g[0] = sign_extend(ep_g[0], mi.endpoint_bits);
        ep_b[0] = sign_extend(ep_b[0], mi.endpoint_bits);
    }

    let num_subsets: usize = if mi.partition_bits != 0 {2} else {1};

    (1..num_subsets * 2).for_each(|ii| {
        if _signed || mi.transformed != 0 {
            ep_r[ii] = sign_extend(ep_r[ii], mi.delta_bits[0]);
            ep_g[ii] = sign_extend(ep_g[ii], mi.delta_bits[1]);
            ep_b[ii] = sign_extend(ep_b[ii], mi.delta_bits[2]);
        }

        if mi.transformed != 0 {
            let mask = (1 << mi.endpoint_bits) - 1;

            ep_r[ii] = ep_r[ii].overflowing_add(ep_r[0]).0 & mask;
            ep_g[ii] = ep_g[ii].overflowing_add(ep_g[0]).0 & mask;
            ep_b[ii] = ep_b[ii].overflowing_add(ep_b[0]).0 & mask;

            if _signed {
                ep_r[ii] = sign_extend(ep_r[ii], mi.endpoint_bits);
                ep_g[ii] = sign_extend(ep_g[ii], mi.endpoint_bits);
                ep_b[ii] = sign_extend(ep_b[ii], mi.endpoint_bits);
            }
        }
    });

    (0..num_subsets * 2).for_each(|ii| {
        ep_r[ii] = unquantize(ep_r[ii], _signed, mi.endpoint_bits);
        ep_g[ii] = unquantize(ep_g[ii], _signed, mi.endpoint_bits);
        ep_b[ii] = unquantize(ep_b[ii], _signed, mi.endpoint_bits);
    });

    let partition_set_idx = if mi.partition_bits != 0 {
        bit.read(5) as usize
    } else {
        0
    };
    let index_bits = if mi.partition_bits != 0 { 3 } else { 4 };
    let factors = S_BPTC_FACTORS[index_bits - 2];

    (0..4_usize).for_each(|yy| {
        (0..4_usize).for_each(|xx| {
            let idx = yy * 4 + xx;

            let mut subset_index = 0;
            let mut index_anchor = 0;

            if 0 != mi.partition_bits {
                subset_index = (S_BPTC_P2[partition_set_idx] >> idx) & 1;
                index_anchor = if subset_index != 0 {
                    S_BPTC_A2[partition_set_idx]
                } else {
                    0
                };
            }

            let anchor = idx == index_anchor;
            let num = index_bits - anchor as usize;
            let index = bit.read(num) as usize;

            let fc = factors[index] as u32;
            let fca = 64 - fc;
            let fcb = fc;

            subset_index *= 2;
            let rr = finish_unquantize(
                ((ep_r[subset_index] as u32 * fca + ep_r[subset_index + 1] as u32 * fcb + 32) >> 6) as u16,
                _signed,
            );
            let gg = finish_unquantize(
                ((ep_g[subset_index] as u32 * fca + ep_g[subset_index + 1] as u32 * fcb + 32) >> 6) as u16,
                _signed,
            );
            let bb = finish_unquantize(
                ((ep_b[subset_index] as u32 * fca + ep_b[subset_index + 1] as u32 * fcb + 32) >> 6) as u16,
                _signed,
            );

            _dst[idx] = color(f16_to_u8(rr), f16_to_u8(gg), f16_to_u8(bb), 255);
        });
    });
}

pub fn decode_bc6(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_offset: usize = 0;

    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_bc6_block(&data[data_offset..], &mut buffer, false);
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


fn decode_bc7_block(_src: &[u8], _dst: &mut [u32]) {
    let mut bit = BitReader::new(_src, 0);
    let mode = {
        let mut mode = 0;
        while 0 == bit.read(1) && mode < 8 {
            mode += 1;
        }
        mode
    };

    if mode == 8 {
        _dst[0..16 * 4].fill(0);
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
        ep_g[ii * 2] = (bit.read(mi.color_bits) << mode_pbits) as u8;
        ep_g[ii * 2 + 1] = (bit.read(mi.color_bits) << mode_pbits) as u8;
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
            let fa: u16 = factors[1-index_selection_mode][index[1-index_selection_mode]] as u16;

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
            _dst[idx] = color(rr, gg, bb, aa);
        });
    });
}

pub fn decode_bc7(data: &[u8], m_width: usize, m_height: usize, image: &mut [u32]) {
    let m_block_width: usize = 4;
    let m_block_height: usize = 4;
    let m_blocks_x = (m_width + m_block_width - 1) / m_block_width;
    let m_blocks_y = (m_height + m_block_height - 1) / m_block_height;
    let mut buffer: [u32; 16] = [0; 16];
    let mut data_offset: usize = 0;

    (0..m_blocks_y).for_each(|by| {
        (0..m_blocks_x).for_each(|bx| {
            decode_bc7_block(&data[data_offset..], &mut buffer);
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
