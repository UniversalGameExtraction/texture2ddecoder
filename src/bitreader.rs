#[inline]
fn getbits_raw(buf: &[u8], bit_offset: usize, num_bits: usize, dst: &mut [u8]) {
    let bytes_offset = bit_offset / 8;
    let bytes_end: usize = (bit_offset + num_bits + 7) / 8;
    dst[0..(bytes_end - bytes_offset)].copy_from_slice(&buf[bytes_offset..bytes_end]);
}

#[inline]
pub fn getbits(buf: &[u8], bit_offset: usize, num_bits: usize) -> i32 {
    let shift = bit_offset % 8;

    let mut raw = [0u8; 4];
    getbits_raw(buf, bit_offset, num_bits, &mut raw);

    // shift the bits we don't need out
    i32::from_le_bytes(raw) >> shift & ((1 << num_bits) - 1)
}

#[inline]
pub fn getbits64(buf: &[u8], bit: isize, len: usize) -> u64 {
    let mask: u64 = if len == 64 {
        0xffffffffffffffff
    } else {
        (1 << len) - 1
    };
    if len == 0 {
        0
    } else if bit >= 64 {
        u64::from_le_bytes(buf[8..16].try_into().unwrap()) >> (bit - 64) & mask
    } else if bit <= 0 {
        u64::from_le_bytes(buf[..8].try_into().unwrap()) << 0u64.overflowing_sub(bit as u64).0
            & mask
    } else if bit as usize + len <= 64 {
        u64::from_le_bytes(buf[..8].try_into().unwrap()) >> bit & mask
    } else {
        u64::from_le_bytes(buf[..8].try_into().unwrap()) >> bit
            | u64::from_le_bytes(buf[8..16].try_into().unwrap()) << (64 - bit) & mask
    }
}

pub struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl BitReader<'_> {
    #[inline]
    pub const fn new(data: &[u8], bit_pos: usize) -> BitReader {
        BitReader { data, bit_pos }
    }

    #[inline]
    pub fn read(&mut self, num_bits: usize) -> u16 {
        let ret = self.peek(0, num_bits);
        self.bit_pos += num_bits;
        ret
    }

    #[inline]
    pub fn peek(&self, offset: usize, num_bits: usize) -> u16 {
        let bit_pos = self.bit_pos + offset;
        let shift = bit_pos & 7;

        let mut raw = [0u8; 4];
        getbits_raw(self.data, bit_pos, num_bits, &mut raw);
        let data: u32 = u32::from_le_bytes(raw);

        (data >> shift as u32) as u16 & ((1 << num_bits as u16) - 1)
    }
}
