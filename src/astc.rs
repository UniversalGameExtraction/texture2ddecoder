#![allow(clippy::too_many_arguments)]
use crate::bitreader::{getbits, getbits64};
use crate::color::{color, copy_block_buffer};
use crate::f16::fp16_ieee_to_fp32_value;
use core::result::Result;

#[inline]
fn floor(x: f32) -> f32 {
    let mut i = x as i32;
    if x < 0.0 && x != i as f32 {
        i -= 1;
    }
    i as f32
}

static BIT_REVERSE_TABLE: [u8; 256] = [
    0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0,
    0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8,
    0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4,
    0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC,
    0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2,
    0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA,
    0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6,
    0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE,
    0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1,
    0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9,
    0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5,
    0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD,
    0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3,
    0x0B, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB,
    0x07, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
    0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF,
];

static WEIGHT_PREC_TABLE_A: [i32; 16] = [0, 0, 0, 3, 0, 5, 3, 0, 0, 0, 5, 3, 0, 5, 3, 0];
static WEIGHT_PREC_TABLE_B: [i32; 16] = [0, 0, 1, 0, 2, 0, 1, 3, 0, 0, 1, 2, 4, 2, 3, 5];

static CEM_TABLE_A: [usize; 19] = [0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 0, 0];
static CEM_TABLE_B: [usize; 19] = [8, 6, 5, 7, 5, 4, 6, 4, 3, 5, 3, 2, 4, 2, 1, 3, 1, 2, 1];

#[inline]
fn bit_reverse_u8(c: u8, bits: u8) -> u8 {
    let x = BIT_REVERSE_TABLE[c as usize].overflowing_shr(8 - bits as u32);
    match x.1 {
        false => x.0,
        true => 0,
    }
}

#[inline]
fn bit_reverse_u64(d: u64, bits: usize) -> u64 {
    let ret = (BIT_REVERSE_TABLE[(d & 0xff) as usize] as u64) << 56
        | (BIT_REVERSE_TABLE[(d >> 8 & 0xff) as usize] as u64) << 48
        | (BIT_REVERSE_TABLE[(d >> 16 & 0xff) as usize] as u64) << 40
        | (BIT_REVERSE_TABLE[(d >> 24 & 0xff) as usize] as u64) << 32
        | (BIT_REVERSE_TABLE[(d >> 32 & 0xff) as usize] as u64) << 24
        | (BIT_REVERSE_TABLE[(d >> 40 & 0xff) as usize] as u64) << 16
        | (BIT_REVERSE_TABLE[(d >> 48 & 0xff) as usize] as u64) << 8
        | (BIT_REVERSE_TABLE[(d >> 56 & 0xff) as usize] as u64);
    ret >> (64 - bits as u64)
}

#[inline]
const fn u8ptr_to_u16(ptr: &[u8]) -> u16 {
    u16::from_le_bytes([ptr[0], ptr[1]])
}

// #[inline]
// fn bit_transfer_signed(a: &mut i32, b: &mut i32) {
//     *b = (*b >> 1) | (*a & 0x80);
//     *a = (*a >> 1) & 0x3f;
//     if *a & 0x20 != 0 {
//         *a -= 0x40;
//     }
// }

#[inline]
fn bit_transfer_signed_alt(v: &mut [i32], a: usize, b: usize) {
    v[b] = (v[b] >> 1) | (v[a] & 0x80);
    v[a] = (v[a] >> 1) & 0x3f;
    if v[a] & 0x20 != 0 {
        v[a] -= 0x40;
    }
}

#[inline]
fn set_endpoint(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = r1;
    endpoint[1] = g1;
    endpoint[2] = b1;
    endpoint[3] = a1;
    endpoint[4] = r2;
    endpoint[5] = g2;
    endpoint[6] = b2;
    endpoint[7] = a2;
}

#[inline]
fn set_endpoint_clamp(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = r1.clamp(0, 255);
    endpoint[1] = g1.clamp(0, 255);
    endpoint[2] = b1.clamp(0, 255);
    endpoint[3] = a1.clamp(0, 255);
    endpoint[4] = r2.clamp(0, 255);
    endpoint[5] = g2.clamp(0, 255);
    endpoint[6] = b2.clamp(0, 255);
    endpoint[7] = a2.clamp(0, 255);
}

#[inline]
fn set_endpoint_blue(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = (r1 + b1) >> 1;
    endpoint[1] = (g1 + b1) >> 1;
    endpoint[2] = b1;
    endpoint[3] = a1;
    endpoint[4] = (r2 + b2) >> 1;
    endpoint[5] = (g2 + b2) >> 1;
    endpoint[6] = b2;
    endpoint[7] = a2;
}

#[inline]
fn set_endpoint_blue_clamp(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = ((r1 + b1) >> 1).clamp(0, 255);
    endpoint[1] = ((g1 + b1) >> 1).clamp(0, 255);
    endpoint[2] = b1.clamp(0, 255);
    endpoint[3] = a1.clamp(0, 255);
    endpoint[4] = ((r2 + b2) >> 1).clamp(0, 255);
    endpoint[5] = ((g2 + b2) >> 1).clamp(0, 255);
    endpoint[6] = b2.clamp(0, 255);
    endpoint[7] = a2.clamp(0, 255);
}

#[inline]
fn set_endpoint_hdr(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = r1;
    endpoint[1] = g1;
    endpoint[2] = b1;
    endpoint[3] = a1;
    endpoint[4] = r2;
    endpoint[5] = g2;
    endpoint[6] = b2;
    endpoint[7] = a2;
}

#[inline]
fn set_endpoint_hdr_clamp(
    endpoint: &mut [i32],
    r1: i32,
    g1: i32,
    b1: i32,
    a1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    a2: i32,
) {
    endpoint[0] = r1.clamp(0, 0xfff);
    endpoint[1] = g1.clamp(0, 0xfff);
    endpoint[2] = b1.clamp(0, 0xfff);
    endpoint[3] = a1.clamp(0, 0xfff);
    endpoint[4] = r2.clamp(0, 0xfff);
    endpoint[5] = g2.clamp(0, 0xfff);
    endpoint[6] = b2.clamp(0, 0xfff);
    endpoint[7] = a2.clamp(0, 0xfff);
}

// typedef uint_fast8_t (*t_select_folor_func_ptr)(int, int, int);

#[inline]
const fn select_color(v0: i32, v1: i32, weight: i32) -> u8 {
    (((((v0 << 8 | v0) * (64 - weight) + (v1 << 8 | v1) * weight + 32) >> 6) * 255 + 32768) / 65536)
        as u8
}

#[inline]
fn select_color_hdr(v0: i32, v1: i32, weight: i32) -> u8 {
    let c: u16 = (((v0 << 4) * (64 - weight) + (v1 << 4) * weight + 32) >> 6) as u16;
    let mut m: u16 = c & 0x7ff;
    if m < 512 {
        m *= 3;
    } else if m < 1536 {
        m = 4 * m - 512;
    } else {
        m = 5 * m - 2048;
    }
    let f: f32 = fp16_ieee_to_fp32_value((c >> 1 & 0x7c00) | m >> 3);
    if f32::is_finite(f) {
        (floor(f * 255.0) as i32).clamp(0, 255) as u8
    } else {
        255
    }
}

#[inline]
fn f32_to_u8(f: f32) -> u8 {
    floor(f * 255.0).clamp(0.0, 255.0) as u8
}

#[inline]
fn f16ptr_to_u8(ptr: &[u8]) -> u8 {
    f32_to_u8(fp16_ieee_to_fp32_value(u16::from_le_bytes([
        ptr[0], ptr[1],
    ])))
}

struct BlockData {
    bw: usize,
    bh: usize,
    width: usize,
    height: usize,
    part_num: usize,
    dual_plane: bool,
    plane_selector: usize,
    weight_range: usize,
    weight_num: usize,
    cem: [usize; 4],
    cem_range: usize,
    endpoint_value_num: usize,
    endpoints: [[i32; 8]; 4],
    weights: [[i32; 2]; 144],
    partition: [usize; 144],
}

impl BlockData {
    const fn default() -> Self {
        Self {
            bw: 0,
            bh: 0,
            width: 0,
            height: 0,
            part_num: 0,
            dual_plane: false,
            plane_selector: 0,
            weight_range: 0,
            weight_num: 0,
            cem: [0; 4],
            cem_range: 0,
            endpoint_value_num: 0,
            endpoints: [[0; 8]; 4],
            weights: [[0; 2]; 144],
            partition: [0; 144],
        }
    }
}

#[derive(Clone, Copy, Default)]
struct IntSeqData {
    bits: u64,
    nonbits: u64,
}

fn decode_intseq(
    buf: &[u8],
    offset: usize,
    a: usize,
    b: usize,
    count: usize,
    reverse: bool,
    out: &mut [IntSeqData],
) {
    // TODO: reduce code duplication
    static MT: [usize; 5] = [0, 2, 4, 5, 7];
    static MQ: [usize; 3] = [0, 3, 5];
    static TRITS_TABLE: [[u64; 256]; 5] = [
        [
            0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0,
            1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1,
            2, 2, 0, 1, 2, 1, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2,
            1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0,
            0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0,
            1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1,
            2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 1, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2,
            2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1,
            0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2,
        ],
        [
            0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2,
            2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1,
            1, 2, 2, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0,
            0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2,
            2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2,
            0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 1, 0, 0, 0, 0, 1, 1, 1, 0,
            2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 1,
        ],
        [
            0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0,
            0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1,
            1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1,
            2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2,
            1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1,
            1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2,
            2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0,
            2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2,
            0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ],
    ];
    static QUINTS_TABLE: [[u64; 128]; 3] = [
        [
            0, 1, 2, 3, 4, 0, 4, 4, 0, 1, 2, 3, 4, 1, 4, 4, 0, 1, 2, 3, 4, 2, 4, 4, 0, 1, 2, 3, 4,
            3, 4, 4, 0, 1, 2, 3, 4, 0, 4, 0, 0, 1, 2, 3, 4, 1, 4, 1, 0, 1, 2, 3, 4, 2, 4, 2, 0, 1,
            2, 3, 4, 3, 4, 3, 0, 1, 2, 3, 4, 0, 2, 3, 0, 1, 2, 3, 4, 1, 2, 3, 0, 1, 2, 3, 4, 2, 2,
            3, 0, 1, 2, 3, 4, 3, 2, 3, 0, 1, 2, 3, 4, 0, 0, 1, 0, 1, 2, 3, 4, 1, 0, 1, 0, 1, 2, 3,
            4, 2, 0, 1, 0, 1, 2, 3, 4, 3, 0, 1,
        ],
        [
            0, 0, 0, 0, 0, 4, 4, 4, 1, 1, 1, 1, 1, 4, 4, 4, 2, 2, 2, 2, 2, 4, 4, 4, 3, 3, 3, 3, 3,
            4, 4, 4, 0, 0, 0, 0, 0, 4, 0, 4, 1, 1, 1, 1, 1, 4, 1, 4, 2, 2, 2, 2, 2, 4, 2, 4, 3, 3,
            3, 3, 3, 4, 3, 4, 0, 0, 0, 0, 0, 4, 0, 0, 1, 1, 1, 1, 1, 4, 1, 1, 2, 2, 2, 2, 2, 4, 2,
            2, 3, 3, 3, 3, 3, 4, 3, 3, 0, 0, 0, 0, 0, 4, 0, 0, 1, 1, 1, 1, 1, 4, 1, 1, 2, 2, 2, 2,
            2, 4, 2, 2, 3, 3, 3, 3, 3, 4, 3, 3,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0,
            0, 3, 4, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1,
            1, 1, 1, 1, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 2, 2, 4,
            4, 2, 2, 2, 2, 2, 2, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3,
            3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4,
        ],
    ];

    if count == 0 {
        return;
    }

    let mut n = 0;
    let mut p: isize = offset as isize;
    match a {
        3 => {
            let mask = (1 << b) - 1;
            let block_count = count.div_ceil(5);
            let last_block_count = (count + 4) % 5 + 1;
            let block_size = 8 + 5 * b;
            let last_block_size = (block_size * last_block_count).div_ceil(5);

            if reverse {
                (0..block_count).for_each(|i| {
                    let now_size = if i < block_count - 1 {
                        block_size
                    } else {
                        last_block_size
                    };
                    let d =
                        bit_reverse_u64(getbits64(buf, p - now_size as isize, now_size), now_size);
                    let x = ((d >> b & 3)
                        | (d >> (b * 2) & 0xc)
                        | (d >> (b * 3) & 0x10)
                        | (d >> (b * 4) & 0x60)
                        | (d >> (b * 5) & 0x80)) as usize;
                    for j in 0..5 {
                        if n < count {
                            out[n] = IntSeqData {
                                bits: (d >> (MT[j] + b * j)) & mask,
                                nonbits: TRITS_TABLE[j][x],
                            };
                            n += 1;
                        }
                    }
                    p -= block_size as isize;
                });
            } else {
                (0..block_count).for_each(|i| {
                    let now_size = if i < block_count - 1 {
                        block_size
                    } else {
                        last_block_size
                    };
                    let d = getbits64(buf, p, now_size);
                    let x = ((d >> b & 3)
                        | (d >> (b * 2) & 0xc)
                        | (d >> (b * 3) & 0x10)
                        | (d >> (b * 4) & 0x60)
                        | (d >> (b * 5) & 0x80)) as usize;
                    for j in 0..5 {
                        if n < count {
                            out[n] = IntSeqData {
                                bits: (d >> (MT[j] + b * j)) & mask,
                                nonbits: TRITS_TABLE[j][x],
                            };
                            n += 1;
                        }
                    }
                    p += block_size as isize;
                });
            }
        }
        5 => {
            let mask = (1 << b) - 1;
            let block_count = count.div_ceil(3);
            let last_block_count = (count + 2) % 3 + 1;
            let block_size = 7 + 3 * b;
            let last_block_size = (block_size * last_block_count).div_ceil(3);

            if reverse {
                (0..block_count).for_each(|i| {
                    let now_size = if i < block_count - 1 {
                        block_size
                    } else {
                        last_block_size
                    };
                    let d =
                        bit_reverse_u64(getbits64(buf, p - now_size as isize, now_size), now_size);
                    let x = ((d >> b & 7) | (d >> (b * 2) & 0x18) | (d >> (b * 3) & 0x60)) as usize;
                    for j in 0..3 {
                        if n < count {
                            out[n] = IntSeqData {
                                bits: (d >> (MQ[j] + b * j)) & mask,
                                nonbits: QUINTS_TABLE[j][x],
                            };
                            n += 1;
                        }
                    }
                    p -= block_size as isize;
                });
            } else {
                (0..block_count).for_each(|i| {
                    let now_size = if i < block_count - 1 {
                        block_size
                    } else {
                        last_block_size
                    };
                    let d = getbits64(buf, p, now_size);
                    let x = ((d >> b & 7) | (d >> (b * 2) & 0x18) | (d >> (b * 3) & 0x60)) as usize;
                    for j in 0..3 {
                        if n < count {
                            out[n] = IntSeqData {
                                bits: (d >> (MQ[j] + b * j)) & mask,
                                nonbits: QUINTS_TABLE[j][x],
                            };
                            n += 1;
                        }
                    }
                    p += block_size as isize;
                });
            }
        }
        _ => {
            if reverse {
                p -= b as isize;
                while n < count {
                    out[n] = IntSeqData {
                        bits: bit_reverse_u8(getbits(buf, p as usize, b) as u8, b as u8) as u64,
                        nonbits: 0,
                    };
                    n += 1;
                    p -= b as isize;
                }
            } else {
                while n < count {
                    out[n] = IntSeqData {
                        bits: getbits(buf, p as usize, b) as u64,
                        nonbits: 0,
                    };
                    n += 1;
                    p += b as isize;
                }
            }
        }
    }
}

fn decode_block_params(buf: &[u8], block_data: &mut BlockData) {
    block_data.dual_plane = (buf[1] & 4) != 0;
    block_data.weight_range = ((buf[0] >> 4 & 1) | (buf[1] << 2 & 8)) as usize;

    if buf[0] & 3 != 0 {
        block_data.weight_range |= (buf[0] << 1 & 6) as usize;
        match buf[0] & 0xc {
            0 => {
                block_data.width = ((u8ptr_to_u16(buf) >> 7 & 3) + 4) as usize;
                block_data.height = ((buf[0] >> 5 & 3) + 2) as usize;
            }
            4 => {
                block_data.width = ((u8ptr_to_u16(buf) >> 7 & 3) + 8) as usize;
                block_data.height = ((buf[0] >> 5 & 3) + 2) as usize;
            }
            8 => {
                block_data.width = ((buf[0] >> 5 & 3) + 2) as usize;
                block_data.height = ((u8ptr_to_u16(buf) >> 7 & 3) + 8) as usize;
            }
            12 => {
                if buf[1] & 1 != 0 {
                    block_data.width = ((buf[0] >> 7 & 1) + 2) as usize;
                    block_data.height = ((buf[0] >> 5 & 3) + 2) as usize;
                } else {
                    block_data.width = ((buf[0] >> 5 & 3) + 2) as usize;
                    block_data.height = ((buf[0] >> 7 & 1) + 6) as usize;
                }
            }
            _ => {}
        }
    } else {
        block_data.weight_range |= (buf[0] >> 1 & 6) as usize;
        match u8ptr_to_u16(buf) & 0x180 {
            0 => {
                block_data.width = 12;
                block_data.height = ((buf[0] >> 5 & 3) + 2) as usize;
            }
            0x80 => {
                block_data.width = ((buf[0] >> 5 & 3) + 2) as usize;
                block_data.height = 12;
            }
            0x100 => {
                block_data.width = ((buf[0] >> 5 & 3) + 6) as usize;
                block_data.height = ((buf[1] >> 1 & 3) + 6) as usize;
                block_data.dual_plane = false;
                block_data.weight_range &= 7;
            }
            0x180 => {
                block_data.width = if buf[0] & 0x20 != 0 { 10 } else { 6 };
                block_data.height = if buf[0] & 0x20 != 0 { 6 } else { 10 };
            }
            _ => {}
        }
    }

    block_data.part_num = ((buf[1] >> 3 & 3) + 1) as usize;

    block_data.weight_num = block_data.width * block_data.height;
    if block_data.dual_plane {
        block_data.weight_num *= 2;
    }

    let mut config_bits: usize;
    let mut cem_base = 0;

    let weight_bits = match WEIGHT_PREC_TABLE_A[block_data.weight_range] {
        3 => {
            block_data.weight_num as i32 * WEIGHT_PREC_TABLE_B[block_data.weight_range]
                + (block_data.weight_num as i32 * 8 + 4) / 5
        }
        5 => {
            block_data.weight_num as i32 * WEIGHT_PREC_TABLE_B[block_data.weight_range]
                + (block_data.weight_num as i32 * 7 + 2) / 3
        }
        _ => block_data.weight_num as i32 * WEIGHT_PREC_TABLE_B[block_data.weight_range],
    };

    if block_data.part_num == 1 {
        block_data.cem[0] = (u8ptr_to_u16(&buf[1..]) >> 5 & 0xf) as usize;
        config_bits = 17;
    } else {
        cem_base = (u8ptr_to_u16(&buf[2..]) >> 7 & 3) as usize;
        if cem_base == 0 {
            let cem = (buf[3] >> 1 & 0xf) as usize;
            block_data.cem[0..block_data.part_num].fill(cem);
            config_bits = 29;
        } else {
            (0..block_data.part_num).for_each(|i| {
                block_data.cem[i] = ((buf[3] >> (i + 1) & 1) as usize + cem_base - 1) << 2
            });
            match block_data.part_num {
                2 => {
                    block_data.cem[0] |= (buf[3] >> 3 & 3) as usize;
                    block_data.cem[1] |= (getbits(buf, 126 - weight_bits as usize, 2)) as usize;
                }
                3 => {
                    block_data.cem[0] |= (buf[3] >> 4 & 1) as usize;
                    block_data.cem[0] |= (getbits(buf, 122 - weight_bits as usize, 2) & 2) as usize;
                    block_data.cem[1] |= (getbits(buf, 124 - weight_bits as usize, 2)) as usize;
                    block_data.cem[2] |= (getbits(buf, 126 - weight_bits as usize, 2)) as usize;
                }
                4 => {
                    (0..4).for_each(|i| {
                        block_data.cem[i] |=
                            (getbits(buf, 120 + i * 2 - weight_bits as usize, 2)) as usize;
                    });
                }
                _ => {}
            }
            config_bits = 25 + block_data.part_num * 3;
        }
    }

    if block_data.dual_plane {
        config_bits += 2;
        block_data.plane_selector = getbits(
            buf,
            if cem_base != 0 {
                130 - weight_bits as usize - block_data.part_num * 3
            } else {
                126 - weight_bits as usize
            },
            2,
        ) as usize;
    }

    let remain_bits = 128 - config_bits - weight_bits as usize;

    block_data.endpoint_value_num = 0;

    (0..block_data.part_num)
        .for_each(|i| block_data.endpoint_value_num += (block_data.cem[i] >> 1 & 6) + 2);

    let mut endpoint_bits: usize;
    for i in 0..CEM_TABLE_A.len() {
        match CEM_TABLE_A[i] {
            3 => {
                endpoint_bits = block_data.endpoint_value_num * CEM_TABLE_B[i]
                    + (block_data.endpoint_value_num * 8).div_ceil(5);
            }
            5 => {
                endpoint_bits = block_data.endpoint_value_num * CEM_TABLE_B[i]
                    + (block_data.endpoint_value_num * 7).div_ceil(3);
            }
            _ => {
                endpoint_bits = block_data.endpoint_value_num * CEM_TABLE_B[i];
            }
        }
        if endpoint_bits <= remain_bits {
            block_data.cem_range = i;
            break;
        }
    }
}

fn decode_endpoints_hdr7(endpoints: &mut [i32], v: &[i32]) {
    let modeval = (v[2] >> 4 & 0x8) | (v[1] >> 5 & 0x4) | (v[0] >> 6);
    let (major_component, mode) = {
        if (modeval & 0xc) != 0xc {
            (modeval >> 2, modeval & 3)
        } else if modeval != 0xf {
            (modeval & 3, 4)
        } else {
            (0, 5)
        }
    };
    let mut c: [i32; 4] = [v[0] & 0x3f, v[1] & 0x1f, v[2] & 0x1f, v[3] & 0x1f];

    match mode {
        0 => {
            c[3] |= v[3] & 0x60;
            c[0] |= v[3] >> 1 & 0x40;
            c[0] |= v[2] << 1 & 0x80;
            c[0] |= v[1] << 3 & 0x300;
            c[0] |= v[2] << 5 & 0x400;
            c[0] <<= 1;
            c[1] <<= 1;
            c[2] <<= 1;
            c[3] <<= 1;
        }
        1 => {
            c[1] |= v[1] & 0x20;
            c[2] |= v[2] & 0x20;
            c[0] |= v[3] >> 1 & 0x40;
            c[0] |= v[2] << 1 & 0x80;
            c[0] |= v[1] << 2 & 0x100;
            c[0] |= v[3] << 4 & 0x600;
            c[0] <<= 1;
            c[1] <<= 1;
            c[2] <<= 1;
            c[3] <<= 1;
        }
        2 => {
            c[3] |= v[3] & 0xe0;
            c[0] |= v[2] << 1 & 0xc0;
            c[0] |= v[1] << 3 & 0x300;
            c[0] <<= 2;
            c[1] <<= 2;
            c[2] <<= 2;
            c[3] <<= 2;
        }
        3 => {
            c[1] |= v[1] & 0x20;
            c[2] |= v[2] & 0x20;
            c[3] |= v[3] & 0x60;
            c[0] |= v[3] >> 1 & 0x40;
            c[0] |= v[2] << 1 & 0x80;
            c[0] |= v[1] << 2 & 0x100;
            c[0] <<= 3;
            c[1] <<= 3;
            c[2] <<= 3;
            c[3] <<= 3;
        }
        4 => {
            c[1] |= v[1] & 0x60;
            c[2] |= v[2] & 0x60;
            c[3] |= v[3] & 0x20;
            c[0] |= v[3] >> 1 & 0x40;
            c[0] |= v[3] << 1 & 0x80;
            c[0] <<= 4;
            c[1] <<= 4;
            c[2] <<= 4;
            c[3] <<= 4;
        }
        5 => {
            c[1] |= v[1] & 0x60;
            c[2] |= v[2] & 0x60;
            c[3] |= v[3] & 0x60;
            c[0] |= v[3] >> 1 & 0x40;
            c[0] <<= 5;
            c[1] <<= 5;
            c[2] <<= 5;
            c[3] <<= 5;
        }
        _ => {}
    }
    if mode != 5 {
        c[1] = c[0] - c[1];
        c[2] = c[0] - c[2];
    }
    match major_component {
        1 => {
            set_endpoint_hdr_clamp(
                endpoints,
                c[1] - c[3],
                c[0] - c[3],
                c[2] - c[3],
                0x780,
                c[1],
                c[0],
                c[2],
                0x780,
            );
        }
        2 => {
            set_endpoint_hdr_clamp(
                endpoints,
                c[2] - c[3],
                c[1] - c[3],
                c[0] - c[3],
                0x780,
                c[2],
                c[1],
                c[0],
                0x780,
            );
        }
        _ => {
            set_endpoint_hdr_clamp(
                endpoints,
                c[0] - c[3],
                c[1] - c[3],
                c[2] - c[3],
                0x780,
                c[0],
                c[1],
                c[2],
                0x780,
            );
        }
    }
}

fn decode_endpoints_hdr11(endpoints: &mut [i32], v: &[i32], alpha1: i32, alpha2: i32) {
    let major_component = (v[4] >> 7) | (v[5] >> 6 & 2);
    if major_component == 3 {
        set_endpoint_hdr(
            endpoints,
            v[0] << 4,
            v[2] << 4,
            v[4] << 5 & 0xfe0,
            alpha1,
            v[1] << 4,
            v[3] << 4,
            v[5] << 5 & 0xfe0,
            alpha2,
        );
        return;
    }
    let mode = (v[1] >> 7) | (v[2] >> 6 & 2) | (v[3] >> 5 & 4);
    let mut va = v[0] | (v[1] << 2 & 0x100);
    let mut vb0 = v[2] & 0x3f;
    let mut vb1 = v[3] & 0x3f;
    let mut vc = v[1] & 0x3f;
    let mut vd0: i32;
    let mut vd1: i32;

    match mode {
        0 | 2 => {
            vd0 = v[4] & 0x7f;
            if vd0 & 0x40 != 0 {
                vd0 |= 0xff80;
            }
            vd1 = v[5] & 0x7f;
            if vd1 & 0x40 != 0 {
                vd1 |= 0xff80;
            }
        }
        1 | 3 | 5 | 7 => {
            vd0 = v[4] & 0x3f;
            if vd0 & 0x20 != 0 {
                vd0 |= 0xffc0;
            }
            vd1 = v[5] & 0x3f;
            if vd1 & 0x20 != 0 {
                vd1 |= 0xffc0;
            }
        }
        _ => {
            vd0 = v[4] & 0x1f;
            if vd0 & 0x10 != 0 {
                vd0 |= 0xffe0;
            }
            vd1 = v[5] & 0x1f;
            if vd1 & 0x10 != 0 {
                vd1 |= 0xffe0;
            }
        }
    }

    match mode {
        0 => {
            vb0 |= v[2] & 0x40;
            vb1 |= v[3] & 0x40;
        }
        1 => {
            vb0 |= v[2] & 0x40;
            vb1 |= v[3] & 0x40;
            vb0 |= v[4] << 1 & 0x80;
            vb1 |= v[5] << 1 & 0x80;
        }
        2 => {
            va |= v[2] << 3 & 0x200;
            vc |= v[3] & 0x40;
        }
        3 => {
            va |= v[4] << 3 & 0x200;
            vc |= v[5] & 0x40;
            vb0 |= v[2] & 0x40;
            vb1 |= v[3] & 0x40;
        }
        4 => {
            va |= v[4] << 4 & 0x200;
            va |= v[5] << 5 & 0x400;
            vb0 |= v[2] & 0x40;
            vb1 |= v[3] & 0x40;
            vb0 |= v[4] << 1 & 0x80;
            vb1 |= v[5] << 1 & 0x80;
        }
        5 => {
            va |= v[2] << 3 & 0x200;
            va |= v[3] << 4 & 0x400;
            vc |= v[5] & 0x40;
            vc |= v[4] << 1 & 0x80;
        }
        6 => {
            va |= v[4] << 4 & 0x200;
            va |= v[5] << 5 & 0x400;
            va |= v[4] << 5 & 0x800;
            vc |= v[5] & 0x40;
            vb0 |= v[2] & 0x40;
            vb1 |= v[3] & 0x40;
        }
        7 => {
            va |= v[2] << 3 & 0x200;
            va |= v[3] << 4 & 0x400;
            va |= v[4] << 5 & 0x800;
            vc |= v[5] & 0x40;
        }
        _ => {}
    }

    let shamt = (mode >> 1) ^ 3;
    va <<= shamt;
    vb0 <<= shamt;
    vb1 <<= shamt;
    vc <<= shamt;
    let mult = 1 << shamt;
    vd0 *= mult;
    vd1 *= mult;

    match major_component {
        1 => {
            set_endpoint_hdr_clamp(
                endpoints,
                va - vb0 - vc - vd0,
                va - vc,
                va - vb1 - vc - vd1,
                alpha1,
                va - vb0,
                va,
                va - vb1,
                alpha2,
            );
        }
        2 => {
            set_endpoint_hdr_clamp(
                endpoints,
                va - vb1 - vc - vd1,
                va - vb0 - vc - vd0,
                va - vc,
                alpha1,
                va - vb1,
                va - vb0,
                va,
                alpha2,
            );
        }
        _ => {
            set_endpoint_hdr_clamp(
                endpoints,
                va - vc,
                va - vb0 - vc - vd0,
                va - vb1 - vc - vd1,
                alpha1,
                va,
                va - vb0,
                va - vb1,
                alpha2,
            );
        }
    }
}

fn decode_endpoints(buf: &[u8], data: &mut BlockData) {
    static TRITS_TABLE: [usize; 7] = [0, 204, 93, 44, 22, 11, 5];
    static QUINTS_TABLE: [usize; 6] = [0, 113, 54, 26, 13, 6];
    let mut seq: [IntSeqData; 32] = [IntSeqData::default(); 32];
    let mut ev: [i32; 32] = [0; 32];
    decode_intseq(
        buf,
        if data.part_num == 1 { 17 } else { 29 },
        CEM_TABLE_A[data.cem_range],
        CEM_TABLE_B[data.cem_range],
        data.endpoint_value_num,
        false,
        &mut seq,
    );

    match CEM_TABLE_A[data.cem_range] {
        3 => {
            let mut b = 0;
            let c = TRITS_TABLE[CEM_TABLE_B[data.cem_range]];
            (0..data.endpoint_value_num).for_each(|i| {
                let a = (seq[i].bits & 1) * 0x1ff;
                let x = seq[i].bits >> 1;
                match CEM_TABLE_B[data.cem_range] {
                    1 => {
                        b = 0;
                    }
                    2 => {
                        b = 0b100010110 * x;
                    }
                    3 => {
                        b = x << 7 | x << 2 | x;
                    }
                    4 => {
                        b = x << 6 | x;
                    }
                    5 => {
                        b = x << 5 | x >> 2;
                    }
                    6 => {
                        b = x << 4 | x >> 4;
                    }
                    _ => {}
                }
                ev[i] = ((a & 0x80) | ((seq[i].nonbits * c as u64 + b) ^ a) >> 2) as i32;
            });
        }
        5 => {
            let mut b = 0;
            let c = QUINTS_TABLE[CEM_TABLE_B[data.cem_range]];
            (0..data.endpoint_value_num).for_each(|i| {
                let a = (seq[i].bits & 1) * 0x1ff;
                let x = seq[i].bits >> 1;
                match CEM_TABLE_B[data.cem_range] {
                    1 => {
                        b = 0;
                    }
                    2 => {
                        b = 0b100001100 * x;
                    }
                    3 => {
                        b = x << 7 | x << 1 | x >> 1;
                    }
                    4 => {
                        b = x << 6 | x >> 1;
                    }
                    5 => {
                        b = x << 5 | x >> 3;
                    }
                    _ => {}
                }
                ev[i] = ((a & 0x80) | ((seq[i].nonbits * c as u64 + b) ^ a) >> 2) as i32;
            });
        }
        _ => match CEM_TABLE_B[data.cem_range] {
            1 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits * 0xff) as i32;
                });
            }
            2 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits * 0x55) as i32;
                });
            }
            3 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits << 5 | seq[i].bits << 2 | seq[i].bits >> 1) as i32;
                });
            }
            4 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits << 4 | seq[i].bits) as i32;
                });
            }
            5 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits << 3 | seq[i].bits >> 2) as i32;
                });
            }
            6 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits << 2 | seq[i].bits >> 4) as i32;
                });
            }
            7 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = (seq[i].bits << 1 | seq[i].bits >> 6) as i32;
                });
            }
            8 => {
                (0..data.endpoint_value_num).for_each(|i| {
                    ev[i] = seq[i].bits as i32;
                });
            }
            _ => {}
        },
    }

    let mut v: &mut [i32] = &mut ev;
    for cem in 0..data.part_num {
        match data.cem[cem] {
            0 => {
                set_endpoint(
                    &mut data.endpoints[cem],
                    v[0],
                    v[0],
                    v[0],
                    255,
                    v[1],
                    v[1],
                    v[1],
                    255,
                );
            }
            1 => {
                let l0 = (v[0] >> 2) | (v[1] & 0xc0);
                let l1 = (l0 + (v[1] & 0x3f)).clamp(0, 255);
                set_endpoint(&mut data.endpoints[cem], l0, l0, l0, 255, l1, l1, l1, 255);
            }
            2 => {
                let y0;
                let y1;
                if v[0] <= v[1] {
                    y0 = v[0] << 4;
                    y1 = v[1] << 4;
                } else {
                    y0 = (v[1] << 4) + 8;
                    y1 = (v[0] << 4) - 8;
                }
                set_endpoint_hdr(
                    &mut data.endpoints[cem],
                    y0,
                    y0,
                    y0,
                    0x780,
                    y1,
                    y1,
                    y1,
                    0x780,
                );
            }
            3 => {
                let y0;
                let d;
                if v[0] & 0x80 != 0 {
                    y0 = (v[1] & 0xe0) << 4 | (v[0] & 0x7f) << 2;
                    d = (v[1] & 0x1f) << 2;
                } else {
                    y0 = (v[1] & 0xf0) << 4 | (v[0] & 0x7f) << 1;
                    d = (v[1] & 0x0f) << 1;
                }
                let y1 = (y0 + d).clamp(0, 0xfff);
                set_endpoint_hdr(
                    &mut data.endpoints[cem],
                    y0,
                    y0,
                    y0,
                    0x780,
                    y1,
                    y1,
                    y1,
                    0x780,
                );
            }
            4 => {
                set_endpoint(
                    &mut data.endpoints[cem],
                    v[0],
                    v[0],
                    v[0],
                    v[2],
                    v[1],
                    v[1],
                    v[1],
                    v[3],
                );
            }
            5 => {
                bit_transfer_signed_alt(v, 1, 0);
                bit_transfer_signed_alt(v, 3, 2);
                v[1] += v[0];
                set_endpoint_clamp(
                    &mut data.endpoints[cem],
                    v[0],
                    v[0],
                    v[0],
                    v[2],
                    v[1],
                    v[1],
                    v[1],
                    v[2] + v[3],
                );
            }
            6 => {
                set_endpoint(
                    &mut data.endpoints[cem],
                    (v[0] * v[3]) >> 8,
                    (v[1] * v[3]) >> 8,
                    (v[2] * v[3]) >> 8,
                    255,
                    v[0],
                    v[1],
                    v[2],
                    255,
                );
            }
            7 => {
                decode_endpoints_hdr7(&mut data.endpoints[cem], v);
            }
            8 => {
                if v[0] + v[2] + v[4] <= v[1] + v[3] + v[5] {
                    set_endpoint(
                        &mut data.endpoints[cem],
                        v[0],
                        v[2],
                        v[4],
                        255,
                        v[1],
                        v[3],
                        v[5],
                        255,
                    );
                } else {
                    set_endpoint_blue(
                        &mut data.endpoints[cem],
                        v[1],
                        v[3],
                        v[5],
                        255,
                        v[0],
                        v[2],
                        v[4],
                        255,
                    );
                }
            }
            9 => {
                bit_transfer_signed_alt(v, 1, 0);
                bit_transfer_signed_alt(v, 3, 2);
                bit_transfer_signed_alt(v, 5, 4);
                if v[1] + v[3] + v[5] >= 0 {
                    set_endpoint_clamp(
                        &mut data.endpoints[cem],
                        v[0],
                        v[2],
                        v[4],
                        255,
                        v[0] + v[1],
                        v[2] + v[3],
                        v[4] + v[5],
                        255,
                    );
                } else {
                    set_endpoint_blue_clamp(
                        &mut data.endpoints[cem],
                        v[0] + v[1],
                        v[2] + v[3],
                        v[4] + v[5],
                        255,
                        v[0],
                        v[2],
                        v[4],
                        255,
                    );
                }
            }
            10 => {
                set_endpoint(
                    &mut data.endpoints[cem],
                    (v[0] * v[3]) >> 8,
                    (v[1] * v[3]) >> 8,
                    (v[2] * v[3]) >> 8,
                    v[4],
                    v[0],
                    v[1],
                    v[2],
                    v[5],
                );
            }
            11 => {
                decode_endpoints_hdr11(&mut data.endpoints[cem], v, 0x780, 0x780);
            }
            12 => {
                if v[0] + v[2] + v[4] <= v[1] + v[3] + v[5] {
                    set_endpoint(
                        &mut data.endpoints[cem],
                        v[0],
                        v[2],
                        v[4],
                        v[6],
                        v[1],
                        v[3],
                        v[5],
                        v[7],
                    );
                } else {
                    set_endpoint_blue(
                        &mut data.endpoints[cem],
                        v[1],
                        v[3],
                        v[5],
                        v[7],
                        v[0],
                        v[2],
                        v[4],
                        v[6],
                    );
                }
            }
            13 => {
                bit_transfer_signed_alt(v, 1, 0);
                bit_transfer_signed_alt(v, 3, 2);
                bit_transfer_signed_alt(v, 5, 4);
                bit_transfer_signed_alt(v, 7, 6);

                if v[1] + v[3] + v[5] >= 0 {
                    set_endpoint_clamp(
                        &mut data.endpoints[cem],
                        v[0],
                        v[2],
                        v[4],
                        v[6],
                        v[0] + v[1],
                        v[2] + v[3],
                        v[4] + v[5],
                        v[6] + v[7],
                    );
                } else {
                    set_endpoint_blue_clamp(
                        &mut data.endpoints[cem],
                        v[0] + v[1],
                        v[2] + v[3],
                        v[4] + v[5],
                        v[6] + v[7],
                        v[0],
                        v[2],
                        v[4],
                        v[6],
                    );
                }
            }
            14 => {
                decode_endpoints_hdr11(&mut data.endpoints[cem], v, v[6], v[7]);
            }
            15 => {
                let mode = ((v[6] >> 7) & 1) | ((v[7] >> 6) & 2);
                v[6] &= 0x7f;
                v[7] &= 0x7f;
                if mode == 3 {
                    decode_endpoints_hdr11(&mut data.endpoints[cem], v, v[6] << 5, v[7] << 5);
                } else {
                    v[6] |= (v[7] << (mode + 1)) & 0x780;
                    v[7] = ((v[7] & (0x3f >> mode)) ^ (0x20 >> mode)) - (0x20 >> mode);
                    v[6] <<= 4 - mode;
                    v[7] <<= 4 - mode;
                    decode_endpoints_hdr11(
                        &mut data.endpoints[cem],
                        v,
                        v[6],
                        (v[6] + v[7]).clamp(0, 0xfff),
                    );
                }
            }
            _ => {
                panic!("Unsupported ASTC format");
            }
        }
        v = &mut v[(data.cem[cem] / 4 + 1) * 2..];
    }
}

fn decode_weights(buf: &[u8], data: &mut BlockData) {
    let mut seq: [IntSeqData; 128] = [IntSeqData::default(); 128];
    let mut wv: [i32; 128] = [0; 128];
    decode_intseq(
        buf,
        128,
        WEIGHT_PREC_TABLE_A[data.weight_range] as usize,
        WEIGHT_PREC_TABLE_B[data.weight_range] as usize,
        data.weight_num,
        true,
        &mut seq,
    );

    if WEIGHT_PREC_TABLE_A[data.weight_range] == 0 {
        match WEIGHT_PREC_TABLE_B[data.weight_range] {
            1 => {
                (0..data.weight_num).for_each(|i| wv[i] = if seq[i].bits != 0 { 63 } else { 0 });
            }
            2 => {
                (0..data.weight_num).for_each(|i| {
                    wv[i] = (seq[i].bits << 4 | seq[i].bits << 2 | seq[i].bits) as i32
                });
            }
            3 => {
                (0..data.weight_num).for_each(|i| wv[i] = (seq[i].bits << 3 | seq[i].bits) as i32);
            }
            4 => {
                (0..data.weight_num)
                    .for_each(|i| wv[i] = (seq[i].bits << 2 | seq[i].bits >> 2) as i32);
            }
            5 => {
                (0..data.weight_num)
                    .for_each(|i| wv[i] = (seq[i].bits << 1 | seq[i].bits >> 4) as i32);
            }
            _ => {
                panic!("Unsupported ASTC format");
            }
        }
        (0..data.weight_num).for_each(|i| {
            if wv[i] > 32 {
                wv[i] += 1
            }
        });
    } else if WEIGHT_PREC_TABLE_B[data.weight_range] == 0 {
        let s = if WEIGHT_PREC_TABLE_A[data.weight_range] == 3 {
            32
        } else {
            16
        };
        (0..data.weight_num).for_each(|i| wv[i] = (seq[i].nonbits * s) as i32);
    } else {
        if WEIGHT_PREC_TABLE_A[data.weight_range] == 3 {
            match WEIGHT_PREC_TABLE_B[data.weight_range] {
                1 => {
                    (0..data.weight_num).for_each(|i| wv[i] = (seq[i].nonbits * 50) as i32);
                }
                2 => {
                    (0..data.weight_num).for_each(|i| {
                        wv[i] = (seq[i].nonbits * 23) as i32;
                        if seq[i].bits & 2 != 0 {
                            wv[i] += 0b1000101;
                        }
                    });
                }
                3 => {
                    (0..data.weight_num).for_each(|i| {
                        wv[i] = (seq[i].nonbits * 11
                            + ((seq[i].bits << 4 | seq[i].bits >> 1) & 0b1100011))
                            as i32
                    });
                }
                _ => {
                    panic!("Unsupported ASTC format");
                }
            }
        } else if WEIGHT_PREC_TABLE_A[data.weight_range] == 5 {
            match WEIGHT_PREC_TABLE_B[data.weight_range] {
                1 => {
                    (0..data.weight_num).for_each(|i| wv[i] = (seq[i].nonbits * 28) as i32);
                }
                2 => {
                    (0..data.weight_num).for_each(|i| {
                        wv[i] = (seq[i].nonbits * 13) as i32;
                        if seq[i].bits & 2 != 0 {
                            wv[i] += 0b1000010;
                        }
                    });
                }
                _ => {
                    panic!("Unsupported ASTC format");
                }
            }
        }
        (0..data.weight_num).for_each(|i| {
            let a = (seq[i].bits & 1) * 0x7f;
            wv[i] = ((a & 0x20) | ((wv[i] as u64 ^ a) >> 2)) as i32;
            if wv[i] > 32 {
                wv[i] += 1;
            }
        });
    }

    let ds = (1024 + data.bw / 2) / (data.bw - 1);
    let dt = (1024 + data.bh / 2) / (data.bh - 1);
    let pn = if data.dual_plane { 2 } else { 1 };

    let mut i = 0;
    for t in 0..data.bh {
        for s in 0..data.bw {
            let gs = (ds * s * (data.width - 1) + 32) >> 6;
            let gt = (dt * t * (data.height - 1) + 32) >> 6;
            let fs = gs & 0xf;
            let ft = gt & 0xf;
            let v = (gs >> 4) + (gt >> 4) * data.width;
            let w11: i32 = ((fs * ft + 8) >> 4) as i32;
            let w10: i32 = ft as i32 - w11;
            let w01: i32 = fs as i32 - w11;
            let w00: i32 = 16 - fs as i32 - ft as i32 + w11;

            for p in 0..pn {
                let p00 = wv[v * pn + p];
                let p01 = wv[(v + 1) * pn + p];
                let p10 = wv[(v + data.width) * pn + p];
                let p11 = wv[(v + data.width + 1) * pn + p];
                data.weights[i][p] = (p00 * w00 + p01 * w01 + p10 * w10 + p11 * w11 + 8) >> 4;
            }

            i += 1;
        }
    }
}

fn select_partition(buf: &[u8], data: &mut BlockData) {
    let small_block = data.bw * data.bh < 31;
    // TODO - check if this cast is correct, original code uses (int *)buf
    let seed = (i32::from_le_bytes(buf[0..4].try_into().unwrap()) >> 13 & 0x3ff)
        | (data.part_num as i32 - 1) << 10;

    let mut rnum = seed as u32;
    rnum ^= rnum >> 15;
    rnum = rnum.overflowing_sub(rnum << 17).0;
    rnum = rnum.overflowing_add(rnum << 7).0;
    rnum = rnum.overflowing_add(rnum << 4).0;
    rnum ^= rnum >> 5;
    rnum = rnum.overflowing_add(rnum << 16).0;
    rnum ^= rnum >> 7;
    rnum ^= rnum >> 3;
    rnum ^= rnum << 6;
    rnum ^= rnum >> 17;

    let mut seeds: [i32; 8] = [0; 8];
    (0..8).for_each(|i| {
        let v = rnum >> (i * 4) & 0xF;
        seeds[i] = (v * v) as i32;
    });
    let sh: [i32; 2] = [
        if seed & 2 != 0 { 4 } else { 5 },
        if data.part_num == 3 { 6 } else { 5 },
    ];

    if seed & 1 != 0 {
        (0..8).for_each(|i| seeds[i] >>= sh[i % 2]);
    } else {
        (0..8).for_each(|i| seeds[i] >>= sh[1 - i % 2]);
    }

    let mut i = 0;
    if small_block {
        for t in 0..data.bh {
            for s in 0..data.bw {
                let x = s << 1;
                let y = t << 1;
                let a = (seeds[0] * x as i32 + seeds[1] * y as i32 + (rnum >> 14) as i32) & 0x3f;
                let b = (seeds[2] * x as i32 + seeds[3] * y as i32 + (rnum >> 10) as i32) & 0x3f;
                let c = if data.part_num < 3 {
                    0
                } else {
                    (seeds[4] * x as i32 + seeds[5] * y as i32 + (rnum >> 6) as i32) & 0x3f
                };
                let d = if data.part_num < 4 {
                    0
                } else {
                    (seeds[6] * x as i32 + seeds[7] * y as i32 + (rnum >> 2) as i32) & 0x3f
                };
                data.partition[i] = {
                    if a >= b && a >= c && a >= d {
                        0
                    } else if b >= c && b >= d {
                        1
                    } else if c >= d {
                        2
                    } else {
                        3
                    }
                };
                i += 1;
            }
        }
    } else {
        for y in 0..data.bh {
            for x in 0..data.bw {
                let a = (seeds[0] * x as i32 + seeds[1] * y as i32 + (rnum >> 14) as i32) & 0x3f;
                let b = (seeds[2] * x as i32 + seeds[3] * y as i32 + (rnum >> 10) as i32) & 0x3f;
                let c = if data.part_num < 3 {
                    0
                } else {
                    (seeds[4] * x as i32 + seeds[5] * y as i32 + (rnum >> 6) as i32) & 0x3f
                };
                let d = if data.part_num < 4 {
                    0
                } else {
                    (seeds[6] * x as i32 + seeds[7] * y as i32 + (rnum >> 2) as i32) & 0x3f
                };
                data.partition[i] = {
                    if a >= b && a >= c && a >= d {
                        0
                    } else if b >= c && b >= d {
                        1
                    } else if c >= d {
                        2
                    } else {
                        3
                    }
                };
                i += 1;
            }
        }
    }
}

fn applicate_color(data: &mut BlockData, outbuf: &mut [u32]) {
    static FUNC_TABLE_C: [fn(i32, i32, i32) -> u8; 16] = [
        select_color,
        select_color,
        select_color_hdr,
        select_color_hdr,
        select_color,
        select_color,
        select_color,
        select_color_hdr,
        select_color,
        select_color,
        select_color,
        select_color_hdr,
        select_color,
        select_color,
        select_color_hdr,
        select_color_hdr,
    ];
    static FUNC_TABLE_A: [fn(i32, i32, i32) -> u8; 16] = [
        select_color,
        select_color,
        select_color_hdr,
        select_color_hdr,
        select_color,
        select_color,
        select_color,
        select_color_hdr,
        select_color,
        select_color,
        select_color,
        select_color_hdr,
        select_color,
        select_color,
        select_color,
        select_color_hdr,
    ];
    if data.dual_plane {
        let mut ps: [usize; 4] = [0; 4];
        ps[data.plane_selector] = 1;
        if data.part_num > 1 {
            (0..(data.bw * data.bh)).for_each(|i| {
                let p = data.partition[i];
                let r: u8 = FUNC_TABLE_C[data.cem[p]](
                    data.endpoints[p][0],
                    data.endpoints[p][4],
                    data.weights[i][ps[0]],
                );
                let g: u8 = FUNC_TABLE_C[data.cem[p]](
                    data.endpoints[p][1],
                    data.endpoints[p][5],
                    data.weights[i][ps[1]],
                );
                let b: u8 = FUNC_TABLE_C[data.cem[p]](
                    data.endpoints[p][2],
                    data.endpoints[p][6],
                    data.weights[i][ps[2]],
                );
                let a: u8 = FUNC_TABLE_A[data.cem[p]](
                    data.endpoints[p][3],
                    data.endpoints[p][7],
                    data.weights[i][ps[3]],
                );
                outbuf[i] = color(r, g, b, a);
            });
        } else {
            (0..(data.bw * data.bh)).for_each(|i| {
                let r: u8 = FUNC_TABLE_C[data.cem[0]](
                    data.endpoints[0][0],
                    data.endpoints[0][4],
                    data.weights[i][ps[0]],
                );
                let g: u8 = FUNC_TABLE_C[data.cem[0]](
                    data.endpoints[0][1],
                    data.endpoints[0][5],
                    data.weights[i][ps[1]],
                );
                let b: u8 = FUNC_TABLE_C[data.cem[0]](
                    data.endpoints[0][2],
                    data.endpoints[0][6],
                    data.weights[i][ps[2]],
                );
                let a: u8 = FUNC_TABLE_A[data.cem[0]](
                    data.endpoints[0][3],
                    data.endpoints[0][7],
                    data.weights[i][ps[3]],
                );
                outbuf[i] = color(r, g, b, a);
            });
        }
    } else if data.part_num > 1 {
        (0..(data.bw * data.bh)).for_each(|i| {
            let p = data.partition[i];
            let r: u8 = FUNC_TABLE_C[data.cem[p]](
                data.endpoints[p][0],
                data.endpoints[p][4],
                data.weights[i][0],
            );
            let g: u8 = FUNC_TABLE_C[data.cem[p]](
                data.endpoints[p][1],
                data.endpoints[p][5],
                data.weights[i][0],
            );
            let b: u8 = FUNC_TABLE_C[data.cem[p]](
                data.endpoints[p][2],
                data.endpoints[p][6],
                data.weights[i][0],
            );
            let a: u8 = FUNC_TABLE_A[data.cem[p]](
                data.endpoints[p][3],
                data.endpoints[p][7],
                data.weights[i][0],
            );
            outbuf[i] = color(r, g, b, a);
        });
    } else {
        (0..(data.bw * data.bh)).for_each(|i| {
            let r: u8 = FUNC_TABLE_C[data.cem[0]](
                data.endpoints[0][0],
                data.endpoints[0][4],
                data.weights[i][0],
            );
            let g: u8 = FUNC_TABLE_C[data.cem[0]](
                data.endpoints[0][1],
                data.endpoints[0][5],
                data.weights[i][0],
            );
            let b: u8 = FUNC_TABLE_C[data.cem[0]](
                data.endpoints[0][2],
                data.endpoints[0][6],
                data.weights[i][0],
            );
            let a: u8 = FUNC_TABLE_A[data.cem[0]](
                data.endpoints[0][3],
                data.endpoints[0][7],
                data.weights[i][0],
            );
            outbuf[i] = color(r, g, b, a);
        });
    }
}

#[inline]
pub fn decode_astc_block(buf: &[u8], block_width: usize, block_height: usize, outbuf: &mut [u32]) {
    if buf[0] == 0xfc && (buf[1] & 1) == 1 {
        let c: u32 = if buf[1] & 2 != 0 {
            color(
                f16ptr_to_u8(&buf[8..]),
                f16ptr_to_u8(&buf[10..]),
                f16ptr_to_u8(&buf[12..]),
                f16ptr_to_u8(&buf[14..]),
            )
        } else {
            color(buf[9], buf[11], buf[13], buf[15])
        };
        outbuf[0..(block_width * block_height)].fill(c);
    } else if ((buf[0] & 0xc3) == 0xc0 && (buf[1] & 1) == 1) || (buf[0] & 0xf) == 0 {
        let c: u32 = color(255, 0, 255, 255);
        outbuf[0..(block_width * block_height)].fill(c);
    } else {
        let mut block_data = BlockData::default();
        block_data.bw = block_width;
        block_data.bh = block_height;
        decode_block_params(buf, &mut block_data);
        decode_endpoints(buf, &mut block_data);
        decode_weights(buf, &mut block_data);
        if block_data.part_num > 1 {
            select_partition(buf, &mut block_data);
        }
        applicate_color(&mut block_data, outbuf);
    }
}

pub fn decode_astc(
    data: &[u8],
    width: usize,
    height: usize,
    block_width: usize,
    block_height: usize,
    image: &mut [u32],
) -> Result<(), &'static str> {
    let num_blocks_x = width.div_ceil(block_width);
    let num_blocks_y = height.div_ceil(block_height);
    let mut buffer: [u32; 144] = [0; 144];
    let mut data_offset = 0;

    if data.len() < num_blocks_x * num_blocks_y * 16 {
        return Err("Not enough data to decode image!");
    }

    let expected_image_size = width * height;
    if image.len() < expected_image_size {
        return Err("Image buffer is too small!");
    }

    if block_width * block_height > 144 {
        return Err("Block size is too big!");
    }

    (0..num_blocks_y).for_each(|by| {
        (0..num_blocks_x).for_each(|bx| {
            decode_astc_block(&data[data_offset..], block_width, block_height, &mut buffer);
            copy_block_buffer(
                bx,
                by,
                width,
                height,
                block_width,
                block_height,
                &buffer,
                image,
            );
            data_offset += 16;
        });
    });

    Ok(())
}

// generate some sized astc block decode functions
macro_rules! astc_decode_func {
    ($x: expr, $y: expr) => {
        paste::item! {
            pub fn [<decode_astc_ $x _ $y>](
                data: &[u8],
                width: usize,
                height: usize,
                image: &mut [u32],
            ) -> Result<(), &'static str> {
                decode_astc(data, width, height, $x, $y, image)
            }
        }
    };
}

astc_decode_func!(4, 4);
astc_decode_func!(5, 4);
astc_decode_func!(5, 5);
astc_decode_func!(6, 5);
astc_decode_func!(6, 6);
astc_decode_func!(8, 5);
astc_decode_func!(8, 6);
astc_decode_func!(8, 8);
astc_decode_func!(10, 5);
astc_decode_func!(10, 6);
astc_decode_func!(10, 8);
astc_decode_func!(10, 10);
astc_decode_func!(12, 10);
astc_decode_func!(12, 12);
