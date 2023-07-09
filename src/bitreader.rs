use core::cmp::min;

pub struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl BitReader<'_> {
    pub fn new(data: &[u8], bit_pos: usize) -> BitReader {
        BitReader { data, bit_pos }
    }

    pub fn read(&mut self, num_bits: usize) -> u16 {
        let ret = self.peek(0, num_bits);
        self.bit_pos += num_bits;
        ret
    }

    pub fn peek(&self, offset: usize, num_bits: usize) -> u16 {
        let bit_pos = self.bit_pos + offset;
        let shift = bit_pos & 7;
        let pos = bit_pos / 8;

        let num_bytes = min(4, 16 - pos);
        let end = pos + num_bytes;
        let start = end - 4;

        let data: u32 = u32::from_le_bytes(self.data[start..end].try_into().unwrap());

        (data >> shift as u32) as u16 & ((1 << num_bits as u16) - 1)
    }
}
