# texture2ddecoder-rs

A wip pure Rust (no-std) texture decoder based on [AssetStudio's Texture2DDecoder](https://github.com/Perfare/AssetStudio/tree/master/Texture2DDecoder).

## Features

### alloc (optional, default)

- ~35% faster pvrtc decoding

## Roadmap

- implementing & testing all formats
- documentation
- replacing u32 color output with RGBA structure
- finding the original sources for the decoders
- supporting more than BGRA32 output
- adding additional formats

## Format Progress

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

## License
