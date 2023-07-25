# texture2ddecoder [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs]][docs.rs] [![License_MIT]][license_mit] [![License_APACHE]][license_apache] 

[Build Status]: https://img.shields.io/github/actions/workflow/status/UniversalGameExtraction/texture2ddecoder/ci.yml?branch=main
[actions]: https://github.com/UniversalGameExtraction/texture2ddecoder/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/texture2ddecoder.svg
[crates.io]: https://crates.io/crates/texture2ddecoder
[Docs]: https://docs.rs/texture2ddecoder/badge.svg
[docs.rs]: https://docs.rs/crate/texture2ddecoder/
[License_MIT]: https://img.shields.io/badge/License-MIT-yellow.svg
[license_mit]: https://raw.githubusercontent.com/UniversalGameExtraction/texture2ddecoder/main/LICENSE-MIT
[License_APACHE]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[license_apache]: https://raw.githubusercontent.com/UniversalGameExtraction/texture2ddecoder/main/LICENSE-APACHE

A pure Rust no-std texture decoder for the following formats:
 - [ATC - Adreno Texture Compression](https://registry.khronos.org/OpenGL/extensions/AMD/AMD_compressed_ATC_texture.txt) ([detailed paper](http://www.guildsoftware.com/papers/2012.Converting.DXTC.to.ATC.pdf))
 - [ASTC - Adaptive Scalable Texture Compression](https://en.wikipedia.org/wiki/Adaptive_Scalable_Texture_Compression)
 - [BCn - Block Compression](https://en.wikipedia.org/wiki/S3_Texture_Compression)
 - [ETC - Ericsson Texture Compression](https://en.wikipedia.org/wiki/Ericsson_Texture_Compression)
 - [PVRTC - PowerVR Texture Compression](https://en.wikipedia.org/wiki/PVRTC)
 - (WIP) [Crunch](https://github.com/BinomialLLC/crunch) & [Unity's Crunch](https://github.com/Unity-Technologies/crunch)

## Features

### alloc (optional, default)

- ~35% faster pvrtc decoding

## Functions
Provides a decode function for each format, as well as a block decode function all formats besides PVRTC.
Besides some exceptions, the signature of the decode functions is as follows:
```rust
    fn decode_format(data: &[u8], width: usize, height: usize, image: &mut [u32]) -> Result<(), &'static str>
    // data: the compressed data, expected to be width * height / block_size in size
    // width: the width of the image
    // height: the height of the image
    // image: the buffer to write the decoded image to, expected to be width * height in size
    fn decode_format_block(data: &[u8], outbuf: &mut [u32]) -> Result<(), &'static str>
    // data: the compressed data (block), expected to be block_size in size
    // outbuf: the buffer to write the decoded image to, expected to be block_size in size
```
The exceptions are:
- ASTC: the (block) decode function takes the block size as an additional parameter
- BC6: there are two additional decode functions for the signed and unsigned variants
- PVRTC: the decode function takes the block size as an additional parameter, and there are two additional decode functions for the 2bpp and 4bpp variants
To make these excetions easier to use, there are helper functions to enable decode functions with identical arguments and returns.
Here is a list of the formats and their corresponding functions:
- ATC
  - decode_atc_rgb4
  - decode_atc_rgb4_block
  - decode_atc_rgba8
  - decode_atc_rgba8_block
- ASTC
  - decode_astc
  - decode_astc_block
  - various decode_astc_(block_)_x_y functions, where x and y are the block size
- BCn
  - decode_bc1
  - decode_bc1_block
  - decode_bc3
  - decode_bc3_block
  - decode_bc4
  - decode_bc4_block
  - decode_bc5
  - decode_bc5_block
  - decode_bc6
  - decode_bc6_block
  - decode_bc6_signed
  - decode_bc6_block_signed
  - decode_bc6_unsigned
  - decode_bc6_block_unsigned
  - decode_bc7
  - decode_bc7_block
- ETC
  - decode_etc1
  - decode_etc1_block
  - decode_etc2_rgb
  - decode_etc2_rgb_block
  - decode_etc2_rgba1
  - decode_etc2_rgba1_block
  - decode_etc2_rgba8
  - decode_etc2_rgba8_block
  - decode_eacr
  - decode_eacr_block
  - decode_eacr_signed
  - decode_eacr_signed_block
  - decode_eacrg
  - decode_eacrg_block
- PVRTC
  - decode_pvrtc
  - decode_pvrtc_2bpp
  - decode_pvrtc_4bpp

## Roadmap
- implementing & testing all formats
- documentation
- replacing u32 color output with RGBA structure
- finding the original sources for the decoders
- supporting more than BGRA32 output
- adding additional formats

### Format Progress

- [x] ATC-RGB
- [x] ATC-RGBA
- [x] ASTC
- [x] BC1
- [x] BC3
- [x] BC4
- [x] BC5
- [x] BC6
- [x] BC7
- [x] EAC-R
- [x] EAC-RG
- [x] ETC1
- [x] ETC2
- [x] ETC2-A1
- [x] ETC2-A8
- [x] PVRTCI-2bpp
- [x] PVRTCI-4bpp
- [ ] Crunched (not implemented)
  - [ ] DXT1
  - [ ] DXT5
  - [ ] ETC1
  - [ ] ETC2-A8

## License & Credits

This crate itself is dual-licensed under MIT + Apache2.

The texture compression codecs themselves have following licenses:
| Codec          | License       | Source                                                                                                                                |
|----------------|---------------|---------------------------------------------------------------------------------------------------------------------------------------|
| ATC            | MIT           | [Perfare/AssetStudio - Texture2DDecoderNative/atc.cpp](https://github.com/Perfare/AssetStudio/tree/master/atc.cpp)                    |
| ASTC           | MIT\*         | [Ishotihadus/mikunyan - ext/decoders/native/astc.c](https://github.com/Ishotihadus/mikunyan/tree/master/ext/decoders/native/astc.c)   |
| BCn            | MIT\*         | [Perfare/AssetStudio - Texture2DDecoderNative/bcn.cpp](https://github.com/Perfare/AssetStudio/tree/master/bcn.cpp)                    |
| ETC            | MIT\*         | [Ishotihadus/mikunyan - ext/decoders/native/etc.c](https://github.com/Ishotihadus/mikunyan/tree/master/ext/decoders/native/etc.c)     |
| f16            | MIT           | [Maratyszcza/FP16](https://github.com/Maratyszcza/FP16)                                                                               |
| PVRTC          | MIT\*         | [Ishotihadus/mikunyan - ext/decoders/native/pvrtc.c](https://github.com/Ishotihadus/mikunyan/tree/master/ext/decoders/native/pvrtc.c) |
| Crunch         | PUBLIC DOMAIN | [BinomialLLC/crunch](https://github.com/BinomialLLC/crunch)                                                                           |
| Crunch (Unity) | ZLIB          | [Unity-Technologies/crunch](https://github.com/Unity-Technologies/crunch)                                                             |
\* in doubt if these are the original source and have not just taken/adopted the code from somewhere else