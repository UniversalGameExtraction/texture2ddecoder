use crate::bcn::bc1::decode_bc1_block;

#[inline]
pub fn decode_bc2_alpha(data: &[u8], outbuf: &mut [u32], channel: usize) {
    let channel_shift = channel * 8;
    let channel_mask = 0xFFFFFFFF ^ (0xFF << channel_shift);
    outbuf[0..16].iter_mut().enumerate().for_each(|(i, p)| {
        // 4 bit alpha encoding - one byte for two pixels
        // -> byte index increases every two entriees
        // -> bit offset switched between 0 and 4
        let byte_idx = i >> 1;
        let bit_off = (i & 1) << 2;
        let mut av = (data[byte_idx] >> bit_off) & 0xF;
        av = (av << 4) | av;
        *p = (*p & channel_mask) | ((av as u32) << channel_shift);
    })
}

#[inline]
pub fn decode_bc2_block(data: &[u8], outbuf: &mut [u32]) {
    decode_bc1_block(&data[8..], outbuf);
    decode_bc2_alpha(data, outbuf, 3);
}
