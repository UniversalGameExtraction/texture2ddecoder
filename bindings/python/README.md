# texture2ddecoder_rs

Python bindings for texture2ddecoder.

# usage

This module provides a set of functions for decoding 2D textures. Each function takes a byte array representing the texture data, along with the width and height of the texture. The functions return a byte array representing the decoded texture data.

### ATC Decoding

- `decode_atc_rgb4(data: bytes, width: int, height: int) -> bytes`
- `decode_atc_rgba8(data: bytes, width: int, height: int) -> bytes`

### ASTC Decoding

- `decode_astc_4_4(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_5_4(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_5_5(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_6_5(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_6_6(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_8_5(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_8_6(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_8_8(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_10_5(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_10_6(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_10_8(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_10_10(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_12_10(data: bytes, width: int, height: int) -> bytes`
- `decode_astc_12_12(data: bytes, width: int, height: int) -> bytes`
- `decode_astc(data: bytes, width: int, height: int, block_width: int, block_height: int) -> bytes`

### BCN Decoding

- `decode_bc1(data: bytes, width: int, height: int) -> bytes`
- `decode_bc3(data: bytes, width: int, height: int) -> bytes`
- `decode_bc4(data: bytes, width: int, height: int) -> bytes`
- `decode_bc5(data: bytes, width: int, height: int) -> bytes`
- `decode_bc6_signed(data: bytes, width: int, height: int) -> bytes`
- `decode_bc6_unsigned(data: bytes, width: int, height: int) -> bytes`
- `decode_bc7(data: bytes, width: int, height: int) -> bytes`

### ETC Decoding

- `decode_etc1(data: bytes, width: int, height: int) -> bytes`
- `decode_etc2_rgb(data: bytes, width: int, height: int) -> bytes`
- `decode_etc2_rgba1(data: bytes, width: int, height: int) -> bytes`
- `decode_etc2_rgba8(data: bytes, width: int, height: int) -> bytes`
- `decode_eacr(data: bytes, width: int, height: int) -> bytes`
- `decode_eacr_signed(data: bytes, width: int, height: int) -> bytes`
- `decode_eacrg(data: bytes, width: int, height: int) -> bytes`
- `decode_eacrg_signed(data: bytes, width: int, height: int) -> bytes`

### PVRTC Decoding

- `decode_pvrtc_2bpp(data: bytes, width: int, height: int) -> bytes`
- `decode_pvrtc_4bpp(data: bytes, width: int, height: int) -> bytes`

### Crunch Decoding

- `decode_crunch(data: bytes, width: int, height: int) -> bytes`
- `decode_unity_crunch(data: bytes, width: int, height: int) -> bytes`
