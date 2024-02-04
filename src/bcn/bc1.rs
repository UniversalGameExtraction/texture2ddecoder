use crate::color::{color, rgb565_le};

#[inline]
pub fn decode_bc1_block(data: &[u8], outbuf: &mut [u32]) {
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
        c[3] = color(0, 0, 0, 0);
    }
    let mut d: usize = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    (0..16).for_each(|i| {
        outbuf[i] = c[d & 3];
        d >>= 2;
    });
}
