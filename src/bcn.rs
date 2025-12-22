use crate::macros::block_decoder;

#[cfg(feature = "bc1")]
pub(crate) mod bc1;
#[cfg(feature = "bc2")]
pub(crate) mod bc2;
#[cfg(feature = "bc3")]
pub(crate) mod bc3;
#[cfg(feature = "bc4")]
pub(crate) mod bc4;
#[cfg(feature = "bc5")]
pub(crate) mod bc5;
#[cfg(feature = "bc6")]
pub(crate) mod bc6;
#[cfg(feature = "bc7")]
pub(crate) mod bc7;
pub(crate) mod consts;

#[cfg(feature = "bc1")]
pub use bc1::decode_bc1_block;
#[cfg(feature = "bc1")]
pub use bc1::decode_bc1a_block;
#[cfg(feature = "bc2")]
pub use bc2::decode_bc2_block;
#[cfg(feature = "bc3")]
pub use bc3::decode_bc3_block;
#[cfg(feature = "bc4")]
pub use bc4::decode_bc4_block;
#[cfg(feature = "bc5")]
pub use bc5::decode_bc5_block;
#[cfg(feature = "bc6")]
pub use bc6::{decode_bc6_block, decode_bc6_block_signed, decode_bc6_block_unsigned};
#[cfg(feature = "bc7")]
pub use bc7::decode_bc7_block;

#[cfg(feature = "bc1")]
block_decoder!("bc1", 4, 4, 8, decode_bc1_block);
#[cfg(feature = "bc1")]
block_decoder!("bc1a", 4, 4, 8, decode_bc1a_block);
#[cfg(feature = "bc2")]
block_decoder!("bc2", 4, 4, 16, decode_bc2_block);
#[cfg(feature = "bc3")]
block_decoder!("bc3", 4, 4, 16, decode_bc3_block);
#[cfg(feature = "bc4")]
block_decoder!("bc4", 4, 4, 8, decode_bc4_block);
#[cfg(feature = "bc5")]
block_decoder!("bc5", 4, 4, 16, decode_bc5_block);
#[cfg(feature = "bc6")]
block_decoder!("bc6_signed", 4, 4, 16, decode_bc6_block_signed);
#[cfg(feature = "bc6")]
block_decoder!("bc6_unsigned", 4, 4, 16, decode_bc6_block_unsigned);
#[cfg(feature = "bc7")]
block_decoder!("bc7", 4, 4, 16, decode_bc7_block);

#[cfg(feature = "bc6")]
pub fn decode_bc6(
    data: &[u8],
    width: usize,
    height: usize,
    image: &mut [u32],
    signed: bool,
) -> Result<(), &'static str> {
    match signed {
        true => decode_bc6_signed(data, width, height, image),
        false => decode_bc6_unsigned(data, width, height, image),
    }
}
