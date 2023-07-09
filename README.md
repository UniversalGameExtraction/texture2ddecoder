# texture2ddecoder-rs

A wip pure Rust (no-std) texture decoder based on [AssetStudio's Texture2DDecoder](https://github.com/Perfare/AssetStudio/tree/master/Texture2DDecoder).

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
- [ ] ASTC (not implemented)
- [x] BC1
- [x] BC3
- [ ] BC4 (bad test result)
- [ ] BC5 (bad test result)
- [ ] BC6 (missing test)
- [ ] BC7 (missing test)
- [ ] EAC-R (probably fine, needs better test)
- [ ] EAC-RG (probably fine, needs better test)
- [x] ETC1 (works, but missing test)
- [x] ETC2
- [x] ETC2-A1
- [x] ETC2-A8
- [ ] PVRTCI-2bpp (bad test result)
- [x] PVRTCI-4bpp
- [ ] Crunched (not implemented)
  - [ ] DXT1
  - [ ] DXT5
  - [ ] ETC1
  - [ ] ETC2-A8

## License

