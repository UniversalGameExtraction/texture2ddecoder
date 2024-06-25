use crate::bcn::bc1::decode_bc1_block;

#[inline]
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
pub fn decode_bc3_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc1_block(&data[8..], outbuf);
    decode_bc3_alpha(data, outbuf, 3);
}
