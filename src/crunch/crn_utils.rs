#![allow(unused_parens)]
#[inline]
pub fn is_power_of_2(x: usize) -> bool{
    if(x == 0){
        return false;
    }
    return ((x & (x - 1)) == 0);
}

#[inline]
pub fn next_pow2(mut val: usize) -> usize{
    val -= 1;
    val |= val >> 32;
    val |= val >> 16;
    val |= val >> 8;
    val |= val >> 4;
    val |= val >> 2;
    val |= val >> 1;
    return val + 1;
}

#[inline]
pub fn floor_log2i(mut v: u32) -> u32{
    let mut l: u32 = 0;
    while(v > 1){
        v >>= 1;
        l += 1;
    }
    return l;
}

#[inline]
pub fn ceil_log2i(v: u32) -> u32{
    let mut l: u32 = floor_log2i(v);
    if ((l != 32) && (v > (1 << l))){
        l += 1;
    }
    return l;
}

// Returns the total number of bits needed to encode v.
#[inline]
pub fn total_bits(mut v: u32) -> u32{
    let mut l: u32 = 0;
    while (v > 0){
        v >>= 1;
        l += 1;
    }
    return l;
}

#[inline]
pub fn limit(x: &mut u32, n: u32){
   let v: i32 = (*x as i32) - n as i32;
   let msk: i32 = (v >> 31);
   *x = (((*x as i32) & msk) | (v & !msk)) as u32;
}