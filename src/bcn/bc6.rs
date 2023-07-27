use crate::bcn::consts::{S_BPTC_A2, S_BPTC_FACTORS, S_BPTC_P2};
use crate::bitreader::BitReader;
use crate::color::color;

struct Bc6hModeInfo {
    transformed: bool,
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
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 10,
        delta_bits: [5, 5, 5],
    },
    // 01
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 7,
        delta_bits: [6, 6, 6],
    },
    // 00010 5-bits
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [5, 4, 4],
    },
    // 00011
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 10,
        delta_bits: [10, 10, 10],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00110
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [4, 5, 4],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 0,
        endpoint_bits: 11,
        delta_bits: [9, 9, 9],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 11,
        delta_bits: [4, 4, 5],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 0,
        endpoint_bits: 12,
        delta_bits: [8, 8, 8],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 9,
        delta_bits: [5, 5, 5],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 0,
        endpoint_bits: 16,
        delta_bits: [4, 4, 4],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [6, 5, 5],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [5, 6, 5],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: true,
        partition_bits: 5,
        endpoint_bits: 8,
        delta_bits: [5, 5, 6],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
    // 00010
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 5,
        endpoint_bits: 6,
        delta_bits: [6, 6, 6],
    },
    // -
    Bc6hModeInfo {
        transformed: false,
        partition_bits: 0,
        endpoint_bits: 0,
        delta_bits: [0, 0, 0],
    },
];

fn unquantize(_value: u16, _signed: bool, _endpoint_bits: usize) -> u16 {
    let max_value: u16 = 1 << (_endpoint_bits - 1);

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
            unq = ((((_value as u32) << 15) + 0x4000) >> (_endpoint_bits - 1)) as u16;
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

    ((((_value as u32) << 15) + 0x4000) >> (_endpoint_bits - 1)) as u16
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
    (f * 255.0).clamp(0.0, 255.0) as u8
}

#[inline]
fn f16_to_u8(h: u16) -> u8 {
    f32_to_u8(crate::f16::fp16_ieee_to_fp32_value(h))
    //f32_to_u8(f16::from_bits(h).to_f32())
}

pub fn decode_bc6_block(data: &[u8], outbuf: &mut [u32], signed: bool) {
    let mut bit: BitReader = BitReader::new(data, 0);

    let mut mode: u8 = bit.read(2) as u8;

    let mut ep_r: [u16; 4] = [0; 4]; //{ /* rw, rx, ry, rz */ };
    let mut ep_g: [u16; 4] = [0; 4]; //{ /* gw, gx, gy, gz */ };
    let mut ep_b: [u16; 4] = [0; 4]; //{ /* bw, bx, by, bz */ };

    if mode & 2 != 0 {
        // 5-bit mode
        mode |= (bit.read(3) << 2) as u8;

        if 0 == S_BC6H_MODE_INFO[mode as usize].endpoint_bits {
            outbuf[0..16].fill(0);
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

    if signed {
        ep_r[0] = sign_extend(ep_r[0], mi.endpoint_bits);
        ep_g[0] = sign_extend(ep_g[0], mi.endpoint_bits);
        ep_b[0] = sign_extend(ep_b[0], mi.endpoint_bits);
    }

    let num_subsets: usize = if mi.partition_bits != 0 { 2 } else { 1 };

    (1..num_subsets * 2).for_each(|ii| {
        if signed || mi.transformed {
            ep_r[ii] = sign_extend(ep_r[ii], mi.delta_bits[0]);
            ep_g[ii] = sign_extend(ep_g[ii], mi.delta_bits[1]);
            ep_b[ii] = sign_extend(ep_b[ii], mi.delta_bits[2]);
        }

        if mi.transformed {
            let mask = (1 << mi.endpoint_bits) - 1;

            ep_r[ii] = ep_r[ii].overflowing_add(ep_r[0]).0 & mask;
            ep_g[ii] = ep_g[ii].overflowing_add(ep_g[0]).0 & mask;
            ep_b[ii] = ep_b[ii].overflowing_add(ep_b[0]).0 & mask;

            if signed {
                ep_r[ii] = sign_extend(ep_r[ii], mi.endpoint_bits);
                ep_g[ii] = sign_extend(ep_g[ii], mi.endpoint_bits);
                ep_b[ii] = sign_extend(ep_b[ii], mi.endpoint_bits);
            }
        }
    });

    (0..num_subsets * 2).for_each(|ii| {
        ep_r[ii] = unquantize(ep_r[ii], signed, mi.endpoint_bits);
        ep_g[ii] = unquantize(ep_g[ii], signed, mi.endpoint_bits);
        ep_b[ii] = unquantize(ep_b[ii], signed, mi.endpoint_bits);
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
                ((ep_r[subset_index] as u32 * fca + ep_r[subset_index + 1] as u32 * fcb + 32) >> 6)
                    as u16,
                signed,
            );
            let gg = finish_unquantize(
                ((ep_g[subset_index] as u32 * fca + ep_g[subset_index + 1] as u32 * fcb + 32) >> 6)
                    as u16,
                signed,
            );
            let bb = finish_unquantize(
                ((ep_b[subset_index] as u32 * fca + ep_b[subset_index + 1] as u32 * fcb + 32) >> 6)
                    as u16,
                signed,
            );

            outbuf[idx] = color(f16_to_u8(rr), f16_to_u8(gg), f16_to_u8(bb), 255);
        });
    });
}

#[inline]
pub fn decode_bc6_block_signed(data: &[u8], outbuf: &mut [u32]) {
    decode_bc6_block(data, outbuf, true);
}

#[inline]
pub fn decode_bc6_block_unsigned(data: &[u8], outbuf: &mut [u32]) {
    decode_bc6_block(data, outbuf, false);
}
