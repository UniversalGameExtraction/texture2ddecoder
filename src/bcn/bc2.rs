use crate::bcn::bc1::decode_bc1_block;

#[inline]
pub fn decode_bc2_alpha(data: &[u8], outbuf: &mut [u32], channel: usize) {
    let channel_shift = channel * 8;
    let channel_mask = 0xFFFFFFFF ^ (0xFF << channel_shift);
    (0..16).for_each(|i| {
        let bit_i = i * 4;
        let by_i = bit_i >> 3;
        let av = 0xf & (data[by_i] >> (bit_i & 7));
        let av = (av << 4) | av;
        outbuf[i] = (outbuf[i] & channel_mask) | (av as u32) << channel_shift;
    });
}

#[inline]
pub fn decode_bc2_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc1_block(&data[8..], outbuf);
    decode_bc2_alpha(data, outbuf, 3);
}
