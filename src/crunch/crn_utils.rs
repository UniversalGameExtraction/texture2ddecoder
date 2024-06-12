#[inline]
pub fn ceil_log2i(v: u32) -> u32 {
    let mut l: u32 = v.ilog2();
    if (l != 32) && (v > (1 << l)) {
        l += 1;
    }
    l
}

// Returns the total number of bits needed to encode v.
#[inline]
pub fn total_bits(mut v: u32) -> u32 {
    let mut l: u32 = 0;
    while v > 0 {
        v >>= 1;
        l += 1;
    }
    l
}

#[inline]
pub fn limit(x: &mut u32, n: u32) {
    let v: i32 = (*x as i32) - n as i32;
    let msk: i32 = v >> 31;
    *x = (((*x as i32) & msk) | (v & !msk)) as u32;
}
