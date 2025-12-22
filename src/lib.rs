//! A pure Rust no-std texture decoder for the following formats:
//! - [ATC - Adreno Texture Compression](https://registry.khronos.org/OpenGL/extensions/AMD/AMD_compressed_ATC_texture.txt)
//! - [ASTC - Adaptive Scalable Texture Compression](https://en.wikipedia.org/wiki/Adaptive_Scalable_Texture_Compression)
//! - [BCn - Block Compression](https://en.wikipedia.org/wiki/S3_Texture_Compression)
//! - [ETC - Ericsson Texture Compression](https://en.wikipedia.org/wiki/Ericsson_Texture_Compression)
//! - [PVRTC - PowerVR Texture Compression](https://en.wikipedia.org/wiki/PVRTC)
//! - [Crunch](https://github.com/BinomialLLC/crunch) & [Unity's Crunch](https://github.com/Unity-Technologies/crunch)
//!
//! ## Functions
//! Provides a decode function for each format, as well as a block decode function all formats besides PVRTC.
//! Besides some exceptions, the signature of the decode functions is as follows:
//! ```ignore
//!     fn decode_format(data: &[u8], width: usize, height: usize, image: &mut [u32]) -> Result<(), &'static str>
//!     // data: the compressed data, expected to be width * height / block_size in size
//!     // width: the width of the image
//!     // height: the height of the image
//!     // image: the buffer to write the decoded image to, expected to be width * height in size
//!     fn decode_format_block(data: &[u8], image: &mut [u32]) -> Result<(), &'static str>
//!     // data: the compressed data (block), expected to be block_size in size
//!     // image: the buffer to write the decoded image to, expected to be block_size in size
//! ```
//! The exceptions are:
//! - ASTC: the (block) decode function takes the block size as an additional parameter
//! - BC6: there are two additional decode functions for the signed and unsigned variants
//! - PVRTC: the decode function takes the block size as an additional parameter, and there are two additional decode functions for the 2bpp and 4bpp variants
//! - Crunch & Unity's Crunch: The texture's dimensions and metadata are stored in the file itself, one's must parse the header with crnd_get_texture_info() from crn_texture_info struct first, then pass the metadata to the decoder as in the format. There's no block decomp. function.
//! 
//! To make these excetions easier to use, there are helper functions to enable decode functions with identical arguments and returns.
//! 
//! Here is a list of the formats and their corresponding functions:
//! - ATC
//!   - [`decode_atc_rgb4()`]
//!   - [`decode_atc_rgb4_block()`]
//!   - [`decode_atc_rgba8()`]
//!   - [`decode_atc_rgba8_block()`]
//! - ASTC
//!   - [`decode_astc()`]
//!   - [`decode_astc_block()`]
//!   - various decode_astc_(block_)_x_y functions, where x and y are the block size
//! - BCn
//!   - [`decode_bc1()`]
//!   - [`decode_bc1_block()`]
//!   - [`decode_bc3()`]
//!   - [`decode_bc3_block()`]
//!   - [`decode_bc4()`]
//!   - [`decode_bc4_block()`]
//!   - [`decode_bc5()`]
//!   - [`decode_bc5_block()`]
//!   - [`decode_bc6()`]
//!   - [`decode_bc6_block()`]
//!   - [`decode_bc6_signed()`]
//!   - [`decode_bc6_block_signed()`]
//!   - [`decode_bc6_unsigned()`]
//!   - [`decode_bc6_block_unsigned()`]
//!   - [`decode_bc7()`]
//!   - [`decode_bc7_block()`]
//! - ETC
//!   - [`decode_etc1()`]
//!   - [`decode_etc1_block()`]
//!   - [`decode_etc2_rgb()`]
//!   - [`decode_etc2_rgb_block()`]
//!   - [`decode_etc2_rgba1()`]
//!   - [`decode_etc2_rgba1_block()`]
//!   - [`decode_etc2_rgba8()`]
//!   - [`decode_etc2_rgba8_block()`]
//!   - [`decode_eacr()`]
//!   - [`decode_eacr_block()`]
//!   - [`decode_eacr_signed()`]
//!   - [`decode_eacr_signed_block()`]
//!   - [`decode_eacrg()`]
//!   - [`decode_eacrg_block()`]
//! - PVRTC
//!   - [`decode_pvrtc()`]
//!   - [`decode_pvrtc_2bpp()`]
//!   - [`decode_pvrtc_4bpp()`]
//! - Crunch
//!   - [`decode_crunch()`]
//! - Unity Crunch
//!   - [`decode_unity_crunch()`]
//!
#![no_std]

#[cfg(not(any(feature = "astc", feature = "atc", feature = "bc1", feature = "bc2", feature = "bc3", feature = "bc4", feature = "bc5", feature = "bc6", feature = "bc7", feature = "crunch", feature = "etc", feature = "pvrtc")))]
compile_error!("at least one of the texture formats must be enabled");

mod bitreader;
mod color;
#[cfg(any(feature = "astc", feature = "bc6"))]
mod f16;
mod macros;

#[cfg(feature = "astc")]
mod astc;
#[cfg(feature = "atc")]
mod atc;
#[cfg(any(feature = "bc1", feature = "bc2", feature = "bc3", feature = "bc4", feature = "bc5", feature = "bc6", feature = "bc7"))]
mod bcn;
#[cfg(feature = "crunch")]
mod crnlib;
#[cfg(all(feature = "crunch", feature = "alloc"))]
mod crunch;
#[cfg(feature = "etc")]
mod etc;
#[cfg(feature = "pvrtc")]
mod pvrtc;
#[cfg(all(feature = "crunch", feature = "alloc"))]
mod unitycrunch;

// import decode functions
#[cfg(feature = "astc")]
pub use astc::*;
#[cfg(feature = "atc")]
pub use atc::*;
#[cfg(any(feature = "bc1", feature = "bc2", feature = "bc3", feature = "bc4", feature = "bc5", feature = "bc6", feature = "bc7"))]
pub use bcn::*;
#[cfg(feature = "crunch")]
pub use crnlib::CrnTextureInfo;
#[cfg(feature = "crunch")]
pub use crunch::decode_crunch;
#[cfg(feature = "etc")]
pub use etc::*;
#[cfg(feature = "pvrtc")]
pub use pvrtc::*;
#[cfg(feature = "crunch")]
pub use unitycrunch::decode_unity_crunch;
