

// static BitReverseTable: [u8; 256] = [
//   0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0, 0x08, 0x88, 0x48,
//   0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8, 0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4,
//   0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4, 0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C,
//   0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC, 0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2,
//   0x32, 0xB2, 0x72, 0xF2, 0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A,
//   0xFA, 0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6, 0x0E, 0x8E,
//   0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE, 0x01, 0x81, 0x41, 0xC1, 0x21,
//   0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1, 0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9,
//   0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9, 0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55,
//   0xD5, 0x35, 0xB5, 0x75, 0xF5, 0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD,
//   0x7D, 0xFD, 0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3, 0x0B,
//   0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB, 0x07, 0x87, 0x47, 0xC7,
//   0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7, 0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F,
//   0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF];

// static WeightPrecTableA: [i32; 16] = [0, 0, 0, 3, 0, 5, 3, 0, 0, 0, 5, 3, 0, 5, 3, 0];
// static WeightPrecTableB:[ i32; 16] = [0, 0, 1, 0, 2, 0, 1, 3, 0, 0, 1, 2, 4, 2, 3, 5];

// static CemTableA: [i32; 16] = [0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 5, 0, 3, 0, 0];
// static CemTableB: [i32; 16] = [8, 6, 5, 7, 5, 4, 6, 4, 3, 5, 3, 2, 4, 2, 1, 3, 1, 2, 1];

// #[inline]
// const fn bit_reverse_u8(c: u8, bits: u8) -> u8 {
//     BitReverseTable[c as usize] >> (8 - bits)
// }

// #[inline]
// const fn bit_reverse_u64(d: u64, bits: u64) -> u64 {
//     let d = d as usize;
//     let ret = BitReverseTable[d & 0xff] << 56 | BitReverseTable[d >> 8 & 0xff] << 48 |
//       BitReverseTable[d >> 16 & 0xff] << 40 | BitReverseTable[d >> 24 & 0xff] << 32 |
//       BitReverseTable[d >> 32 & 0xff] << 24 | BitReverseTable[d >> 40 & 0xff] << 16 |
//       BitReverseTable[d >> 48 & 0xff] << 8 | BitReverseTable[d >> 56 & 0xff];
//     ret >> (64 - bits)
// }

// #[inline]
// const fn getbits(buf: &[u8], bit: i32, len: usize) -> i32 {
//     panic!("getbits not implemented");
//     //return (*(int *)(buf + bit / 8) >> (bit % 8)) & ((1 << len) - 1);
// }

// #[inline]
// const fn getbits64(buf: &[u8], bit: u64, len: u64) -> u64 {
//     panic!("getbits64 not implemented");
//     // uint_fast64_t mask = len == 64 ? 0xffffffffffffffff : (1ull << len) - 1;
//     // if (len < 1)
//     //     return 0;
//     // else if (bit >= 64)
//     //     return (*(uint_fast64_t *)(buf + 8)) >> (bit - 64) & mask;
//     // else if (bit <= 0)
//     //     return (*(uint_fast64_t *)buf) << -bit & mask;
//     // else if (bit + len <= 64)
//     //     return (*(uint_fast64_t *)buf) >> bit & mask;
//     // else
//     //     return ((*(uint_fast64_t *)buf) >> bit | *(uint_fast64_t *)(buf + 8) << (64 - bit)) & mask;
// }

// #[inline]
// const fn u8ptr_to_u16(ptr: &[u8]) -> u16 {
//     u16::from_le_bytes([ptr[0], ptr[1]])
// }

// #[inline]
// const fn clamp(n: i32) -> u8 {
//     if n < 0 {0} else { if n > 255 {255} else {n as u8} }
// }

// #[inline]
// const fn bit_transfer_signed(a: &mut i32, b: &mut i32) {
//     *b = (*b >> 1) | (*a & 0x80);
//     *a = (*a >> 1) & 0x3f;
//     if (*a & 0x20){
//         *a -= 0x40;
//     }
// }

// #[inline]
// const fn set_endpoint(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = r1;
//     endpoint[1] = g1;
//     endpoint[2] = b1;
//     endpoint[3] = a1;
//     endpoint[4] = r2;
//     endpoint[5] = g2;
//     endpoint[6] = b2;
//     endpoint[7] = a2;
// }

// #[inline]
// const fn set_endpoint_clamp(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = clamp(r1);
//     endpoint[1] = clamp(g1);
//     endpoint[2] = clamp(b1);
//     endpoint[3] = clamp(a1);
//     endpoint[4] = clamp(r2);
//     endpoint[5] = clamp(g2);
//     endpoint[6] = clamp(b2);
//     endpoint[7] = clamp(a2);
// }

// #[inline]
// const fn set_endpoint_blue(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = (r1 + b1) >> 1;
//     endpoint[1] = (g1 + b1) >> 1;
//     endpoint[2] = b1;
//     endpoint[3] = a1;
//     endpoint[4] = (r2 + b2) >> 1;
//     endpoint[5] = (g2 + b2) >> 1;
//     endpoint[6] = b2;
//     endpoint[7] = a2;
// }

// #[inline]
// const fn set_endpoint_blue_clamp(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = clamp((r1 + b1) >> 1);
//     endpoint[1] = clamp((g1 + b1) >> 1);
//     endpoint[2] = clamp(b1);
//     endpoint[3] = clamp(a1);
//     endpoint[4] = clamp((r2 + b2) >> 1);
//     endpoint[5] = clamp((g2 + b2) >> 1);
//     endpoint[6] = clamp(b2);
//     endpoint[7] = clamp(a2);
// }

// #[inline]
// const fn clamp_hdr(n: i32)  -> u16 {
//     if n < 0 {0} else { if n > 0xfff {0xfff} else {n as u8} }
// }

// #[inline]
// const fn set_endpoint_hdr(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = r1;
//     endpoint[1] = g1;
//     endpoint[2] = b1;
//     endpoint[3] = a1;
//     endpoint[4] = r2;
//     endpoint[5] = g2;
//     endpoint[6] = b2;
//     endpoint[7] = a2;
// }

// #[inline]
// const fn set_endpoint_hdr_clamp(endpoint: &mut [u8], r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, a2: i32) {
//     endpoint[0] = clamp_hdr(r1);
//     endpoint[1] = clamp_hdr(g1);
//     endpoint[2] = clamp_hdr(b1);
//     endpoint[3] = clamp_hdr(a1);
//     endpoint[4] = clamp_hdr(r2);
//     endpoint[5] = clamp_hdr(g2);
//     endpoint[6] = clamp_hdr(b2);
//     endpoint[7] = clamp_hdr(a2);
// }

// // typedef uint_fast8_t (*t_select_folor_func_ptr)(int, int, int);

// #[inline]
// const fn select_color(v0: i32, v1: i32, weight: i32) -> u8 {
//     ((((v0 << 8 | v0) * (64 - weight) + (v1 << 8 | v1) * weight + 32) >> 6) * 255 + 32768) / 65536
// }

// #[inline]
// const fn select_color_hdr(v0: i32, v1: i32, weight: i32) -> u8 {
//     let c: u16 = ((v0 << 4) * (64 - weight) + (v1 << 4) * weight + 32) >> 6;
//     let mut m: u16 = c & 0x7ff;
//     if (m < 512){
//         m *= 3;
//     }
//     else if (m < 1536) {
//         m = 4 * m - 512;
//     }
//     else {
//         m = 5 * m - 2048;
//     }
//     let f: f32 = fp16_ieee_to_fp32_value((c >> 1 & 0x7c00) | m >> 3);
//     if f32::is_finite(f) {clamp((f * 255).floor())} else {255};
// }

// #[inline]
// const fn f32_to_u8(f: f32) -> u8 {
//     c = (f * 255).floor();
//     if (c < 0) {
//         0
//     }
//     else if c > 255{
//         255
//     }
//     else {
//         c
//     }
// }

// #[inline]
// const fn f16ptr_to_u8(ptr: *const u8) -> u8 {
//     f32_to_u8(f16::from_le_bytes([ptr[0],ptr[1]]).to_f32())
// }


// struct BlockData {
//     bw: usize,
//     bh: usize,
//     width: usize,
//     height: usize,
//     part_num: usize,
//     dual_plane: usize,
//     plane_selector: usize,
//     weight_range: usize,
//     weight_num: usize,
//     cem: [usize;4],
//     cem_range: usize,
//     endpoint_value_num: usize,
//     endpoints: [[usize; 8]; 4],
//     weights: [[usize; 2]; 144],
//     partition: [usize;144],
// }

// struct IntSeqData {
//     bits: usize,
//     nonbits: usize,
// }

// fn decode_intseq(buf: &[u8], offset: usize, a: usize, b: usize, count: usize, reverse: bool, out: &mut [IntSeqData]) {
//     // TODO: reduce code duplication
//     static mt: [usize; 5] = [0, 2, 4, 5, 7];
//     static mq: [usize; 3] = [0, 3, 5];
//     static TritsTable: [[usize; 256];5] = [
//         [0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 0, 0,
//          1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 1, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1,
//          2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2,
//          2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0,
//          0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0,
//          1, 2, 2, 0, 1, 2, 1, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1,
//          2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 0, 0, 1, 2, 1, 0, 1, 2, 2, 0, 1, 2, 2],
//         [0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//          1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2,
//          2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2,
//          0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1,
//          1, 1, 1, 1, 2, 2, 2, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2,
//          2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2,
//          2, 1, 0, 0, 0, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 2, 1],
//         [0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0,
//          0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0,
//          0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2,
//          2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2,
//          1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1,
//          1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1,
//          1, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 2, 2],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1, 1,
//          1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//          2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//          0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//          1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//          2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0,
//          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
//          2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//          1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//          1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2,
//          2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]];
//     static QuintsTable: [[usize; 128];3] = [
//         [0, 1, 2, 3, 4, 0, 4, 4, 0, 1, 2, 3, 4, 1, 4, 4, 0, 1, 2, 3, 4, 2, 4, 4, 0, 1, 2, 3, 4, 3, 4, 4,
//          0, 1, 2, 3, 4, 0, 4, 0, 0, 1, 2, 3, 4, 1, 4, 1, 0, 1, 2, 3, 4, 2, 4, 2, 0, 1, 2, 3, 4, 3, 4, 3,
//          0, 1, 2, 3, 4, 0, 2, 3, 0, 1, 2, 3, 4, 1, 2, 3, 0, 1, 2, 3, 4, 2, 2, 3, 0, 1, 2, 3, 4, 3, 2, 3,
//          0, 1, 2, 3, 4, 0, 0, 1, 0, 1, 2, 3, 4, 1, 0, 1, 0, 1, 2, 3, 4, 2, 0, 1, 0, 1, 2, 3, 4, 3, 0, 1],
//         [0, 0, 0, 0, 0, 4, 4, 4, 1, 1, 1, 1, 1, 4, 4, 4, 2, 2, 2, 2, 2, 4, 4, 4, 3, 3, 3, 3, 3, 4, 4, 4,
//          0, 0, 0, 0, 0, 4, 0, 4, 1, 1, 1, 1, 1, 4, 1, 4, 2, 2, 2, 2, 2, 4, 2, 4, 3, 3, 3, 3, 3, 4, 3, 4,
//          0, 0, 0, 0, 0, 4, 0, 0, 1, 1, 1, 1, 1, 4, 1, 1, 2, 2, 2, 2, 2, 4, 2, 2, 3, 3, 3, 3, 3, 4, 3, 3,
//          0, 0, 0, 0, 0, 4, 0, 0, 1, 1, 1, 1, 1, 4, 1, 1, 2, 2, 2, 2, 2, 4, 2, 2, 3, 3, 3, 3, 3, 4, 3, 3],
//         [0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 3, 4,
//          1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4, 1, 1, 1, 1, 1, 1, 4, 4,
//          2, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4, 2, 2, 2, 2, 2, 2, 4, 4,
//          3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4, 3, 3, 3, 3, 3, 3, 4, 4]];
    


//     if count <= 0{
//         return;
//     }

//     let mut n = 0;
//     let mut p = offset;
//     match a{
//         3 => {
//             let mask = (1 << b) - 1;
//             let block_count = (count + 4) / 5;
//             let last_block_count = (count + 4) % 5 + 1;
//             let block_size = 8 + 5 * b;
//             let last_block_size = (block_size * last_block_count + 4) / 5;
            
            
//             if reverse {
//                 (0..block_count).map(|i|{
//                     let now_size = if (i < block_count - 1) {block_size} else {last_block_size};
//                     let d = bit_reverse_u64(getbits64(buf, p - now_size, now_size), now_size);
//                     let x = (d >> b & 3) | (d >> b * 2 & 0xc) | (d >> b * 3 & 0x10) | (d >> b * 4 & 0x60) | (d >> b * 5 & 0x80);
//                     for j in 0..5{
//                         if n < count{
//                             out[n] = IntSeqData{d: (d >> (mt[j] + b * j)) & mask, q: TritsTable[j][x]};
//                             n += 1;
//                         }
//                     }
//                     p -= block_size;
//                 });
//             } else {
//                 (0..block_count).map(|i|{
//                     let now_size = if (i < block_count - 1) {block_size} else {last_block_size};
//                     let d = getbits64(buf, p, now_size);
//                     let x = (d >> b & 3) | (d >> b * 2 & 0xc) | (d >> b * 3 & 0x10) | (d >> b * 4 & 0x60) | (d >> b * 5 & 0x80);
//                     for j in 0..5{
//                         if n < count{
//                             out[n] = IntSeqData{d: (d >> (mt[j] + b * j)) & mask, q: TritsTable[j][x]};
//                             n += 1;
//                         }
//                     }
//                     p += block_size;
//                 });
//             }
//         }
//         5 => {
//             let mask = (1 << b) - 1;
//             let block_count = (count + 2) / 3;
//             let last_block_count = (count + 2) % 3 + 1;
//             let block_size = 7 + 3 * b;
//             let last_block_size = (block_size * last_block_count + 2) / 3;
            
//             if (reverse) {
//                 (0..block_count).map(|i|{
//                     let now_size = if (i < block_count - 1) {block_size} else {last_block_size};
//                     let d = bit_reverse_u64(getbits64(buf, p - now_size, now_size), now_size);
//                     let x = (d >> b & 7) | (d >> b * 2 & 0x18) | (d >> b * 3 & 0x60);
//                     for j in 0..3{
//                         if n < count{
//                             out[n] = IntSeqData{d: (d >> (mq[j] + b * j)) & mask, q: QuintsTable[j][x]};
//                             n += 1;
//                         }
//                     }
//                     p -= block_size;
//                 });
//             } else {
//                 (0..block_count).map(|i|{
//                     let now_size = if (i < block_count - 1) {block_size} else {last_block_size};
//                     let d = getbits64(buf, p, now_size);
//                     let x = (d >> b & 7) | (d >> b * 2 & 0x18) | (d >> b * 3 & 0x60);
//                     for j in 0..3{
//                         if n < count{
//                             out[n] = IntSeqData{d: (d >> (mq[j] + b * j)) & mask, q: QuintsTable[j][x]};
//                             n += 1;
//                         }
//                     }
//                     p += block_size;
//                 });
//             }
//         }
//         _ => {
//             if (reverse){
//                 while (n < count) {
//                     out[n] = IntSeqData{d:bit_reverse_u8(getbits(buf, p, b), b), q:0};
//                     n+=1;
//                     p-=b;
//                 }
//             }
//             else {
//                 while (n < count) {
//                     out[n] = IntSeqData{d:bit_reverse_u8(getbits(buf, p, b), 0), q:0};
//                     n+=1;
//                     p+=b;
//                 }
//             }
//         }
//     }
// }

// fn decode_block_params(buf: &[u8], block_data: &mut BlockData) {
//     block_data.dual_plane = (buf[1] & 4) != 0;
//     block_data.weight_range = (buf[0] >> 4 & 1) | (buf[1] << 2 & 8);

//     if (buf[0] & 3 != 0) {
//         block_data.weight_range |= buf[0] << 1 & 6;
//         match buf[0] & 0xc{
//             0 => {
//                 block_data.width = (u8ptr_to_u16(buf) >> 7 & 3) + 4;
//                 block_data.height = (buf[0] >> 5 & 3) + 2;
//                 },
//             4 => {
//                 block_data.width = (u8ptr_to_u16(buf) >> 7 & 3) + 8;
//                 block_data.height = (buf[0] >> 5 & 3) + 2;
//                 },
//             8 => {
//                 block_data.width = (buf[0] >> 5 & 3) + 2;
//                 block_data.height = (u8ptr_to_u16(buf) >> 7 & 3) + 8;
//                 },
//             12 => {
//                 if (buf[1] & 1 != 0) {
//                     block_data.width = (buf[0] >> 7 & 1) + 2;
//                     block_data.height = (buf[0] >> 5 & 3) + 2;
//                 } else {
//                     block_data.width = (buf[0] >> 5 & 3) + 2;
//                     block_data.height = (buf[0] >> 7 & 1) + 6;
//                 }
//                 },
//             _ => {}
//         }
//     } else {
//         block_data.weight_range |= buf[0] >> 1 & 6;
//         match (u8ptr_to_u16(buf) & 0x180 != 0) {
//             0 => {
//                 block_data.width = 12;
//                 block_data.height = (buf[0] >> 5 & 3) + 2;
//                 },
//             0x80 => {
//                 block_data.width = (buf[0] >> 5 & 3) + 2;
//                 block_data.height = 12;
//                 },
//             0x100 => {
//                 block_data.width = (buf[0] >> 5 & 3) + 6;
//                 block_data.height = (buf[1] >> 1 & 3) + 6;
//                 block_data.dual_plane = 0;
//                 block_data.weight_range &= 7;
//                 },
//             0x180 => {
//                 block_data.width = if (buf[0] & 0x20) {10} else {6};
//                 block_data.height = if (buf[0] & 0x20) {6} else {10};
//                 },
//             _ => {}
//         }
//     }

//     block_data.part_num = (buf[1] >> 3 & 3) + 1;

//     block_data.weight_num = block_data.width * block_data.height;
//     if (block_data.dual_plane){
//         block_data.weight_num *= 2;
//     }
//     let mut weight_bits = 0;
//     let mut config_bits = 0;
//     let mut cem_base = 0;

//     match (WeightPrecTableA[block_data.weight_range]) {
//         3 => {
//             weight_bits =
//             block_data.weight_num * WeightPrecTableB[block_data.weight_range] + (block_data.weight_num * 8 + 4) / 5;
//             },
//         5 => {
//             weight_bits =
//             block_data.weight_num * WeightPrecTableB[block_data.weight_range] + (block_data.weight_num * 7 + 2) / 3;
//             },
//         _ => {
//             weight_bits = block_data.weight_num * WeightPrecTableB[block_data.weight_range];
//         }
//     }

//     if (block_data.part_num == 1) {
//         block_data.cem[0] = u8ptr_to_u16(buf + 1) >> 5 & 0xf;
//         config_bits = 17;
//     } else {
//         cem_base = u8ptr_to_u16(buf + 2) >> 7 & 3;
//         if (cem_base == 0) {
//             let cem = buf[3] >> 1 & 0xf;
//             block_data.cem = [cem; block_data.part_num];
//             config_bits = 29;
//         } else {
//             (0..block_data.part_num).for_each(|i| block_data.cem[i] = ((buf[3] >> (i + 1) & 1) + cem_base - 1) << 2);
//             match (block_data.part_num) {
//                 2 => {
//                     block_data.cem[0] |= buf[3] >> 3 & 3;
//                     block_data.cem[1] |= getbits(buf, 126 - weight_bits, 2);
//                     },
//                 3 => {
//                     block_data.cem[0] |= buf[3] >> 4 & 1;
//                     block_data.cem[0] |= getbits(buf, 122 - weight_bits, 2) & 2;
//                     block_data.cem[1] |= getbits(buf, 124 - weight_bits, 2);
//                     block_data.cem[2] |= getbits(buf, 126 - weight_bits, 2);
//                     },
//                 4 => {
//                     (0..4).for_each(|i| block_data.cem[i] |= getbits(buf, 120 + i * 2 - weight_bits, 2));
//                     },
//                 _ => {}
//             }
//             config_bits = 25 + block_data.part_num * 3;
//         }
//     }

//     if (block_data.dual_plane) {
//         config_bits += 2;
//         block_data.plane_selector =
//           getbits(buf, if cem_base {130 - weight_bits - block_data.part_num * 3} else {126 - weight_bits}, 2);
//     }

//     let remain_bits = 128 - config_bits - weight_bits;

//     block_data.endpoint_value_num = 0;

//     (0..block_data.part_num).for_each(|i| block_data.endpoint_value_num += (block_data.cem[i] >> 1 & 6) + 2);

//     let mut endpoint_bits;
//     CemTableA.for_each(|entry|{
//         match entry{
//             3 => {
//                 endpoint_bits =
//                   block_data.endpoint_value_num * CemTableB[i] + (block_data.endpoint_value_num * 8 + 4) / 5;
//                 },
//             5 => {
//                 endpoint_bits =
//                   block_data.endpoint_value_num * CemTableB[i] + (block_data.endpoint_value_num * 7 + 2) / 3;
//                 },
//             _ => {
//                 endpoint_bits = block_data.endpoint_value_num * CemTableB[i];
//             }
//         }
//         if (endpoint_bits <= remain_bits) {
//             block_data.cem_range = i;
//             break;
//         }
//     });
// }

// fn decode_endpoints_hdr7(endpoints: &mut [i32], v: &[i32]) {
//     let modeval = (v[2] >> 4 & 0x8) | (v[1] >> 5 & 0x4) | (v[0] >> 6);
//     let (major_component, mode) = {
//         if ((modeval & 0xc) != 0xc) {
//             (modeval >> 2, modeval & 3)
//         } else if (modeval != 0xf) {
//             (modeval & 3, 4)
//         } else {
//             (0, 5)
//         }
//     };
//     let mut c: [i32; 4] = [v[0] & 0x3f, v[1] & 0x1f, v[2] & 0x1f, v[3] & 0x1f];

//     match (mode) {
//         0 => {
//             c[3] |= v[3] & 0x60;
//             c[0] |= v[3] >> 1 & 0x40;
//             c[0] |= v[2] << 1 & 0x80;
//             c[0] |= v[1] << 3 & 0x300;
//             c[0] |= v[2] << 5 & 0x400;
//             c[0] <<= 1;
//             c[1] <<= 1;
//             c[2] <<= 1;
//             c[3] <<= 1;
//             },
//         1 => {
//             c[1] |= v[1] & 0x20;
//             c[2] |= v[2] & 0x20;
//             c[0] |= v[3] >> 1 & 0x40;
//             c[0] |= v[2] << 1 & 0x80;
//             c[0] |= v[1] << 2 & 0x100;
//             c[0] |= v[3] << 4 & 0x600;
//             c[0] <<= 1;
//             c[1] <<= 1;
//             c[2] <<= 1;
//             c[3] <<= 1;
//             },
//         2 => {
//             c[3] |= v[3] & 0xe0;
//             c[0] |= v[2] << 1 & 0xc0;
//             c[0] |= v[1] << 3 & 0x300;
//             c[0] <<= 2;
//             c[1] <<= 2;
//             c[2] <<= 2;
//             c[3] <<= 2;
//             },
//         3 => {
//             c[1] |= v[1] & 0x20;
//             c[2] |= v[2] & 0x20;
//             c[3] |= v[3] & 0x60;
//             c[0] |= v[3] >> 1 & 0x40;
//             c[0] |= v[2] << 1 & 0x80;
//             c[0] |= v[1] << 2 & 0x100;
//             c[0] <<= 3;
//             c[1] <<= 3;
//             c[2] <<= 3;
//             c[3] <<= 3;
//             },
//         4 => {
//             c[1] |= v[1] & 0x60;
//             c[2] |= v[2] & 0x60;
//             c[3] |= v[3] & 0x20;
//             c[0] |= v[3] >> 1 & 0x40;
//             c[0] |= v[3] << 1 & 0x80;
//             c[0] <<= 4;
//             c[1] <<= 4;
//             c[2] <<= 4;
//             c[3] <<= 4;
//             },
//         5 => {
//             c[1] |= v[1] & 0x60;
//             c[2] |= v[2] & 0x60;
//             c[3] |= v[3] & 0x60;
//             c[0] |= v[3] >> 1 & 0x40;
//             c[0] <<= 5;
//             c[1] <<= 5;
//             c[2] <<= 5;
//             c[3] <<= 5;
//             },
//         _ => {}
//     }
//     if (mode != 5) {
//         c[1] = c[0] - c[1];
//         c[2] = c[0] - c[2];
//     }
//     if (major_component == 1){
//         set_endpoint_hdr_clamp(endpoints, c[1] - c[3], c[0] - c[3], c[2] - c[3], 0x780, c[1], c[0], c[2], 0x780);
//     }
//     else if (major_component == 2) {
//         set_endpoint_hdr_clamp(endpoints, c[2] - c[3], c[1] - c[3], c[0] - c[3], 0x780, c[2], c[1], c[0], 0x780);
//     }
//     else {
//         set_endpoint_hdr_clamp(endpoints, c[0] - c[3], c[1] - c[3], c[2] - c[3], 0x780, c[0], c[1], c[2], 0x780);
//     }
// }

// fn decode_endpoints_hdr11(endpoints: &mut[i32], v: &[i32], alpha1: i32, alpha2: i32) {
//     let major_component = (v[4] >> 7) | (v[5] >> 6 & 2);
//     if (major_component == 3) {
//         set_endpoint_hdr(endpoints, v[0] << 4, v[2] << 4, v[4] << 5 & 0xfe0, alpha1, v[1] << 4, v[3] << 4,
//                          v[5] << 5 & 0xfe0, alpha2);
//         return;
//     }
//     let mode = (v[1] >> 7) | (v[2] >> 6 & 2) | (v[3] >> 5 & 4);
//     let va = v[0] | (v[1] << 2 & 0x100);
//     let vb0 = v[2] & 0x3f;
//     let vb1 = v[3] & 0x3f;
//     let vc = v[1] & 0x3f;
//     let vd0: i16;
//     let vd1: i16;

//     match (mode) {
//         0|2 => {
//             vd0 = v[4] & 0x7f;
//             if (vd0 & 0x40){
//                 vd0 |= 0xff80;
//             }
//             vd1 = v[5] & 0x7f;
//             if (vd1 & 0x40){
//                 vd1 |= 0xff80;
//             }
//         },
//         1|3|5|7 => {
//             vd0 = v[4] & 0x3f;
//             if (vd0 & 0x20){
//                 vd0 |= 0xffc0;
//             }
//             vd1 = v[5] & 0x3f;
//             if (vd1 & 0x20){
//                 vd1 |= 0xffc0;
//             }
//         },
//         _ => {
//             vd0 = v[4] & 0x1f;
//             if (vd0 & 0x10){
//                 vd0 |= 0xffe0;
//             }
//             vd1 = v[5] & 0x1f;
//             if (vd1 & 0x10){
//                 vd1 |= 0xffe0;
//             }
//         }
//     }

//     match (mode) {
//         0 => {
//             vb0 |= v[2] & 0x40;
//             vb1 |= v[3] & 0x40;
//             },
//         1 => {
//             vb0 |= v[2] & 0x40;
//             vb1 |= v[3] & 0x40;
//             vb0 |= v[4] << 1 & 0x80;
//             vb1 |= v[5] << 1 & 0x80;
//             },
//         2 => {
//             va |= v[2] << 3 & 0x200;
//             vc |= v[3] & 0x40;
//             },
//         3 => {
//             va |= v[4] << 3 & 0x200;
//             vc |= v[5] & 0x40;
//             vb0 |= v[2] & 0x40;
//             vb1 |= v[3] & 0x40;
//             },
//         4 => {
//             va |= v[4] << 4 & 0x200;
//             va |= v[5] << 5 & 0x400;
//             vb0 |= v[2] & 0x40;
//             vb1 |= v[3] & 0x40;
//             vb0 |= v[4] << 1 & 0x80;
//             vb1 |= v[5] << 1 & 0x80;
//             },
//         5 => {
//             va |= v[2] << 3 & 0x200;
//             va |= v[3] << 4 & 0x400;
//             vc |= v[5] & 0x40;
//             vc |= v[4] << 1 & 0x80;
//             },
//         6 => {
//             va |= v[4] << 4 & 0x200;
//             va |= v[5] << 5 & 0x400;
//             va |= v[4] << 5 & 0x800;
//             vc |= v[5] & 0x40;
//             vb0 |= v[2] & 0x40;
//             vb1 |= v[3] & 0x40;
//             },
//         7 => {
//             va |= v[2] << 3 & 0x200;
//             va |= v[3] << 4 & 0x400;
//             va |= v[4] << 5 & 0x800;
//             vc |= v[5] & 0x40;
//             },
//         _ => {}
//     }

//     let shamt = (mode >> 1) ^ 3;
//     va <<= shamt;
//     vb0 <<= shamt;
//     vb1 <<= shamt;
//     vc <<= shamt;
//     let mult = 1 << shamt;
//     vd0 *= mult;
//     vd1 *= mult;

//     if (major_component == 1){
//         set_endpoint_hdr_clamp(endpoints, va - vb0 - vc - vd0, va - vc, va - vb1 - vc - vd1, alpha1, va - vb0, va,
//                                va - vb1, alpha2);
//         }
//     else if (major_component == 2){
//         set_endpoint_hdr_clamp(endpoints, va - vb1 - vc - vd1, va - vb0 - vc - vd0, va - vc, alpha1, va - vb1, va - vb0,
//                                va, alpha2);
//         }
//     else {
//         set_endpoint_hdr_clamp(endpoints, va - vc, va - vb0 - vc - vd0, va - vb1 - vc - vd1, alpha1, va, va - vb0,
//                                va - vb1, alpha2);
//         }
// }

// fn decode_endpoints(buf: &[u8], data: &mut[BlockData]) {
//     static TritsTable: [usize; 6] = [0, 204, 93, 44, 22, 11, 5];
//     static QuintsTable: [usize; 6] = [0, 113, 54, 26, 13, 6];
//     let mut seq: [IntSeqData; 32] = [IntSeqData::default(); 32];
//     let mut ev: [i32; 32] = [0; 32];
//     decode_intseq(buf, if data.part_num == 1 {17} else {29}, CemTableA[data.cem_range], CemTableB[data.cem_range],
//                   data.endpoint_value_num, 0, seq);

//     match (CemTableA[data.cem_range]) {
//         3 => {
//             let mut b;
//             let c = TritsTable[CemTableB[data.cem_range]];
//             (0..data.endpoint_value_num).for_each(|i|{
//                 let a = (seq[i].bits & 1) * 0x1ff;
//                 let x = seq[i].bits >> 1;
//                 match (CemTableB[data.cem_range]) {
//                     1 => {
//                         b = 0;
//                         },
//                     2 => {
//                         b = 0b100010110 * x;
//                         },
//                     3 => {
//                         b = x << 7 | x << 2 | x;
//                         },
//                     4 => {
//                         b = x << 6 | x;
//                         },
//                     5 => {
//                         b = x << 5 | x >> 2;
//                         },
//                     6 => {
//                         b = x << 4 | x >> 4;
//                         },
//                     _ => {}
//                 }
//                 ev[i] = (a & 0x80) | ((seq[i].nonbits * c + b) ^ a) >> 2;
//             });
//         },
//         5 => {
//             let mut b;
//             let c = TritsTable[CemTableB[data.cem_range]];
//             (0..data.endpoint_value_num).for_each(|i|{
//                 let a = (seq[i].bits & 1) * 0x1ff;
//                 let x = seq[i].bits >> 1;
//                 match (CemTableB[data.cem_range]) {
//                 1 => {
//                     b = 0;
//                     },
//                 2 => {
//                     b = 0b100001100 * x;
//                     },
//                 3 => {
//                     b = x << 7 | x << 1 | x >> 1;
//                     },
//                 4 => {
//                     b = x << 6 | x >> 1;
//                     },
//                 5 => {
//                     b = x << 5 | x >> 3;
//                     },
//                 }
//                 ev[i] = (a & 0x80) | ((seq[i].nonbits * c + b) ^ a) >> 2;
//             });
//         },
//         _ => {
//             match (CemTableB[data.cem_range]) {
//                 1 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits * 0xff;
//                     });
//                 },
//                 2 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits * 0x55;
//                     });
//                 },
//                 3 => {
//                     (0..data.endpoint_value_num).for_each({
//                         ev[i] = seq[i].bits << 5 | seq[i].bits << 2 | seq[i].bits >> 1;
//                     });
//                 },
//                 4 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits << 4 | seq[i].bits;
//                     });
//                 },
//                 5 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits << 3 | seq[i].bits >> 2;
//                     });
//                 },
//                 6 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits << 2 | seq[i].bits >> 4;
//                     });
//                 },
//                 7 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits << 1 | seq[i].bits >> 6;
//                     });
//                 },
//                 8 => {
//                     (0..data.endpoint_value_num).for_each(|i|{
//                         ev[i] = seq[i].bits;
//                     });
//                 },
//                 _ => {}
//             }
//         }
//     }

//     let mut v = &ev;
//     for cem in 0..data.part_num{
//         match (data.cem[cem]) {
//             0 => {
//                 set_endpoint(data.endpoints[cem], v[0], v[0], v[0], 255, v[1], v[1], v[1], 255);
//                 },
//             1 => {
//                 let l0 = (v[0] >> 2) | (v[1] & 0xc0);
//                 let l1 = clamp(l0 + (v[1] & 0x3f));
//                 set_endpoint(data.endpoints[cem], l0, l0, l0, 255, l1, l1, l1, 255);
//             },
//             2 => {
//                 let y0;
//                 let y1;
//                 if (v[0] <= v[1]) {
//                     y0 = v[0] << 4;
//                     y1 = v[1] << 4;
//                 } else {
//                     y0 = (v[1] << 4) + 8;
//                     y1 = (v[0] << 4) - 8;
//                 }
//                 set_endpoint_hdr(data.endpoints[cem], y0, y0, y0, 0x780, y1, y1, y1, 0x780);
//             },
//             3 => {
//                 let y0;
//                 let d;
//                 if (v[0] & 0x80) {
//                     y0 = (v[1] & 0xe0) << 4 | (v[0] & 0x7f) << 2;
//                     d = (v[1] & 0x1f) << 2;
//                 } else {
//                     y0 = (v[1] & 0xf0) << 4 | (v[0] & 0x7f) << 1;
//                     d = (v[1] & 0x0f) << 1;
//                 }
//                 let y1 = clamp_hdr(y0 + d);
//                 set_endpoint_hdr(data.endpoints[cem], y0, y0, y0, 0x780, y1, y1, y1, 0x780);
//             },
//             4 => {
//                 set_endpoint(data.endpoints[cem], v[0], v[0], v[0], v[2], v[1], v[1], v[1], v[3]);
//                 },
//             5 => {
//                 bit_transfer_signed(&v[1], &v[0]);
//                 bit_transfer_signed(&v[3], &v[2]);
//                 v[1] += v[0];
//                 set_endpoint_clamp(data.endpoints[cem], v[0], v[0], v[0], v[2], v[1], v[1], v[1], v[2] + v[3]);
//                 },
//             6 => {
//                 set_endpoint(data.endpoints[cem], v[0] * v[3] >> 8, v[1] * v[3] >> 8, v[2] * v[3] >> 8, 255, v[0], v[1],
//                             v[2], 255);
//                 },
//             7 => {
//                 decode_endpoints_hdr7(data.endpoints[cem], v);
//                 },
//             8 => {
//                 if (v[0] + v[2] + v[4] <= v[1] + v[3] + v[5]){
//                     set_endpoint(data.endpoints[cem], v[0], v[2], v[4], 255, v[1], v[3], v[5], 255);
//                 }
//                 else {
//                     set_endpoint_blue(data.endpoints[cem], v[1], v[3], v[5], 255, v[0], v[2], v[4], 255);
//                 }
//             },
//             9 => {
//                 bit_transfer_signed(&v[1], &v[0]);
//                 bit_transfer_signed(&v[3], &v[2]);
//                 bit_transfer_signed(&v[5], &v[4]);
//                 if (v[1] + v[3] + v[5] >= 0){
//                     set_endpoint_clamp(data.endpoints[cem], v[0], v[2], v[4], 255, v[0] + v[1], v[2] + v[3], v[4] + v[5],
//                                     255);
//                     }
//                 else{
//                     set_endpoint_blue_clamp(data.endpoints[cem], v[0] + v[1], v[2] + v[3], v[4] + v[5], 255, v[0], v[2],
//                                             v[4], 255);
//                     }
//                 },
//             10 => {
//                 set_endpoint(data.endpoints[cem], v[0] * v[3] >> 8, v[1] * v[3] >> 8, v[2] * v[3] >> 8, v[4], v[0], v[1],
//                             v[2], v[5]);
//                 },
//             11 => {
//                 decode_endpoints_hdr11(data.endpoints[cem], v, 0x780, 0x780);
//                 },
//             12 => {
//                 if (v[0] + v[2] + v[4] <= v[1] + v[3] + v[5]){
//                     set_endpoint(data.endpoints[cem], v[0], v[2], v[4], v[6], v[1], v[3], v[5], v[7]);
//                 }
//                 else {
//                     set_endpoint_blue(data.endpoints[cem], v[1], v[3], v[5], v[7], v[0], v[2], v[4], v[6]);
//                 }
//             },
//             13 => {
//                 bit_transfer_signed(&v[1], &v[0]);
//                 bit_transfer_signed(&v[3], &v[2]);
//                 bit_transfer_signed(&v[5], &v[4]);
//                 bit_transfer_signed(&v[7], &v[6]);
//                 if (v[1] + v[3] + v[5] >= 0){
//                     set_endpoint_clamp(data.endpoints[cem], v[0], v[2], v[4], v[6], v[0] + v[1], v[2] + v[3], v[4] + v[5],
//                                     v[6] + v[7]);
//                     }
//                 else {
//                     set_endpoint_blue_clamp(data.endpoints[cem], v[0] + v[1], v[2] + v[3], v[4] + v[5], v[6] + v[7], v[0],
//                                             v[2], v[4], v[6]);
//                     }
//                 },
//             14 => {
//                 decode_endpoints_hdr11(data.endpoints[cem], v, v[6], v[7]);
//                 },
//             15 => {
//                 let mode = ((v[6] >> 7) & 1) | ((v[7] >> 6) & 2);
//                 v[6] &= 0x7f;
//                 v[7] &= 0x7f;
//                 if (mode == 3) {
//                     decode_endpoints_hdr11(data.endpoints[cem], v, v[6] << 5, v[7] << 5);
//                 } else {
//                     v[6] |= (v[7] << (mode + 1)) & 0x780;
//                     v[7] = ((v[7] & (0x3f >> mode)) ^ (0x20 >> mode)) - (0x20 >> mode);
//                     v[6] <<= 4 - mode;
//                     v[7] <<= 4 - mode;
//                     decode_endpoints_hdr11(data.endpoints[cem], v, v[6], clamp_hdr(v[6] + v[7]));
//                 }
//             },
//             _ => {panic!("Unsupported ASTC format");}
//         }
//         v = v[(data.cem[cem] / 4 + 1) * 2..];
//     }
// }

// fn decode_weights(buf: &[u8], data: &mut BlockData) {
//     IntSeqData seq[128];
//     int wv[128] = {};
//     decode_intseq(buf, 128, WeightPrecTableA[data.weight_range], WeightPrecTableB[data.weight_range],
//                   data.weight_num, 1, seq);

//     if (WeightPrecTableA[data.weight_range] == 0) {
//         match (WeightPrecTableB[data.weight_range]) {
//         1 => {
//             for (int i = 0; i < data.weight_num; i++)
//                 wv[i] = seq[i].bits ? 63 : 0;
//             },
//         2 => {
//             for (int i = 0; i < data.weight_num; i++)
//                 wv[i] = seq[i].bits << 4 | seq[i].bits << 2 | seq[i].bits;
//             },
//         3 => {
//             for (int i = 0; i < data.weight_num; i++)
//                 wv[i] = seq[i].bits << 3 | seq[i].bits;
//             },
//         4 => {
//             for (int i = 0; i < data.weight_num; i++)
//                 wv[i] = seq[i].bits << 2 | seq[i].bits >> 2;
//             },
//         5 => {
//             for (int i = 0; i < data.weight_num; i++)
//                 wv[i] = seq[i].bits << 1 | seq[i].bits >> 4;
//             },
//         }
//         for (int i = 0; i < data.weight_num; i++)
//             if (wv[i] > 32)
//                 ++wv[i];
//     } else if (WeightPrecTableB[data.weight_range] == 0) {
//         int s = WeightPrecTableA[data.weight_range] == 3 ? 32 : 16;
//         for (int i = 0; i < data.weight_num; i++)
//             wv[i] = seq[i].nonbits * s;
//     } else {
//         if (WeightPrecTableA[data.weight_range] == 3) {
//             match (WeightPrecTableB[data.weight_range]) {
//             1 => {
//                 for (int i = 0; i < data.weight_num; i++)
//                     wv[i] = seq[i].nonbits * 50;
//                 },
//             2 => {
//                 for (int i = 0; i < data.weight_num; i++) {
//                     wv[i] = seq[i].nonbits * 23;
//                     if (seq[i].bits & 2)
//                         wv[i] += 0b1000101;
//                 }
//                 },
//             3 => {
//                 for (int i = 0; i < data.weight_num; i++)
//                     wv[i] = seq[i].nonbits * 11 + ((seq[i].bits << 4 | seq[i].bits >> 1) & 0b1100011);
//                 },
//             }
//         } else if (WeightPrecTableA[data.weight_range] == 5) {
//             match (WeightPrecTableB[data.weight_range]) {
//             1 => {
//                 for (int i = 0; i < data.weight_num; i++)
//                     wv[i] = seq[i].nonbits * 28;
//                 },
//             2 => {
//                 for (int i = 0; i < data.weight_num; i++) {
//                     wv[i] = seq[i].nonbits * 13;
//                     if (seq[i].bits & 2)
//                         wv[i] += 0b1000010;
//                 }
//                 },
//             }
//         }
//         for (int i = 0; i < data.weight_num; i++) {
//             int a = (seq[i].bits & 1) * 0x7f;
//             wv[i] = (a & 0x20) | ((wv[i] ^ a) >> 2);
//             if (wv[i] > 32)
//                 ++wv[i];
//         }
//     }

//     int ds = (1024 + data.bw / 2) / (data.bw - 1);
//     int dt = (1024 + data.bh / 2) / (data.bh - 1);
//     int pn = data.dual_plane ? 2 : 1;

//     for (int t = 0, i = 0; t < data.bh; t++) {
//         for (int s = 0; s < data.bw; s++, i++) {
//             int gs = (ds * s * (data.width - 1) + 32) >> 6;
//             int gt = (dt * t * (data.height - 1) + 32) >> 6;
//             int fs = gs & 0xf;
//             int ft = gt & 0xf;
//             int v = (gs >> 4) + (gt >> 4) * data.width;
//             int w11 = (fs * ft + 8) >> 4;
//             int w10 = ft - w11;
//             int w01 = fs - w11;
//             int w00 = 16 - fs - ft + w11;

//             for (int p = 0; p < pn; p++) {
//                 int p00 = wv[v * pn + p];
//                 int p01 = wv[(v + 1) * pn + p];
//                 int p10 = wv[(v + data.width) * pn + p];
//                 int p11 = wv[(v + data.width + 1) * pn + p];
//                 data.weights[i][p] = (p00 * w00 + p01 * w01 + p10 * w10 + p11 * w11 + 8) >> 4;
//             }
//         }
//     }
// }

// fn select_partition(buf: &[u8], data: &mut BlockData) {
//     int small_block = data.bw * data.bh < 31;
//     int seed = (*(int *)buf >> 13 & 0x3ff) | (data.part_num - 1) << 10;

//     uint32_t rnum = seed;
//     rnum ^= rnum >> 15;
//     rnum -= rnum << 17;
//     rnum += rnum << 7;
//     rnum += rnum << 4;
//     rnum ^= rnum >> 5;
//     rnum += rnum << 16;
//     rnum ^= rnum >> 7;
//     rnum ^= rnum >> 3;
//     rnum ^= rnum << 6;
//     rnum ^= rnum >> 17;

//     int seeds[8];
//     for (int i = 0; i < 8; i++) {
//         seeds[i] = (rnum >> (i * 4)) & 0xF;
//         seeds[i] *= seeds[i];
//     }

//     int sh[2] = {seed & 2 ? 4 : 5, data.part_num == 3 ? 6 : 5};

//     if (seed & 1)
//         for (int i = 0; i < 8; i++)
//             seeds[i] >>= sh[i % 2];
//     else
//         for (int i = 0; i < 8; i++)
//             seeds[i] >>= sh[1 - i % 2];

//     if (small_block) {
//         for (int t = 0, i = 0; t < data.bh; t++) {
//             for (int s = 0; s < data.bw; s++, i++) {
//                 int x = s << 1;
//                 int y = t << 1;
//                 int a = (seeds[0] * x + seeds[1] * y + (rnum >> 14)) & 0x3f;
//                 int b = (seeds[2] * x + seeds[3] * y + (rnum >> 10)) & 0x3f;
//                 int c = data.part_num < 3 ? 0 : (seeds[4] * x + seeds[5] * y + (rnum >> 6)) & 0x3f;
//                 int d = data.part_num < 4 ? 0 : (seeds[6] * x + seeds[7] * y + (rnum >> 2)) & 0x3f;
//                 data.partition[i] = (a >= b && a >= c && a >= d) ? 0 : (b >= c && b >= d) ? 1 : (c >= d) ? 2 : 3;
//             }
//         }
//     } else {
//         for (int y = 0, i = 0; y < data.bh; y++) {
//             for (int x = 0; x < data.bw; x++, i++) {
//                 int a = (seeds[0] * x + seeds[1] * y + (rnum >> 14)) & 0x3f;
//                 int b = (seeds[2] * x + seeds[3] * y + (rnum >> 10)) & 0x3f;
//                 int c = data.part_num < 3 ? 0 : (seeds[4] * x + seeds[5] * y + (rnum >> 6)) & 0x3f;
//                 int d = data.part_num < 4 ? 0 : (seeds[6] * x + seeds[7] * y + (rnum >> 2)) & 0x3f;
//                 data.partition[i] = (a >= b && a >= c && a >= d) ? 0 : (b >= c && b >= d) ? 1 : (c >= d) ? 2 : 3;
//             }
//         }
//     }
// }

// fn applicate_color(const data: &mut BlockData, uint32_t *outbuf) {
//     static const t_select_folor_func_ptr FuncTableC[] = {
//       select_color, select_color,     select_color_hdr, select_color_hdr, select_color, select_color,
//       select_color, select_color_hdr, select_color,     select_color,     select_color, select_color_hdr,
//       select_color, select_color,     select_color_hdr, select_color_hdr};
//     static const t_select_folor_func_ptr FuncTableA[] = {
//       select_color, select_color,     select_color_hdr, select_color_hdr, select_color, select_color,
//       select_color, select_color_hdr, select_color,     select_color,     select_color, select_color_hdr,
//       select_color, select_color,     select_color,     select_color_hdr};
//     if (data.dual_plane) {
//         int ps[] = {0, 0, 0, 0};
//         ps[data.plane_selector] = 1;
//         if (data.part_num > 1) {
//             for (int i = 0; i < data.bw * data.bh; i++) {
//                 int p = data.partition[i];
//                 uint_fast8_t r =
//                   FuncTableC[data.cem[p]](data.endpoints[p][0], data.endpoints[p][4], data.weights[i][ps[0]]);
//                 uint_fast8_t g =
//                   FuncTableC[data.cem[p]](data.endpoints[p][1], data.endpoints[p][5], data.weights[i][ps[1]]);
//                 uint_fast8_t b =
//                   FuncTableC[data.cem[p]](data.endpoints[p][2], data.endpoints[p][6], data.weights[i][ps[2]]);
//                 uint_fast8_t a =
//                   FuncTableA[data.cem[p]](data.endpoints[p][3], data.endpoints[p][7], data.weights[i][ps[3]]);
//                 outbuf[i] = color(r, g, b, a);
//             }
//         } else {
//             for (int i = 0; i < data.bw * data.bh; i++) {
//                 uint_fast8_t r =
//                   FuncTableC[data.cem[0]](data.endpoints[0][0], data.endpoints[0][4], data.weights[i][ps[0]]);
//                 uint_fast8_t g =
//                   FuncTableC[data.cem[0]](data.endpoints[0][1], data.endpoints[0][5], data.weights[i][ps[1]]);
//                 uint_fast8_t b =
//                   FuncTableC[data.cem[0]](data.endpoints[0][2], data.endpoints[0][6], data.weights[i][ps[2]]);
//                 uint_fast8_t a =
//                   FuncTableA[data.cem[0]](data.endpoints[0][3], data.endpoints[0][7], data.weights[i][ps[3]]);
//                 outbuf[i] = color(r, g, b, a);
//             }
//         }
//     } else if (data.part_num > 1) {
//         for (int i = 0; i < data.bw * data.bh; i++) {
//             int p = data.partition[i];
//             uint_fast8_t r =
//               FuncTableC[data.cem[p]](data.endpoints[p][0], data.endpoints[p][4], data.weights[i][0]);
//             uint_fast8_t g =
//               FuncTableC[data.cem[p]](data.endpoints[p][1], data.endpoints[p][5], data.weights[i][0]);
//             uint_fast8_t b =
//               FuncTableC[data.cem[p]](data.endpoints[p][2], data.endpoints[p][6], data.weights[i][0]);
//             uint_fast8_t a =
//               FuncTableA[data.cem[p]](data.endpoints[p][3], data.endpoints[p][7], data.weights[i][0]);
//             outbuf[i] = color(r, g, b, a);
//         }
//     } else {
//         for (int i = 0; i < data.bw * data.bh; i++) {
//             uint_fast8_t r =
//               FuncTableC[data.cem[0]](data.endpoints[0][0], data.endpoints[0][4], data.weights[i][0]);
//             uint_fast8_t g =
//               FuncTableC[data.cem[0]](data.endpoints[0][1], data.endpoints[0][5], data.weights[i][0]);
//             uint_fast8_t b =
//               FuncTableC[data.cem[0]](data.endpoints[0][2], data.endpoints[0][6], data.weights[i][0]);
//             uint_fast8_t a =
//               FuncTableA[data.cem[0]](data.endpoints[0][3], data.endpoints[0][7], data.weights[i][0]);
//             outbuf[i] = color(r, g, b, a);
//         }
//     }
// }

// fn decode_block(buf: &[u8], const int bw, const int bh, uint32_t *outbuf) {
//     if (buf[0] == 0xfc && (buf[1] & 1) == 1) {
//         uint_fast32_t c;
//         if (buf[1] & 2)
//             c = color(f16ptr_to_u8(buf + 8), f16ptr_to_u8(buf + 10), f16ptr_to_u8(buf + 12), f16ptr_to_u8(buf + 14));
//         else
//             c = color(buf[9], buf[11], buf[13], buf[15]);
//         for (int i = 0; i < bw * bh; i++)
//             outbuf[i] = c;
//     } else if (((buf[0] & 0xc3) == 0xc0 && (buf[1] & 1) == 1) || (buf[0] & 0xf) == 0) {
//         uint_fast32_t c = color(255, 0, 255, 255);
//         for (int i = 0; i < bw * bh; i++)
//             outbuf[i] = c;
//     } else {
//         BlockData block_data;
//         block_data.bw = bw;
//         block_data.bh = bh;
//         decode_block_params(buf, &block_data);
//         decode_endpoints(buf, &block_data);
//         decode_weights(buf, &block_data);
//         if (block_data.part_num > 1)
//             select_partition(buf, &block_data);
//         applicate_color(&block_data, outbuf);
//     }
// }

// int decode_astc(const uint8_t *data, const long w, const long h, const int bw, const int bh, uint32_t *image) {
//     const long num_blocks_x = (w + bw - 1) / bw;
//     const long num_blocks_y = (h + bh - 1) / bh;
//     uint32_t buffer[144];
//     const uint8_t *d = data;
//     for (long by = 0; by < num_blocks_y; by++) {
//         for (long bx = 0; bx < num_blocks_x; bx++, d += 16) {
//             decode_block(d, bw, bh, buffer);
//             copy_block_buffer(bx, by, w, h, bw, bh, buffer, image);
//         }
//     }
//     return 1;
// }
