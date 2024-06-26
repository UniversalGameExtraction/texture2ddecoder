/*
    License Information

    FP16 library is derived from https://github.com/Maratyszcza/FP16.
    The library is licensed under the MIT License shown below.


    The MIT License (MIT)

    Copyright (c) 2017 Facebook Inc.
    Copyright (c) 2017 Georgia Institute of Technology
    Copyright 2019 Google LLC

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
    rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
    permit persons to whom the Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
    WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
    COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
    OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

// Helper functions for Rust for converting between IEEE single-precision and half-precision floating-point
#[inline]
const fn __builtin_clz(x: u32) -> u32 {
    let mut ret: u32 = 0;
    let mut x = x;
    let mut i = 0;
    // use loop instead of for so we can use const
    loop {
        if (x & 0x80000000 != 0) | (i == 32) {
            break;
        }
        ret += 1;
        x <<= 1;
        i += 1;
    }
    ret
}

#[inline]
fn fp32_from_bits(x: u32) -> f32 {
    f32::from_le_bytes(u32::to_le_bytes(x))
}

#[inline]
fn fp32_to_bits(x: f32) -> u32 {
    u32::from_le_bytes(f32::to_le_bytes(x))
}

/// Convert a 16-bit floating-point number in IEEE half-precision format, in bit representation, to
/// a 32-bit floating-point number in IEEE single-precision format.
/// The implementation relies on IEEE-like (no assumption about rounding mode and no operations on denormals)
/// floating-point operations and bitcasts between integer and floating-point variables.
#[inline]
pub fn fp16_ieee_to_fp32_value(h: u16) -> f32 {
    // Extend the half-precision floating-point number to 32 bits and shift to the upper part of the 32-bit word:
    //      +---+-----+------------+-------------------+
    //      | S |EEEEE|MM MMMM MMMM|0000 0000 0000 0000|
    //      +---+-----+------------+-------------------+
    // Bits  31  26-30    16-25            0-15
    //
    // S - sign bit, E - bits of the biased exponent, M - bits of the mantissa, 0 - zero bits.

    let w: u32 = (h as u32) << 16;

    // Extract the sign of the input number into the high bit of the 32-bit word:
    //
    //      +---+----------------------------------+
    //      | S |0000000 00000000 00000000 00000000|
    //      +---+----------------------------------+
    // Bits  31                 0-31

    let sign: u32 = w & 0x80000000;

    // Extract mantissa and biased exponent of the input number into the high bits of the 32-bit word:
    //
    //      +-----+------------+---------------------+
    //      |EEEEE|MM MMMM MMMM|0 0000 0000 0000 0000|
    //      +-----+------------+---------------------+
    // Bits  27-31    17-26            0-16

    let two_w: u32 = w.overflowing_add(w).0;

    // Shift mantissa and exponent into bits 23-28 and bits 13-22 so they become mantissa and exponent
    // of a single-precision floating-point number:
    //
    //       S|Exponent |          Mantissa
    //      +-+---+-----+------------+----------------+
    //      |0|000|EEEEE|MM MMMM MMMM|0 0000 0000 0000|
    //      +-+---+-----+------------+----------------+
    // Bits   | 23-31   |           0-22
    //
    // Next, there are some adjustments to the exponent:
    // - The exponent needs to be corrected by the difference in exponent bias between single-precision and half-precision
    //   formats (0x7F - 0xF = 0x70)
    // - Inf and NaN values in the inputs should become Inf and NaN values after conversion to the single-precision number.
    //   Therefore, if the biased exponent of the half-precision input was 0x1F (max possible value), the biased exponent
    //   of the single-precision output must be 0xFF (max possible value). We do this correction in two steps:
    //   - First, we adjust the exponent by (0xFF - 0x1F) = 0xE0 (see exp_offset below) rather than by 0x70 suggested
    //     by the difference in the exponent bias (see above).
    //   - Then we multiply the single-precision result of exponent adjustment by 2**(-112) to reverse the effect of
    //     exponent adjustment by 0xE0 less the necessary exponent adjustment by 0x70 due to difference in exponent bias.
    //     The floating-point multiplication hardware would ensure than Inf and NaN would retain their value on at least
    //     partially IEEE754-compliant implementations.
    //
    // Note that the above operations do not handle denormal inputs (where biased exponent == 0). However, they also do not
    // operate on denormal inputs, and do not produce denormal results.

    let exp_offset: u32 = 0xE0 << 23;
    // #if defined(__STDC_VERSION__) && (__STDC_VERSION__ >= 199901L) || defined(__GNUC__) && !defined(__STRICT_ANSI__)
    // 	let exp_scale: f32 = 0x1.0p-112f;
    // #else
    let exp_scale: f32 = fp32_from_bits(0x7800000);
    // #endif
    let normalized_value: f32 = fp32_from_bits((two_w >> 4) + exp_offset) * exp_scale;

    // Convert denormalized half-precision inputs into single-precision results (always normalized).
    // Zero inputs are also handled here.
    //
    // In a denormalized number the biased exponent is zero, and mantissa has on-zero bits.
    // First, we shift mantissa into bits 0-9 of the 32-bit word.
    //
    //                  zeros           |  mantissa
    //      +---------------------------+------------+
    //      |0000 0000 0000 0000 0000 00|MM MMMM MMMM|
    //      +---------------------------+------------+
    // Bits             10-31                0-9
    //
    // Now, remember that denormalized half-precision numbers are represented as:
    //    FP16 = mantissa * 2**(-24).
    // The trick is to construct a normalized single-precision number with the same mantissa and thehalf-precision input
    // and with an exponent which would scale the corresponding mantissa bits to 2**(-24).
    // A normalized single-precision floating-point number is represented as:
    //    FP32 = (1 + mantissa * 2**(-23)) * 2**(exponent - 127)
    // Therefore, when the biased exponent is 126, a unit change in the mantissa of the input denormalized half-precision
    // number causes a change of the constructud single-precision number by 2**(-24), i.e. the same ammount.
    //
    // The last step is to adjust the bias of the constructed single-precision number. When the input half-precision number
    // is zero, the constructed single-precision number has the value of
    //    FP32 = 1 * 2**(126 - 127) = 2**(-1) = 0.5
    // Therefore, we need to subtract 0.5 from the constructed single-precision number to get the numerical equivalent of
    // the input half-precision number.

    let magic_mask: u32 = 126 << 23;
    let magic_bias: f32 = 0.5;
    let denormalized_value: f32 = fp32_from_bits((two_w >> 17) | magic_mask) - magic_bias;

    // - Choose either results of conversion of input as a normalized number, or as a denormalized number, depending on the
    //   input exponent. The variable two_w contains input exponent in bits 27-31, therefore if its smaller than 2**27, the
    //   input is either a denormal number, or zero.
    // - Combine the result of conversion of exponent and mantissa with the sign of the input number.

    let denormalized_cutoff: u32 = 1 << 27;
    let result: u32 = sign
        | (if two_w < denormalized_cutoff {
            fp32_to_bits(denormalized_value)
        } else {
            fp32_to_bits(normalized_value)
        });
    fp32_from_bits(result)
}
