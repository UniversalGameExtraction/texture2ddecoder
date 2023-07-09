#![allow(clippy::too_many_arguments)]

pub static TRANSPARENT_MASK: u32 = {
    #[cfg(target_endian = "little")]
    {
        0x00ffffff
    }
    #[cfg(target_endian = "big")]
    {
        0xffffff00
    }
};

pub static TRANSPARENT_SHIFT: u32 = {
    #[cfg(target_endian = "little")]
    {
        24
    }
    #[cfg(target_endian = "big")]
    {
        0
    }
};

#[inline]
pub const fn color(r: u8, g: u8, b: u8, a: u8) -> u32 {
    u32::from_le_bytes([b, g, r, a])
}

// #[cfg(target_endian = "little")]
// #[inline]
// pub fn alpha_mask(a: u8) -> u32 {
//     TRANSPARENT_MASK | (a as u32) << 24
// }

// #[cfg(target_endian = "big")]
// #[inline]
// pub fn alpha_mask(a: u8) -> u32 {
//     TRANSPARENT_MASK | a as u32
// }

// #[cfg(target_endian = "little")]
#[inline]
pub const fn rgb565_le(d: u16) -> (u8, u8, u8) {
    (
        (d >> 8 & 0xf8) as u8 | (d >> 13) as u8,
        (d >> 3 & 0xfc) as u8 | (d >> 9 & 3) as u8,
        (d << 3) as u8 | (d >> 2 & 7) as u8,
    )
}

// #[cfg(target_endian = "big")]
// #[inline]
// pub fn rgb565_le(d: u16) -> (u8, u8, u8) {
//     (
//         (d & 0xf8) as u8 | (d >> 5 & 7) as u8,
//         (d << 5 & 0xe0) as u8 | (d >> 11 & 0x1c) as u8 | (d >> 1 & 3) as u8,
//         (d >> 5 & 0xf8) as u8 | (d >> 10 & 0x7) as u8,
//     )
// }

// #[cfg(target_endian = "little")]
// #[inline]
// pub fn rgb565_be(d: u16) -> (u8, u8, u8) {
//     (
//         (d & 0xf8) as u8 | (d >> 5 & 7) as u8,
//         (d << 5 & 0xe0) as u8 | (d >> 11 & 0x1c) as u8 | (d >> 1 & 3) as u8,
//         (d >> 5 & 0xf8) as u8 | (d >> 10 & 0x7) as u8,
//     )
// }

// #[cfg(target_endian = "big")]
// #[inline]
// pub fn rgb565_be(d: u16) -> (u8, u8, u8) {
//     (
//         (d >> 8 & 0xf8) as u8 | (d >> 13) as u8,
//         (d >> 3 & 0xfc) as u8 | (d >> 9 & 3) as u8,
//         (d << 3) as u8 | (d >> 2 & 7) as u8,
//     )
// }

#[inline]
pub fn copy_block_buffer(
    bx: usize,
    by: usize,
    w: usize,
    h: usize,
    bw: usize,
    bh: usize,
    buffer: &[u32],
    image: &mut [u32],
) {
    let x: usize = bw * bx;
    let xl: usize = if bw * (bx + 1) > w { w - bw * bx } else { bw };

    let mut buffer_offset = 0;
    for y in by * bh..h {
        let image_offset = y * w + x;
        image[image_offset..image_offset + xl]
            .copy_from_slice(&buffer[buffer_offset..buffer_offset + xl]);

        buffer_offset += bw;
        if buffer_offset >= buffer.len() {
            break;
        }
    }
}
